//! The GDDR6 behind the DRAM tiles: which channels a chip has, and how to name
//! them.
//!
//! A p150 exposes 32 GiB as eight 4 GiB channels, each on three NoC endpoints
//! that alias one memory (`BlackholeA0/README.md:5,21`). Phase 9 keeps tensors
//! there, so this is where the row-35 lesson applies next: a channel that is
//! fused off or failed training must not be nameable, and nothing here can be
//! obtained except from the chip's own answer.
//!
//! **A channel number is logical, not physical.** A chip with a fused DRAM bank
//! has one fewer channel, and the channels shift down: tt-metal's coordinate
//! manager builds a grid of `8 - popcount(harvested)` DRAM channels
//! (`blackhole_coordinate_manager.cpp:577-581`), and every consumer — the
//! allocator, the firmware's bank table, host transfers — names a channel in
//! that compacted space. [`Dram::channel`](crate::arch::blackhole::dram::Dram::channel) takes the same index, and the
//! translated coordinate it resolves to is a function of *both* the index and
//! the harvesting mask: UMD permutes the eight virtual endpoint slots so that no
//! surviving channel lands on the fused bank's
//! (`blackhole_coordinate_manager.cpp:320-417`).

use crate::arch::blackhole::noc::NocCoord;
use crate::arch::blackhole::noc::niu::Niu;

/// Channels on a full Blackhole (UMD `blackhole::NUM_DRAM_BANKS`).
pub const CHANNELS: u8 = 8;

/// Bytes of a channel this workspace addresses.
///
/// The specification and UMD say 4 GiB (`DRAM_BANK_SIZE`). ttsim stops at
/// `0xFF00_0000` and kills the process above it (`probe_niu`, and
/// `probe_dram::a_four_gib_window_reaches_the_whole_channel`: "DRAM write
/// overrun"). The top 16 MiB are therefore given up rather than probed on
/// silicon, where an access the NoC cannot complete takes the host down
/// (divergence row 35). 8 x 4080 MiB is still 31.9 GiB.
pub const CHANNEL_BYTES: u64 = 0xFF00_0000;

/// DRAM-to-L1 NoC reads need source and destination congruent mod 64.
///
/// Wormhole's table says 32 (`WormholeB0/NoC/Alignment.md:18`), and Blackhole's
/// `NoC/MemoryMap.md:106` links an `Alignment.md` that its tree does not have.
/// ttsim refuses a read congruent mod 32 but not 64 as `UndefinedBehavior`
/// (`probe_dram::which_congruence_ttsim_demands_of_a_dram_read`, divergence row
/// 64), which fits Blackhole's 512-bit flits against Wormhole's 256. The
/// stricter rule is followed on both targets. L1-to-DRAM writes need C16.
pub const ALIGN: u64 = 64;

/// The DRAM channels of one chip: which banks it has, and therefore which
/// logical channels exist.
///
/// The mask is the one thing a chip has to be asked for (`ENABLED_GDDR` tag 36,
/// read through [`crate::arch::blackhole::arc`]); a host that has done that hands it down with
/// [`Dram::from_enabled_mask`], which is also the shape the data mover receives
/// in its `USABLE` mailbox. [`Dram::FULL`] is ttsim's chip.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Dram {
    /// Bit `p` set: physical bank `p` is present. The chip's `ENABLED_GDDR`
    /// word, unmodified.
    enabled: u8,
}

impl Dram {
    /// ttsim's chips: every channel present. ttsim publishes no GDDR telemetry;
    /// all 24 endpoints answer and alias in threes (`probe_dram`).
    pub const FULL: Dram = Dram { enabled: 0xFF };

    /// A chip from its `ENABLED_GDDR` telemetry word.
    ///
    /// `None` unless the word names a chip this workspace can drive: at least
    /// one bank, and at most one fused. UMD refuses a second harvested bank
    /// (`blackhole_coordinate_manager.cpp:60-62`), and its translated table is
    /// only defined with one hole, so a word with two holes is not something to
    /// guess a coordinate for.
    pub const fn from_enabled_mask(mask: u8) -> Option<Dram> {
        if mask != 0 && (!mask).count_ones() <= 1 {
            Some(Dram { enabled: mask })
        } else {
            None
        }
    }

    /// The `ENABLED_GDDR` word this chip was built from.
    pub const fn enabled_mask(self) -> u8 {
        self.enabled
    }

    /// How many channels this chip has.
    fn count(self) -> u8 {
        self.enabled.count_ones() as u8
    }

    /// Channel `logical`, if this chip has that many. The only way to obtain a
    /// [`DramChannel`].
    ///
    /// Logical, in tt-metal's compacted order: channel 0 is the first *surviving*
    /// bank, which is not physical bank 0 once a bank is fused.
    pub fn channel(&self, logical: u8) -> Option<DramChannel> {
        (logical < self.count()).then_some(DramChannel {
            dram: *self,
            index: logical,
        })
    }

    /// Every channel, in logical order.
    pub fn channels(&self) -> impl Iterator<Item = DramChannel> + '_ {
        let dram = *self;
        (0..dram.count()).map(move |index| DramChannel { dram, index })
    }
}

/// A channel of one chip, named the way the rest of the stack names it. See
/// [`Dram::channel`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DramChannel {
    dram: Dram,
    /// Logical channel: the index of the survivor, not its physical bank.
    index: u8,
}

impl DramChannel {
    /// The logical channel number, which is what every address and every
    /// tt-metal API uses.
    pub const fn index(self) -> u8 {
        self.index
    }

    /// The physical bank this channel is, skipping the fused one.
    pub const fn physical(self) -> u8 {
        let mut seen = 0u8;
        let mut bank = 0u8;
        while bank < CHANNELS {
            if self.dram.enabled & (1u8 << bank) != 0 {
                if seen == self.index {
                    return bank;
                }
                seen += 1;
            }
            bank += 1;
        }
        panic!("a DramChannel is only made for an enabled bank")
    }

    /// tt-metal's *preferred worker endpoint* for this channel on `niu`.
    ///
    /// This is the `worker_endpoint` subchannel of the channel's `dram_views`
    /// entry (`blackhole_140_arch.yaml:23-75`), resolved to a translated
    /// coordinate the way tt-metal resolves it (`get_dram_core_for_channel`,
    /// `soc_descriptor.cpp:259-263`, then
    /// `get_preferred_worker_core_for_dram_view`, `metal_soc_descriptor.cpp:13-21`).
    /// It is the endpoint tt-metal hands its firmware for a bank
    /// (`risc_firmware_initializer.cpp:544-560`), and the only one this workspace
    /// drives: no request carries a port, so every access to a channel goes out
    /// of the endpoint the chip prefers for its NoC.
    ///
    /// The subchannels are NoC #0: 2 on channels 0 and 4-7, 0 on channels 1-3 —
    /// the endpoint the chip's management firmware (CMFW) reads DRAM telemetry
    /// through, "to avoid SYS-1419" (the yaml's own note); and NoC #1: 1 on every
    /// channel. The two differ on every channel, so the SYS-1419 rule — both NoCs
    /// issuing to one endpoint — holds by construction.
    ///
    /// Translated because both targets accept it and silicon may accept nothing
    /// else: with translation on, `X = 0` has no meaning in the Tensix rows
    /// (`NoC/Coordinates.md:35-42`), so half the raw DRAM coordinates name
    /// nothing, and a NoC access that nothing answers is a hung host (row 35).
    /// `MEASURED` on ttsim for the full chip
    /// (`probe_dram::does_ttsim_answer_at_translated_dram_coordinates`).
    pub fn preferred(self, niu: Niu) -> NocCoord {
        let subchannel = match niu {
            Niu::Noc0 => match self.index {
                1..=3 => 0,
                _ => 2,
            },
            Niu::Noc1 => 1,
        };
        let (x, y) = slot(self.dram.enabled, self.index);
        NocCoord::new(x, y + subchannel).expect("a DRAM endpoint is inside the NoC's grid")
    }

    /// `[offset, offset + len)` of this channel, if it lies inside
    /// [`CHANNEL_BYTES`].
    pub fn range(self, offset: u64, len: u64) -> Option<DramRange> {
        let end = offset.checked_add(len)?;
        (end <= CHANNEL_BYTES).then_some(DramRange {
            channel: self,
            offset,
            len,
        })
    }
}

/// The eight translated endpoint slots, and which logical channel gets which.
///
/// A channel's three endpoints sit at this base and the two Y values above it;
/// all three reach the same memory (`MEASURED` on ttsim,
/// `probe_dram::which_endpoints_alias_one_channel`).
///
/// UMD's arithmetic (`blackhole_coordinate_manager.cpp:320-417`), transcribed:
/// the ranges enumerate *logical* channels, `map_dram_banks` assigns each three
/// consecutive Y from its start, and the tail block gives the fused physical
/// bank the slot `(18, 21..23)`, so no logical channel can land on it.
///
/// With nothing fused this is the plain table; with one bank fused the two
/// columns swap roles and the survivors rotate around the hole.
fn slot(enabled: u8, logical: u8) -> (u8, u8) {
    let fused = !enabled;
    if fused == 0 {
        return (17 + logical / 4, 12 + 3 * (logical % 4));
    }
    // At most one bank is fused (`Dram::from_enabled_mask`), so the single set
    // bit is the hole.
    let h = fused.trailing_zeros() as u8;
    if h < 4 {
        // East column first, then the west column around the hole.
        let mirror_east = h + 3;
        if logical < 3 {
            (18, 12 + 3 * logical)
        } else if logical < mirror_east {
            (17, 12 + 3 * (logical - 3))
        } else if logical == mirror_east {
            (17, 21)
        } else {
            (17, 12 + 3 * (logical - 4))
        }
    } else {
        // West column first, then the east column.
        let mirror_west = h - 4;
        if logical < mirror_west {
            (17, 12 + 3 * logical)
        } else if logical == mirror_west {
            (17, 21)
        } else if logical < 4 {
            (17, 12 + 3 * (logical - 1))
        } else {
            (18, 12 + 3 * (logical - 4))
        }
    }
}

/// A byte range inside one usable channel. See [`DramChannel::range`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DramRange {
    channel: DramChannel,
    offset: u64,
    len: u64,
}

impl DramRange {
    pub const fn channel(self) -> DramChannel {
        self.channel
    }
    pub const fn offset(self) -> u64 {
        self.offset
    }
    pub const fn len(self) -> u64 {
        self.len
    }
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_full_chip_uses_umds_plain_table() {
        let d = Dram::FULL;
        let e = |c: u8, niu: Niu| {
            let ch = d.channel(c).unwrap();
            let n = ch.preferred(niu);
            (n.x(), n.y())
        };
        // Slot (17, 12) for channel 0, (17, 21) for 3, (18, 12) for 4, (18, 21)
        // for 7. NoC #0 drives subchannel 2 on channels 0 and 4-7 and 0 on 1-3;
        // NoC #1 drives subchannel 1 on every channel.
        assert_eq!(e(0, Niu::Noc0), (17, 14));
        assert_eq!(e(0, Niu::Noc1), (17, 13));
        assert_eq!(e(1, Niu::Noc0), (17, 15));
        assert_eq!(e(2, Niu::Noc0), (17, 18));
        assert_eq!(e(3, Niu::Noc0), (17, 21));
        assert_eq!(e(3, Niu::Noc1), (17, 22));
        assert_eq!(e(4, Niu::Noc0), (18, 14));
        assert_eq!(e(7, Niu::Noc0), (18, 23));
        // The slot a channel's three endpoints hang off.
        assert_eq!(slot(0xFF, 0), (17, 12));
        assert_eq!(slot(0xFF, 7), (18, 21));
    }

    /// The whole harvested table, one fused bank at a time, written out from
    /// UMD's `map_dram_banks` calls rather than from [`slot`]: with bank `h`
    /// fused, `(x, y)` is the base of logical channel `L`'s three endpoints.
    #[test]
    fn every_harvest_position_uses_umds_translated_table() {
        let table: [[(u8, u8); 7]; 8] = [
            // h = 0: east column takes 0-2, west takes 4-6, and 3 mirrors at Y 21.
            [
                (18, 12),
                (18, 15),
                (18, 18),
                (17, 21),
                (17, 12),
                (17, 15),
                (17, 18),
            ],
            // h = 1.
            [
                (18, 12),
                (18, 15),
                (18, 18),
                (17, 12),
                (17, 21),
                (17, 15),
                (17, 18),
            ],
            // h = 2.
            [
                (18, 12),
                (18, 15),
                (18, 18),
                (17, 12),
                (17, 15),
                (17, 21),
                (17, 18),
            ],
            // h = 3.
            [
                (18, 12),
                (18, 15),
                (18, 18),
                (17, 12),
                (17, 15),
                (17, 18),
                (17, 21),
            ],
            // h = 4: west column takes 1-3, then 0 mirrors at Y 21.
            [
                (17, 21),
                (17, 12),
                (17, 15),
                (17, 18),
                (18, 12),
                (18, 15),
                (18, 18),
            ],
            // h = 5.
            [
                (17, 12),
                (17, 21),
                (17, 15),
                (17, 18),
                (18, 12),
                (18, 15),
                (18, 18),
            ],
            // h = 6.
            [
                (17, 12),
                (17, 15),
                (17, 21),
                (17, 18),
                (18, 12),
                (18, 15),
                (18, 18),
            ],
            // h = 7: the even split, and the one case where the static table
            // (`17 + L/4`, `12 + 3*(L%4)`) is still right.
            [
                (17, 12),
                (17, 15),
                (17, 18),
                (17, 21),
                (18, 12),
                (18, 15),
                (18, 18),
            ],
        ];
        for (h, want) in table.iter().enumerate() {
            let h = h as u8;
            let dram = Dram::from_enabled_mask(!(1u8 << h)).expect("one fused bank is a chip");
            assert_eq!(dram.channels().count(), 7, "h = {h}");
            for (logical, &(x, y)) in want.iter().enumerate() {
                let logical = logical as u8;
                let ch = dram.channel(logical).expect("channel exists");
                assert_eq!(
                    (ch.index(), ch.physical()),
                    (logical, if logical < h { logical } else { logical + 1 })
                );
                // The slot, and the two endpoints the NoCs drive out of it.
                assert_eq!(slot(!(1u8 << h), logical), (x, y), "h = {h} L = {logical}");
                let sub0 = match logical {
                    1..=3 => 0,
                    _ => 2,
                };
                let c0 = ch.preferred(Niu::Noc0);
                assert_eq!((c0.x(), c0.y()), (x, y + sub0), "h = {h} L = {logical}");
                let c1 = ch.preferred(Niu::Noc1);
                assert_eq!((c1.x(), c1.y()), (x, y + 1), "h = {h} L = {logical}");
            }
        }
    }

    /// UMD's `fill_dram_noc0_translated_mapping` transcribed literally — the
    /// `map_dram_banks(start, end, x, y)` calls and their start rows — as an
    /// oracle for [`slot`]. The hand-written table above and this are two
    /// separate readings of `blackhole_coordinate_manager.cpp:320-400`.
    fn umd_slots(h: Option<u8>) -> [(u8, u8); CHANNELS as usize] {
        let mut out = [(0u8, 0u8); CHANNELS as usize];
        {
            let mut map = |start: u8, end: u8, x: u8, y0: u8| {
                let mut b = start;
                while b < end {
                    out[b as usize] = (x, y0 + 3 * (b - start));
                    b += 1;
                }
            };
            match h {
                None => {
                    map(0, 4, 17, 12);
                    map(4, 8, 18, 12);
                }
                Some(h) if h < 4 => {
                    let mirror_east = h + 3;
                    map(0, 3, 18, 12);
                    map(3, mirror_east, 17, 12);
                    map(mirror_east + 1, 7, 17, 12 + (mirror_east - 3) * 3);
                    map(mirror_east, mirror_east + 1, 17, 21);
                }
                Some(h) => {
                    let mirror_west = h - 4;
                    map(0, mirror_west, 17, 12);
                    map(mirror_west + 1, 4, 17, 12 + mirror_west * 3);
                    map(mirror_west, mirror_west + 1, 17, 21);
                    map(4, 7, 18, 12);
                }
            }
        }
        out
    }

    #[test]
    fn the_slot_function_matches_umds_range_calls() {
        let full = umd_slots(None);
        for (logical, &want) in full.iter().enumerate() {
            assert_eq!(slot(0xFF, logical as u8), want, "full chip L = {logical}");
        }
        for h in 0..CHANNELS {
            let want = umd_slots(Some(h));
            for logical in 0..(CHANNELS - 1) {
                assert_eq!(
                    slot(!(1u8 << h), logical),
                    want[logical as usize],
                    "h = {h} L = {logical}"
                );
            }
        }
    }

    /// The fused bank's slot is the one no logical channel may name.
    #[test]
    fn no_surviving_channel_lands_on_the_fused_banks_slot() {
        for h in 0..CHANNELS {
            let dram = Dram::from_enabled_mask(!(1u8 << h)).unwrap();
            for ch in dram.channels() {
                let (x, y) = slot(dram.enabled_mask(), ch.index());
                assert!(
                    !(x == 18 && y >= 21),
                    "h = {h}: channel {} names the fused slot {:?}",
                    ch.index(),
                    (x, y)
                );
                for niu in [Niu::Noc0, Niu::Noc1] {
                    let e = ch.preferred(niu);
                    assert!(
                        !(e.x() == 18 && e.y() >= 21),
                        "h = {h}: channel {} drives the fused slot {:?}",
                        ch.index(),
                        (e.x(), e.y())
                    );
                }
            }
        }
    }

    #[test]
    fn a_mask_that_is_not_a_chip_is_refused() {
        // No bank at all.
        assert!(Dram::from_enabled_mask(0).is_none());
        // Two or more fused: UMD refuses that part.
        for mask in [0xFCu8, 0xF0, 0x0F, 0x81, 0x82] {
            assert!(Dram::from_enabled_mask(mask).is_none(), "{mask:#x}");
        }
        // One fused, or none, is a chip.
        assert!(Dram::from_enabled_mask(0xFF).is_some());
        for h in 0..CHANNELS {
            assert!(Dram::from_enabled_mask(!(1u8 << h)).is_some(), "h = {h}");
        }
        assert_eq!(Dram::FULL.enabled_mask(), 0xFF);
        assert_eq!(Dram::from_enabled_mask(0xFE).unwrap().enabled_mask(), 0xFE);
    }

    #[test]
    fn channel_numbers_are_logical_and_stop_at_the_count() {
        let dram = Dram::from_enabled_mask(!(1u8 << 2)).unwrap();
        assert!(dram.channel(6).is_some());
        assert!(dram.channel(7).is_none());
        assert_eq!(dram.channels().count(), 7);
        // Bank 2 is the fused one: channel 2 is physical 3.
        assert_eq!(dram.channel(2).unwrap().physical(), 3);
        assert!(!dram.channels().any(|c| c.physical() == 2));
        for (i, ch) in dram.channels().enumerate() {
            assert_eq!(ch.index(), i as u8);
        }
    }

    /// The two NoCs never drive the same endpoint of a channel — tt-metal's
    /// preferred table satisfies SYS-1419's rule without a predicate, so there is
    /// no ownership rule left to apply.
    #[test]
    fn the_two_nocs_never_share_an_endpoint() {
        let chips = core::iter::once(Dram::FULL)
            .chain((0..CHANNELS).map(|h| Dram::from_enabled_mask(!(1u8 << h)).unwrap()));
        for dram in chips {
            for ch in dram.channels() {
                let (a, b) = (ch.preferred(Niu::Noc0), ch.preferred(Niu::Noc1));
                assert_ne!(
                    (a.x(), a.y()),
                    (b.x(), b.y()),
                    "channel {} ({:#x}) drives one endpoint from both NoCs",
                    ch.index(),
                    dram.enabled_mask()
                );
            }
        }
    }

    #[test]
    fn ranges_stop_below_the_extent_ttsim_models() {
        let c = Dram::FULL.channel(0).unwrap();
        assert!(c.range(CHANNEL_BYTES - 4, 4).is_some());
        assert!(c.range(CHANNEL_BYTES - 4, 8).is_none());
        assert!(c.range(u64::MAX, 2).is_none());
    }
}
