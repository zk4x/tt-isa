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
pub mod quasar;

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
/// Only fields **both** generated tables contain belong here. The `THCON` tile
/// descriptor and the 16-bit `ThreadConfig` fields are Blackhole's alone: Quasar's
/// `cfg_defines.h` has no `ThreadConfig` section at all and names its unpackers
/// `THCON_UNPACKER0..2`, so those five fields live in
/// [`BlackholeSpecials`] instead.
///
/// Names are kept exactly as `cfg_defines.h` spells them, as in the generated table.
pub trait Cfg {
    const STATE_RESET_EN: ConfigField;
    const RISC_DEST_ACCESS_CTRL_SEC0_fmt: ConfigField;
    const RISC_DEST_ACCESS_CTRL_SEC1_fmt: ConfigField;
    const RISC_DEST_ACCESS_CTRL_SEC2_fmt: ConfigField;
    const PRNG_SEED_Seed_Val: ConfigField;
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

/// What one architecture has and another does not.
///
/// One type, not a trait per chip and not an `Option` per mechanism: a mechanism
/// is either in a variant's payload, because the chip has it, or absent from the
/// type, because it does not. A caller that needs a special matches the variant,
/// so a use on the wrong architecture is a compile error at the call site rather
/// than a `None` to be handled at every use.
///
/// Which variant an architecture answers with is
/// [`Arch::SPECIALS`], and it is the only place a chip's own mechanisms are
/// named.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Specials {
    /// Blackhole's own mechanisms.
    Blackhole(BlackholeSpecials),
    /// Quasar's own mechanisms.
    ///
    /// Nothing is catalogued here yet: the ones known so far (the tile-level
    /// reset register and the per-engine window stride) are reachable through
    /// [`Arch::reset`] and [`Arch::engine`], so a field would duplicate them.
    Quasar,
}

/// The parts of a Blackhole tile that Quasar's headers do not describe.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BlackholeSpecials {
    /// A store here appends a timestamper event.
    pub timestamp: u64,
    /// Buffer enables and the sticky stream reset.
    pub timestamp_cntl: u64,
    /// Sticky full/overflow bits and buffer 0's write position.
    pub timestamp_status: u64,
    /// Buffer 0's first 16-byte unit.
    pub timestamp_buf0_start: u64,
    /// Buffer 0's last 16-byte unit.
    pub timestamp_buf0_end: u64,

    /// Reading this latches a configuration word into
    /// [`BlackholeSpecials::cfgreg_rddata`].
    ///
    /// Quasar's headers define no configuration read instruction, which is why
    /// the pair is here rather than on the trait.
    pub cfgreg_rd_cntl: u64,
    /// The latched configuration word.
    pub cfgreg_rddata: u64,

    /// Loading from here blocks until the thread's instructions have retired.
    pub coprocessor_done_check: u64,
    /// Loading from here blocks until the MOP expander has finished.
    pub mop_expander_done_check: u64,
    /// One past the range whose loads wait on the thread's own instructions.
    pub pc_buf_hazard_end: u64,

    /// Semaphore access from T0/T1/T2: a load reads, a store posts or gets.
    pub semaphore_access: u64,

    /// First byte of the NoC-visible local-data-RAM aperture.
    pub local_data_ram_noc_base: u64,
    /// One past its last byte.
    pub local_data_ram_noc_end: u64,
    /// Address space each core is given inside it.
    pub local_data_ram_noc_stride: u64,
    /// Cycles the local data RAM spends zeroing itself after reset is released.
    ///
    /// NoC accesses inside this window are silently discarded.
    pub local_ram_zeroing_cycles: u32,

    /// B's second instruction buffer, reaching Tensix thread 1.
    ///
    /// Storing here from T0/T1/T2 hangs the RISC-V (`PushTensixInstruction.md:8`).
    pub instrn1_buf: u64,
    /// B's third instruction buffer, reaching Tensix thread 2.
    ///
    /// Same hazard as [`BlackholeSpecials::instrn1_buf`].
    pub instrn2_buf: u64,

    /// The `THCON` tile descriptor: four words describing how an unpacker walks
    /// its input.
    ///
    /// Quasar's table has no equivalent aggregate — its `THCON` section is
    /// `THCON_UNPACKER0..2` with per-field registers and no 128-bit descriptor.
    pub thcon_tile_descriptor: ConfigSpan,
    /// The `THCON` output data format of the tile descriptor's consumer.
    pub thcon_out_data_format: ConfigField,

    /// `ThreadConfig[CurrentThread].CFG_STATE_ID_StateID`: which `Config` bank the
    /// thread is reading.
    ///
    /// Quasar has no `ThreadConfig` variable at all, so the 16-bit fields are
    /// Blackhole's.
    pub cfg_state_id: ThreadConfigField,
    /// `ThreadConfig` bit that stops the thread clearing `SrcA`'s dvalid.
    pub clr_dvalid_srca_disable: ThreadConfigField,
    /// `ThreadConfig` bit that stops the thread clearing `SrcB`'s dvalid.
    pub clr_dvalid_srcb_disable: ThreadConfigField,
}

impl BlackholeSpecials {
    /// The address `core` stores a Tensix instruction to, to reach `thread`.
    ///
    /// The table in `PushTensixInstruction.md:5-9`. T0/T1/T2 each reach only their
    /// own thread, through the *same* buffer address: the pair identifies the
    /// thread, not the address. B reaches all three, one buffer each. The cells no
    /// core may write hang the RISC-V unrecoverably, so they are refused here
    /// rather than documented.
    ///
    /// This is Blackhole's alone. Quasar has no cross-hart push: every hart writes
    /// its own buffer, so there is no table to consult.
    pub fn pushed_to(&self, core: Core, thread: u32) -> Result<u64, EncodeError> {
        match (core, thread) {
            (Core::B, 0) | (Core::T0, 0) => Ok(blackhole::Blackhole::INSTRN_BUF_BASE),
            (Core::B, 1) => Ok(self.instrn1_buf),
            (Core::B, 2) => Ok(self.instrn2_buf),
            (Core::T1, 1) | (Core::T2, 2) => Ok(blackhole::Blackhole::INSTRN_BUF_BASE),
            (Core::Dm(_) | Core::Math { .. }, _) => {
                Err(EncodeError::NoSuchCore { core: core.name() })
            }
            _ => Err(EncodeError::IllegalPush {
                core: core.name(),
                thread,
            }),
        }
    }
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

    /// The mechanisms this architecture has and another does not.
    const SPECIALS: Specials;

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

    /// Clears a core's reset-PC override.
    const DISABLE_RESET: u64;

    // --- The debug timestamper ---

    /// The cycle counter's low half; reading it also latches the high half.
    const WALL_CLOCK_L: u64;
    /// The counter's live high half, for the multi-reader retry loop.
    const WALL_CLOCK_L_PLUS_4: u64;
    /// The high half latched by the last [`WALL_CLOCK_L`](Self::WALL_CLOCK_L) read.
    const WALL_CLOCK_H: u64;

    // --- Local data RAM, as the NoC sees it ---
    //
    // Blackhole's NoC reaches another core's local data RAM through a dedicated
    // aperture; see `Specials::Blackhole`.

    // --- Pushing Tensix instructions ---

    /// Where a `sw` pushes an instruction to the pushing core's own thread.
    const INSTRN_BUF_BASE: u64;

    // --- PCBuf, Manual TTSync and the backend's reset mask ---

    /// Base of the PCBuf / Manual TTSync region.
    const PC_BUF_BASE: u64;
    /// The MOP expander's configuration.
    const MOP_CFG_BASE: u64;
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

    /// The configuration field carrying `thread`'s RISC-V view of `Dst`.
    fn dst_access_fmt(thread: u32) -> Result<ConfigField, EncodeError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensix::Core;

    type Bh = blackhole::Blackhole;

    /// Blackhole's own mechanisms, unwrapped: the tests below that name a
    /// Blackhole-only register go through this.
    fn bh_specials() -> BlackholeSpecials {
        match Bh::SPECIALS {
            Specials::Blackhole(s) => s,
            Specials::Quasar => panic!("Blackhole answers with Blackhole's specials"),
        }
    }

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

    /// The specials are one value, and an architecture answers with its own
    /// variant — not with an `Option` whose absence every caller has to handle.
    #[test]
    fn each_architecture_answers_with_its_own_specials() {
        match Bh::SPECIALS {
            Specials::Blackhole(s) => {
                assert_eq!(s.semaphore_access, 0xFFE8_0020);
                assert_eq!(s.cfgreg_rd_cntl, 0xFFB1_2058);
                assert_eq!(s.timestamp, 0xFFB1_21FC);
                assert!(s.local_data_ram_noc_base < s.local_data_ram_noc_end);
                assert!(s.coprocessor_done_check < s.pc_buf_hazard_end);
                assert!(s.mop_expander_done_check < s.pc_buf_hazard_end);
            }
            Specials::Quasar => panic!("Blackhole answers with Blackhole's specials"),
        }
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
        let s = bh_specials();
        // B reaches all three threads, one buffer each.
        assert_eq!(s.pushed_to(Core::B, 0), Ok(Bh::INSTRN_BUF_BASE));
        assert_eq!(s.pushed_to(Core::B, 1), Ok(s.instrn1_buf));
        assert_eq!(s.pushed_to(Core::B, 2), Ok(s.instrn2_buf));
        // T0/T1/T2 reach only their own thread, through the same address: the pair
        // identifies the thread, not the buffer.
        assert_eq!(s.pushed_to(Core::T0, 0), Ok(Bh::INSTRN_BUF_BASE));
        assert_eq!(s.pushed_to(Core::T1, 1), Ok(Bh::INSTRN_BUF_BASE));
        assert_eq!(s.pushed_to(Core::T2, 2), Ok(Bh::INSTRN_BUF_BASE));

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
                s.pushed_to(core, thread),
                Err(EncodeError::IllegalPush {
                    core: core.name(),
                    thread
                }),
                "{} -> {thread}",
                core.name()
            );
        }
        // A Quasar hart is not a core this table knows.
        for core in [Core::Dm(0), Core::Dm(7)] {
            assert_eq!(
                s.pushed_to(core, 0),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
        }
        assert_eq!(
            s.pushed_to(
                Core::Math {
                    engine: 3,
                    trisc: 3
                },
                0
            ),
            Err(EncodeError::NoSuchCore { core: "E3_MATH3" })
        );
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
        assert!(
            crate::arch::blackhole::mailbox::MAILBOX_BASE
                + crate::arch::blackhole::mailbox::MAILBOX_SIZE
                <= Bh::L1_SIZE
        );
        assert!(
            crate::arch::blackhole::l1::DATA.end <= crate::arch::blackhole::l1::PROGRAM_CACHE.base
        );
        assert_eq!(crate::arch::blackhole::l1::PROGRAM_CACHE.end, Bh::L1_SIZE);
    }

    type Qsr = quasar::Quasar;

    #[test]
    fn quasar_has_its_twenty_four_harts() {
        assert_eq!(Qsr::cores(), &Core::QUASAR);
        // Every hart it claims answers every per-core question.
        for &core in Qsr::cores() {
            Qsr::reset(core).expect("a hart has a reset bit");
            Qsr::default_reset_pc(core).expect("a hart has a reset PC");
            Qsr::reset_pc_register(core).expect("a hart answers about its register");
            Qsr::reset_pc_override(core).expect("a hart answers about its override");
        }
        // Blackhole's five are not this chip's.
        for core in Core::BLACKHOLE {
            assert_eq!(
                Qsr::default_reset_pc(core),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
        }
    }

    /// A TRISC is held by its own engine's `SOFT_RESET_0` at `+0xB8`, bits 11..15;
    /// a mover by the tile-level reset register, bits 8..16. There is no register
    /// that holds both (`grendel_implementation.hpp` `SOFT_RESET_DM0..DM7`).
    #[test]
    fn quasar_resets_a_trisc_in_its_engine_and_a_mover_in_the_tile() {
        for engine in 0..4u8 {
            let regs = Qsr::engine(engine).unwrap();
            for trisc in 0..4u8 {
                let core = Core::Math { engine, trisc };
                assert_eq!(
                    Qsr::reset(core),
                    Ok((regs.base + 0xB8, 11 + u32::from(trisc)))
                );
                assert_eq!(
                    Qsr::reset_pc_register(core),
                    Ok(Some(regs.reset_pc[trisc as usize]))
                );
                assert_eq!(
                    Qsr::reset_pc_override(core),
                    Ok(Some(regs.reset_pc_override[trisc as usize]))
                );
            }
        }
        for hart in 0..8u8 {
            assert_eq!(
                Qsr::reset(Core::Dm(hart)),
                Ok((0x0301_79B0, 8 + u32::from(hart)))
            );
            // A mover has no reset-PC register: it starts from L1 word 0.
            assert_eq!(Qsr::reset_pc_register(Core::Dm(hart)), Ok(None));
            assert_eq!(Qsr::reset_pc_override(Core::Dm(hart)), Ok(None));
        }
        assert_eq!(Qsr::engine(4), None);
    }

    #[test]
    fn quasar_engines_are_64k_apart_and_their_slots_consecutive() {
        for engine in 0..4u8 {
            let regs = Qsr::engine(engine).unwrap();
            assert_eq!(regs.base, 0x0180_0000 + 0x1_0000 * u64::from(engine));
            assert_eq!(regs.reset_pc[0], regs.base + 0x100);
            assert_eq!(regs.reset_pc_override[0], (regs.base + 0x110, 1));
            for slot in 1..4 {
                assert_eq!(regs.reset_pc[slot], regs.reset_pc[slot - 1] + 4);
                // One override register, one bit per TRISC.
                assert_eq!(regs.reset_pc_override[slot].0, regs.base + 0x110);
                assert_eq!(regs.reset_pc_override[slot].1, 1 << slot);
            }
        }
    }

    #[test]
    fn quasar_starts_a_mover_at_its_firmware_and_a_trisc_at_its_slot() {
        for hart in 0..8u8 {
            assert_eq!(Qsr::default_reset_pc(Core::Dm(hart)), Ok(0x000E_780));
        }
        for trisc in 0..4u8 {
            assert_eq!(
                Qsr::default_reset_pc(Core::Math { engine: 0, trisc }),
                Ok(0x0001_1780 + 5 * 1024 * u32::from(trisc))
            );
        }
        // The engine does not move a TRISC's entry point: the four slots are one
        // image per role, shared across engines.
        assert_eq!(
            Qsr::default_reset_pc(Core::Math {
                engine: 3,
                trisc: 0
            }),
            Qsr::default_reset_pc(Core::Math {
                engine: 0,
                trisc: 0
            })
        );
    }

    #[test]
    fn quasar_has_one_dst_section_per_trisc() {
        let fields = [0u32, 1, 2, 3].map(|t| Qsr::dst_access_fmt(t).unwrap());
        // Word and shift together: SEC0 and SEC3 share a shift but sit in
        // different words of `Config`, so the shift alone does not identify one.
        let keys = fields.map(|f| (f.addr32(), f.shamt()));
        for (i, a) in keys.iter().enumerate() {
            for b in &keys[i + 1..] {
                assert_ne!(a, b, "two Dst sections share a field: {keys:?}");
            }
        }
        assert_eq!(
            Qsr::dst_access_fmt(4),
            Err(EncodeError::NoSuchThread { thread: 4 })
        );
    }

    /// The two chips share a `Core` and nothing else: each refuses every hart the
    /// other has, and each answers with its own specials.
    #[test]
    fn the_two_architectures_refuse_each_others_harts() {
        for core in Core::QUASAR {
            assert_eq!(
                Bh::reset(core),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
            assert!(Qsr::reset(core).is_ok(), "{}", core.name());
        }
        for core in Core::BLACKHOLE {
            assert_eq!(
                Qsr::reset(core),
                Err(EncodeError::NoSuchCore { core: core.name() })
            );
            assert!(Bh::reset(core).is_ok(), "{}", core.name());
        }
        assert_eq!(Qsr::SPECIALS, Specials::Quasar);
        assert_ne!(Bh::SPECIALS, Specials::Quasar);
        assert_eq!(Qsr::ENGINES, 4);
        assert_eq!(Qsr::L1_SIZE, 4 * 1024 * 1024);
        assert_eq!(Bh::L1_SIZE, 1536 * 1024);
    }
}
