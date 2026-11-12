//! Ethernet tiles: the queue register map and the E1 data mover's contract.
//!
//! Sources: `BlackholeA0/EthernetTile/{README,EthernetTxRx}.md` for the queues,
//! and tt-metal's `hw/inc/internal/ethernet/tt_eth_ss_regs.h` for the offsets
//! and the one-hot `ETH_TXQ_CMD_*` command bits.
//!
//! Only E1 is ours. E0 belongs to Tenstorrent's base firmware, which trains and
//! retrains the link (`BabyRISCV/README.md`).

/// The E1 image's mailbox, laid out as [`crate::mailbox::offset`].
pub const MAILBOX_BASE: u64 = 0x1_F000;

/// Our transfer buffers, `0x20000..0x70000`: 320 KiB.
pub const BUFFERS: core::ops::Range<u64> = 0x2_0000..0x7_0000;

/// TX queue `i`'s register block (`EthernetTxRx.md`, "TX Queue Memory Map").
pub const fn txq_base(i: u8) -> u64 {
    0xFFB9_0000 + i as u64 * 0x1000
}
/// RX queue `i`'s register block (`EthernetTxRx.md`, "RX Queue Memory Map").
pub const fn rxq_base(i: u8) -> u64 {
    0xFFB9_4000 + i as u64 * 0x1000
}

/// TX queue register offsets (`EthernetTxRx.md`, `tt_eth_ss_regs.h`).
pub mod txq {
    pub const CMD: u64 = 0x04;
    pub const STATUS: u64 = 0x08;
    pub const TRANSFER_START_ADDR: u64 = 0x14;
    pub const TRANSFER_SIZE_BYTES: u64 = 0x18;
    pub const DEST_ADDR: u64 = 0x1C;
    /// `ETH_TXQ_CMD_START_DATA`: issue a packet transfer, with resend. `CMD` is
    /// written one-hot.
    pub const CMD_START_DATA: u32 = 1 << 1;
    /// `ETH_TXQ_STATUS_CMD_ONGOING_BIT`: a transfer is still in flight.
    pub const STATUS_CMD_ONGOING: u32 = 1 << 16;
}

/// RX queue register offsets (`EthernetTxRx.md`, `tt_eth_ss_regs.h`).
pub mod rxq {
    /// `ETH_RXQ_OUTSTANDING_WR_CNT`: writes received from the network and not
    /// yet committed to L1. Read-only; not decoded by ttsim (divergence row 62).
    pub const OUTSTANDING_WR_CNT: u64 = 0x50;
}

/// The queue this workspace sends on.
///
/// On silicon base firmware puts all three queues in TT-link mode; queue 2 is
/// the one that carries no MMIO or Overlay traffic, so it is the least likely to
/// be shared, and its `TRANSFER_CNT` read 0 before our first send (`MEASURED`).
/// On ttsim only queues 0 and 1 are in TT-link mode, yet an L1 write on queue 2
/// is delivered all the same (divergence row 56).
pub const DATA_QUEUE: u8 = 2;

/// The E1 data mover's contract, shared by `tt-firmware`'s `eth_e1` and the
/// host, so the layout cannot drift between them.
///
/// One transfer per link direction at a time. Both ends run the same image,
/// and each is a sender and a receiver:
///
/// 1. The host writes a send descriptor into the sending E1's mailbox, `SEQ`
///    last.
/// 2. The sender pulls the data from a Tensix tile into [`TX_STAGE`] (or takes
///    it as the host staged it), TT-link-writes it to the partner's
///    [`RX_LAND`], and then TT-link-writes an [`INBOX`] record: sequence number,
///    length and destination.
/// 3. The receiver sees a new record, waits until its RX queue has no writes
///    outstanding, NoC-writes [`RX_LAND`] into the destination Tensix tile (or
///    leaves it), and TT-link-writes an [`ACK`] back into the sender's mailbox.
///
/// The wait is what makes step 3 safe. TT-link delivers one queue's packets in
/// order -- the receiver discards anything out of sequence -- so every data
/// packet was *accepted* before the record was. Nothing documented says it was
/// *committed to L1* before the record was, but `ETH_RXQ_OUTSTANDING_WR_CNT`
/// counts exactly the accepted-but-uncommitted writes (`EthernetTxRx.md`). Once
/// the record is visible and that count is zero, everything before it is in L1.
/// The host, on the sending chip, learns of delivery from [`ACKED`], and never
/// has to talk to the other chip.
pub mod mover {
    use super::MAILBOX_BASE as M;

    /// This tile's raw NoC #0 coordinate, written by the host before release:
    /// core identity is not discoverable at run time.
    pub const MY_X: u64 = M + 0x10;
    pub const MY_Y: u64 = M + 0x14;
    /// Nonzero: wait for the RX queue's outstanding writes to drain before
    /// forwarding. Set by the host on silicon; zero on ttsim, which commits RX
    /// writes synchronously and does not decode the counter (row 62).
    pub const LANDING_WAIT: u64 = M + 0x18;
    /// Nonzero: record each step of a send and a receive in [`TRACE_RING`],
    /// stamped by E1's own cycle counter (an Ethernet tile has no
    /// timestamper). Zeroed at start with the rest of the mailbox.
    pub const TRACE: u64 = M + 0x1C;
    /// Events recorded so far. The host zeroes it to empty the ring; the
    /// mover stops recording when the ring is full rather than wrapping.
    pub const TRACE_COUNT: u64 = M + 0x20;

    /// Send descriptor. `SEND_SEQ` is written last and is nonzero.
    pub const SEND_SEQ: u64 = M + 0x40;
    pub const SEND_SRC_X: u64 = M + 0x44;
    pub const SEND_SRC_Y: u64 = M + 0x48;
    pub const SEND_SRC_ADDR: u64 = M + 0x4C;
    pub const SEND_LEN: u64 = M + 0x50;
    pub const SEND_DST_X: u64 = M + 0x54;
    pub const SEND_DST_Y: u64 = M + 0x58;
    pub const SEND_DST_ADDR: u64 = M + 0x5C;

    /// The last `SEND_SEQ` whose data and record were handed to the link.
    pub const SENT: u64 = M + 0x80;
    /// The last sequence number the partner acknowledged, after forwarding.
    pub const ACKED: u64 = M + 0x84;
    /// Nonzero if the mover gave up; see [`error`].
    pub const ERROR: u64 = M + 0x88;
    /// The last inbox sequence number this tile forwarded.
    pub const RECEIVED: u64 = M + 0x8C;

    /// Where the partner's record lands: eight words, `seq` in word 0 *and*
    /// word 7, so a record whose two 16-byte halves have not both arrived is
    /// not mistaken for a whole one.
    pub const INBOX: u64 = M + 0x100;
    /// Where the partner's acknowledgement lands: `seq` in word 0.
    pub const ACK: u64 = M + 0x140;
    /// Local staging for the outgoing record and acknowledgement. The TX queue
    /// may re-read them for resends, so each is only rewritten after the next
    /// handshake has proved the last one arrived.
    pub const RECORD_STAGE: u64 = M + 0x180;
    pub const ACK_STAGE: u64 = M + 0x1C0;

    pub const RECORD_BYTES: usize = 32;
    /// Words of a record: seq, len, dst x, dst y, dst addr, 0, 0, seq.
    pub const RECORD_WORDS: usize = 8;

    /// "No tile": in `SEND_SRC_X`, the data is already in [`TX_STAGE`]; in
    /// `SEND_DST_X`, leave it in [`RX_LAND`].
    pub const NO_TILE: u32 = 0xFF;

    /// Outgoing data, and the landing zone for the partner's.
    pub const TX_STAGE: u64 = 0x2_0000;
    pub const RX_LAND: u64 = 0x4_0000;
    /// Largest single transfer.
    pub const MAX_LEN: u32 = 0x2_0000;
    /// Largest single TT-link command the mover issues. Conservative: 4 KiB
    /// is what the host-driven gates measured, and nothing documents a limit.
    pub const TT_LINK_CHUNK: u32 = 0x4000;

    /// The trace ring: [`TRACE_EVENTS`] records of [`TRACE_RECORD_BYTES`],
    /// each `[event, seq, cycles, 0]`: E1's 32-bit `cycle`, which wraps (every
    /// ~3.2 s), so compare with `wrapping_sub`. In the transfer buffers, past
    /// `RX_LAND`.
    pub const TRACE_RING: u64 = 0x6_0000;
    pub const TRACE_EVENTS: u32 = 1024;
    pub const TRACE_RECORD_BYTES: u64 = 16;

    /// What the mover records ([`TRACE`]), in the order a send and its
    /// receive pass through them.
    pub mod event {
        /// The sender saw a new send descriptor.
        pub const SEND_PICKUP: u32 = 1;
        /// The sender's NoC read of the Tensix source into `TX_STAGE` landed
        /// (only for a Tensix source).
        pub const NOC_IN_DONE: u32 = 2;
        /// The data's TT-link commands have all been latched by the TX queue.
        pub const DATA_SENT: u32 = 3;
        /// The record's TT-link command has been latched; `SENT` follows.
        pub const RECORD_SENT: u32 = 4;
        /// The receiver saw a whole new record.
        pub const RECORD_SEEN: u32 = 5;
        /// The receiver's RX queue had no writes outstanding: the data is in
        /// its L1.
        pub const LANDED: u32 = 6;
        /// The receiver's NoC write into the Tensix destination completed
        /// (only for a Tensix destination).
        pub const NOC_OUT_DONE: u32 = 7;
        /// The receiver's acknowledgement has been latched by its TX queue.
        pub const ACK_SENT: u32 = 8;
        /// The sender saw the acknowledgement arrive.
        pub const ACK_SEEN: u32 = 9;
    }

    /// `ERROR` codes.
    pub mod error {
        /// A descriptor with a length that is zero, not a multiple of 16, or
        /// over [`super::MAX_LEN`].
        pub const LENGTH: u32 = 1;
        /// A Tensix address not 16-byte aligned (the NoC copy needs it
        /// congruent with the 16-aligned staging buffers).
        pub const ALIGNMENT: u32 = 2;
        /// The RX queue never reported its writes committed.
        pub const LANDING: u32 = 3;
    }

    // The layout stays inside the mailbox page and the customer buffers, and
    // every TT-link address is 16-byte aligned -- checked at compile time.
    const _: () = {
        assert!(ACK_STAGE + 16 <= super::BUFFERS.start);
        assert!(RX_LAND + MAX_LEN as u64 <= super::BUFFERS.end);
        assert!(TX_STAGE + MAX_LEN as u64 <= RX_LAND);
        assert!(RX_LAND + MAX_LEN as u64 <= TRACE_RING);
        assert!(TRACE_RING + TRACE_EVENTS as u64 * TRACE_RECORD_BYTES <= super::BUFFERS.end);
        assert!(TRACE_COUNT + 4 <= SEND_SEQ);
        assert!(INBOX % 16 == 0 && ACK % 16 == 0 && RECORD_STAGE % 16 == 0);
        assert!(ACK_STAGE % 16 == 0 && TX_STAGE % 16 == 0 && RX_LAND % 16 == 0);
    };
}
