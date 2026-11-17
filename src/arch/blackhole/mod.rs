//! Blackhole: the architecture this crate was written for.
//!
//! A p100a and a p150 are the same architecture; what differs between parts is
//! harvesting, which the ARC reports and which no constant here can know.
//!
//! Spec: `BlackholeA0/TensixTile/BabyRISCV/README.md` for the tile's map,
//! `SoftReset.md` for reset control, `HiMemTile.md` for the NoC-visible aperture.

use super::{Arch, BlackholeSpecials, Cfg, EngineRegs, Specials};
use crate::cfg::ConfigField;
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
}

impl Arch for Blackhole {
    type Cfg = Fields;

    const NAME: &'static str = "Blackhole";

    /// Blackhole's own mechanisms, gathered in one value: the timestamper, the
    /// configuration read pair, the load-adjacency hazards, the semaphore
    /// aperture and the NoC view of the local data RAM.
    const SPECIALS: Specials = Specials::Blackhole(BlackholeSpecials {
        timestamp: 0xFFB1_21FC,
        timestamp_cntl: 0xFFB1_2200,
        timestamp_status: 0xFFB1_2204,
        timestamp_buf0_start: 0xFFB1_2208,
        timestamp_buf0_end: 0xFFB1_220C,
        cfgreg_rd_cntl: 0xFFB1_2058,
        cfgreg_rddata: 0xFFB1_2078,
        coprocessor_done_check: Self::PC_BUF_BASE + 0x04,
        mop_expander_done_check: Self::PC_BUF_BASE + 0x08,
        pc_buf_hazard_end: Self::PC_BUF_BASE + 0xFFFF,
        // Readable from T0, T1 and T2 only (`SyncUnit.md`, "RISCV access to
        // semaphores").
        semaphore_access: 0xFFE8_0020,
        local_data_ram_noc_base: 0xFFB1_4000,
        local_data_ram_noc_end: 0xFFB1_E000,
        local_data_ram_noc_stride: 0x2000,
        // `BabyRISCV/README.md:152`: no NoC access lands inside this window.
        local_ram_zeroing_cycles: 2048,
        instrn1_buf: 0xFFE5_0000,
        instrn2_buf: 0xFFE6_0000,
        thcon_tile_descriptor: crate::cfg::blackhole::thcon::THCON_SEC0_REG0_TileDescriptor,
        thcon_out_data_format: crate::cfg::blackhole::thcon::THCON_SEC0_REG2_Out_data_format,
        cfg_state_id: crate::cfg::blackhole::thread::CFG_STATE_ID_StateID,
        clr_dvalid_srca_disable: crate::cfg::blackhole::thread::CLR_DVALID_SrcA_Disable,
        clr_dvalid_srcb_disable: crate::cfg::blackhole::thread::CLR_DVALID_SrcB_Disable,
    });

    /// L1 scratchpad RAM. Instructions can only be fetched from here
    /// (`BabyRISCV/README.md:39`).
    const L1_BASE: u64 = 0x0000_0000;
    /// 1536 KiB per Tensix tile (`BabyRISCV/README.md:102`).
    const L1_SIZE: u64 = 1536 * 1024;
    const LOCAL_DATA_RAM_BASE: u64 = 0xFFB0_0000;
    const DEBUG_REGS_BASE: u64 = 0xFFB1_2000;

    /// `RISC_SOFT_RESET_0`: the five cores and the backend, one bit per unit
    /// (`SoftReset.md:10-36`). Read through [`Arch::reset`].
    const DISABLE_RESET: u64 = 0xFFB1_2224;

    /// `RISCV_DEBUG_REG_WALL_CLOCK_L`: reading it also latches the high half.
    const WALL_CLOCK_L: u64 = 0xFFB1_21F0;
    const WALL_CLOCK_L_PLUS_4: u64 = 0xFFB1_21F4;
    const WALL_CLOCK_H: u64 = 0xFFB1_21F8;

    /// From T0/T1/T2 this feeds that core's own thread; from B, thread 0 after
    /// the MOP expander (`PushTensixInstruction.md:5-9`).
    const INSTRN_BUF_BASE: u64 = 0xFFE4_0000;

    /// Base of the PCBuf / Manual TTSync / semaphore region
    /// (`BabyRISCV/README.md:126-128`).
    const PC_BUF_BASE: u64 = 0xFFE8_0000;
    const MOP_CFG_BASE: u64 = 0xFFB8_0000;

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

    /// One register holds the whole tile: `RISC_SOFT_RESET_0` at `0xFFB1_21B0`
    /// (`SoftReset.md:10-36`).
    fn reset(core: Core) -> Result<(u64, u32), EncodeError> {
        const SOFT_RESET_0: u64 = 0xFFB1_21B0;
        match core {
            Core::B => Ok((SOFT_RESET_0, 11)),
            Core::T0 => Ok((SOFT_RESET_0, 12)),
            Core::T1 => Ok((SOFT_RESET_0, 13)),
            Core::T2 => Ok((SOFT_RESET_0, 14)),
            Core::NC => Ok((SOFT_RESET_0, 18)),
            Core::Dm(_) | Core::Math { .. } => Err(EncodeError::NoSuchCore { core: core.name() }),
        }
    }

    /// A Blackhole tile has one engine block, the tile's own debug registers.
    const ENGINES: u8 = 1;

    /// The block at `0xFFB1_2000`, with the tile's four slots in the order the
    /// reset-PC registers take them: T0, T1, T2, NC.
    fn engine(engine: u8) -> Option<EngineRegs> {
        if engine != 0 {
            return None;
        }
        Some(EngineRegs {
            base: Self::DEBUG_REGS_BASE,
            reset_pc: [0xFFB1_2228, 0xFFB1_222C, 0xFFB1_2230, 0xFFB1_2238],
            reset_pc_override: [
                (0xFFB1_2234, 1 << 0),
                (0xFFB1_2234, 1 << 1),
                (0xFFB1_2234, 1 << 2),
                (0xFFB1_223C, 1 << 0),
            ],
        })
    }

    fn default_reset_pc(core: Core) -> Result<u32, EncodeError> {
        match core {
            Core::B => Ok(0x0_0000),
            Core::T0 => Ok(0x0_6000),
            Core::T1 => Ok(0x0_A000),
            Core::T2 => Ok(0x0_E000),
            Core::NC => Ok(0x1_2000),
            Core::Dm(_) | Core::Math { .. } => Err(EncodeError::NoSuchCore { core: core.name() }),
        }
    }

    /// **RISCV B has none**: its entry point is hardwired to L1 offset `0`, a
    /// linker-script constraint rather than a preference.
    fn reset_pc_register(core: Core) -> Result<Option<u64>, EncodeError> {
        let slot = match core {
            Core::B => return Ok(None),
            Core::T0 => 0,
            Core::T1 => 1,
            Core::T2 => 2,
            Core::NC => 3,
            Core::Dm(_) | Core::Math { .. } => {
                return Err(EncodeError::NoSuchCore { core: core.name() });
            }
        };
        Ok(Self::engine(0).map(|regs| regs.reset_pc[slot]))
    }

    fn reset_pc_override(core: Core) -> Result<Option<(u64, u32)>, EncodeError> {
        let slot = match core {
            Core::B => return Ok(None),
            Core::T0 => 0,
            Core::T1 => 1,
            Core::T2 => 2,
            Core::NC => 3,
            Core::Dm(_) | Core::Math { .. } => {
                return Err(EncodeError::NoSuchCore { core: core.name() });
            }
        };
        Ok(Self::engine(0).map(|regs| regs.reset_pc_override[slot]))
    }

    /// The table in `PushTensixInstruction.md:5-9`. T0/T1/T2 each reach only their
    /// own thread, through the *same* buffer address: the pair identifies the
    /// thread, not the address. B reaches all three, one buffer each.
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
const _: () = {
    let s = match Blackhole::SPECIALS {
        Specials::Blackhole(s) => s,
        _ => panic!("Blackhole answers with Blackhole's specials"),
    };
    assert!(s.coprocessor_done_check > Blackhole::PC_BUF_BASE);
    assert!(s.coprocessor_done_check < s.pc_buf_hazard_end);
    assert!(s.mop_expander_done_check < s.pc_buf_hazard_end);
};
