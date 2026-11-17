//! The architectures this crate describes, one type each.
//!
//! A Tenstorrent chip is described here as a type that implements [`Arch`]: the
//! addresses, sizes and per-core facts of one architecture, and the functions that
//! answer questions the hardware poses (which core owns a reset bit, which buffer
//! reaches a Tensix thread). Nothing in this module is chosen at run time, and no
//! architecture is compiled out: everything is one artifact, and the caller picks a
//! type.
//!
//! # Why a trait rather than one struct per architecture alone
//!
//! A chip is not a table of numbers. Reading a mailbox, walking a read loop and
//! staging a program are algorithms over those numbers, and there is exactly one
//! implementation of each. `fn read_loop<A: Arch>(…)` compiles once per
//! architecture and is written once, which is what a plain struct per chip cannot
//! give: without a bound, a generic function cannot call `A::L1_SIZE`.
//!
//! The trait is deliberately **not** object-safe: [`Arch`] carries associated
//! `const`s, so there is no `&dyn Arch`, and none is wanted. A run-time choice
//! between architectures belongs to the caller, which holds a concrete type either
//! way.
//!
//! Field names keep the spelling `cfg_defines.h` gives them (`THCON_SEC0_REG0_TileDescriptor`,
//! `RISC_DEST_ACCESS_CTRL_SEC0_fmt`), so that a name in the specification can be
//! searched for here without translation — the same rule the generated tables follow.
#![allow(non_upper_case_globals)]
//! # What lives where
//!
//! Every constant here is an associated const, so a layout proof stays a
//! compile-time proof inside the impl (`const _: () = assert!(…)`). The one thing
//! that cannot be const is the *per-core* lookup — `fn default_reset_pc(core)` is a
//! match — so modules that need a per-core address in a `const` position name the
//! architecture's own constant instead.
//!
//! A question that a chip cannot answer — the reset bit of a core it does not have
//! — is [`EncodeError::NoSuchCore`], never a made-up number.

pub mod blackhole;

use crate::cfg::{ConfigField, ConfigSpan, ThreadConfigField};
use crate::isa::EncodeError;
use crate::tensix::Core;

/// The configuration fields an algorithm names, one implementation per
/// architecture.
///
/// The field table (`cfg::blackhole`, `cfg::quasar`) is namespaced by section, so
/// generic code cannot reach `alu::STATE_RESET_EN`: a module path is not a type. The
/// implementation of this trait is that reachability, and it carries only the fields
/// the algorithms use — a field nobody names does not belong here.
///
/// Names are kept exactly as `cfg_defines.h` spells them, as in the generated table.
pub trait Cfg {
    const STATE_RESET_EN: ConfigField;
    const RISC_DEST_ACCESS_CTRL_SEC0_fmt: ConfigField;
    const RISC_DEST_ACCESS_CTRL_SEC1_fmt: ConfigField;
    const RISC_DEST_ACCESS_CTRL_SEC2_fmt: ConfigField;
    const PRNG_SEED_Seed_Val: ConfigField;
    const THCON_SEC0_REG0_TileDescriptor: ConfigSpan;
    const THCON_SEC0_REG2_Out_data_format: ConfigField;

    const CFG_STATE_ID_StateID: ThreadConfigField;
    const CLR_DVALID_SrcA_Disable: ThreadConfigField;
    const CLR_DVALID_SrcB_Disable: ThreadConfigField;
}

/// One compute engine's register block inside a tile.
///
/// A tile may hold more than one of these — Quasar has four, at
/// `0x0180_0000 + 0x10000·n` (`tt-2xx/quasar/tensix_neo_reg.h`) — and the cores
/// that belong to one are reset through it. Blackhole has a single block for the
/// whole tile, so its engine 0 is the tile's own debug register block.
///
/// The four slots are the block's own core slots, in the block's order, not the
/// [`Core`] list's: on Blackhole they are T0, T1, T2 and NC, on Quasar trisc 0..3.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EngineRegs {
    /// The block's first byte, as another core or the host addresses it.
    ///
    /// This is the *window* view. The harts inside the engine reach the same
    /// registers through their own local map, which is
    /// [`Arch::DEBUG_REGS_BASE`] — the same number on Blackhole, a different one
    /// on Quasar.
    pub base: u64,
    /// Reset-PC register of each slot.
    pub reset_pc: [u64; 4],
    /// Reset-PC override register and enable bit of each slot.
    ///
    /// Per slot rather than a slice of one word: Blackhole's NC override lives in
    /// its own register (`0xFFB1_223C`) whose bit is 0, while T0..T2 share
    /// `0xFFB1_2234` with bits 0..3.
    pub reset_pc_override: [(u64, u32); 4],
}

/// One Tenstorrent architecture, as a type.
///
/// Implementors are zero-sized: an architecture is a compile-time fact, and the
/// functions below are associated rather than methods because an instance would
/// carry nothing.
pub trait Arch {
    /// The configuration fields this architecture's algorithms name.
    type Cfg: Cfg;

    /// The architecture's name, as the specification and the tooling write it.
    const NAME: &'static str;

    // --- The tile's address space, as seen from inside the tile ---
    //
    // These are the same numbers the host writes into a TLB window's
    // `local_offset`, so they serve both sides.

    /// L1 scratchpad RAM's base within the tile.
    const L1_BASE: u64;
    /// Bytes of L1 this architecture's tiles have.
    const L1_SIZE: u64;
    /// Local data RAM: each core's stack and small data.
    const LOCAL_DATA_RAM_BASE: u64;
    /// The tile's debug registers (reset control, timestamper, synchronisation).
    const DEBUG_REGS_BASE: u64;

    /// Reading this latches a configuration word into
    /// [`CFGREG_RDDATA`](Self::CFGREG_RDDATA).
    const CFGREG_RD_CNTL: u64;
    /// The latched configuration word.
    const CFGREG_RDDATA: u64;

    /// Clears a core's reset-PC override.
    const DISABLE_RESET: u64;

    /// Semaphore access from T0/T1/T2: a load reads, a store posts or gets.
    const SEMAPHORE_ACCESS: u64;

    // --- The debug timestamper ---

    /// The cycle counter's low half; reading it also latches the high half.
    const WALL_CLOCK_L: u64;
    /// The counter's live high half, for the multi-reader retry loop.
    const WALL_CLOCK_L_PLUS_4: u64;
    /// The high half latched by the last [`WALL_CLOCK_L`](Self::WALL_CLOCK_L) read.
    const WALL_CLOCK_H: u64;
    /// A store here appends a timestamper event.
    const TIMESTAMP: u64;
    /// Buffer enables and the sticky stream reset.
    const TIMESTAMP_CNTL: u64;
    /// Sticky full/overflow bits and buffer 0's write position.
    const TIMESTAMP_STATUS: u64;
    /// Buffer 0's first 16-byte unit.
    const TIMESTAMP_BUF0_START: u64;
    /// Buffer 0's last 16-byte unit.
    const TIMESTAMP_BUF0_END: u64;

    // --- Local data RAM, as the NoC sees it ---

    /// First byte of the NoC-visible local-data-RAM aperture.
    const LOCAL_DATA_RAM_NOC_BASE: u64;
    /// One past its last byte.
    const LOCAL_DATA_RAM_NOC_END: u64;
    /// Address space each core is given inside it.
    const LOCAL_DATA_RAM_NOC_STRIDE: u64;
    /// Cycles the local data RAM spends zeroing itself after reset is released.
    ///
    /// NoC accesses inside this window are silently discarded.
    const LOCAL_RAM_ZEROING_CYCLES: u32;

    // --- Pushing Tensix instructions ---

    /// Where a `sw` pushes an instruction to the pushing core's own thread.
    const INSTRN_BUF_BASE: u64;
    /// B's second thread buffer. Storing here from T0/T1/T2 hangs the RISC-V.
    const INSTRN1_BUF_BASE: u64;
    /// B's third thread buffer. Same hazard as
    /// [`INSTRN1_BUF_BASE`](Self::INSTRN1_BUF_BASE).
    const INSTRN2_BUF_BASE: u64;

    // --- PCBuf, Manual TTSync and the backend's reset mask ---

    /// Base of the PCBuf / Manual TTSync region.
    const PC_BUF_BASE: u64;
    /// Loading from here blocks until the thread's instructions have retired.
    const COPROCESSOR_DONE_CHECK: u64;
    /// Loading from here blocks until the MOP expander has finished.
    const MOP_EXPANDER_DONE_CHECK: u64;
    /// The MOP expander's configuration.
    const MOP_CFG_BASE: u64;
    /// One past the range whose loads wait on the thread's own instructions.
    const PC_BUF_HAZARD_END: u64;
    /// Bits of the reset register ([`reset`](Self::reset)) that hold the Tensix
    /// backend, as opposed to the cores, in reset.
    const BACKEND_RESET_MASK: u32;

    /// `Config`, `ConfigDualWrite` and `ThreadConfig`, mapped one after the other.
    const TENSIX_CFG_BASE: u64;
    /// Tensix `Dst`, mapped into T0/T1/T2's address space. Not reachable over the
    /// NoC.
    const DST_BASE: u64;

    // --- Per-core facts ---
    //
    // A core this architecture does not have is [`EncodeError::NoSuchCore`].

    /// The cores this architecture's tiles have, in the order the specification
    /// lists them.
    fn cores() -> &'static [Core];

    /// Register and bit holding `core` in reset.
    ///
    /// A register and a bit rather than an address, because the two do not always
    /// belong to the core's engine: on Quasar the 16 TRISCs are held by their own
    /// engine's `SOFT_RESET_0` (bits 11..15), while the eight movers are held by
    /// one tile-level register (`TENSIX_SOFT_RESET_ADDR`, bits 8..16) that no
    /// engine block contains.
    fn reset(core: Core) -> Result<(u64, u32), EncodeError>;

    /// The mask of [`reset`](Self::reset).
    fn soft_reset_mask(core: Core) -> Result<u32, EncodeError> {
        Ok(1 << Self::reset(core)?.1)
    }

    /// How many engine blocks a tile has: one on Blackhole, four on Quasar.
    const ENGINES: u8;

    /// The register block of engine `engine`, in tile order.
    ///
    /// `None` for an engine the tile does not have, so a constant table built
    /// over [`ENGINES`](Self::ENGINES) cannot ask for a fifth.
    fn engine(engine: u8) -> Option<EngineRegs>;

    /// Where `core` begins executing after reset is released.
    fn default_reset_pc(core: Core) -> Result<u32, EncodeError>;

    /// The register holding `core`'s reset PC, if it has one.
    fn reset_pc_register(core: Core) -> Result<Option<u64>, EncodeError>;

    /// The register and bit that let `core` start from an arbitrary address.
    fn reset_pc_override(core: Core) -> Result<Option<(u64, u32)>, EncodeError>;

    /// The address `core` stores a Tensix instruction to, to reach `thread`.
    ///
    /// `thread` is 0, 1 or 2 ([`TensixThread::INDEX`](crate::tensix::TensixThread)).
    /// The legal pairs are a sparse table, and the illegal ones hang the RISC-V
    /// unrecoverably, so they are refused here rather than documented.
    fn pushed_to(core: Core, thread: u32) -> Result<u64, EncodeError>;

    /// The configuration field carrying `thread`'s RISC-V view of `Dst`.
    fn dst_access_fmt(thread: u32) -> Result<ConfigField, EncodeError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensix::Core;

    type Bh = blackhole::Blackhole;

    #[test]
    fn blackhole_has_its_five_cores() {
        assert_eq!(Bh::cores(), &Core::BLACKHOLE);
        // Every core it claims answers every per-core question.
        for &core in Bh::cores() {
            Bh::reset(core).expect("a core has a reset bit");
            Bh::default_reset_pc(core).expect("a core has a reset PC");
            Bh::reset_pc_register(core).expect("a core answers about its register");
            Bh::reset_pc_override(core).expect("a core answers about its override");
        }
        // …and the cores it does not have are refused, not answered.
        for core in [Core::Dm(0), Core::Dm(7)] {
            assert_eq!(
                Bh::default_reset_pc(core),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
            assert_eq!(
                Bh::reset(core),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
        }
        // One engine block for the tile, and no second.
        assert_eq!(Bh::ENGINES, 1);
        assert!(Bh::engine(0).is_some());
        assert_eq!(Bh::engine(1), None);
    }

    #[test]
    fn reset_bits_are_distinct_and_leave_the_backend_alone() {
        let mut mask = 0u32;
        for &core in Bh::cores() {
            let m = Bh::soft_reset_mask(core).unwrap();
            assert_eq!(mask & m, 0, "{} reuses a reset bit", core.name());
            mask |= m;
            // The backend mask must not release a core, and a core's mask must not
            // release the backend: the two are separate callers.
            assert_eq!(
                Bh::BACKEND_RESET_MASK & m,
                0,
                "the backend mask would also release {}",
                core.name()
            );
        }
        assert_eq!(Bh::soft_reset_mask(Core::B), Ok(1 << 11));
        assert_eq!(Bh::soft_reset_mask(Core::NC), Ok(1 << 18));
        // Bit 10 is the matrix and vector unit; without it SFPU instructions are
        // silently discarded.
        assert_ne!(Bh::BACKEND_RESET_MASK & (1 << 10), 0);
        // The three unpacker bits move together.
        assert_eq!(Bh::BACKEND_RESET_MASK & 0b1000_0011, 0b1000_0011);
        // Nothing above bit 23 has an effect.
        assert_eq!(Bh::BACKEND_RESET_MASK >> 24, 0);
    }

    #[test]
    fn only_the_documented_push_cells_resolve() {
        // B reaches all three threads, one buffer each.
        assert_eq!(Bh::pushed_to(Core::B, 0), Ok(Bh::INSTRN_BUF_BASE));
        assert_eq!(Bh::pushed_to(Core::B, 1), Ok(Bh::INSTRN1_BUF_BASE));
        assert_eq!(Bh::pushed_to(Core::B, 2), Ok(Bh::INSTRN2_BUF_BASE));
        // T0/T1/T2 reach only their own thread, through the same address: the pair
        // identifies the thread, not the buffer.
        assert_eq!(Bh::pushed_to(Core::T0, 0), Ok(Bh::INSTRN_BUF_BASE));
        assert_eq!(Bh::pushed_to(Core::T1, 1), Ok(Bh::INSTRN_BUF_BASE));
        assert_eq!(Bh::pushed_to(Core::T2, 2), Ok(Bh::INSTRN_BUF_BASE));

        // The other cells hang the RISC-V unrecoverably, so they are refused.
        for (core, thread) in [
            (Core::T0, 1),
            (Core::T0, 2),
            (Core::T1, 0),
            (Core::T1, 2),
            (Core::T2, 0),
            (Core::T2, 1),
            (Core::NC, 0),
            (Core::NC, 2),
        ] {
            assert_eq!(
                Bh::pushed_to(core, thread),
                Err(EncodeError::IllegalPush {
                    core: core.name(),
                    thread
                }),
                "{} -> {thread}",
                core.name()
            );
        }
        for core in [Core::Dm(0), Core::Dm(7)] {
            assert_eq!(
                Bh::pushed_to(core, 0),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
        }
    }

    #[test]
    fn each_thread_has_its_own_dst_access_format() {
        let fields = [0u32, 1, 2].map(|thread| Bh::dst_access_fmt(thread).unwrap());
        let shamts = fields.map(|f| f.shamt());
        assert!(
            shamts[0] != shamts[1] && shamts[1] != shamts[2] && shamts[0] != shamts[2],
            "the three fields must be distinct: {shamts:?}"
        );
        assert_eq!(
            Bh::dst_access_fmt(3),
            Err(EncodeError::NoSuchThread { thread: 3 })
        );
    }

    #[test]
    fn a_tiles_l1_layout_fits_the_architecture() {
        // The mailbox and the data arena are placed inside L1 rather than past it.
        assert!(crate::mailbox::MAILBOX_BASE + crate::mailbox::MAILBOX_SIZE <= Bh::L1_SIZE);
        assert!(crate::l1::DATA.end <= crate::l1::PROGRAM_CACHE.base);
        assert_eq!(crate::l1::PROGRAM_CACHE.end, Bh::L1_SIZE);
    }
}
