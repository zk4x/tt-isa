//! The L1 contract between host and firmware.
//!
//! Same words as Blackhole's, at the same offsets from the base, because the
//! protocol they carry — liveness, one result, one launch, one acknowledgement —
//! is not the architecture's. The base and the barrier are tt-metal's:
//! `MEM_MAILBOX_BASE` 16 and `MEM_L1_BARRIER` 12 (`dev_mem_map.h:87-93`), with
//! the region `MEM_MAILBOX_SIZE` 59904 bytes long and ending at `MEM_MAILBOX_END`
//! `0xEA10`, which the linker places the LLK debug area after.
//!
//! What is *not* here is what a Quasar hart cannot act on. A Blackhole T core
//! pushes Tensix instructions to a coprocessor, so that mailbox has a push window,
//! a MOP-expander configuration and a semaphore-release word; a Quasar TRISC
//! executes its own ELF, and its movers run RV64 code, so none of those words
//! exist. The Dst dump words stay: any hart can copy rows out.

use crate::arch::{Arch, quasar::Quasar};

/// `MEM_L1_BARRIER` (`dev_mem_map.h:89`): the word a launch's host-to-device
/// barrier writes and reads back, tt-metal's `l1_membar`.
///
/// It sits *before* [`MAILBOX_BASE`] on this chip, unlike Blackhole's barrier
/// after the header, so a host that writes the barrier after the launch words
/// still orders them: with write-combined mappings a store can be sitting in the
/// host's buffer when the next one leaves.
pub const BARRIER: u64 = 12;

/// `MEM_MAILBOX_BASE` (`dev_mem_map.h:87`). Hardcoded in tt-metal because a
/// compiler bug could not resolve the expression, and kept as the same literal
/// here.
pub const MAILBOX_BASE: u64 = 16;

/// `MEM_MAILBOX_SIZE`: the region the firmware may assume is its own
/// (`dev_mem_map.h:91`). The words below use under two hundred bytes of it.
pub const MAILBOX_SIZE: u64 = 59_904;

/// `MEM_MAILBOX_END`: where the LLK debug area begins after rounding
/// (`dev_mem_map.h:93`).
pub const MAILBOX_END: u64 = MAILBOX_BASE + MAILBOX_SIZE;

/// Offsets of the words the firmware runtime itself writes, from whichever base
/// the image was linked with.
pub mod offset {
    pub const STATUS: u64 = 0x00;
    pub const HEARTBEAT: u64 = 0x04;
    pub const PANIC_CODE: u64 = 0x08;
    pub const RESULT: u64 = 0x0C;
}

/// Firmware liveness and status, in the encoding both sides know.
pub const STATUS: u64 = MAILBOX_BASE + offset::STATUS;
/// Monotonically incrementing counter, the heartbeat proper.
pub const HEARTBEAT: u64 = MAILBOX_BASE + offset::HEARTBEAT;
/// Set alongside a panicked status to say where.
pub const PANIC_CODE: u64 = MAILBOX_BASE + offset::PANIC_CODE;
/// Where a computation leaves its answer.
pub const RESULT: u64 = MAILBOX_BASE + offset::RESULT;
/// Second result word, for results wider than 32 bits.
pub const RESULT_HI: u64 = MAILBOX_BASE + 0x10;
/// First operand, written by the host before the core is released.
pub const OPERAND_A: u64 = MAILBOX_BASE + 0x14;
/// Second operand.
pub const OPERAND_B: u64 = MAILBOX_BASE + 0x18;

/// Which hart the running firmware is: a mover's index 0..=7, or
/// `8 + 4 * engine + trisc` for a compute engine's TRISC.
///
/// Written by the host because a hart cannot always work it out — Blackhole's
/// `mhartid` reads zero on every core — and because both sides already agree on
/// the [`crate::tensix::Core`] numbering.
pub const THREAD_INDEX: u64 = MAILBOX_BASE + 0x1C;

/// How many words the host has staged at [`PROGRAM_ADDR`].
pub const PROGRAM_LEN: u64 = MAILBOX_BASE + 0x24;
/// How many `Dst` rows the firmware should copy to the dump.
pub const DUMP_ROW_COUNT: u64 = MAILBOX_BASE + 0x28;
/// First `Dst` row to copy.
pub const DUMP_ROW_FIRST: u64 = MAILBOX_BASE + 0x2C;

/// Non-zero: the firmware records its progress through the tile's timestamper,
/// which the host has configured.
pub const TRACE: u64 = MAILBOX_BASE + 0x30;

/// The launch's go signal: the host writes a new non-zero value after the
/// launch words are in place, and the resident runner writes it to [`ACK`] when
/// it finishes. Zero: nothing to run.
pub const GENERATION: u64 = MAILBOX_BASE + 0x38;
/// The last [`GENERATION`] the resident runner finished.
pub const ACK: u64 = MAILBOX_BASE + 0x3C;

/// The program's first word: an L1 address in the kernel-config region
/// (`crate::arch::quasar::l1::KERNEL_CONFIG`), named together with
/// [`PROGRAM_LEN`]. There is no fixed slot: the runner refuses a zero address, or
/// one outside the region.
pub const PROGRAM_ADDR: u64 = MAILBOX_BASE + 0x40;

/// The arguments base an executor hands its program in `a0`: the launch's `u32`
/// array in L1.
pub const ARGS: u64 = MAILBOX_BASE + 0x70;

/// Which harts the launch enables, one bit each: the movers in bits 0..=7
/// (`DM0`..`DM7`), then each engine's four TRISCs from bit 8
/// (`8 + 4 * engine + trisc`), matching [`crate::tensix::Core`]'s numbering.
pub const ENABLES: u64 = MAILBOX_BASE + 0x74;

const _: () = assert!(BARRIER + 4 <= MAILBOX_BASE);
const _: () = assert!(ENABLES + 4 <= MAILBOX_BASE + 0x78);
const _: () = assert!(MAILBOX_BASE + 0x78 <= MAILBOX_END);
const _: () = assert!(MAILBOX_END < Quasar::L1_SIZE);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mailbox_is_inside_l1_and_after_the_barrier() {
        assert_eq!(BARRIER, 12);
        assert_eq!(MAILBOX_BASE, 16);
        assert_eq!(MAILBOX_END, 0xEA10);
        assert!(MAILBOX_END <= Quasar::L1_SIZE);
    }

    /// The words keep Blackhole's offsets, so a host routine that fills one
    /// layout fills the other.
    #[test]
    fn the_words_are_blackholes_offsets() {
        let words = [
            (STATUS, 0x00),
            (HEARTBEAT, 0x04),
            (PANIC_CODE, 0x08),
            (RESULT, 0x0C),
            (RESULT_HI, 0x10),
            (OPERAND_A, 0x14),
            (OPERAND_B, 0x18),
            (THREAD_INDEX, 0x1C),
            (PROGRAM_LEN, 0x24),
            (DUMP_ROW_COUNT, 0x28),
            (DUMP_ROW_FIRST, 0x2C),
            (TRACE, 0x30),
            (GENERATION, 0x38),
            (ACK, 0x3C),
            (PROGRAM_ADDR, 0x40),
            (ARGS, 0x70),
            (ENABLES, 0x74),
        ];
        for (word, off) in words {
            assert_eq!(word, MAILBOX_BASE + off, "word at {off:#x}");
        }
    }

    /// Every word lies in the block the firmware owns, and the barrier is
    /// outside it.
    #[test]
    fn the_block_ends_before_the_region_does() {
        assert!(BARRIER + 4 <= MAILBOX_BASE);
        assert!(ENABLES + 4 <= MAILBOX_BASE + 0x78);
        assert!(MAILBOX_BASE + 0x78 <= MAILBOX_END);
    }
}
