//! Quasar: 24 harts in four compute engines, whose movers are not Tensix threads.
//!
//! The tile is not one register block and five cores. It has four *compute engines*
//! ("neos"), each with four TRISCs, its own reset register, its own 32 KiB `DEST`
//! and its own register window at `0x0180_0000 + 0x10000·n`; and eight data movers
//! that belong to no single engine. Nothing crosses: a hart issues Tensix
//! instructions only for itself, so there is no push table here.
//!
//! Spec: `tt-2xx/quasar/{core_config.h,dev_mem_map.h,tt_t6_trisc_map.h,tensix.h,
//! tensix_neo_reg.h,t6_debug_map.h}` in tt-metal, and `grendel_implementation.hpp`
//! in tt-umd for the tile-level reset register and the mover bits.

use super::{Arch, Cfg, EngineRegs, Specials};
use crate::cfg::ConfigField;
use crate::isa::EncodeError;
use crate::tensix::Core;

/// The Quasar architecture, as a type.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Quasar;

/// The configuration fields Quasar's generated table offers the fields both
/// architectures name.
///
/// Its own backend is not written yet, so this is the shared five and nothing
/// else: a field nobody names does not belong here.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Fields;

impl Cfg for Fields {
    const STATE_RESET_EN: ConfigField = crate::cfg::quasar::alu::STATE_RESET_EN;
    const RISC_DEST_ACCESS_CTRL_SEC0_fmt: ConfigField =
        crate::cfg::quasar::alu::RISC_DEST_ACCESS_CTRL_SEC0_fmt;
    const RISC_DEST_ACCESS_CTRL_SEC1_fmt: ConfigField =
        crate::cfg::quasar::alu::RISC_DEST_ACCESS_CTRL_SEC1_fmt;
    const RISC_DEST_ACCESS_CTRL_SEC2_fmt: ConfigField =
        crate::cfg::quasar::alu::RISC_DEST_ACCESS_CTRL_SEC2_fmt;
    const PRNG_SEED_Seed_Val: ConfigField = crate::cfg::quasar::global::PRNG_SEED_Seed_Val;
}

/// Stall resources, semaphores and the hand-offs built on them.
pub mod backend;
/// The two DRAM channels and their endpoints.
pub mod dram;
/// How a hart issues a Tensix instruction and a mover issues a NoC transaction.
pub mod issue;
/// The fixed regions of a tile's 4 MiB L1.
pub mod l1;
/// The L1 contract between host and firmware.
pub mod mailbox;
/// NoC coordinates, the one NIU's register block and the fixed grid.
pub mod noc;
/// The SFPU's register-file formats and immediate encodings.
pub mod sfpu;

/// Where a hart's reset bit lives, as another core or the host sees it.
const TILE_RESET: u64 = 0x0301_79B0;

/// First engine window, and the stride between windows.
const ENGINE_BASE: u64 = 0x0180_0000;
const ENGINE_STRIDE: u64 = 0x0001_0000;

/// Offset of `SOFT_RESET_0` inside an engine window.
const SOFT_RESET_0: u64 = 0x00B8;
/// Offset of TRISC0's reset-PC register; the four are consecutive words.
const TRISC_RESET_PC: u64 = 0x0100;
/// Offset of the TRISC reset-PC override register.
const TRISC_RESET_PC_OVERRIDE: u64 = 0x0110;

/// `MEM_DM_FIRMWARE_BASE`: where the movers' firmware begins.
///
/// The chain is `MEM_MAILBOX_BASE` 16 + `MEM_MAILBOX_SIZE` 59904 = `0xEA10`,
/// rounded to `0xEA20`; `MEM_LLK_DEBUG_BASE` + 1 KiB = `0xEE20`, rounded to
/// `MEM_INTERRUPT_TABLE_BASE` `0xEF00`; + `MEM_INTERRUPT_TABLE_SIZE` 256 B =
/// `0xF000` (`dev_mem_map.h:103-113`). All eight movers share the one 12 KiB
/// image there; each is told which hart it is.
const DM_FIRMWARE: u32 = 0x0000_F000;
/// `MEM_TRISC0_FIRMWARE_BASE`, and the stride to the next TRISC's
/// (`dev_mem_map.h:114-117`, `MEM_TRISC_FIRMWARE_SIZE` = 5 KiB).
const TRISC0_FIRMWARE: u32 = 0x0001_2000;
const TRISC_FIRMWARE_STRIDE: u32 = 5 * 1024;

impl Arch for Quasar {
    type Cfg = Fields;

    const NAME: &'static str = "Quasar";

    /// Quasar's own mechanisms are all reachable through the trait: the tile reset
    /// register comes back from [`Arch::reset`], the engine windows from
    /// [`Arch::engine`], and the movers' firmware bases from
    /// [`Arch::default_reset_pc`].
    const SPECIALS: Specials = Specials::Quasar;

    const L1_BASE: u64 = 0x0000_0000;
    /// `MEM_L1_SIZE`: 4 MiB, with a second 4 MiB uncached alias above it
    /// (`dev_mem_map.h:32-36`). The alias is the same RAM, not more of it.
    const L1_SIZE: u64 = 4 * 1024 * 1024;
    /// `MEM_LOCAL_BASE`: the movers' local data RAM, inside L1
    /// (`dev_mem_map.h:43`).
    const LOCAL_DATA_RAM_BASE: u64 = 0x0080_2000;
    /// `LOCAL_REGS_BASE`: the hart's own register block (`tt_t6_trisc_map.h:17`).
    const DEBUG_REGS_BASE: u64 = 0x0080_0000;

    /// `RISCV_DEBUG_REG_DISABLE_RESET` (`tensix.h:222`).
    const DISABLE_RESET: u64 = 0x0080_0224;

    /// `RISCV_DEBUG_REG_WALL_CLOCK_0/1/1_AT` (`tensix.h:209-211`).
    const WALL_CLOCK_L: u64 = 0x0080_01F0;
    const WALL_CLOCK_L_PLUS_4: u64 = 0x0080_01F4;
    const WALL_CLOCK_H: u64 = 0x0080_01F8;

    /// `IBUFFER_BASE`: a hart's own instruction FIFO (`tt_t6_trisc_map.h:21`).
    const INSTRN_BUF_BASE: u64 = 0x0080_F000;

    /// `PCBUFFER_BASE` (`tt_t6_trisc_map.h:22`).
    const PC_BUF_BASE: u64 = 0x0081_0000;
    /// `MOP_CFG_BASE` (`tt_t6_trisc_map.h:20`).
    const MOP_CFG_BASE: u64 = 0x0080_E000;

    /// Every non-RISC field of `SOFT_RESET_0`: unpackers 0-2, packers 3-5, glue 8,
    /// THCON 9, FPU 10, and the `SrcA`/`SrcB`/`Dst` register banks at 15-17 plus
    /// `SRCA_REG_SOFT_RESET_FPU` 19-22 (`t6_debug_map.h:380-419`).
    ///
    /// The RISCs are bits 11-14 and are held by [`Arch::reset`], not by this mask.
    /// Bits 6-7 have no field in the generated map — Quasar's TDMA has no separate
    /// mover or search unit — so unlike Blackhole's "every bit below the RISCs"
    /// they are left out: `0x7B873F`, not `0x7B87FF`.
    ///
    /// `tensix.h:254-265` still carries the pre-TDMA macros, which put the
    /// unpacker in two bits and the packer in four; the generated register map is
    /// the one that matches the register. tt-metal has no Quasar whole-backend
    /// value to copy: the firmware only touches `RISC_CONTROL_SOFT_RESET`
    /// (`tt-2xx/risc_common.h:116-140`) and the host only the tile register's DM
    /// bits, and TT-LLK's `_PIPELINE_RESET_MASK` is Wormhole/Blackhole's, skipped
    /// for `ChipArchitecture.QUASAR`.
    const BACKEND_RESET_MASK: u32 = 0x7B_873F;

    /// `CFG_REGS_BASE` (`tt_t6_trisc_map.h:31`): one `Config[2][94*4]` per engine,
    /// reached through the hart's own local map.
    const TENSIX_CFG_BASE: u64 = 0x0082_0000;
    /// `DEST_REGS_BASE` (`tt_t6_trisc_map.h:30`): 32 KiB per engine, not reachable
    /// over the NoC.
    const DST_BASE: u64 = 0x0081_8000;

    fn cores() -> &'static [Core] {
        &Core::QUASAR
    }

    /// A TRISC is held by its engine's `SOFT_RESET_0`, bits 11-14; a mover by the
    /// tile-level `TENSIX_SOFT_RESET_ADDR`, bits 8-16 (`grendel_implementation.hpp`
    /// `SOFT_RESET_DM0..DM7`).
    fn reset(core: Core) -> Result<(u64, u32), EncodeError> {
        match core {
            Core::Math { engine, trisc } => Ok((
                Self::engine(engine)
                    .ok_or(EncodeError::NoSuchCore { core: core.name() })?
                    .base
                    + SOFT_RESET_0,
                11 + u32::from(trisc),
            )),
            Core::Dm(hart) => Ok((TILE_RESET, 8 + u32::from(hart))),
            Core::B | Core::T0 | Core::T1 | Core::T2 | Core::NC => {
                Err(EncodeError::NoSuchCore { core: core.name() })
            }
        }
    }

    const ENGINES: u8 = 4;

    fn engine(engine: u8) -> Option<EngineRegs> {
        if engine >= Self::ENGINES {
            return None;
        }
        let base = ENGINE_BASE + ENGINE_STRIDE * u64::from(engine);
        let override_reg = base + TRISC_RESET_PC_OVERRIDE;
        Some(EngineRegs {
            base,
            reset_pc: [
                base + TRISC_RESET_PC,
                base + TRISC_RESET_PC + 0x4,
                base + TRISC_RESET_PC + 0x8,
                base + TRISC_RESET_PC + 0xC,
            ],
            reset_pc_override: [
                (override_reg, 1 << 0),
                (override_reg, 1 << 1),
                (override_reg, 1 << 2),
                (override_reg, 1 << 3),
            ],
        })
    }

    /// The firmware bases: a mover starts at `MEM_DM_FIRMWARE_BASE`, a TRISC at its
    /// own slot above it.
    ///
    /// A mover has no reset-PC register: the host writes a jump to this address into
    /// L1 word 0 and releases it, which is what `fw_launch_addr` does on this chip
    /// (`qa_hal_tensix.cpp:111-112`).
    fn default_reset_pc(core: Core) -> Result<u32, EncodeError> {
        match core {
            Core::Dm(_) => Ok(DM_FIRMWARE),
            Core::Math { trisc, .. } => {
                Ok(TRISC0_FIRMWARE + TRISC_FIRMWARE_STRIDE * u32::from(trisc))
            }
            Core::B | Core::T0 | Core::T1 | Core::T2 | Core::NC => {
                Err(EncodeError::NoSuchCore { core: core.name() })
            }
        }
    }

    fn reset_pc_register(core: Core) -> Result<Option<u64>, EncodeError> {
        match core {
            Core::Dm(_) => Ok(None),
            Core::Math { engine, trisc } => {
                Ok(Self::engine(engine).map(|regs| regs.reset_pc[trisc as usize]))
            }
            Core::B | Core::T0 | Core::T1 | Core::T2 | Core::NC => {
                Err(EncodeError::NoSuchCore { core: core.name() })
            }
        }
    }

    fn reset_pc_override(core: Core) -> Result<Option<(u64, u32)>, EncodeError> {
        match core {
            Core::Dm(_) => Ok(None),
            Core::Math { engine, trisc } => {
                Ok(Self::engine(engine).map(|regs| regs.reset_pc_override[trisc as usize]))
            }
            Core::B | Core::T0 | Core::T1 | Core::T2 | Core::NC => {
                Err(EncodeError::NoSuchCore { core: core.name() })
            }
        }
    }

    /// One `Dst` section per TRISC: `RISC_DEST_ACCESS_CTRL_SEC0..3_fmt`, at words
    /// 3, 5, 6 and 7 of `Config` (`cfg_defines.h:153-246`).
    fn dst_access_fmt(thread: u32) -> Result<ConfigField, EncodeError> {
        use crate::cfg::quasar::alu;
        match thread {
            0 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC0_fmt),
            1 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC1_fmt),
            2 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC2_fmt),
            3 => Ok(alu::RISC_DEST_ACCESS_CTRL_SEC3_fmt),
            _ => Err(EncodeError::NoSuchThread { thread }),
        }
    }
}
