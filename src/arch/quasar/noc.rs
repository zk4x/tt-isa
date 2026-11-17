//! Quasar's NoC: one NIU, a 64-bit address space and RoCC command buffers.
//!
//! There is no second NoC on this chip (`NUM_NOCS 1`) and no choice between
//! initiators: a request is issued from one of the four command buffers at
//! `NOC_CMD_BUF_OFFSET` strides, and the issue itself is a RoCC instruction
//! (`__builtin_riscv_ttrocc_cmdbuf_issue_trans`), not a store to a control
//! register — `NOC_CMD_CTRL` exists but holds no trigger state on this part
//! (`noc_cmd_buf_common.h`, `NOC_CMD_BUF_READ_REG`'s `NOC_CMD_CTRL` arm).
//!
//! An address carries its tile coordinate at bit 36, not in a separate
//! register: `NOC_ADDR_LOCAL_BITS` is 36 and `NOC_ADDR_NODE_ID_BITS` is 6, so
//! `x | y << 6` sits above a 36-bit local address (`noc_parameters.h:304-305`,
//! `NOC_XY_ADDR`). The registers below are that same split, spelled out: a
//! buffer's target and return addresses are three words each, and the third is
//! the coordinate.
//!
//! Spec: `tt-2xx/quasar/noc/noc_parameters.h`,
//! `tt-2xx/quasar/noc/tt_tensix_noc_overlay_reg.h`,
//! `tt-2xx/quasar/noc_cmd_buf_common.h`, and `soc_descriptors/quasar_32_arch.yaml`.

/// Width of the coordinate space a tile may name, from the descriptor's
/// `grid: x_size` (`quasar_32_arch.yaml`).
pub const GRID_WIDTH: u8 = 11;
/// Height of that space (`grid: y_size`).
pub const GRID_HEIGHT: u8 = 8;

/// A raw (untranslated) NoC coordinate.
///
/// Coordinate virtualisation is on but is the identity here: tt-umd's
/// translated coordinate is the physical one, and `VIRTUAL_TENSIX_START_X/Y`
/// is `(2, 2)`, which is where the functional workers begin
/// (`noc_parameters.h:9-14`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NocCoord {
    x: u8,
    y: u8,
}

impl NocCoord {
    /// Both axes are 6-bit fields (`NOC_ADDR_NODE_ID_BITS`), so anything wider
    /// is a programming error rather than an out-of-range tile.
    pub const fn new(x: u8, y: u8) -> Option<Self> {
        if x < 64 && y < 64 {
            Some(NocCoord { x, y })
        } else {
            None
        }
    }

    pub const fn x(self) -> u8 {
        self.x
    }

    pub const fn y(self) -> u8 {
        self.y
    }

    /// Pack as `x | (y << 6)`, which is `NOC_XY_ENCODING`/`NOC_XY_COORD` and
    /// what a command buffer's coordinate word carries.
    pub const fn packed(self) -> u16 {
        (self.x as u16) | ((self.y as u16) << 6)
    }
}

/// The tick a transaction id counts in: `NOC_MAX_TRANSACTION_ID` is `0xF`, so
/// ids run 0..=15 (`noc_parameters.h:35`).
pub struct TxnId(u8);

impl TxnId {
    pub const fn new(id: u8) -> Option<Self> {
        if id < 16 { Some(TxnId(id)) } else { None }
    }

    /// The ID itself, `0..16`.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// An address in a tile, with that tile's coordinate as the initiating command
/// buffer names it.
///
/// The local part is 36 bits (`NOC_LOCAL_ADDR_MASK`), so an endpoint address
/// is `u64`: L1 is 4 MiB but the field is wider, and tt-metal's own helpers
/// take the address already shifted (`noc_cmd_buf_set_targ_addr`).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    pub x: u8,
    pub y: u8,
    pub addr: u64,
}

impl Endpoint {
    /// The coordinate word: `x | y << 6` (`NOC_XY_ENCODING`).
    pub const fn hi(self) -> u32 {
        (self.x as u32 & 0x3F) | ((self.y as u32 & 0x3F) << 6)
    }

    /// The whole encoded address, `y << 42 | x << 36 | addr` — what
    /// `NOC_XY_ADDR` builds and what a command buffer's three address words
    /// spell out.
    pub const fn address(self) -> u64 {
        ((self.y as u64 & 0x3F) << 42)
            | ((self.x as u64 & 0x3F) << 36)
            | (self.addr & NOC_LOCAL_ADDR_MASK)
    }
}

/// The 36-bit local-address field of an encoded NoC address
/// (`NOC_LOCAL_ADDR_MASK`).
pub const NOC_LOCAL_ADDR_MASK: u64 = (1 << 36) - 1;

/// Largest payload one command buffer moves in one packet: 8 KiB, the
/// boot-time packetization limit (`NOC_OVERLAY_MAX_BYTES_IN_PACKET`), below the
/// hardware's 64 KiB default to avoid congestion.
pub const MAX_REQUEST_BYTES: u32 = 8 * 1024;

/// In Tensix tiles, L1 is below this and MMIO at or above it (WH
/// `NoC/Alignment.md`, unchanged here).
pub const MMIO_START: u64 = 0xFF00_0000;

/// The NIU register block, as offsets from [`niu::BASE`].
///
/// Every register is per command buffer: `BASE + buf * CMD_BUF_OFFSET + reg`,
/// with the NoC instance above that (`NOC_CMD_BUF_WRITE_REG` in
/// `noc_cmd_buf_common.h`). The few global registers — node id, endpoint id,
/// clear-outstanding — ignore the buffer.
pub mod niu {
    /// `TT_NOC_REG_MAP_BASE_ADDR` (`tt_tensix_noc_overlay_reg.h:77`): the one
    /// NIU's register block in the tile's address space.
    pub const BASE: u64 = 0x0200_0000;

    /// `NOC_CMD_BUF_OFFSET`: stride between the four command buffers
    /// (`noc_parameters.h:47-48`).
    pub const CMD_BUF_OFFSET: u64 = 0x0000_0800;
    /// `NOC_CMD_BUF_OFFSET_BIT`, the same stride as a shift.
    pub const CMD_BUF_OFFSET_BIT: u8 = 11;
    /// `NOC_INSTANCE_OFFSET`: stride between NoC instances. One is used, but
    /// the field exists in every register address.
    pub const NOC_INSTANCE_OFFSET: u64 = 0x0001_0000;

    /// `NOC_CMD_BUF_READ_REG`'s three command buffers: writes, reads and
    /// atomics (`noc_cmd_buf_common.h:34-37`).
    pub const WR_CMD_BUF: u8 = 0;
    pub const RD_CMD_BUF: u8 = 1;
    pub const AT_CMD_BUF: u8 = 2;

    /// Target address, low word.
    pub const TARG_ADDR_LO: u64 = 0x00;
    /// Target address, middle word.
    pub const TARG_ADDR_MID: u64 = 0x04;
    /// Target coordinate (`NOC_TARG_ADDR_COORDINATE` aliases this word).
    pub const TARG_ADDR_HI: u64 = 0x08;
    /// Return address, low word.
    pub const RET_ADDR_LO: u64 = 0x0C;
    /// Return address, middle word.
    pub const RET_ADDR_MID: u64 = 0x10;
    /// Return coordinate (`NOC_RET_ADDR_COORDINATE`).
    pub const RET_ADDR_HI: u64 = 0x14;

    /// `NOC_CTRL_LO`: the transaction's control bits (write/atomic/VC/...).
    pub const CTRL_LO: u64 = 0x18;
    /// `NOC_CTRL_HI`.
    pub const CTRL_HI: u64 = 0x1C;
    /// Broadcast start/end coordinates, low word.
    pub const BRCST_LO: u64 = 0x20;
    /// Broadcast coordinates, high word.
    pub const BRCST_HI: u64 = 0x24;

    /// `NOC_AT_LEN`: the transfer length. There is no separate byte-enable
    /// length register on this part (`NOC_AT_LEN_BE` aliases it, and byte
    /// enables have their own words below).
    pub const AT_LEN: u64 = 0x28;
    /// `NOC_L1_ACC_AT_INSTRN`: L1 accumulation / atomic instruction word.
    pub const L1_ACC_AT_INSTRN: u64 = 0x2C;
    /// `NOC_SEC_CTRL`: the security fence's control.
    pub const SEC_CTRL: u64 = 0x30;
    /// `NOC_AT_DATA`: the atomic operand, and the value of an inline write.
    pub const AT_DATA: u64 = 0x34;

    /// Inline-write payload, low word.
    pub const INLINE_DATA_LO: u64 = 0x38;
    /// Inline-write payload, high word.
    pub const INLINE_DATA_HI: u64 = 0x3C;

    /// Byte enables, eight words: 64 bits each, one bit per byte
    /// (`NOC_BYTE_ENABLE_0..7`). The fence only checks 64 B of a byte-enable
    /// write that can touch 256 B, so fence ranges go on 256-byte boundaries.
    pub const BYTE_ENABLE: [u64; 8] = [0x40, 0x44, 0x48, 0x4C, 0x50, 0x54, 0x58, 0x5C];

    /// `NOC_CMD_CTRL`. On Blackhole this word's write issues the request; here
    /// the issue is a RoCC instruction and this register stores no trigger, so
    /// it is read-only state as far as an issuer is concerned.
    pub const CMD_CTRL: u64 = 0x60;
    /// `NOC_NODE_ID`: this tile's raw coordinate, `x | y << 6`. Global, not per
    /// buffer.
    pub const NODE_ID: u64 = 0x64;
    /// `NOC_ENDPOINT_ID`: this tile's identity. Global.
    pub const ENDPOINT_ID: u64 = 0x68;

    /// `NOC_CLEAR_OUTSTANDING_REQ_CNT`: writing `1 << id` zeroes that
    /// transaction id's counter. Global.
    pub const CLEAR_OUTSTANDING_REQ_CNT: u64 = 0x7C;
    /// `CMD_BUF_AVAIL`: which command buffers may be armed, `[28:24]`,
    /// `[20:16]`, `[12:8]`, `[4:0]`.
    pub const CMD_BUF_AVAIL: u64 = 0x80;
    /// `CMD_BUF_OVFL`: a buffer was armed while unavailable.
    pub const CMD_BUF_OVFL: u64 = 0x84;

    /// `NOC_CFG(cnt)`.
    pub const fn cfg(cnt: u32) -> u64 {
        0x100 + (cnt as u64) * 4
    }

    /// `NOC_STATUS(cnt)`.
    pub const fn status(cnt: u32) -> u64 {
        0x200 + (cnt as u64) * 4
    }

    /// `NOC_SEC_FENCE_RANGE(cnt)`, 32 ranges.
    pub const fn sec_fence_range(cnt: u32) -> u64 {
        0x400 + (cnt as u64) * 4
    }

    /// `NOC_SEC_FENCE_ATTRIBUTE(cnt)`, 8 attributes.
    pub const fn sec_fence_attribute(cnt: u32) -> u64 {
        0x480 + (cnt as u64) * 4
    }

    /// `NOC_SEC_FENCE_MASTER_LEVEL`.
    pub const SEC_FENCE_MASTER_LEVEL: u64 = 0x4A0;
    /// `NOC_SEC_FENCE_FIFO_STATUS`.
    pub const SEC_FENCE_FIFO_STATUS: u64 = 0x4A4;
    /// `NOC_SEC_FENCE_FIFO_RDDATA`.
    pub const SEC_FENCE_FIFO_RDDATA: u64 = 0x4A8;
}

/// Where the chip's tiles are, from `soc_descriptors/quasar_32_arch.yaml`.
///
/// This chip has no harvesting (`harvested_workers: []`), no ARC (`arc: []`)
/// and no Ethernet (`eth: []`), so the grid is a fixed table rather than
/// something a probe has to report: every functional worker answers.
pub mod grid {
    /// The functional workers: `x` 2..=9, `y` 2..=5, 32 of them
    /// (`functional_workers`).
    pub const WORKERS: [(u8, u8); 32] = [
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (6, 2),
        (7, 2),
        (8, 2),
        (9, 2),
        (2, 3),
        (3, 3),
        (4, 3),
        (5, 3),
        (6, 3),
        (7, 3),
        (8, 3),
        (9, 3),
        (2, 4),
        (3, 4),
        (4, 4),
        (5, 4),
        (6, 4),
        (7, 4),
        (8, 4),
        (9, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (5, 5),
        (6, 5),
        (7, 5),
        (8, 5),
        (9, 5),
    ];

    /// The dispatch engines: `[10-1], [10-6], [1-6]` (`dispatch`).
    pub const DISPATCH: [(u8, u8); 3] = [(10, 1), (10, 6), (1, 6)];

    /// The tile that only routes: `0-2` (`router_only`).
    pub const ROUTER_ONLY: (u8, u8) = (0, 2);

    /// Is `(x, y)` a functional worker?
    pub const fn is_worker(x: u8, y: u8) -> bool {
        x >= 2 && x <= 9 && y >= 2 && y <= 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_reject_out_of_range() {
        assert!(NocCoord::new(63, 63).is_some());
        assert!(NocCoord::new(64, 0).is_none());
        assert!(NocCoord::new(0, 64).is_none());
    }

    #[test]
    fn a_coordinate_packs_as_noc_xy_encoding() {
        let c = NocCoord::new(9, 5).unwrap();
        assert_eq!(c.packed(), (9 | (5 << 6)) as u16);
    }

    /// `NOC_XY_ADDR(x, y, addr)` = `y << 42 | x << 36 | addr`
    /// (`noc_parameters.h:442-444`).
    #[test]
    fn an_endpoint_matches_noc_xy_addr() {
        let ep = Endpoint {
            x: 3,
            y: 4,
            addr: 0x1_2345,
        };
        assert_eq!(ep.address(), (4u64 << 42) | (3u64 << 36) | 0x1_2345);
        assert_eq!(ep.hi(), 3 | (4 << 6));
        // The coordinate word is the address shifted down by the local bits.
        assert_eq!(ep.address() >> 36, ep.hi() as u64);
    }

    #[test]
    fn transaction_ids_stop_at_the_register_width() {
        assert!(TxnId::new(15).is_some());
        assert!(TxnId::new(16).is_none());
        assert_eq!(TxnId::new(3).unwrap().index(), 3);
    }

    /// The grid is the descriptor's: four rows of eight workers, no harvesting.
    #[test]
    fn the_grid_is_the_descriptors() {
        assert_eq!(grid::WORKERS.len(), 32);
        for (i, &(x, y)) in grid::WORKERS.iter().enumerate() {
            assert!(grid::is_worker(x, y), "{x},{y}");
            // x runs first, so the table is row-major.
            assert_eq!(x, 2 + (i % 8) as u8);
            assert_eq!(y, 2 + (i / 8) as u8);
        }
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let listed = grid::WORKERS.contains(&(x, y));
                assert_eq!(listed, grid::is_worker(x, y), "{x},{y}");
            }
        }
        assert!(!grid::is_worker(grid::ROUTER_ONLY.0, grid::ROUTER_ONLY.1));
        for (x, y) in grid::DISPATCH {
            assert!(!grid::is_worker(x, y), "dispatch {x},{y} is not a worker");
            assert!(x < GRID_WIDTH && y < GRID_HEIGHT);
        }
    }

    #[test]
    fn the_niu_is_one_block_with_four_command_buffers() {
        assert_eq!(niu::BASE, 0x0200_0000);
        // The last command buffer's last register stays inside the block: the
        // buffers are 0x800 apart and no register offset exceeds that.
        assert!(niu::BYTE_ENABLE[7] < niu::CMD_BUF_OFFSET);
        assert!(niu::SEC_FENCE_FIFO_RDDATA < niu::CMD_BUF_OFFSET);
        assert_eq!(niu::CMD_BUF_OFFSET, 1 << niu::CMD_BUF_OFFSET_BIT);
        // `NOC_CMD_BUF_INSTANCE_OFFSET(noc, buf)`: the instance bit sits above
        // every buffer.
        assert!(niu::CMD_BUF_OFFSET * 4 <= niu::NOC_INSTANCE_OFFSET);
    }

    #[test]
    fn a_packet_is_the_overlay_limit() {
        assert_eq!(MAX_REQUEST_BYTES, 8 * 1024);
        assert!(MAX_REQUEST_BYTES <= 64 * 1024);
        assert!(MMIO_START > (1 << 36) - 1 || MAX_REQUEST_BYTES < MMIO_START as u32);
    }
}
