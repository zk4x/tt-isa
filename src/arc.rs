//! The ARC tile and its telemetry table: how to ask the chip what it *is*.
//!
//! Everything else in this crate describes hardware that behaves the same on
//! every Blackhole. This module exists because two things do not: which Tensix
//! columns were fused off for yield, and whether the NoC translates coordinates.
//! Both vary per ASIC, and both must be known *before* the first access to a
//! Tensix tile, because getting either wrong is not a failed read.
//!
//! A NoC read to a fused-off tile never returns. The tile's NIU is powered down,
//! so nothing answers, the host's MMIO read through a TLB window never completes,
//! and the chip is left with a hung NoC. The driver's recovery for that is an
//! ASIC + M3 reset (`blackhole.c:608-613`, `reset_arg = 3`), and the ARC's own
//! watchdog reaches the same place after `auto_reset_timeout` seconds
//! (`blackhole.c:704`). Either way the PCIe link drops. On a host that has the
//! card passed through to a VM, that link-down takes the host with it.
//!
//! So this is not an optimisation or a nicety. It is the only safe order of
//! operations: ask the ARC, then address Tensix.
//!
//! The constants are taken from the loaded driver rather than the ISA
//! specification, which does not document the telemetry table. They are quoted
//! from `/usr/src/tenstorrent-2.11.0` — the DKMS source for the `tenstorrent`
//! module this host binds — with tag numbers from UMD's `TelemetryTag`
//! (`umd/device/types/telemetry.hpp`), the only published enumeration of them.

use crate::noc::NocCoord;

/// The ARC tile's coordinate.
///
/// `blackhole.c:59-60` (`ARC_X 8`, `ARC_Y 0`). Unusually for this crate, the
/// value is safe to use without first knowing whether coordinate translation is
/// enabled: `NoC/Coordinates.md:28-29` gives rows `Y = 0` and `Y = 1` as the two
/// rows where "X translation not applied", so `(8, 0)` denotes the same tile in
/// raw and translated space alike.
///
/// That invariance is what makes the whole bootstrap possible. Reading the
/// harvesting mask requires addressing a tile, and every *other* coordinate's
/// meaning depends on the translation state we are trying to read.
pub const ARC_X: u8 = 8;
/// See [`ARC_X`].
pub const ARC_Y: u8 = 0;

/// The ARC tile on NoC #0.
pub fn arc_tile() -> NocCoord {
    // Both axes are in range by construction, so the Option cannot be None.
    NocCoord::new(ARC_X, ARC_Y).expect("the ARC coordinate is within the grid")
}

/// Reset-unit scratch register `n` (`blackhole.c:61`).
pub const fn reset_scratch(n: u32) -> u64 {
    0x8003_0400 + (n as u64) * 4
}

/// The ARC's boot-status word (`blackhole.c:66-68`, `RESET_SCRATCH(2)`).
///
/// Its low three bits are a state machine word; the ARC is up far enough to
/// answer messages when they read `0x5` (`(arc_boot_status & 0x7) == 0x5`,
/// `BlackholeTTDevice::wait_arc_core_start`, `blackhole_tt_device.cpp:182-203`).
/// tt-kmd waits on a different bit of the same word
/// (`ARC_BOOT_STATUS_READY_FOR_MSG`, `0x1`, `blackhole.c:531-552`); tt-metal's
/// value is the stricter of the two, so it is the one copied here.
pub const BOOT_STATUS: u64 = reset_scratch(2);
/// The value [`BOOT_STATUS`]'s low three bits take once the ARC is up.
pub const BOOT_STATUS_READY: u32 = 0x5;

/// Points at the telemetry table header (`blackhole.c:62`, `RESET_SCRATCH(13)`).
pub const TELEMETRY_PTR: u64 = reset_scratch(13);
/// Points at the telemetry data block (`blackhole.c:63`, `RESET_SCRATCH(12)`).
pub const TELEMETRY_DATA: u64 = reset_scratch(12);

/// Base of the ARC's CSM, the only address range the telemetry table may live in
/// (`telemetry.h:82`).
pub const CSM_BASE: u64 = 0x1000_0000;
/// Size of the ARC's CSM (`telemetry.h:83`).
pub const CSM_SIZE: u64 = 1 << 19;

/// Does `[addr, addr + len)` lie within the ARC's CSM?
///
/// The driver bounds-checks every telemetry address this way (`telemetry.h:89-92`)
/// before issuing the read, and so must we: the pointers come from firmware, and
/// a CSM read is a NoC read. An address outside the CSM is not merely wrong, it
/// is a read to an unmapped ARC address, which is the hang this module exists to
/// avoid.
pub const fn is_within_csm(addr: u64, len: u64) -> bool {
    addr >= CSM_BASE && addr <= (CSM_BASE + CSM_SIZE).saturating_sub(len)
}

/// Upper bound on telemetry table entries (`telemetry.h:87`).
pub const TELEMETRY_MAX_ENTRIES: u32 = 1 << 16;

/// Highest telemetry major version this code understands.
///
/// The driver refuses anything above 1 (`blackhole.c:494-497`) rather than
/// guessing at a layout it does not know, and the same reasoning applies here.
pub const TELEMETRY_MAX_MAJOR_VERSION: u32 = 1;

/// Telemetry tag numbers, from UMD's `TelemetryTag`
/// (`umd/device/types/telemetry.hpp:11-75`).
///
/// Only the tags this workspace reads are listed. The driver's own enumeration
/// (`telemetry.h:20-46`) is a strict subset of UMD's, which is why the driver's
/// sysfs cannot answer the harvesting question: `tt_telemetry_probe` builds its
/// address cache only for tags backing a sysfs or hwmon attribute
/// (`telemetry.c:42-77`), so a lookup of any other tag returns
/// `TELEM_ADDR_INVALID`.
pub mod tag {
    /// Bitmask of *enabled* Tensix columns.
    ///
    /// Enabled rather than harvested, so a firmware that does not publish the tag
    /// cannot be mistaken for a chip with nothing fused off — the absent tag and
    /// the all-harvested mask are both zero, and zero enabled columns is
    /// self-evidently not a usable chip.
    ///
    /// The bits index the harvesting order, *not* ascending NOC columns: tt-metal
    /// permutes them through `HARVESTING_NOC_LOCATIONS` before use
    /// (`CoordinateManager::shuffle_tensix_harvesting_mask`,
    /// `coordinate_manager.cpp:409-430`; `BlackholeTTDevice::get_chip_info`,
    /// `blackhole_tt_device.cpp:143-146`). Only the population count is used here,
    /// for the reason on [`Tensix::from_enabled_column_mask`][tensix].
    ///
    /// [tensix]: crate::noc::grid::Tensix::from_enabled_column_mask
    pub const ENABLED_TENSIX_COL: u16 = 34;

    /// Bitmask of *enabled* GDDR channels, one bit per channel.
    ///
    /// tt-metal inverts and masks it to eight bits
    /// (`BlackholeTTDevice::get_chip_info`, `blackhole_tt_device.cpp:146-149`).
    pub const ENABLED_GDDR: u16 = 36;

    /// GDDR training status, two bits per channel (`TelemetryTag::GDDR_STATUS`,
    /// `telemetry.hpp:22`): `0b01` trained, `0b10` and `0b11` failed, `0b00` still
    /// training (`FirmwareInfoProvider::get_modern_dram_statuses`,
    /// `firmware_info_provider.cpp:470-488`). tt-metal waits for every enabled
    /// channel to read `0b01` before it touches DRAM
    /// (`TTDevice::wait_dram_channel_training`, `tt_device.cpp:320-373`).
    pub const GDDR_STATUS: u16 = 22;

    /// The chip's AI clock, in MHz (`TelemetryTag::AICLK`, `telemetry.hpp:14`).
    ///
    /// This is what tt-metal's `get_clock` reads
    /// (`blackhole_tt_device.cpp:205-211`), and what `wait_for_aiclk_value`
    /// polls until a power-state change has taken effect (`chip.cpp:240-296`).
    pub const AICLK: u16 = 14;

    /// The highest AI clock the chip will run, in MHz
    /// (`TelemetryTag::AICLK_LIMIT_MAX`, `telemetry.hpp:63`).
    ///
    /// For a Blackhole whose firmware is newer than 18.7 this is tt-metal's
    /// `get_max_clock_freq` (`firmware_info_provider.cpp:197`), the value an
    /// `AICLK_GO_BUSY` is confirmed against. Card 0 reads `0x546` (1350).
    pub const AICLK_LIMIT_MAX: u16 = 63;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scratch_registers_match_the_driver() {
        // blackhole.c:61-63.
        assert_eq!(reset_scratch(0), 0x8003_0400);
        assert_eq!(TELEMETRY_DATA, 0x8003_0430);
        assert_eq!(TELEMETRY_PTR, 0x8003_0434);
    }

    #[test]
    fn csm_bounds_reject_what_the_driver_rejects() {
        assert!(is_within_csm(CSM_BASE, 4));
        assert!(is_within_csm(CSM_BASE + CSM_SIZE - 4, 4));
        // One byte past the end, and the classic off-by-four.
        assert!(!is_within_csm(CSM_BASE + CSM_SIZE, 4));
        assert!(!is_within_csm(CSM_BASE + CSM_SIZE - 3, 4));
        assert!(!is_within_csm(CSM_BASE - 4, 4));
        // A zero pointer is the shape an absent telemetry table takes.
        assert!(!is_within_csm(0, 4));
    }

    #[test]
    fn the_arc_coordinate_is_translation_invariant() {
        // NoC/Coordinates.md:28-29: X translation is not applied when Y is 0 or 1,
        // so this coordinate means the same thing whether or not translation is on.
        // If this ever changes, the bootstrap in `tt_device::telemetry` loses the
        // one tile it can address before it knows the translation state.
        assert_eq!(ARC_Y, 0);
        let arc = arc_tile();
        assert_eq!((arc.x(), arc.y()), (ARC_X, ARC_Y));
    }
}
