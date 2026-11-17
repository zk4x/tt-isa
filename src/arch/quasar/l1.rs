//! The fixed regions of a Quasar tile's 4 MiB L1.
//!
//! Everything the firmware and the fabric reserve is named here, in tt-metal's
//! vocabulary (`dev_mem_map.h`), and everything above [`MAP_END`](crate::arch::quasar::l1::MAP_END) is free for
//! tile data: buffers, dataflow-buffer rings and the kernel-config region the
//! host fills. Kernels never name an address in [`DATA`](crate::arch::quasar::l1::DATA): the caller declares
//! what it needs and the planner places it there.
//!
//! The regions are tt-metal's, computed through the header's own expression
//! chain. Two of them are not contiguous with their neighbours — the mailbox
//! rounds up to `MEM_LLK_DEBUG_BASE`, and the LLK debug area rounds up to
//! `MEM_INTERRUPT_TABLE_BASE` — so this module asserts order and containment,
//! not a partition.

use crate::arch::Arch;
use crate::arch::quasar::{Quasar, mailbox};

/// A byte range of L1, `[base, end)`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Region {
    pub name: &'static str,
    pub base: u64,
    pub end: u64,
}

impl Region {
    pub const fn contains(&self, at: u64, len: u64) -> bool {
        at >= self.base && at + len <= self.end
    }

    pub const fn len(&self) -> u64 {
        self.end - self.base
    }

    pub const fn is_empty(&self) -> bool {
        self.base == self.end
    }
}

/// `MEM_BOOT_CODE_BASE` (0) through the barrier: the word the tile fetches its
/// first instruction from, `MEM_NOC_ATOMIC_RET_VAL_ADDR` at 4, and
/// `MEM_L1_BARRIER` at 12 (`dev_mem_map.h:83-89`).
pub const BOOT: Region = Region {
    name: "boot words",
    base: 0,
    end: mailbox::MAILBOX_BASE,
};

/// `MEM_MAILBOX_BASE` .. `MEM_MAILBOX_END` (`dev_mem_map.h:87-93`), owned by
/// [`mailbox`].
pub const MAILBOX: Region = Region {
    name: "mailbox",
    base: mailbox::MAILBOX_BASE,
    end: mailbox::MAILBOX_END,
};

/// `MEM_LLK_DEBUG_BASE` .. + `MEM_LLK_DEBUG_SIZE` (1 KiB): the LLK's debug ring,
/// rounded up to 32 bytes after the mailbox (`dev_mem_map.h:95`).
pub const LLK_DEBUG: Region = Region {
    name: "LLK debug",
    base: 0x0000_EA20,
    end: 0x0000_EE20,
};

/// `MEM_INTERRUPT_TABLE_BASE` .. + `MEM_INTERRUPT_TABLE_SIZE` (256 B), rounded
/// up to 256 bytes (`dev_mem_map.h:108-109`).
pub const INTERRUPT_TABLE: Region = Region {
    name: "interrupt table",
    base: 0x0000_EF00,
    end: 0x0000_F000,
};

/// `MEM_DM_FIRMWARE_BASE` .. + `MEM_DM_FIRMWARE_SIZE` (12 KiB): one image shared
/// by all eight movers, each told which hart it is (`dev_mem_map.h:112`).
pub const DM_FIRMWARE: Region = Region {
    name: "DM firmware",
    base: 0x0000_F000,
    end: 0x0001_2000,
};

/// `MEM_TRISC0_FIRMWARE_BASE` .. + 4 × `MEM_TRISC_FIRMWARE_SIZE` (5 KiB each):
/// one image per TRISC role, shared by the four engines (`dev_mem_map.h:113-116`).
pub const TRISC_FIRMWARE: Region = Region {
    name: "TRISC firmware",
    base: 0x0001_2000,
    end: 0x0001_7000,
};

/// `MEM_DM_GLOBAL_BASE` .. `MEM_DM_LOCAL_BASE`: each mover's globals followed by
/// each TRISC's (`dev_mem_map.h:117-121`).
pub const GLOBALS: Region = Region {
    name: "firmware globals",
    base: 0x0001_7000,
    end: 0x0002_4000,
};

/// `MEM_DM_LOCAL_BASE` .. + 8 × `MEM_DM_LOCAL_SIZE` (8 KiB each): each mover's
/// stack and small data (`dev_mem_map.h:52-57,122`).
pub const DM_LOCALS: Region = Region {
    name: "DM locals",
    base: 0x0002_4000,
    end: 0x0003_4000,
};

/// `MEM_NOC_COUNTER_BASE` .. + `MEM_NOC_COUNTER_L1_SIZE` (80 B)
/// (`dev_mem_map.h:125-127`).
pub const NOC_COUNTERS: Region = Region {
    name: "NoC counters",
    base: 0x0003_4000,
    end: 0x0003_4050,
};

/// `MEM_FABRIC_COUNTER_BASE` .. + `MEM_FABRIC_COUNTER_L1_SIZE` (112 B)
/// (`dev_mem_map.h:130-134`).
pub const FABRIC_COUNTERS: Region = Region {
    name: "fabric counters",
    base: 0x0003_4050,
    end: 0x0003_40C0,
};

/// `MEM_FABRIC_CONNECTION_LOCK_BASE` .. + 144 B (`dev_mem_map.h:138-140`).
pub const FABRIC_CONNECTION: Region = Region {
    name: "fabric connection lock",
    base: 0x0003_40C0,
    end: 0x0003_4150,
};

/// `MEM_TENSIX_ROUTING_TABLE_BASE` .. `MEM_TENSIX_FABRIC_CONNECTIONS_BASE`: the
/// routing table, its 1D/2D path union and the exit-node table
/// (`dev_mem_map.h:143-167`).
pub const ROUTING: Region = Region {
    name: "routing table",
    base: 0x0003_4150,
    end: 0x0003_4BE0,
};

/// `MEM_TENSIX_FABRIC_CONNECTIONS_BASE` .. + 656 B (`dev_mem_map.h:166-167`).
pub const FABRIC_CONNECTIONS: Region = Region {
    name: "fabric connections",
    base: 0x0003_4BE0,
    end: 0x0003_4E70,
};

/// `MEM_PACKET_HEADER_POOL_BASE` .. + 96 × 144 B (`dev_mem_map.h:170-185`).
pub const PACKET_HEADERS: Region = Region {
    name: "packet header pool",
    base: 0x0003_4E70,
    end: 0x0003_8470,
};

/// The three semaphore side-regions, each on a 64-byte line: per-hart NoC-atomic
/// return slots (`MEM_NOC_CAS_RET_BASE`, 64 B), the external semaphore lock words
/// (`MEM_NOC_SEM_LOCK_BASE`, 256 B) and the DM cached-only pool
/// (`MEM_DM_CACHED_SEM_BASE`, 128 B) (`dev_mem_map.h:187-211`).
pub const SEMAPHORES: Region = Region {
    name: "semaphore side regions",
    base: 0x0003_8480,
    end: 0x0003_8640,
};

/// `MEM_MAP_END`: the last reserved byte. Everything at or above it is tile
/// data (LLK `llk_memory_checks.h`, `L1_REGION_START = MEM_MAP_END`).
pub const MAP_END: u64 = 0x0003_8640;

/// `MEM_MAP_END` .. + `MEM_KERNEL_CONFIG_SIZE` (100 KiB): where the host stages a
/// kernel's configuration and text for every hart (`dev_mem_map.h:215-216`).
pub const KERNEL_CONFIG: Region = Region {
    name: "kernel config",
    base: MAP_END,
    end: MAP_END + 100 * 1024,
};

/// The rest of L1: every buffer, dataflow-buffer ring and staging area, placed by
/// the planner.
pub const DATA: Region = Region {
    name: "data arena",
    base: KERNEL_CONFIG.end,
    end: Quasar::L1_SIZE,
};

/// Every named region, lowest first.
pub const REGIONS: [Region; 16] = [
    BOOT,
    MAILBOX,
    LLK_DEBUG,
    INTERRUPT_TABLE,
    DM_FIRMWARE,
    TRISC_FIRMWARE,
    GLOBALS,
    DM_LOCALS,
    NOC_COUNTERS,
    FABRIC_COUNTERS,
    FABRIC_CONNECTION,
    ROUTING,
    FABRIC_CONNECTIONS,
    PACKET_HEADERS,
    SEMAPHORES,
    KERNEL_CONFIG,
];

const _: () = assert!(BOOT.end <= MAILBOX.base);
const _: () = assert!(MAILBOX.end <= LLK_DEBUG.base);
const _: () = assert!(LLK_DEBUG.end <= INTERRUPT_TABLE.base);
const _: () = assert!(INTERRUPT_TABLE.end <= DM_FIRMWARE.base);
const _: () = assert!(DM_FIRMWARE.end <= TRISC_FIRMWARE.base);
const _: () = assert!(TRISC_FIRMWARE.end <= GLOBALS.base);
const _: () = assert!(GLOBALS.end <= DM_LOCALS.base);
const _: () = assert!(DM_LOCALS.end <= NOC_COUNTERS.base);
const _: () = assert!(NOC_COUNTERS.end <= FABRIC_COUNTERS.base);
const _: () = assert!(FABRIC_COUNTERS.end <= FABRIC_CONNECTION.base);
const _: () = assert!(FABRIC_CONNECTION.end <= ROUTING.base);
const _: () = assert!(ROUTING.end <= FABRIC_CONNECTIONS.base);
const _: () = assert!(FABRIC_CONNECTIONS.end <= PACKET_HEADERS.base);
const _: () = assert!(PACKET_HEADERS.end <= SEMAPHORES.base);
const _: () = assert!(SEMAPHORES.end == MAP_END);
const _: () = assert!(KERNEL_CONFIG.base == MAP_END);
const _: () = assert!(DATA.base == KERNEL_CONFIG.end);
const _: () = assert!(DATA.end == Quasar::L1_SIZE);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_regions_are_ordered_and_inside_l1() {
        for region in REGIONS {
            assert!(region.base < region.end, "{}: empty", region.name);
            assert!(region.end <= Quasar::L1_SIZE, "{}: past L1", region.name);
        }
        for pair in REGIONS.windows(2) {
            assert!(
                pair[0].end <= pair[1].base,
                "{} overlaps {}",
                pair[0].name,
                pair[1].name
            );
        }
        // The free area is the larger part of the tile.
        assert!(DATA.len() > 3 * 1024 * 1024);
    }

    /// The boundaries tt-metal's expression chain produces, spelled out, so a
    /// header revision that moves one of them fails here rather than at a card.
    #[test]
    fn the_boundaries_are_dev_mem_maps() {
        assert_eq!(mailbox::MAILBOX_END, 0xEA10);
        assert_eq!(LLK_DEBUG.base, 0xEA20);
        assert_eq!(INTERRUPT_TABLE.base, 0xEF00);
        assert_eq!(DM_FIRMWARE.base, 0xF000);
        assert_eq!(TRISC_FIRMWARE.base, 0x12000);
        assert_eq!(TRISC_FIRMWARE.end, 0x17000);
        assert_eq!(DM_LOCALS.end, 0x34000);
        assert_eq!(MAP_END, 0x38640);
        assert_eq!(KERNEL_CONFIG.end, 0x51640);
    }

    /// The mover and TRISC firmware bases are the ones `Arch` launches from: a
    /// region that disagreed with `default_reset_pc` would load an image
    /// somewhere the core never reaches.
    #[test]
    fn the_firmware_regions_hold_the_reset_pcs() {
        use crate::tensix::Core;
        for hart in 0..8u8 {
            let pc = Quasar::default_reset_pc(Core::Dm(hart)).unwrap() as u64;
            assert!(DM_FIRMWARE.contains(pc, 4), "mover {hart} at {pc:#x}");
        }
        for trisc in 0..4u8 {
            let pc = Quasar::default_reset_pc(Core::Math { engine: 0, trisc }).unwrap() as u64;
            assert!(TRISC_FIRMWARE.contains(pc, 4), "trisc {trisc} at {pc:#x}");
        }
    }
}
