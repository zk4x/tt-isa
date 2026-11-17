//! The L1 Cache Tag Search Accelerator's configuration words and the probe that
//! drives them.
//!
//! Source: `BlackholeA0/TensixTile/BabyRISCV/L1CacheTagSearchAccel.md` (pinned).
//! The block is **not** an MMIO register window: it is configured through eight
//! words of the Tensix backend `Config` (indices 212..=219, all above
//! `GLOBAL_CFGREG_BASE_ADDR32`, so one store lands in both banks), reached from
//! RISCV B by `sw` at `TENSIX_CFG_BASE + 4 * index`, and it is *triggered* by an
//! ordinary load that RISCV B (and only B) performs against an L1 address the
//! configuration names. The load never reaches L1: the hardware answers it.
//!
//! A latch copies all fourteen fields into a private `LatchedConfig` whenever
//! software **changes** one of the five trigger bits (`Search_Enable`,
//! `Tag_alloc`, `Tag_inv`, `Tag_inv_all`, `Data_Valid_chk`). Writing a tag value
//! or an address while those keep their value does not re-latch, so a
//! configuration is written as [`DISARM`], the value words, then the arming
//! bit: the last trigger flip sees the final fields.
//!
//! Silicon status: `UNVERIFIED`. Nothing here has executed on a Blackhole card;
//! ttsim does not implement the block (divergence row 92).

use crate::cfg::blackhole::global as g;
use crate::cfg::{ConfigBank, ConfigField};

/// The layout of the RISCV B probe's staged script and results
/// (`tt-firmware/src/bin/tag_search_b.rs`), shared with the host test so the two
/// cannot disagree. All of it lies in the data arena (`l1::DATA`).
pub mod probe {
    /// Script header: word 0 is the number of steps; word 1 is non-zero for
    /// blind mode (no `Config` loads, a simulator diagnostic); word 2 is non-zero
    /// to skip the disarm before and after the steps.
    pub const SCRIPT: u64 = 0x2_0000;
    /// Most steps one run executes.
    pub const MAX_STEPS: u32 = 16;
    /// First step.
    pub const STEPS: u64 = SCRIPT + 0x10;
    /// Bytes per step: word 0 the L1 address to load, word 1 the number of
    /// [`super::WriteStep`]s, then `(word, mask, value)` triples.
    pub const STEP_STRIDE: u64 = 0x100;
    /// Where the results go; per step, word 0 is the loaded value and words
    /// 1..=8 are `Config[212..=219]` read back after the writes.
    pub const RESULTS: u64 = 0x2_2000;
    /// Bytes per result record.
    pub const RESULT_STRIDE: u64 = 0x40;
    /// Where the host stages tags, validity vectors and bit vectors.
    pub const DATA: u64 = 0x3_0000;
    /// Nops between the last `Config` store and the load it should trigger.
    pub const SETTLE_NOPS: u32 = 64;
    /// Progress breadcrumb, one word: `(step << 8) | phase`. The probe publishes
    /// it (with a fence) before each phase, so after a hang the last value shows
    /// which phase of which step never completed.
    pub const STAGE: u64 = 0x2_2800;

    /// Phases of one step, in order, then the run-level ones.
    pub mod phase {
        /// Step started, no `Config` store issued yet.
        pub const STEP_BEGIN: u32 = 1;
        /// All the step's `Config` stores issued and fenced.
        pub const CONFIG_WRITTEN: u32 = 2;
        /// `Config[212..=219]` read back into the result record.
        pub const READBACK_DONE: u32 = 3;
        /// Settle nops done and L0 flushed.
        pub const SETTLED: u32 = 4;
        /// About to execute the trigger load: if this is the last phase, the
        /// load never returned.
        pub const LOAD_ISSUED: u32 = 5;
        /// The load returned and its value is in the result record.
        pub const LOAD_RETURNED: u32 = 6;
        /// All steps done and the block disarmed (step field = step count).
        pub const DISARMED: u32 = 7;
        /// The script was invalid; the block was disarmed before failing.
        pub const BAD_SCRIPT: u32 = 8;
        /// The opening disarm is about to run / has run (step field 0).
        pub const PROLOGUE: u32 = 9;

        /// A name for messages.
        pub const fn name(p: u32) -> &'static str {
            match p {
                STEP_BEGIN => "STEP_BEGIN",
                CONFIG_WRITTEN => "CONFIG_WRITTEN",
                READBACK_DONE => "READBACK_DONE",
                SETTLED => "SETTLED",
                LOAD_ISSUED => "LOAD_ISSUED",
                LOAD_RETURNED => "LOAD_RETURNED",
                DISARMED => "DISARMED",
                BAD_SCRIPT => "BAD_SCRIPT",
                PROLOGUE => "PROLOGUE",
                _ => "?",
            }
        }
    }

    /// Encode a breadcrumb.
    pub const fn stage(step: u32, phase: u32) -> u32 {
        (step << 8) | phase
    }
}

/// `Config` word of the block's first field (`L1CacheTagSearchAccel.md`).
pub const FIRST_WORD: u16 = 212;

/// Number of consecutive `Config` words the block uses (212..=219).
pub const WORD_COUNT: usize = 8;

/// `sw` target of `Config` word `word` (bank 0; the words are global).
pub const fn config_address(word: u16) -> u64 {
    g::L1_CACHE_TAG_SEARCH_ACCEL_Search_Enable.riscv_address(ConfigBank::Bank0)
        + (word as u64 - FIRST_WORD as u64) * 4
}

const TRIGGERS: [ConfigField; 5] = [
    g::L1_CACHE_TAG_SEARCH_ACCEL_Search_Enable,
    g::L1_CACHE_TAG_SEARCH_ACCEL_Tag_alloc,
    g::L1_CACHE_TAG_SEARCH_ACCEL_Tag_inv,
    g::L1_CACHE_TAG_SEARCH_ACCEL_Tag_inv_all,
    g::L1_CACHE_TAG_SEARCH_ACCEL_Data_Valid_chk,
];

/// One read-modify-write of a `Config` word: the bits in `mask` become `value`
/// and every other bit (fields the pinned header does not name) is preserved.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WriteStep {
    /// `Config` word index (212..=219).
    pub word: u16,
    /// Bits of the word this step owns.
    pub mask: u32,
    /// New value of those bits.
    pub value: u32,
}

/// Number of writes in a configuration: disarm (3), data (5), arm (3).
pub const PROGRAM_STEPS: usize = 11;

const fn trigger_mask(word: u16) -> u32 {
    let mut m = 0;
    let mut i = 0;
    while i < TRIGGERS.len() {
        if TRIGGERS[i].addr32() == word {
            m |= TRIGGERS[i].mask();
        }
        i += 1;
    }
    m
}

/// Writes that clear the five trigger bits and nothing else: the latch then holds
/// "everything off", and no read is intercepted. Run before and after a probe.
pub const DISARM: [WriteStep; 3] = [
    WriteStep {
        word: 212,
        mask: trigger_mask(212),
        value: 0,
    },
    WriteStep {
        word: 218,
        mask: trigger_mask(218),
        value: 0,
    },
    WriteStep {
        word: 219,
        mask: trigger_mask(219),
        value: 0,
    },
];
