//! Quasar's stall resources and semaphores, and the hand-offs built on them.
//!
//! The pieces are the same as Blackhole's — `STALLWAIT`, `SEMINIT`, `SEMPOST`,
//! `SEMGET`, `SEMWAIT` — but the fields are not: Quasar's `STALLWAIT` and
//! `SEMWAIT` carry a *resource index* per unit (`p_stall`, one 5-bit number each,
//! up to three) instead of Blackhole's block/condition masks, and one 9-bit
//! `stall_res` mask names the instruction classes the wait holds back. The
//! numbers below are `tt_llk_quasar/common/inc/ckernel_instr_params.h`'s
//! `p_stall`, and the sequences are the LLK's own: `_llk_sync_*`
//! (`llk_sync.h`), `t6_semaphore_*` (`ckernel_trisc_common.h`), and the
//! per-thread `hw::t6_semaphore_*` in
//! `hw/inc/api/compute/experimental/semaphore_compute_impl.h:113-137`, which is
//! where each thread's idle-resource list comes from.
//!
//! There is no semaphore aperture to read here: a `SEMWAIT` latches in the
//! issuing hart's Wait Gate, and a value is only readable through the Sync
//! Unit's own CSR, which this layer does not use.

use crate::isa::quasar::encode;
use crate::isa::{EncodeError, Instruction};

/// Which instruction classes a `STALLWAIT`/`SEMWAIT` holds back
/// (`p_stall`'s `STALL_*`, `ckernel_instr_params.h:266-284`).
pub mod block {
    /// Data movement: on a hart that drives one, its `PACR`/`UNPACR` streams.
    pub const TDMA: u32 = 0x1;
    /// The Sync Unit itself. Every wait and every `SEMPOST`/`SEMGET` behind one
    /// names it, so a later semaphore op cannot overtake an earlier wait.
    pub const SYNC: u32 = 0x2;
    pub const PACK: u32 = 0x4;
    pub const UNPACK: u32 = 0x8;
    pub const THCON: u32 = 0x20;
    pub const MATH: u32 = 0x40;
    pub const CFG: u32 = 0x80;
    pub const SFPU: u32 = 0x100;
}

/// The `wait_sem_cond` field of `SEMWAIT` (`p_stall::STALL_ON_*`).
pub mod cond {
    /// Released while the semaphore is non-zero.
    pub const STALL_ON_ZERO: u32 = 0x1;
    /// Released while the semaphore is below its maximum.
    pub const STALL_ON_MAX: u32 = 0x2;
}

/// Wait resource indices (`p_stall`'s unit names, `ckernel_instr_params.h:185-245`).
///
/// The index is a *unit*, not a mask: `STALLWAIT` takes up to three, and 3, 5, 7,
/// 9, 11 and 20 decode to nothing at all (the LLK declares them as non-integers so
/// a use cannot compile). They are absent here for the same reason.
pub mod res {
    pub const NOTHING: u32 = 0;
    pub const THCON: u32 = 1;
    pub const UNPACK0: u32 = 2;
    pub const UNPACK1: u32 = 4;
    pub const UNPACK2: u32 = 6;
    pub const PACK0: u32 = 8;
    pub const PACK1: u32 = 10;
    pub const MATH: u32 = 12;
    pub const SRCA_VLD: u32 = 17;
    pub const SRCB_VLD: u32 = 18;
    /// `TRISC_CFG`: a hart's own configuration writes have reached the `Config` array.
    pub const TRISC_CFG: u32 = 21;
    /// `SFPU1`; the LLK's `WAIT_SFPU` is this one.
    pub const SFPU: u32 = 22;
    pub const CFGEXU: u32 = 23;
}

/// `STALLWAIT`: block `stall_res` until every named resource is idle.
pub fn stallwait(stall_res: u32, waits: &[u32]) -> Result<Instruction, EncodeError> {
    let at = |i: usize| waits.get(i).copied().unwrap_or(res::NOTHING);
    encode::stallwait(stall_res, at(2), at(1), at(0))
}

/// One of the Sync Unit's semaphores, in bank 0.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Semaphore(u8);

impl Semaphore {
    /// Bounds of one bank of eight (`_llk_sync_wait_`'s "indices in [0, 7]").
    pub const COUNT: u8 = 8;

    pub const fn new(index: u8) -> Option<Semaphore> {
        if index < Self::COUNT {
            Some(Semaphore(index))
        } else {
            None
        }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    /// The `sem_sel` field's one-hot selection of this semaphore.
    pub const fn mask(self) -> u32 {
        1 << self.0
    }
}

/// The largest `Value`/`Max` a semaphore holds: both are four bits.
pub const MAX_VALUE: u8 = 15;

const fn ok(r: Result<Instruction, EncodeError>) -> Instruction {
    match r {
        Ok(instruction) => instruction,
        // Every operand below is in range by construction.
        Err(_) => panic!("semaphore operands are in range by construction"),
    }
}

/// `SEMINIT`: seed `sem` with `value` and `max` (`_llk_sync_init_`).
pub const fn init(sem: Semaphore, value: u8, max: u8) -> Result<Instruction, EncodeError> {
    if value > MAX_VALUE {
        return Err(EncodeError::FieldTooLarge {
            instruction: "SEMINIT",
            field: "init_value",
            value: value as u32,
            width: 4,
        });
    }
    if max > MAX_VALUE {
        return Err(EncodeError::FieldTooLarge {
            instruction: "SEMINIT",
            field: "max_value",
            value: max as u32,
            width: 4,
        });
    }
    Ok(ok(encode::seminit(max as u32, value as u32, 0, sem.mask())))
}

/// `SEMPOST`: increment `sem`, saturating at its maximum.
pub const fn post(sem: Semaphore) -> Instruction {
    ok(encode::sempost(0, sem.mask()))
}

/// `SEMGET`: decrement `sem`, saturating at zero.
pub const fn get(sem: Semaphore) -> Instruction {
    ok(encode::semget(0, sem.mask()))
}

/// `SEMWAIT`: hold back `stall_res` until `sem` satisfies `cond`.
pub const fn wait(sem: Semaphore, cond: u32, stall_res: u32) -> Instruction {
    ok(encode::semwait(stall_res, cond, 0, sem.mask()))
}

/// `STALLWAIT(SYNC, idle..)` then `SEMPOST`: announce that this hart's `idle`
/// units have drained, so the post cannot overtake their work.
///
/// This is `t6_semaphore_post<kIdle0, kIdle1, kIdle2>`
/// (`ckernel_trisc_common.h:317-337`): the stall names the Sync Unit, which is
/// the class the `SEMPOST` behind it belongs to.
pub fn post_after(sem: Semaphore, idle: &[u32]) -> [Instruction; 2] {
    [ok(stallwait(block::SYNC, idle)), post(sem)]
}

/// [`post_after`]'s consumer half: `STALLWAIT(SYNC, idle..)` then `SEMGET`.
pub fn get_after(sem: Semaphore, idle: &[u32]) -> [Instruction; 2] {
    [ok(stallwait(block::SYNC, idle)), get(sem)]
}

/// `STALLWAIT(SYNC, idle..)` then `SEMWAIT`: hold back this hart's own
/// `stall_res` classes until `sem` satisfies `cond`.
pub fn wait_after(sem: Semaphore, cond: u32, stall_res: u32, idle: &[u32]) -> [Instruction; 2] {
    [ok(stallwait(block::SYNC, idle)), wait(sem, cond, stall_res)]
}

/// Wait for a post, then take it: [`wait_after`] followed by the `SEMGET` the
/// wait holds back.
pub fn take_after(sem: Semaphore, cond: u32, stall_res: u32, idle: &[u32]) -> [Instruction; 3] {
    [
        ok(stallwait(block::SYNC, idle)),
        wait(sem, cond, stall_res),
        get(sem),
    ]
}

/// `STALLWAIT(SYNC, idle..)` then `SEMINIT` (`t6_semaphore_init`).
pub fn init_after(
    sem: Semaphore,
    value: u8,
    max: u8,
    idle: &[u32],
) -> Result<[Instruction; 2], EncodeError> {
    Ok([ok(stallwait(block::SYNC, idle)), init(sem, value, max)?])
}
