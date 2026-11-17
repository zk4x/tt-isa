//! Blackhole: the architecture this crate was written for.
//!
//! A p100a and a p150 are the same architecture; what differs between parts is
//! harvesting, which the ARC reports and which no constant here can know.
//!
//! Spec: `BlackholeA0/TensixTile/BabyRISCV/README.md` for the tile's map,
//! `SoftReset.md` for reset control, `HiMemTile.md` for the NoC-visible aperture.

use super::{Arch, Cfg};
use crate::cfg::{ConfigField, ConfigSpan, ThreadConfigField};
use crate::isa::EncodeError;
use crate::tensix::Core;

/// The Tensix backend protocol: configuration writes, unit waits and the instruction
/// sequences that drive them.
///
/// This layer is the architecture's own, not the crate's: `WRCFG`/`SETC16`/`MOP`,
/// the packer and unpacker waits and the SFPU bodies have no counterpart with the
/// same shape on Quasar, where `SETRWC`, `STALLWAIT` and its own movers apply. An
/// architecture that shares the protocol inherits the module; one that does not
/// writes its own.
pub mod backend;
/// The SFPU: lane-register bodies over the generated encodings.
pub mod sfpu;
/// Tensix semaphores, and the waits built on them.
pub mod sync;
/// The L1 cache tag-search accelerator: its configuration words and the probe that
/// drives them.
pub mod tag_search;

/// The Blackhole architecture, as a type.
///
/// Zero-sized: which architecture a program is built for is a compile-time fact,
/// and everything it answers is an associated item.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Blackhole;

/// The configuration fields Blackhole's algorithms name.
///
/// The generated table is namespaced by section (`cfg::blackhole::alu::…`), which
/// generic code cannot reach; this is the typed reachability, and nothing else.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Fields;

impl Cfg for Fields {
    const STATE_RESET_EN: ConfigField = crate::cfg::blackhole::alu::STATE_RESET_EN;
    const RISC_DEST_ACCESS_CTRL_SEC0_fmt: ConfigField =
        crate::cfg::blackhole::alu::RISC_DEST_ACCESS_CTRL_SEC0_fmt;
    const RISC_DEST_ACCESS_CTRL_SEC1_fmt: ConfigField =
        crate::cfg::blackhole::alu::RISC_DEST_ACCESS_CTRL_SEC1_fmt;
    const RISC_DEST_ACCESS_CTRL_SEC2_fmt: ConfigField =
        crate::cfg::blackhole::alu::RISC_DEST_ACCESS_CTRL_SEC2_fmt;
    const PRNG_SEED_Seed_Val: ConfigField = crate::cfg::blackhole::global::PRNG_SEED_Seed_Val;
    const THCON_SEC0_REG0_TileDescriptor: ConfigSpan =
        crate::cfg::blackhole::thcon::THCON_SEC0_REG0_TileDescriptor;
    const THCON_SEC0_REG2_Out_data_format: ConfigField =
        crate::cfg::blackhole::thcon::THCON_SEC0_REG2_Out_data_format;

    const CFG_STATE_ID_StateID: ThreadConfigField =
        crate::cfg::blackhole::thread::CFG_STATE_ID_StateID;
    const CLR_DVALID_SrcA_Disable: ThreadConfigField =
        crate::cfg::blackhole::thread::CLR_DVALID_SrcA_Disable;
    const CLR_DVALID_SrcB_Disable: ThreadConfigField =
        crate::cfg::blackhole::thread::CLR_DVALID_SrcB_Disable;
}

impl Arch for Blackhole {
    type Cfg = Fields;

    const NAME: &'static str = "Blackhole";

    /// L1 scratchpad RAM. Instructions can only be fetched from here
    /// (`BabyRISCV/README.md:39`).
    const L1_BASE: u64 = 0x0000_0000;
    /// 1536 KiB per Tensix tile (`BabyRISCV/README.md:102`).
    const L1_SIZE: u64 = 1536 * 1024;
    const LOCAL_DATA_RAM_BASE: u64 = 0xFFB0_0000;
    const DEBUG_REGS_BASE: u64 = 0xFFB1_2000;

    const CFGREG_RD_CNTL: u64 = 0xFFB1_2058;
    const CFGREG_RDDATA: u64 = 0xFFB1_2078;

    /// `RISC_SOFT_RESET_0`: the five cores and the backend, one bit per unit
    /// (`SoftReset.md:10-36`).
    const SOFT_RESET_0: u64 = 0xFFB1_21B0;
    const DISABLE_RESET: u64 = 0xFFB1_2224;

    const TRISC0_RESET_PC: u64 = 0xFFB1_2228;
    const TRISC1_RESET_PC: u64 = 0xFFB1_222C;
    const TRISC2_RESET_PC: u64 = 0xFFB1_2230;
    const TRISC_RESET_PC_OVERRIDE: u64 = 0xFFB1_2234;
    const NCRISC_RESET_PC: u64 = 0xFFB1_2238;
    const NCRISC_RESET_PC_OVERRIDE: u64 = 0xFFB1_223C;

    /// Readable from T0, T1 and T2 only (`SyncUnit.md`, "RISCV access to
    /// semaphores").
    const SEMAPHORE_ACCESS: u64 = 0xFFE8_0020;

    /// `RISCV_DEBUG_REG_WALL_CLOCK_L`: reading it also latches the high half.
    const WALL_CLOCK_L: u64 = 0xFFB1_21F0;
    const WALL_CLOCK_L_PLUS_4: u64 = 0xFFB1_21F4;
    const WALL_CLOCK_H: u64 = 0xFFB1_21F8;
    /// `RISCV_DEBUG_REG_TIMESTAMP`: a store appends an event.
    const TIMESTAMP: u64 = 0xFFB1_21FC;
    const TIMESTAMP_CNTL: u64 = 0xFFB1_2200;
    const TIMESTAMP_STATUS: u64 = 0xFFB1_2204;
    const TIMESTAMP_BUF0_START: u64 = 0xFFB1_2208;
    const TIMESTAMP_BUF0_END: u64 = 0xFFB1_220C;

    /// First byte of the NoC-visible local-data-RAM aperture
    /// (`BabyRISCV/README.md:109`).
    const LOCAL_DATA_RAM_NOC_BASE: u64 = 0xFFB1_4000;
    /// One past its last byte (`README.md:116`).
    const LOCAL_DATA_RAM_NOC_END: u64 = 0xFFB1_E000;
    /// Address space each core is given inside it.
    const LOCAL_DATA_RAM_NOC_STRIDE: u64 = 0x2000;
    /// `BabyRISCV/README.md:152`: no NoC access lands inside this window.
    const LOCAL_RAM_ZEROING_CYCLES: u32 = 2048;

    /// From T0/T1/T2 this feeds that core's own thread; from B, thread 0 after
    /// the MOP expander (`PushTensixInstruction.md:5-9`).
    const INSTRN_BUF_BASE: u64 = 0xFFE4_0000;
    const INSTRN1_BUF_BASE: u64 = 0xFFE5_0000;
    const INSTRN2_BUF_BASE: u64 = 0xFFE6_0000;

    /// Base of the PCBuf / Manual TTSync / semaphore region
    /// (`BabyRISCV/README.md:126-128`).
    const PC_BUF_BASE: u64 = 0xFFE8_0000;
    const COPROCESSOR_DONE_CHECK: u64 = 0xFFE8_0000 + 0x04;
    const MOP_EXPANDER_DONE_CHECK: u64 = 0xFFE8_0000 + 0x08;
    const MOP_CFG_BASE: u64 = 0xFFB8_0000;
    const PC_BUF_HAZARD_END: u64 = 0xFFE8_0000 + 0xFFFF;

    /// Bits 0, 1 and 7 are the unpacker and move together; bit 10 is the matrix
    /// and vector unit, without which SFPU instructions are silently discarded
    /// (`SoftReset.md`). Bits at or above 24 have no effect.
    const BACKEND_RESET_MASK: u32 = {
        let mut mask = 0u32;
        let mut bit = 0;
        while bit < 24 {
            let is_riscv = bit == 11 || bit == 12 || bit == 13 || bit == 14 || bit == 18;
            if !is_riscv {
                mask |= 1 << bit;
            }
            bit += 1;
        }
        mask
    };

    /// `Config`, `ConfigDualWrite` and `ThreadConfig`, mapped one after the
    /// other (`BabyRISCV/README.md:135`).
    const TENSIX_CFG_BASE: u64 = 0xFFEF_0000;
    /// Mapped into T0/T1/T2's address space only (`Dst.md:105`).
    const DST_BASE: u64 = 0xFFBD_8000;

    fn cores() -> &'static [Core] {
        &Core::BLACKHOLE
    }

    fn soft_reset_bit(core: Core) -> Result<u32, EncodeError> {
        match core {
            Core::B => Ok(11),
            Core::T0 => Ok(12),
            Core::T1 => Ok(13),
            Core::T2 => Ok(14),
            Core::NC => Ok(18),
            Core::Dm0 | Core::Dm1 => Err(EncodeError::NoSuchCore { core: core.name() }),
        }
    }

    fn default_reset_pc(core: Core) -> Result<u32, EncodeError> {
        match core {
            Core::B => Ok(0x0_0000),
            Core::T0 => Ok(0x0_6000),
            Core::T1 => Ok(0x0_A000),
            Core::T2 => Ok(0x0_E000),
            Core::NC => Ok(0x1_2000),
            Core::Dm0 | Core::Dm1 => Err(EncodeError::NoSuchCore { core: core.name() }),
        }
    }

    /// **RISCV B has none**: its entry point is hardwired to L1 offset `0`, a
    /// linker-script constraint rather than a preference.
    fn reset_pc_register(core: Core) -> Result<Option<u64>, EncodeError> {
        match core {
            Core::B => Ok(None),
            Core::T0 => Ok(Some(Self::TRISC0_RESET_PC)),
            Core::T1 => Ok(Some(Self::TRISC1_RESET_PC)),
            Core::T2 => Ok(Some(Self::TRISC2_RESET_PC)),
            Core::NC => Ok(Some(Self::NCRISC_RESET_PC)),
            Core::Dm0 | Core::Dm1 => Err(EncodeError::NoSuchCore { core: core.name() }),
        }
    }

    fn reset_pc_override(core: Core) -> Result<Option<(u64, u32)>, EncodeError> {
        match core {
            Core::B => Ok(None),
            Core::T0 => Ok(Some((Self::TRISC_RESET_PC_OVERRIDE, 1 << 0))),
            Core::T1 => Ok(Some((Self::TRISC_RESET_PC_OVERRIDE, 1 << 1))),
            Core::T2 => Ok(Some((Self::TRISC_RESET_PC_OVERRIDE, 1 << 2))),
            Core::NC => Ok(Some((Self::NCRISC_RESET_PC_OVERRIDE, 1 << 0))),
            Core::Dm0 | Core::Dm1 => Err(EncodeError::NoSuchCore { core: core.name() }),
        }
    }

    /// The table in `PushTensixInstruction.md:5-9`. T0/T1/T2 each reach only their
    /// own thread, through the *same* buffer address: the pair identifies the
    /// thread, not the address. B reaches all three, one buffer each.
    fn pushed_to(core: Core, thread: u32) -> Result<u64, EncodeError> {
        match (core, thread) {
            (Core::B, 0) => Ok(Self::INSTRN_BUF_BASE),
            (Core::B, 1) => Ok(Self::INSTRN1_BUF_BASE),
            (Core::B, 2) => Ok(Self::INSTRN2_BUF_BASE),
            (Core::T0, 0) => Ok(Self::INSTRN_BUF_BASE),
            (Core::T1, 1) => Ok(Self::INSTRN_BUF_BASE),
            (Core::T2, 2) => Ok(Self::INSTRN_BUF_BASE),
            (Core::Dm0 | Core::Dm1, _) => Err(EncodeError::NoSuchCore { core: core.name() }),
            _ => Err(EncodeError::IllegalPush {
                core: core.name(),
                thread,
            }),
        }
    }

    fn dst_access_fmt(thread: u32) -> Result<ConfigField, EncodeError> {
        use crate::cfg::blackhole::alu;
        match thread {
            0 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC0_fmt),
            1 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC1_fmt),
            2 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC2_fmt),
            _ => Err(EncodeError::NoSuchThread { thread }),
        }
    }
}

// The Manual TTSync registers sit inside the load-adjacency hazard range, since
// that range is what callers use to decide whether a load is affected.
const _: () = assert!(Blackhole::COPROCESSOR_DONE_CHECK > Blackhole::PC_BUF_BASE);
const _: () = assert!(Blackhole::COPROCESSOR_DONE_CHECK < Blackhole::PC_BUF_HAZARD_END);
const _: () = assert!(Blackhole::MOP_EXPANDER_DONE_CHECK < Blackhole::PC_BUF_HAZARD_END);
