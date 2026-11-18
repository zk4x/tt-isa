//! Quasar's DRAM: two 1 GiB channels, each one NoC endpoint.
//!
//! The part this crate describes is the 32-worker Quasar of
//! `tt_metal/soc_descriptors/quasar_32_arch.yaml`: its `dram` table lists two
//! channels, channel 0 at `(2,7)` and channel 1 at `(4,0)`, and `dram_bank_size`
//! and `dram_view_size` are both 1 GiB. There is no DRAM harvesting — the
//! descriptor has no harvested-bank field and the part has no ARC to ask — so
//! the table is fixed rather than discovered, the same way the worker grid is.
//!
//! Quasar has one NIU (`NUM_NOCS 1`), so a channel has one endpoint and no
//! choice of port between two NoCs. The host reaches DRAM at that endpoint:
//! ttsim's `libttsim_tile_wr_bytes` / `libttsim_tile_rd_bytes` round-trip there,
//! which is the path tt-umd's `TTSimTTDevice` uses for Quasar
//! (`tt_sim_tt_device.cpp`, `tile_write_bytes` with a `CoreType::DRAM` core).
//!
//! `MEASURED` on the qsr ttsim v1.16.0: writes through `tile_wr_bytes` at
//! `(2,7)` and `(4,0)` read back, and `(3,7)` is refused as an unknown tile.
//! The `soc_descriptor.yaml` shipped beside that library still says `(3,7)`
//! for channel 1, so it is stale; tt-metal's descriptor is the one the model
//! agrees with.
//!
//! Spec: `tt_metal/soc_descriptors/quasar_32_arch.yaml` (`dram`,
//! `dram_bank_size`, `dram_view_size`) and `core_config.h:116`
//! (`LOG_BASE_2_OF_DRAM_ALIGNMENT 6`).

use crate::arch::quasar::noc::NocCoord;

/// Channels on the 32-worker part: the descriptor's two `dram` entries.
pub const CHANNELS: u8 = 2;

/// Bytes of a channel: `dram_bank_size` and `dram_view_size` are both 1 GiB.
pub const CHANNEL_BYTES: u64 = 1 << 30;

/// DRAM access alignment: `DRAM_ALIGNMENT` is `1 << 6` (`core_config.h:116`
/// gives `LOG_BASE_2_OF_DRAM_ALIGNMENT` 6, a `TODO: verify` beside it).
pub const ALIGN: u64 = 64;

/// Each channel's endpoint, in logical channel order, from the descriptor's
/// `dram` table.
pub const ENDPOINTS: [(u8, u8); CHANNELS as usize] = [(2, 7), (4, 0)];

/// The DRAM channels of one Quasar part.
///
/// A unit value: the descriptor names no harvesting and there is no ARC to
/// report any, so every part of this SKU has the same two channels. Keeping the
/// type (rather than bare constants) lets consumers hold a channel and reserve
/// space in it without re-deriving the table.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Dram;

impl Dram {
    /// The part's fixed table.
    pub const FULL: Dram = Dram;

    /// The enabled-channel mask, for the `USABLE` word a mover reads.
    pub const fn enabled_mask(self) -> u8 {
        (1 << CHANNELS) - 1
    }

    /// How many channels this part has.
    pub const fn count(self) -> u8 {
        CHANNELS
    }

    /// Channel `logical`, if it exists.
    pub fn channel(self, logical: u8) -> Option<DramChannel> {
        (logical < CHANNELS).then_some(DramChannel { index: logical })
    }

    /// Every channel, in logical order.
    pub fn channels(self) -> impl Iterator<Item = DramChannel> {
        (0..CHANNELS).map(|index| DramChannel { index })
    }
}

/// A channel of one Quasar part. See [`Dram::channel`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DramChannel {
    index: u8,
}

impl DramChannel {
    /// The logical channel number, which is what every address uses.
    pub const fn index(self) -> u8 {
        self.index
    }

    /// The channel's NoC endpoint.
    ///
    /// Unlike Blackhole there is no preferred-port choice: one NIU means one
    /// endpoint per channel.
    pub fn preferred(self) -> NocCoord {
        let (x, y) = ENDPOINTS[self.index as usize];
        NocCoord::new(x, y).expect("a DRAM endpoint is inside the NoC's grid")
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
    fn the_descriptor_names_two_channels_at_its_two_endpoints() {
        let dram = Dram::FULL;
        assert_eq!(dram.count(), 2);
        assert_eq!(dram.enabled_mask(), 0b11);
        assert_eq!(dram.channels().count(), 2);
        let first = dram.channel(0).unwrap().preferred();
        let second = dram.channel(1).unwrap().preferred();
        assert_eq!((first.x(), first.y()), (2, 7));
        assert_eq!((second.x(), second.y()), (4, 0));
        assert!(dram.channel(2).is_none());
    }

    #[test]
    fn a_channel_is_one_gib_and_a_range_stays_inside_it() {
        let channel = Dram::FULL.channel(0).unwrap();
        assert_eq!(CHANNEL_BYTES, 1 << 30);
        assert!(channel.range(0, CHANNEL_BYTES).is_some());
        assert!(channel.range(CHANNEL_BYTES - 64, 64).is_some());
        assert!(channel.range(CHANNEL_BYTES - 64, 65).is_none());
        assert!(channel.range(u64::MAX, 2).is_none());
        assert!(channel.range(0, 0).unwrap().is_empty());
    }
}
