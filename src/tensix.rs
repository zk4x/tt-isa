//! The shared vocabulary of a Tensix tile: which cores there are, and which Tensix
//! thread each of them feeds.
//!
//! Everything else a tile has — addresses, sizes, reset bits, the push table — is a
//! fact about one *architecture*, and lives in [`crate::arch`]. This module holds
//! only what every architecture shares: the core identities and the thread
//! identities.
//!
//! Core identity cannot be discovered at run time — `mhartid` reads zero on every
//! baby RISC-V and `misa` lies (it claims RV32IMABFV when A, B, F and V are each
//! only partial) — so a core is named by the code that starts it, never probed.

/// Which baby RISC-V core in a Tensix tile.
///
/// The set is not the same on every architecture: Blackhole's tiles have
/// [`B`](Core::B), [`NC`](Core::NC) and three TRISCs; Quasar's have two data movers
/// ([`Dm0`](Core::Dm0), [`Dm1`](Core::Dm1)) and two TRISCs. Which of these a tile
/// has is [`Arch::cores`](crate::arch::Arch::cores), and asking a chip about a core
/// it does not have is
/// [`EncodeError::NoSuchCore`](crate::isa::EncodeError::NoSuchCore) rather than a
/// made-up answer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Core {
    /// RISCV B. Pushes Tensix instructions to all three threads.
    B,
    /// RISCV T0, feeding Tensix thread 0.
    T0,
    /// RISCV T1, feeding Tensix thread 1.
    T1,
    /// RISCV T2, feeding Tensix thread 2.
    T2,
    /// RISCV NC. Cannot push Tensix instructions at all, and cannot invalidate its
    /// own instruction cache.
    NC,
    /// Quasar's first data mover.
    Dm0,
    /// Quasar's second data mover.
    Dm1,
}

impl Core {
    /// The five cores of a Blackhole tile, in the order `SoftReset.md` lists them.
    pub const BLACKHOLE: [Core; 5] = [Core::B, Core::T0, Core::T1, Core::T2, Core::NC];

    /// The four cores of a Quasar tile: two movers and two TRISCs.
    ///
    /// Quasar has no NC and no third TRISC (`tt-2xx/quasar/core_config.h`,
    /// `dev_mem_map.h`).
    pub const QUASAR: [Core; 4] = [Core::Dm0, Core::Dm1, Core::T0, Core::T1];

    /// Every core any architecture in this crate has.
    pub const ALL: [Core; 7] = [
        Core::B,
        Core::T0,
        Core::T1,
        Core::T2,
        Core::NC,
        Core::Dm0,
        Core::Dm1,
    ];

    /// The core's name, as the specification and the tooling write it.
    pub const fn name(self) -> &'static str {
        match self {
            Core::B => "B",
            Core::T0 => "T0",
            Core::T1 => "T1",
            Core::T2 => "T2",
            Core::NC => "NC",
            Core::Dm0 => "DM0",
            Core::Dm1 => "DM1",
        }
    }
}

/// Bytes one 128-bit timestamper event occupies in L1.
///
/// The event format is the same on every architecture this crate describes, which
/// is why it is here rather than in [`crate::arch`]: the addresses differ, the
/// record does not.
pub const TIMESTAMP_EVENT_BYTES: u64 = 16;

/// Which Tensix thread an instruction is being pushed to.
///
/// A marker type rather than a number because the legal `(core, thread)` pairs are
/// a sparse table rather than a range — see
/// [`Arch::pushed_to`](crate::arch::Arch::pushed_to).
pub trait TensixThread {
    /// 0, 1 or 2. Needed where a thread index is data rather than a choice, such as
    /// the mailbox word the host and the image cross-check against each other.
    const INDEX: u32;
}

/// Tensix thread 0.
pub struct Thread0;
/// Tensix thread 1.
pub struct Thread1;
/// Tensix thread 2.
pub struct Thread2;

impl TensixThread for Thread0 {
    const INDEX: u32 = 0;
}
impl TensixThread for Thread1 {
    const INDEX: u32 = 1;
}
impl TensixThread for Thread2 {
    const INDEX: u32 = 2;
}
