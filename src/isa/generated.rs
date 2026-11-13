//! The Tensix instruction set, generated from `Diagrams/Src/Bits32.lua`.
//!
//! **Do not edit.** Regenerate with `cargo xtask gen-isa`, which fetches the
//! pinned specification itself; `cargo xtask gen-isa --check` fails when this
//! file is out of date.
//!
//! Source: tt-isa-documentation `f848eb668c2aeae742a88a49a86157e24a0a20c6`,
//! 165 instruction encodings and 19 datum layouts.
//!
//! Provenance: 50 documented for Blackhole, 24 shared with Wormhole and stated
//! to be identical, 28 superseded on Blackhole, 58 Wormhole-only and therefore
//! **`UNVERIFIED`**, 17 **`MEASURED`** on ttsim or silicon where the specification
//! draws only Wormhole's layout (`xtask/src/genisa/Bits32_BH.lua`), and 4
//! Wormhole-only layouts **`CONFIRMED`** unchanged on Blackhole by a gate
//! (`xtask/src/genisa/measured.rs`).
//!
//! Names are the `Bits32.lua` diagram keys, so a name in the specification can
//! be found here without translation — except that a Blackhole-specific form
//! drops its `_BH` suffix and the Wormhole form it replaces moves into the
//! `wormhole` submodule, so the plain name is always the one to use here.
//! That is why this module allows globals that are not upper case.
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]

use super::{DatumLayout, EncodeError, Field, Instruction, InstructionDef, Provenance};

/// Every instruction encoding, by name. Blackhole's form takes the plain
/// name; the Wormhole form it supersedes is under [`defs::wormhole`].
pub mod defs {
    use super::*;

    /// `ATCAS`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ATCAS.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ATCAS: InstructionDef = InstructionDef::new(
        "ATCAS",
        "ATCAS",
        0x64,
        &[
            Field::new("SetVal", 18, 4, false, None),
            Field::new("CmpVal", 14, 4, false, None),
            Field::new("Ofs", 12, 2, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[],
        0x00c00fc0,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ATCAS.md",
    );

    /// `ATSWAP`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ATSWAP.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ATSWAP: InstructionDef = InstructionDef::new(
        "ATSWAP",
        "ATSWAP",
        0x63,
        &[
            Field::new("SingleDataReg", 22, 1, false, None),
            Field::new("Mask", 14, 8, false, None),
            Field::new("DataReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[],
        0x00803000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ATSWAP.md",
    );

    /// `RMWCIB0`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md`.
    pub static RMWCIB0: InstructionDef = InstructionDef::new(
        "RMWCIB0",
        "RMWCIB0",
        0xb3,
        &[
            Field::new("Mask", 16, 8, false, None),
            Field::new("NewValue", 8, 8, false, None),
            Field::new("Index4", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md",
    );

    /// `RMWCIB1`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md`.
    pub static RMWCIB1: InstructionDef = InstructionDef::new(
        "RMWCIB1",
        "RMWCIB1",
        0xb4,
        &[
            Field::new("Mask", 16, 8, false, None),
            Field::new("NewValue", 8, 8, false, None),
            Field::new("Index4", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md",
    );

    /// `RMWCIB2`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md`.
    pub static RMWCIB2: InstructionDef = InstructionDef::new(
        "RMWCIB2",
        "RMWCIB2",
        0xb5,
        &[
            Field::new("Mask", 16, 8, false, None),
            Field::new("NewValue", 8, 8, false, None),
            Field::new("Index4", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md",
    );

    /// `RMWCIB3`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md`.
    pub static RMWCIB3: InstructionDef = InstructionDef::new(
        "RMWCIB3",
        "RMWCIB3",
        0xb6,
        &[
            Field::new("Mask", 16, 8, false, None),
            Field::new("NewValue", 8, 8, false, None),
            Field::new("Index4", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/RMWCIB.md",
    );

    /// `SETDMAREG_Immediate`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETDMAREG_Immediate.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETDMAREG_Immediate: InstructionDef = InstructionDef::new(
        "SETDMAREG_Immediate",
        "SETDMAREG",
        0x45,
        &[
            Field::new("NewValue", 8, 16, false, None),
            Field::new("ResultHalfReg", 0, 7, false, None),
        ],
        &[(Field::new("", 7, 1, false, None), 0)],
        0x00000000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETDMAREG_Immediate.md",
    );

    /// `SETDMAREG_Special`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETDMAREG_Special.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETDMAREG_Special: InstructionDef = InstructionDef::new(
        "SETDMAREG_Special",
        "SETDMAREG",
        0x45,
        &[
            Field::new("ResultSize", 22, 2, false, None),
            Field::new("WhichPackers", 15, 4, false, None),
            Field::new("InputSource", 11, 4, false, None),
            Field::new("InputHalfReg", 8, 3, false, None),
            Field::new("ResultHalfReg", 0, 7, false, None),
        ],
        &[(Field::new("", 7, 1, false, None), 1)],
        0x00380000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETDMAREG_Special.md",
    );

    /// `STALLWAIT_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/STALLWAIT.md`.
    pub static STALLWAIT: InstructionDef = InstructionDef::new(
        "STALLWAIT_BH",
        "STALLWAIT",
        0xa2,
        &[
            Field::new("BlockMask", 15, 9, false, None),
            Field::new("ConditionMask", 0, 13, false, None),
        ],
        &[],
        0x00006000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/STALLWAIT.md",
    );

    /// `SEMWAIT`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SEMWAIT.md`.
    pub static SEMWAIT: InstructionDef = InstructionDef::new(
        "SEMWAIT",
        "SEMWAIT",
        0xa6,
        &[
            Field::new("BlockMask", 15, 9, false, None),
            Field::new("SemaphoreMask", 2, 8, false, None),
            Field::new("ConditionMask", 0, 2, false, None),
        ],
        &[],
        0x00007c00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SEMWAIT.md",
    );

    /// `STREAMWAIT`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/STREAMWAIT.md`.
    pub static STREAMWAIT: InstructionDef = InstructionDef::new(
        "STREAMWAIT",
        "STREAMWAIT",
        0xa7,
        &[
            Field::new("BlockMask", 15, 9, false, None),
            Field::new("TargetValueLo", 4, 10, false, None),
            Field::new("ConditionIndex", 3, 1, false, None),
            Field::new("StreamSelect", 0, 2, false, None),
        ],
        &[],
        0x00004004,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/STREAMWAIT.md",
    );

    /// `SEMINIT`. `WormholeB0/TensixTile/TensixCoprocessor/SEMINIT.md`, which the Blackhole tree states is shared and identical.
    pub static SEMINIT: InstructionDef = InstructionDef::new(
        "SEMINIT",
        "SEMINIT",
        0xa3,
        &[
            Field::new("NewMax", 20, 4, false, None),
            Field::new("NewValue", 16, 4, false, None),
            Field::new("SemaphoreMask", 2, 8, false, None),
        ],
        &[],
        0x0000fc03,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SEMINIT.md",
    );

    /// `REPLAY`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/REPLAY.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static REPLAY: InstructionDef = InstructionDef::new(
        "REPLAY",
        "REPLAY",
        0x04,
        &[
            Field::new("Index", 14, 5, false, None),
            Field::new("Count", 4, 6, false, None),
            Field::new("Exec", 1, 1, false, None),
            Field::new("Load", 0, 1, false, None),
        ],
        &[],
        0x00f83c0c,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/REPLAY.md",
    );

    /// `PACR_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/PACR.md`.
    pub static PACR: InstructionDef = InstructionDef::new(
        "PACR_BH",
        "PACR",
        0x41,
        &[
            Field::new("CfgContext", 21, 2, false, None),
            Field::new("RowPadZero", 18, 3, false, None),
            Field::new("DstAccessMode", 17, 1, false, None),
            Field::new("AddrMod", 15, 2, false, None),
            Field::new("AddrCntContext", 13, 2, false, None),
            Field::new("ZeroWrite", 12, 1, false, None),
            Field::new("ReadIntfSel", 8, 4, false, None),
            Field::new("OvrdThreadId", 7, 1, false, None),
            Field::new("Concat", 4, 1, false, None),
            Field::new("CtxtCtrl", 2, 2, false, None),
            Field::new("Flush", 1, 1, false, None),
            Field::new("Last", 0, 1, false, None),
        ],
        &[
            (Field::new("", 5, 2, false, None), 0),
            (Field::new("", 23, 1, false, None), 0),
        ],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/PACR.md",
    );

    /// `PACR_SETREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/PACR_SETREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static PACR_SETREG: InstructionDef = InstructionDef::new(
        "PACR_SETREG",
        "PACR_SETREG",
        0x4a,
        &[
            Field::new("AddrSel", 22, 1, false, None),
            Field::new("Value10", 12, 10, false, None),
            Field::new("AddrMid", 2, 6, false, None),
        ],
        &[
            (Field::new("", 1, 1, false, None), 1),
            (Field::new("", 8, 4, false, None), 15),
            (Field::new("", 23, 1, false, None), 1),
        ],
        0x00000001,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/PACR_SETREG.md",
    );

    /// `SFPSHFT2`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSHFT2.md`.
    pub static SFPSHFT2: InstructionDef = InstructionDef::new(
        "SFPSHFT2",
        "SFPSHFT2",
        0x94,
        &[
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ff0000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSHFT2.md",
    );

    /// `SFPSHFT2b`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSHFT2.md`.
    pub static SFPSHFT2b: InstructionDef = InstructionDef::new(
        "SFPSHFT2b",
        "SFPSHFT2",
        0x94,
        &[
            Field::new("Imm12", 12, 12, true, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000f00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSHFT2.md",
    );

    /// `SFPLUTFP32_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPLUTFP32.md`.
    pub static SFPLUTFP32: InstructionDef = InstructionDef::new(
        "SFPLUTFP32_BH",
        "SFPLUTFP32",
        0x95,
        &[
            Field::new("Mod1Mirror", 16, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f0ff00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPLUTFP32.md",
    );

    /// `REG2FLOP_Configuration`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/REG2FLOP_Configuration.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static REG2FLOP_Configuration: InstructionDef = InstructionDef::new(
        "REG2FLOP_Configuration",
        "REG2FLOP",
        0x48,
        &[
            Field::new("SizeSel", 22, 2, false, None),
            Field::new("ThConCfgIndex", 6, 7, false, None),
            Field::new("InputReg", 0, 6, false, None),
        ],
        &[(Field::new("", 20, 2, false, None), 0)],
        0x000fe000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/REG2FLOP_Configuration.md",
    );

    /// `REG2FLOP_ADC`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/REG2FLOP_ADC.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static REG2FLOP_ADC: InstructionDef = InstructionDef::new(
        "REG2FLOP_ADC",
        "REG2FLOP",
        0x48,
        &[
            Field::new("SizeSel", 22, 2, false, None),
            Field::new("OverrideThread", 20, 1, false, None),
            Field::new("Shift8", 18, 2, false, None),
            Field::new("ThreadSel", 16, 2, false, None),
            Field::new("Channel", 11, 1, false, None),
            Field::new("ADCSel", 9, 2, false, None),
            Field::new("Cr", 8, 1, false, None),
            Field::new("XYZW", 6, 2, false, None),
            Field::new("InputReg", 0, 6, false, None),
        ],
        &[(Field::new("", 21, 1, false, None), 1)],
        0x0000f000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/REG2FLOP_ADC.md",
    );

    /// `XMOV`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/XMOV.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static XMOV: InstructionDef = InstructionDef::new(
        "XMOV",
        "XMOV",
        0x40,
        &[],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x007fffff,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/XMOV.md",
    );

    /// `WRCFG`. `WormholeB0/TensixTile/TensixCoprocessor/WRCFG.md`, which the Blackhole tree states is shared and identical.
    pub static WRCFG: InstructionDef = InstructionDef::new(
        "WRCFG",
        "WRCFG",
        0xb0,
        &[
            Field::new("InputReg", 16, 6, false, None),
            Field::new("Is128Bit", 15, 1, false, None),
            Field::new("CfgIndex", 0, 11, false, None),
        ],
        &[],
        0x00c07800,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/WRCFG.md",
    );

    /// `RDCFG`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/RDCFG.md`.
    pub static RDCFG: InstructionDef = InstructionDef::new(
        "RDCFG",
        "RDCFG",
        0xb1,
        &[
            Field::new("ResultReg", 16, 6, false, None),
            Field::new("CfgIndex", 0, 11, false, None),
        ],
        &[],
        0x00c0f800,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/RDCFG.md",
    );

    /// `SETC16`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SETC16.md`.
    pub static SETC16: InstructionDef = InstructionDef::new(
        "SETC16",
        "SETC16",
        0xb2,
        &[
            Field::new("CfgIndex", 16, 8, false, None),
            Field::new("NewValue", 0, 16, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SETC16.md",
    );

    /// `STREAMWRCFG`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/STREAMWRCFG.md`.
    pub static STREAMWRCFG: InstructionDef = InstructionDef::new(
        "STREAMWRCFG",
        "STREAMWRCFG",
        0xb7,
        &[
            Field::new("StreamSelect", 21, 2, false, None),
            Field::new("RegIndex", 11, 10, false, None),
            Field::new("CfgIndex", 0, 11, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/STREAMWRCFG.md",
    );

    /// `CFGSHIFTMASK`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/CFGSHIFTMASK.md`.
    pub static CFGSHIFTMASK: InstructionDef = InstructionDef::new(
        "CFGSHIFTMASK",
        "CFGSHIFTMASK",
        0xb8,
        &[
            Field::new("MaskMode", 23, 1, false, None),
            Field::new("AluMode", 20, 3, false, None),
            Field::new("MaskWidth", 15, 5, false, None),
            Field::new("RotateAmt", 10, 5, false, None),
            Field::new("ScratchIndex", 8, 2, false, None),
            Field::new("CfgIndex", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/CFGSHIFTMASK.md",
    );

    /// `SETADC`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETADC.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETADC: InstructionDef = InstructionDef::new(
        "SETADC",
        "SETADC",
        0x50,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("Channel", 20, 1, false, None),
            Field::new("XYZW", 18, 2, false, None),
            Field::new("NewValue", 0, 18, false, None),
        ],
        &[],
        0x00000000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETADC.md",
    );

    /// `SETADCXY`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETADCXY.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETADCXY: InstructionDef = InstructionDef::new(
        "SETADCXY",
        "SETADCXY",
        0x51,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("ThreadOverride", 18, 2, false, None),
            Field::new("Y1Val", 15, 3, false, None),
            Field::new("X1Val", 12, 3, false, None),
            Field::new("Y0Val", 9, 3, false, None),
            Field::new("X0Val", 6, 3, false, None),
            Field::new("Y1", 3, 1, false, None),
            Field::new("X1", 2, 1, false, None),
            Field::new("Y0", 1, 1, false, None),
            Field::new("X0", 0, 1, false, None),
        ],
        &[],
        0x00100030,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETADCXY.md",
    );

    /// `INCADCXY`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/INCADCXY.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static INCADCXY: InstructionDef = InstructionDef::new(
        "INCADCXY",
        "INCADCXY",
        0x52,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("ThreadOverride", 18, 2, false, None),
            Field::new("Y1Inc", 15, 3, false, None),
            Field::new("X1Inc", 12, 3, false, None),
            Field::new("Y0Inc", 9, 3, false, None),
            Field::new("X0Inc", 6, 3, false, None),
        ],
        &[],
        0x0010003f,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/INCADCXY.md",
    );

    /// `ADDRCRXY`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ADDRCRXY.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ADDRCRXY: InstructionDef = InstructionDef::new(
        "ADDRCRXY",
        "ADDRCRXY",
        0x53,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("ThreadOverride", 18, 2, false, None),
            Field::new("Y1Inc", 15, 3, false, None),
            Field::new("X1Inc", 12, 3, false, None),
            Field::new("Y0Inc", 9, 3, false, None),
            Field::new("X0Inc", 6, 3, false, None),
            Field::new("Y1", 3, 1, false, None),
            Field::new("X1", 2, 1, false, None),
            Field::new("Y0", 1, 1, false, None),
            Field::new("X0", 0, 1, false, None),
        ],
        &[],
        0x00100030,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ADDRCRXY.md",
    );

    /// `SETADCZW`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETADCZW.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETADCZW: InstructionDef = InstructionDef::new(
        "SETADCZW",
        "SETADCZW",
        0x54,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("ThreadOverride", 18, 2, false, None),
            Field::new("W1Val", 15, 3, false, None),
            Field::new("Z1Val", 12, 3, false, None),
            Field::new("W0Val", 9, 3, false, None),
            Field::new("Z0Val", 6, 3, false, None),
            Field::new("W1", 3, 1, false, None),
            Field::new("Z1", 2, 1, false, None),
            Field::new("W0", 1, 1, false, None),
            Field::new("Z0", 0, 1, false, None),
        ],
        &[],
        0x00100030,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETADCZW.md",
    );

    /// `INCADCZW`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/INCADCZW.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static INCADCZW: InstructionDef = InstructionDef::new(
        "INCADCZW",
        "INCADCZW",
        0x55,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("ThreadOverride", 18, 2, false, None),
            Field::new("W1Inc", 15, 3, false, None),
            Field::new("Z1Inc", 12, 3, false, None),
            Field::new("W0Inc", 9, 3, false, None),
            Field::new("Z0Inc", 6, 3, false, None),
        ],
        &[],
        0x0010003f,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/INCADCZW.md",
    );

    /// `ADDRCRZW`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ADDRCRZW.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ADDRCRZW: InstructionDef = InstructionDef::new(
        "ADDRCRZW",
        "ADDRCRZW",
        0x56,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("ThreadOverride", 18, 2, false, None),
            Field::new("W1Inc", 15, 3, false, None),
            Field::new("Z1Inc", 12, 3, false, None),
            Field::new("W0Inc", 9, 3, false, None),
            Field::new("Z0Inc", 6, 3, false, None),
            Field::new("W1", 3, 1, false, None),
            Field::new("Z1", 2, 1, false, None),
            Field::new("W0", 1, 1, false, None),
            Field::new("Z0", 0, 1, false, None),
        ],
        &[],
        0x00100030,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ADDRCRZW.md",
    );

    /// `SETADCXX`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETADCXX.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETADCXX: InstructionDef = InstructionDef::new(
        "SETADCXX",
        "SETADCXX",
        0x5e,
        &[
            Field::new("PK", 23, 1, false, None),
            Field::new("U1", 22, 1, false, None),
            Field::new("U0", 21, 1, false, None),
            Field::new("X1Val", 10, 10, false, None),
            Field::new("X0Val", 0, 10, false, None),
        ],
        &[],
        0x00100000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETADCXX.md",
    );

    /// `NOP`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/NOP.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static NOP: InstructionDef = InstructionDef::new(
        "NOP",
        "NOP",
        0x02,
        &[],
        &[],
        0x00ffffff,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/NOP.md",
    );

    /// `SEMPOST`. `WormholeB0/TensixTile/TensixCoprocessor/SEMPOST.md`, which the Blackhole tree states is shared and identical.
    pub static SEMPOST: InstructionDef = InstructionDef::new(
        "SEMPOST",
        "SEMPOST",
        0xa4,
        &[Field::new("SemaphoreMask", 2, 8, false, None)],
        &[],
        0x00fffc03,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SEMPOST.md",
    );

    /// `SEMGET`. `WormholeB0/TensixTile/TensixCoprocessor/SEMGET.md`, which the Blackhole tree states is shared and identical.
    pub static SEMGET: InstructionDef = InstructionDef::new(
        "SEMGET",
        "SEMGET",
        0xa5,
        &[Field::new("SemaphoreMask", 2, 8, false, None)],
        &[],
        0x00fffc03,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SEMGET.md",
    );

    /// `SETDVALID`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETDVALID.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETDVALID: InstructionDef = InstructionDef::new(
        "SETDVALID",
        "SETDVALID",
        0x57,
        &[
            Field::new("FlipSrcB", 1, 1, false, None),
            Field::new("FlipSrcA", 0, 1, false, None),
        ],
        &[],
        0x00fffffc,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETDVALID.md",
    );

    /// `GATESRCRST`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/GATESRCRST.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static GATESRCRST: InstructionDef = InstructionDef::new(
        "GATESRCRST",
        "GATESRCRST",
        0x35,
        &[Field::new("InvalidateSrcBCache", 1, 1, false, None)],
        &[],
        0x00fffffd,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/GATESRCRST.md",
    );

    /// `ZEROSRC`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ZEROSRC.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ZEROSRC: InstructionDef = InstructionDef::new(
        "ZEROSRC",
        "ZEROSRC",
        0x11,
        &[
            Field::new("NegativeInfSrcA", 4, 1, false, None),
            Field::new("SingleBankMatrixUnit", 3, 1, false, None),
            Field::new("BothBanks", 2, 1, false, None),
            Field::new("ClearSrcB", 1, 1, false, None),
            Field::new("ClearSrcA", 0, 1, false, None),
        ],
        &[],
        0x00ffffe0,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ZEROSRC.md",
    );

    /// `TRNSPSRCB`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/TRNSPSRCB.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static TRNSPSRCB: InstructionDef = InstructionDef::new(
        "TRNSPSRCB",
        "TRNSPSRCB",
        0x16,
        &[],
        &[],
        0x00ffffff,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/TRNSPSRCB.md",
    );

    /// `SHIFTXA`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SHIFTXA.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SHIFTXA: InstructionDef = InstructionDef::new(
        "SHIFTXA",
        "SHIFTXA",
        0x17,
        &[Field::new("Direction", 0, 2, false, None)],
        &[],
        0x00fffffc,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SHIFTXA.md",
    );

    /// `CLREXPHIST`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/CLREXPHIST.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static CLREXPHIST: InstructionDef = InstructionDef::new(
        "CLREXPHIST",
        "CLREXPHIST",
        0x21,
        &[],
        &[],
        0x00ffffff,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/CLREXPHIST.md",
    );

    /// `CLEARDVALID`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/CLEARDVALID.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static CLEARDVALID: InstructionDef = InstructionDef::new(
        "CLEARDVALID",
        "CLEARDVALID",
        0x36,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("KeepReadingSameSrc", 1, 1, false, None),
            Field::new("Reset", 0, 1, false, None),
        ],
        &[],
        0x003ffffc,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/CLEARDVALID.md",
    );

    /// `SETRWC`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SETRWC.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SETRWC: InstructionDef = InstructionDef::new(
        "SETRWC",
        "SETRWC",
        0x37,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("DstCtoCr", 21, 1, false, None),
            Field::new("DstCr", 20, 1, false, None),
            Field::new("SrcBCr", 19, 1, false, None),
            Field::new("SrcACr", 18, 1, false, None),
            Field::new("DstVal", 14, 4, false, None),
            Field::new("SrcBVal", 10, 4, false, None),
            Field::new("SrcAVal", 6, 4, false, None),
            Field::new("Fidelity", 3, 1, false, None),
            Field::new("Dst", 2, 1, false, None),
            Field::new("SrcB", 1, 1, false, None),
            Field::new("SrcA", 0, 1, false, None),
        ],
        &[],
        0x00000030,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SETRWC.md",
    );

    /// `INCRWC`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/INCRWC.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static INCRWC: InstructionDef = InstructionDef::new(
        "INCRWC",
        "INCRWC",
        0x38,
        &[
            Field::new("DstCr", 20, 1, false, None),
            Field::new("SrcBCr", 19, 1, false, None),
            Field::new("SrcACr", 18, 1, false, None),
            Field::new("DstInc", 14, 4, false, None),
            Field::new("SrcBInc", 10, 4, false, None),
            Field::new("SrcAInc", 6, 4, false, None),
        ],
        &[],
        0x00e0003f,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/INCRWC.md",
    );

    /// `LOADIND`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/LOADIND.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static LOADIND: InstructionDef = InstructionDef::new(
        "LOADIND",
        "LOADIND",
        0x49,
        &[
            Field::new("Size", 22, 2, false, None),
            Field::new("OffsetHalfReg", 14, 7, false, None),
            Field::new("OffsetIncrement", 12, 2, false, None),
            Field::new("ResultReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[],
        0x00200000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/LOADIND.md",
    );

    /// `STOREIND_L1`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/STOREIND_L1.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static STOREIND_L1: InstructionDef = InstructionDef::new(
        "STOREIND_L1",
        "STOREIND",
        0x66,
        &[
            Field::new("Size", 21, 2, false, None),
            Field::new("OffsetHalfReg", 14, 7, false, None),
            Field::new("OffsetIncrement", 12, 2, false, None),
            Field::new("DataReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x00000000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/STOREIND_L1.md",
    );

    /// `STOREIND_MMIO`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/STOREIND_MMIO.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static STOREIND_MMIO: InstructionDef = InstructionDef::new(
        "STOREIND_MMIO",
        "STOREIND",
        0x66,
        &[
            Field::new("OffsetHalfReg", 14, 7, false, None),
            Field::new("OffsetIncrement", 12, 2, false, None),
            Field::new("DataReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[
            (Field::new("", 22, 1, false, None), 1),
            (Field::new("", 23, 1, false, None), 0),
        ],
        0x00200000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/STOREIND_MMIO.md",
    );

    /// `STOREIND_Src`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/STOREIND_Src.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static STOREIND_Src: InstructionDef = InstructionDef::new(
        "STOREIND_Src",
        "STOREIND",
        0x66,
        &[
            Field::new("StoreToSrcB", 21, 1, false, None),
            Field::new("OffsetHalfReg", 14, 7, false, None),
            Field::new("OffsetIncrement", 12, 2, false, None),
            Field::new("DataReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[
            (Field::new("", 22, 1, false, None), 0),
            (Field::new("", 23, 1, false, None), 0),
        ],
        0x00000000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/STOREIND_Src.md",
    );

    /// `LOADREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/LOADREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static LOADREG: InstructionDef = InstructionDef::new(
        "LOADREG",
        "LOADREG",
        0x68,
        &[
            Field::new("ResultReg", 18, 6, false, None),
            Field::new("AddrLo", 0, 18, false, None),
        ],
        &[],
        0x00000000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/LOADREG.md",
    );

    /// `STOREREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/STOREREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static STOREREG: InstructionDef = InstructionDef::new(
        "STOREREG",
        "STOREREG",
        0x67,
        &[
            Field::new("DataReg", 18, 6, false, None),
            Field::new("AddrLo", 0, 18, false, None),
        ],
        &[],
        0x00000000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/STOREREG.md",
    );

    /// `FLUSHDMA`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/FLUSHDMA.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static FLUSHDMA: InstructionDef = InstructionDef::new(
        "FLUSHDMA",
        "FLUSHDMA",
        0x46,
        &[Field::new("ConditionMask", 0, 4, false, None)],
        &[],
        0x00fffff0,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/FLUSHDMA.md",
    );

    /// `DMANOP`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/DMANOP.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static DMANOP: InstructionDef = InstructionDef::new(
        "DMANOP",
        "DMANOP",
        0x60,
        &[],
        &[],
        0x00ffffff,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/DMANOP.md",
    );

    /// `ADDDMAREG`. **`CONFIRMED`** on Blackhole, ttsim and silicon, by `crates/tt-tests/tests/step38_gpr_add.rs::adddmareg_adds_two_registers`: the layout of `WormholeB0/TensixTile/TensixCoprocessor/ADDDMAREG.md` (a Wormhole page; Blackhole has none), every field exercised.
    pub static ADDDMAREG: InstructionDef = InstructionDef::new(
        "ADDDMAREG",
        "ADDDMAREG",
        0x58,
        &[
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightReg", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x007c0000,
        Provenance::Confirmed {
            evidence: "crates/tt-tests/tests/step38_gpr_add.rs::adddmareg_adds_two_registers",
        },
        "WormholeB0/TensixTile/TensixCoprocessor/ADDDMAREG.md",
    );

    /// `ADDDMAREGi`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ADDDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ADDDMAREGi: InstructionDef = InstructionDef::new(
        "ADDDMAREGi",
        "ADDDMAREG",
        0x58,
        &[
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightImm6", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x007c0000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ADDDMAREG.md",
    );

    /// `SUBDMAREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SUBDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SUBDMAREG: InstructionDef = InstructionDef::new(
        "SUBDMAREG",
        "SUBDMAREG",
        0x59,
        &[
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightReg", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x007c0000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SUBDMAREG.md",
    );

    /// `SUBDMAREGi`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SUBDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SUBDMAREGi: InstructionDef = InstructionDef::new(
        "SUBDMAREGi",
        "SUBDMAREG",
        0x59,
        &[
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightImm6", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x007c0000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SUBDMAREG.md",
    );

    /// `MULDMAREG`. **`CONFIRMED`** on Blackhole, ttsim and silicon, by `crates/tt-tests/tests/step100_scalar_config.rs::scalar_register_and_immediate_semantics`: the layout of `WormholeB0/TensixTile/TensixCoprocessor/MULDMAREG.md` (a Wormhole page; Blackhole has none), every field exercised.
    pub static MULDMAREG: InstructionDef = InstructionDef::new(
        "MULDMAREG",
        "MULDMAREG",
        0x5a,
        &[
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightReg", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x007c0000,
        Provenance::Confirmed {
            evidence: "crates/tt-tests/tests/step100_scalar_config.rs::scalar_register_and_immediate_semantics",
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MULDMAREG.md",
    );

    /// `MULDMAREGi`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/MULDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static MULDMAREGi: InstructionDef = InstructionDef::new(
        "MULDMAREGi",
        "MULDMAREG",
        0x5a,
        &[
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightImm6", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x007c0000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/MULDMAREG.md",
    );

    /// `BITWOPDMAREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/BITWOPDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static BITWOPDMAREG: InstructionDef = InstructionDef::new(
        "BITWOPDMAREG",
        "BITWOPDMAREG",
        0x5b,
        &[
            Field::new("Mode", 18, 3, false, None),
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightReg", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x00600000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/BITWOPDMAREG.md",
    );

    /// `BITWOPDMAREGi`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/BITWOPDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static BITWOPDMAREGi: InstructionDef = InstructionDef::new(
        "BITWOPDMAREGi",
        "BITWOPDMAREG",
        0x5b,
        &[
            Field::new("Mode", 18, 3, false, None),
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightImm6", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x00600000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/BITWOPDMAREG.md",
    );

    /// `SHIFTDMAREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SHIFTDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SHIFTDMAREG: InstructionDef = InstructionDef::new(
        "SHIFTDMAREG",
        "SHIFTDMAREG",
        0x5c,
        &[
            Field::new("Mode", 18, 3, false, None),
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightReg", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x00600000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SHIFTDMAREG.md",
    );

    /// `SHIFTDMAREGi`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/SHIFTDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static SHIFTDMAREGi: InstructionDef = InstructionDef::new(
        "SHIFTDMAREGi",
        "SHIFTDMAREG",
        0x5c,
        &[
            Field::new("Mode", 18, 3, false, None),
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightImm5", 6, 5, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x00600800,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/SHIFTDMAREG.md",
    );

    /// `CMPDMAREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/CMPDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static CMPDMAREG: InstructionDef = InstructionDef::new(
        "CMPDMAREG",
        "CMPDMAREG",
        0x5d,
        &[
            Field::new("Mode", 18, 3, false, None),
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightReg", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 0)],
        0x00600000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/CMPDMAREG.md",
    );

    /// `CMPDMAREGi`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/CMPDMAREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static CMPDMAREGi: InstructionDef = InstructionDef::new(
        "CMPDMAREGi",
        "CMPDMAREG",
        0x5d,
        &[
            Field::new("Mode", 18, 3, false, None),
            Field::new("ResultReg", 12, 6, false, None),
            Field::new("RightImm6", 6, 6, false, None),
            Field::new("LeftReg", 0, 6, false, None),
        ],
        &[(Field::new("", 23, 1, false, None), 1)],
        0x00600000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/CMPDMAREG.md",
    );

    /// `ATGETM`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/ATGETM.md`.
    pub static ATGETM: InstructionDef = InstructionDef::new(
        "ATGETM",
        "ATGETM",
        0xa0,
        &[Field::new("Index", 0, 16, false, None)],
        &[],
        0x00ff0000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/ATGETM.md",
    );

    /// `ATRELM`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/ATRELM.md`.
    pub static ATRELM: InstructionDef = InstructionDef::new(
        "ATRELM",
        "ATRELM",
        0xa1,
        &[Field::new("Index", 0, 16, false, None)],
        &[],
        0x00ff0000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/ATRELM.md",
    );

    /// `ATINCGET`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ATINCGET.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ATINCGET: InstructionDef = InstructionDef::new(
        "ATINCGET",
        "ATINCGET",
        0x61,
        &[
            Field::new("IntWidth", 14, 5, false, None),
            Field::new("Ofs", 12, 2, false, None),
            Field::new("InOutReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[],
        0x00f80000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ATINCGET.md",
    );

    /// `ATINCGETPTR`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/ATINCGETPTR.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static ATINCGETPTR: InstructionDef = InstructionDef::new(
        "ATINCGETPTR",
        "ATINCGETPTR",
        0x62,
        &[
            Field::new("NoIncr", 22, 1, false, None),
            Field::new("IncrLog2", 18, 4, false, None),
            Field::new("IntWidth", 14, 4, false, None),
            Field::new("Ofs", 12, 2, false, None),
            Field::new("ResultReg", 6, 6, false, None),
            Field::new("AddrReg", 0, 6, false, None),
        ],
        &[],
        0x00800000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/ATINCGETPTR.md",
    );

    /// `SFPLOAD_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPLOAD.md`.
    pub static SFPLOAD: InstructionDef = InstructionDef::new(
        "SFPLOAD_BH",
        "SFPLOAD",
        0x70,
        &[
            Field::new("VD", 20, 4, false, None),
            Field::new("Mod0", 16, 4, false, None),
            Field::new("AddrMod", 13, 3, false, None),
            Field::new("Imm10", 0, 10, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPLOAD.md",
    );

    /// `SFPLOADMACRO_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPLOADMACRO.md`.
    pub static SFPLOADMACRO: InstructionDef = InstructionDef::new(
        "SFPLOADMACRO_BH",
        "SFPLOADMACRO",
        0x93,
        &[
            Field::new("MacroIndex", 22, 2, false, None),
            Field::new("VDLo", 20, 2, false, None),
            Field::new("Mod0", 16, 4, false, None),
            Field::new("AddrMod", 13, 3, false, None),
            Field::new("Imm9", 1, 9, false, None),
            Field::new("VDHi", 0, 1, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPLOADMACRO.md",
    );

    /// `SFPSTORE_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSTORE.md`.
    pub static SFPSTORE: InstructionDef = InstructionDef::new(
        "SFPSTORE_BH",
        "SFPSTORE",
        0x72,
        &[
            Field::new("VD", 20, 4, false, None),
            Field::new("Mod0", 16, 4, false, None),
            Field::new("AddrMod", 13, 3, false, None),
            Field::new("Imm10", 0, 10, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSTORE.md",
    );

    /// `MOP`. **`CONFIRMED`** on Blackhole, ttsim and silicon, by `crates/tt-tests/tests/step36_mop.rs::a_mop_expands_as_the_page_models_it`: the layout of `WormholeB0/TensixTile/TensixCoprocessor/MOP.md` (a Wormhole page; Blackhole has none), every field exercised.
    pub static MOP: InstructionDef = InstructionDef::new(
        "MOP",
        "MOP",
        0x01,
        &[
            Field::new("Template", 23, 1, false, None),
            Field::new("Count1", 16, 7, false, None),
            Field::new("MaskLo", 0, 16, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Confirmed {
            evidence: "crates/tt-tests/tests/step36_mop.rs::a_mop_expands_as_the_page_models_it",
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOP.md",
    );

    /// `MOP_CFG`. **`CONFIRMED`** on Blackhole, ttsim and silicon, by `crates/tt-tests/tests/step36_mop.rs::a_mop_expands_as_the_page_models_it`: the layout of `WormholeB0/TensixTile/TensixCoprocessor/MOP_CFG.md` (a Wormhole page; Blackhole has none), every field exercised.
    pub static MOP_CFG: InstructionDef = InstructionDef::new(
        "MOP_CFG",
        "MOP_CFG",
        0x03,
        &[Field::new("MaskHi", 0, 16, false, None)],
        &[],
        0x00ff0000,
        Provenance::Confirmed {
            evidence: "crates/tt-tests/tests/step36_mop.rs::a_mop_expands_as_the_page_models_it",
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOP_CFG.md",
    );

    /// `SFPLOADI`. `WormholeB0/TensixTile/TensixCoprocessor/SFPLOADI.md`, which the Blackhole tree states is shared and identical.
    pub static SFPLOADI: InstructionDef = InstructionDef::new(
        "SFPLOADI",
        "SFPLOADI",
        0x71,
        &[
            Field::new("VD", 20, 4, false, None),
            Field::new("Mod0", 16, 4, false, None),
            Field::new("Imm16", 0, 16, false, None),
        ],
        &[],
        0x00000000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPLOADI.md",
    );

    /// `SFPIADD`. `WormholeB0/TensixTile/TensixCoprocessor/SFPIADD.md`, which the Blackhole tree states is shared and identical.
    pub static SFPIADD: InstructionDef = InstructionDef::new(
        "SFPIADD",
        "SFPIADD",
        0x79,
        &[
            Field::new("Imm12", 12, 12, true, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPIADD.md",
    );

    /// `SFPSWAP`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSWAP.md`.
    pub static SFPSWAP: InstructionDef = InstructionDef::new(
        "SFPSWAP",
        "SFPSWAP",
        0x92,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSWAP.md",
    );

    /// `SFPCONFIG`. `WormholeB0/TensixTile/TensixCoprocessor/SFPCONFIG.md`, which the Blackhole tree states is shared and identical.
    pub static SFPCONFIG: InstructionDef = InstructionDef::new(
        "SFPCONFIG",
        "SFPCONFIG",
        0x91,
        &[
            Field::new("Imm16", 8, 16, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPCONFIG.md",
    );

    /// `SFPMAD`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPMAD.md`.
    pub static SFPMAD: InstructionDef = InstructionDef::new(
        "SFPMAD",
        "SFPMAD",
        0x84,
        &[
            Field::new("VA", 16, 4, false, None),
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f00000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPMAD.md",
    );

    /// `SFPADD`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPADD.md`.
    pub static SFPADD: InstructionDef = InstructionDef::new(
        "SFPADD",
        "SFPADD",
        0x85,
        &[
            Field::new("VA", 16, 4, false, None),
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f00000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPADD.md",
    );

    /// `SFPMUL`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPMUL.md`.
    pub static SFPMUL: InstructionDef = InstructionDef::new(
        "SFPMUL",
        "SFPMUL",
        0x86,
        &[
            Field::new("VA", 16, 4, false, None),
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f00000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPMUL.md",
    );

    /// `SFPLUT`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPLUT.md`.
    pub static SFPLUT: InstructionDef = InstructionDef::new(
        "SFPLUT",
        "SFPLUT",
        0x73,
        &[
            Field::new("VD", 20, 4, false, None),
            Field::new("Mod0", 16, 4, false, None),
        ],
        &[],
        0x0000ffff,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPLUT.md",
    );

    /// `SFPMULI`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPMULI.md`.
    pub static SFPMULI: InstructionDef = InstructionDef::new(
        "SFPMULI",
        "SFPMULI",
        0x74,
        &[
            Field::new("Imm16", 8, 16, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPMULI.md",
    );

    /// `SFPADDI`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPADDI.md`.
    pub static SFPADDI: InstructionDef = InstructionDef::new(
        "SFPADDI",
        "SFPADDI",
        0x75,
        &[
            Field::new("Imm16", 8, 16, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPADDI.md",
    );

    /// `SFPDIVP2`. `WormholeB0/TensixTile/TensixCoprocessor/SFPDIVP2.md`, which the Blackhole tree states is shared and identical.
    pub static SFPDIVP2: InstructionDef = InstructionDef::new(
        "SFPDIVP2",
        "SFPDIVP2",
        0x76,
        &[
            Field::new("Imm8", 12, 8, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f00000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPDIVP2.md",
    );

    /// `SFPEXEXP`. `WormholeB0/TensixTile/TensixCoprocessor/SFPEXEXP.md`, which the Blackhole tree states is shared and identical.
    pub static SFPEXEXP: InstructionDef = InstructionDef::new(
        "SFPEXEXP",
        "SFPEXEXP",
        0x77,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPEXEXP.md",
    );

    /// `SFPEXMAN`. `WormholeB0/TensixTile/TensixCoprocessor/SFPEXMAN.md`, which the Blackhole tree states is shared and identical.
    pub static SFPEXMAN: InstructionDef = InstructionDef::new(
        "SFPEXMAN",
        "SFPEXMAN",
        0x78,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPEXMAN.md",
    );

    /// `SFPSHFT`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSHFT.md`.
    pub static SFPSHFT: InstructionDef = InstructionDef::new(
        "SFPSHFT",
        "SFPSHFT",
        0x7a,
        &[
            Field::new("Imm12", 12, 12, true, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSHFT.md",
    );

    /// `SFPSETCC`. `WormholeB0/TensixTile/TensixCoprocessor/SFPSETCC.md`, which the Blackhole tree states is shared and identical.
    pub static SFPSETCC: InstructionDef = InstructionDef::new(
        "SFPSETCC",
        "SFPSETCC",
        0x7b,
        &[
            Field::new("Imm1", 12, 1, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ffe000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPSETCC.md",
    );

    /// `SFPMOV`. `WormholeB0/TensixTile/TensixCoprocessor/SFPMOV.md`, which the Blackhole tree states is shared and identical.
    pub static SFPMOV: InstructionDef = InstructionDef::new(
        "SFPMOV",
        "SFPMOV",
        0x7c,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPMOV.md",
    );

    /// `SFPABS`. `WormholeB0/TensixTile/TensixCoprocessor/SFPABS.md`, which the Blackhole tree states is shared and identical.
    pub static SFPABS: InstructionDef = InstructionDef::new(
        "SFPABS",
        "SFPABS",
        0x7d,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPABS.md",
    );

    /// `SFPAND_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPAND.md`.
    pub static SFPAND: InstructionDef = InstructionDef::new(
        "SFPAND_BH",
        "SFPAND",
        0x7e,
        &[
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ff0000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPAND.md",
    );

    /// `SFPOR_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPOR.md`.
    pub static SFPOR: InstructionDef = InstructionDef::new(
        "SFPOR_BH",
        "SFPOR",
        0x7f,
        &[
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ff0000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPOR.md",
    );

    /// `SFPNOT`. `WormholeB0/TensixTile/TensixCoprocessor/SFPNOT.md`, which the Blackhole tree states is shared and identical.
    pub static SFPNOT: InstructionDef = InstructionDef::new(
        "SFPNOT",
        "SFPNOT",
        0x80,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
        ],
        &[],
        0x00fff00f,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPNOT.md",
    );

    /// `SFPLZ`. `WormholeB0/TensixTile/TensixCoprocessor/SFPLZ.md`, which the Blackhole tree states is shared and identical.
    pub static SFPLZ: InstructionDef = InstructionDef::new(
        "SFPLZ",
        "SFPLZ",
        0x81,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPLZ.md",
    );

    /// `SFPSETEXP`. `WormholeB0/TensixTile/TensixCoprocessor/SFPSETEXP.md`, which the Blackhole tree states is shared and identical.
    pub static SFPSETEXP: InstructionDef = InstructionDef::new(
        "SFPSETEXP",
        "SFPSETEXP",
        0x82,
        &[
            Field::new("Imm8", 12, 8, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f00000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPSETEXP.md",
    );

    /// `SFPSETMAN`. `WormholeB0/TensixTile/TensixCoprocessor/SFPSETMAN.md`, which the Blackhole tree states is shared and identical.
    pub static SFPSETMAN: InstructionDef = InstructionDef::new(
        "SFPSETMAN",
        "SFPSETMAN",
        0x83,
        &[
            Field::new("Imm12", 12, 12, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPSETMAN.md",
    );

    /// `SFPPUSHC_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPPUSHC.md`.
    pub static SFPPUSHC: InstructionDef = InstructionDef::new(
        "SFPPUSHC_BH",
        "SFPPUSHC",
        0x87,
        &[
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ffff00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPPUSHC.md",
    );

    /// `SFPPOPC`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPPOPC.md`.
    pub static SFPPOPC: InstructionDef = InstructionDef::new(
        "SFPPOPC",
        "SFPPOPC",
        0x88,
        &[
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ffff00,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPPOPC.md",
    );

    /// `SFPSETSGN`. `WormholeB0/TensixTile/TensixCoprocessor/SFPSETSGN.md`, which the Blackhole tree states is shared and identical.
    pub static SFPSETSGN: InstructionDef = InstructionDef::new(
        "SFPSETSGN",
        "SFPSETSGN",
        0x89,
        &[
            Field::new("Imm1", 12, 1, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ffe000,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPSETSGN.md",
    );

    /// `SFPENCC`. `WormholeB0/TensixTile/TensixCoprocessor/SFPENCC.md`, which the Blackhole tree states is shared and identical.
    pub static SFPENCC: InstructionDef = InstructionDef::new(
        "SFPENCC",
        "SFPENCC",
        0x8a,
        &[
            Field::new("Imm2", 12, 2, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ffcf00,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPENCC.md",
    );

    /// `SFPCOMPC`. `WormholeB0/TensixTile/TensixCoprocessor/SFPCOMPC.md`, which the Blackhole tree states is shared and identical.
    pub static SFPCOMPC: InstructionDef = InstructionDef::new(
        "SFPCOMPC",
        "SFPCOMPC",
        0x8b,
        &[Field::new("VD", 4, 4, false, None)],
        &[],
        0x00ffff0f,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPCOMPC.md",
    );

    /// `SFPTRANSP`. `WormholeB0/TensixTile/TensixCoprocessor/SFPTRANSP.md`, which the Blackhole tree states is shared and identical.
    pub static SFPTRANSP: InstructionDef = InstructionDef::new(
        "SFPTRANSP",
        "SFPTRANSP",
        0x8c,
        &[Field::new("VD", 4, 4, false, None)],
        &[],
        0x00ffff0f,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPTRANSP.md",
    );

    /// `SFPXOR`. `WormholeB0/TensixTile/TensixCoprocessor/SFPXOR.md`, which the Blackhole tree states is shared and identical.
    pub static SFPXOR: InstructionDef = InstructionDef::new(
        "SFPXOR",
        "SFPXOR",
        0x8d,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
        ],
        &[],
        0x00fff00f,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPXOR.md",
    );

    /// `SFPSTOCHRND_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSTOCHRND_FloatFloat.md`.
    pub static SFPSTOCHRND: InstructionDef = InstructionDef::new(
        "SFPSTOCHRND_BH",
        "SFP_STOCH_RND",
        0x8e,
        &[
            Field::new("RoundingMode", 21, 2, false, None),
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 3, false, None),
        ],
        &[],
        0x009f0008,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSTOCHRND_FloatFloat.md",
    );

    /// `SFPSTOCHRNDi_BH`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPSTOCHRND_IntInt.md`.
    pub static SFPSTOCHRNDi: InstructionDef = InstructionDef::new(
        "SFPSTOCHRNDi_BH",
        "SFP_STOCH_RND",
        0x8e,
        &[
            Field::new("RoundingMode", 21, 2, false, None),
            Field::new("Imm5", 16, 5, false, None),
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("UseImm5", 3, 1, false, None),
            Field::new("Mod1", 0, 3, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPSTOCHRND_IntInt.md",
    );

    /// `SFPNOP`. `WormholeB0/TensixTile/TensixCoprocessor/SFPNOP.md`, which the Blackhole tree states is shared and identical.
    pub static SFPNOP: InstructionDef = InstructionDef::new(
        "SFPNOP",
        "SFPNOP",
        0x8f,
        &[],
        &[(Field::new("", 7, 1, false, None), 0)],
        0x00ffff7f,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/SFPNOP.md",
    );

    /// `SFPCAST`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPCAST_IntAbs.md`.
    pub static SFPCAST: InstructionDef = InstructionDef::new(
        "SFPCAST",
        "SFPCAST",
        0x90,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPCAST_IntAbs.md",
    );

    /// `SFPLE`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPLE.md`.
    pub static SFPLE: InstructionDef = InstructionDef::new(
        "SFPLE",
        "SFPLE",
        0x96,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPLE.md",
    );

    /// `SFPGT`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPGT.md`.
    pub static SFPGT: InstructionDef = InstructionDef::new(
        "SFPGT",
        "SFPGT",
        0x97,
        &[
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPGT.md",
    );

    /// `SFPMUL24`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPMUL24.md`.
    pub static SFPMUL24: InstructionDef = InstructionDef::new(
        "SFPMUL24",
        "SFPMUL24",
        0x98,
        &[
            Field::new("VA", 16, 4, false, None),
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00f00000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPMUL24.md",
    );

    /// `SFPARECIP`. Documented for Blackhole in `BlackholeA0/TensixTile/TensixCoprocessor/SFPARECIP.md`.
    pub static SFPARECIP: InstructionDef = InstructionDef::new(
        "SFPARECIP",
        "SFPARECIP",
        0x99,
        &[
            Field::new("VB", 12, 4, false, None),
            Field::new("VC", 8, 4, false, None),
            Field::new("VD", 4, 4, false, None),
            Field::new("Mod1", 0, 4, false, None),
        ],
        &[],
        0x00ff0000,
        Provenance::Blackhole,
        "BlackholeA0/TensixTile/TensixCoprocessor/SFPARECIP.md",
    );

    /// `UNPACR_Regular`. `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_Regular.md`, which the Blackhole tree states is shared and identical.
    pub static UNPACR_Regular: InstructionDef = InstructionDef::new(
        "UNPACR_Regular",
        "UNPACR",
        0x42,
        &[
            Field::new("WhichUnpacker", 23, 1, false, None),
            Field::new("Ch1YInc", 21, 2, false, None),
            Field::new("Ch1ZInc", 19, 2, false, None),
            Field::new("Ch0YInc", 17, 2, false, None),
            Field::new("Ch0ZInc", 15, 2, false, None),
            Field::new("ContextNumber", 10, 3, false, None),
            Field::new("ContextADC", 8, 2, false, None),
            Field::new("MultiContextMode", 7, 1, false, None),
            Field::new("FlipSrc", 6, 1, false, None),
            Field::new("AllDatumsAreZero", 4, 1, false, None),
            Field::new("UseContextCounter", 3, 1, false, None),
            Field::new("RowSearch", 2, 1, false, None),
        ],
        &[
            (Field::new("", 1, 1, false, None), 0),
            (Field::new("", 13, 1, false, None), 0),
        ],
        0x00004021,
        Provenance::SharedWithWormhole,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_Regular.md",
    );

    /// `UNPACR_IncrementContextCounter`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_IncrementContextCounter.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static UNPACR_IncrementContextCounter: InstructionDef = InstructionDef::new(
        "UNPACR_IncrementContextCounter",
        "UNPACR",
        0x42,
        &[Field::new("WhichUnpacker", 23, 1, false, None)],
        &[
            (Field::new("", 1, 1, false, None), 0),
            (Field::new("", 13, 1, false, None), 1),
        ],
        0x007fdffd,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_IncrementContextCounter.md",
    );

    /// `UNPACR_FlushCache`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_FlushCache.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static UNPACR_FlushCache: InstructionDef = InstructionDef::new(
        "UNPACR_FlushCache",
        "UNPACR",
        0x42,
        &[
            Field::new("WhichUnpacker", 23, 1, false, None),
            Field::new("MultiContextMode", 7, 1, false, None),
        ],
        &[(Field::new("", 1, 1, false, None), 1)],
        0x007fff7d,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_FlushCache.md",
    );

    /// `UNPACR_NOP_OverlayClear0`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_OverlayClear.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static UNPACR_NOP_OverlayClear0: InstructionDef = InstructionDef::new(
        "UNPACR_NOP_OverlayClear0",
        "UNPACR_NOP",
        0x43,
        &[Field::new("WhichUnpacker", 23, 1, false, None)],
        &[(Field::new("", 0, 3, false, None), 0)],
        0x007ffff8,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_OverlayClear.md",
    );

    /// `UNPACR_NOP_OverlayClear3`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_OverlayClear.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static UNPACR_NOP_OverlayClear3: InstructionDef = InstructionDef::new(
        "UNPACR_NOP_OverlayClear3",
        "UNPACR_NOP",
        0x43,
        &[
            Field::new("WhichUnpacker", 23, 1, false, None),
            Field::new("WhichStream", 16, 6, false, None),
            Field::new("ClearCount", 4, 11, false, None),
        ],
        &[(Field::new("", 0, 3, false, None), 3)],
        0x00408008,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_OverlayClear.md",
    );

    /// `UNPACR_NOP_Nop`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_Nop.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static UNPACR_NOP_Nop: InstructionDef = InstructionDef::new(
        "UNPACR_NOP_Nop",
        "UNPACR_NOP",
        0x43,
        &[Field::new("WhichUnpacker", 23, 1, false, None)],
        &[(Field::new("", 0, 3, false, None), 2)],
        0x007ffff8,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_Nop.md",
    );

    /// `UNPACR_NOP_SETREG`. **`UNVERIFIED`.** `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_SETREG.md` is a Wormhole page and Blackhole has none, so this layout is a hypothesis until silicon or the simulator confirms it.
    pub static UNPACR_NOP_SETREG: InstructionDef = InstructionDef::new(
        "UNPACR_NOP_SETREG",
        "UNPACR_NOP",
        0x43,
        &[
            Field::new("WhichUnpacker", 23, 1, false, None),
            Field::new("AddrSel", 22, 1, false, None),
            Field::new("AddrMid", 16, 6, false, None),
            Field::new("Value11", 4, 11, false, None),
            Field::new("Accumulate", 3, 1, false, None),
        ],
        &[(Field::new("", 0, 3, false, None), 4)],
        0x00008000,
        Provenance::WormholeOnly,
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_SETREG.md",
    );

    /// `UNPACR_NOP_SETDVALID_BH`. **`MEASURED`** on Blackhole silicon by `crates/tt-tests/tests/step104_unpacker_handover.rs::llk_nonclearing_dvalid_a, crates/tt-tests/tests/step104_unpacker_handover.rs::llk_nonclearing_dvalid_b`. Replaces Wormhole fixed mode 7 with the bounded non-clearing publication profile 0x1e9; only WhichUnpacker varies. ttsim refuses this format-selector flavor.
    pub static UNPACR_NOP_SETDVALID: InstructionDef = InstructionDef::new(
        "UNPACR_NOP_SETDVALID_BH",
        "UNPACR_NOP",
        0x43,
        &[Field::new("WhichUnpacker", 23, 1, false, None)],
        &[(Field::new("", 0, 9, false, None), 489)],
        0x007ffe00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step104_unpacker_handover.rs::llk_nonclearing_dvalid_a, crates/tt-tests/tests/step104_unpacker_handover.rs::llk_nonclearing_dvalid_b",
            moved: &[],
            dropped: &[],
            widened: &[],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_SETDVALID.md",
    );

    /// `UNPACR_NOP_ZEROSRC_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step103_source_banks.rs::unpacr_zero_blackhole_bank_and_clear_value_fields, crates/tt-tests/tests/step103_source_banks.rs::unpacr_zero_waits_on_current_unpacker_bank_with_matrix_bank_held`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_ZEROSRC.md`), and on Blackhole `BothBanks`, `WaitLikeUnpacr` sit elsewhere. `NegativeInfSrcA` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static UNPACR_NOP_ZEROSRC: InstructionDef = InstructionDef::new(
        "UNPACR_NOP_ZEROSRC_BH",
        "UNPACR_NOP",
        0x43,
        &[
            Field::new("WhichUnpacker", 23, 1, false, None),
            Field::new("WaitLikeUnpacr", 5, 1, false, None),
            Field::new("BothBanks", 4, 1, false, None),
            Field::new("NegativeInfSrcA", 2, 2, false, None),
        ],
        &[
            (Field::new("", 0, 2, false, None), 1),
            (Field::new("", 6, 1, false, None), 0),
        ],
        0x007fff80,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step103_source_banks.rs::unpacr_zero_blackhole_bank_and_clear_value_fields, crates/tt-tests/tests/step103_source_banks.rs::unpacr_zero_waits_on_current_unpacker_bank_with_matrix_bank_held",
            moved: &["BothBanks", "WaitLikeUnpacr"],
            dropped: &[],
            widened: &["NegativeInfSrcA"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_ZEROSRC.md",
    );

    /// `GMPOOL_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step75_fpu_pooling.rs::matrix_pooling_uses_explicit_weights_and_releases_banks`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/GMPOOL.md`), and on Blackhole  sit elsewhere. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static GMPOOL: InstructionDef = InstructionDef::new(
        "GMPOOL_BH",
        "GMPOOL",
        0x33,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("AddrMod", 15, 2, false, None),
            Field::new("ArgMax", 14, 1, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[(Field::new("", 19, 1, false, None), 1)],
        0x00363c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step75_fpu_pooling.rs::matrix_pooling_uses_explicit_weights_and_releases_banks",
            moved: &[],
            dropped: &[],
            widened: &[],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/GMPOOL.md",
    );

    /// `GAPOOL_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step75_fpu_pooling.rs::matrix_pooling_uses_explicit_weights_and_releases_banks`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/GAPOOL.md`), and on Blackhole  sit elsewhere. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static GAPOOL: InstructionDef = InstructionDef::new(
        "GAPOOL_BH",
        "GAPOOL",
        0x34,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("AddrMod", 15, 2, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[(Field::new("", 19, 1, false, None), 1)],
        0x00367c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step75_fpu_pooling.rs::matrix_pooling_uses_explicit_weights_and_releases_banks",
            moved: &[],
            dropped: &[],
            widened: &[],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/GAPOOL.md",
    );

    /// `MVMUL_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::mvmul_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MVMUL.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MVMUL: InstructionDef = InstructionDef::new(
        "MVMUL_BH",
        "MVMUL",
        0x26,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("BroadcastSrcBRow", 19, 1, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00363c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::mvmul_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MVMUL.md",
    );

    /// `MOVA2D_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/probe_src.rs::mov_to_dst_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MOVA2D.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MOVA2D: InstructionDef = InstructionDef::new(
        "MOVA2D_BH",
        "MOVA2D",
        0x12,
        &[
            Field::new("UseDst32bLo", 23, 1, false, None),
            Field::new("SrcRow", 17, 6, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Move8Rows", 13, 1, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/probe_src.rs::mov_to_dst_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOVA2D.md",
    );

    /// `MOVB2D_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/probe_src.rs::movb2d_move4_rows_is_bit_13_on_blackhole, crates/tt-tests/tests/probe_src.rs::mov_to_dst_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MOVB2D.md`), and on Blackhole `BroadcastCol0`, `Broadcast1RowTo8`, `Move4Rows`, `AddrMod` sit elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MOVB2D: InstructionDef = InstructionDef::new(
        "MOVB2D_BH",
        "MOVB2D",
        0x13,
        &[
            Field::new("UseDst32bLo", 23, 1, false, None),
            Field::new("SrcRow", 17, 6, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Move4Rows", 13, 1, false, None),
            Field::new("Broadcast1RowTo8", 12, 1, false, None),
            Field::new("BroadcastCol0", 11, 1, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00000400,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/probe_src.rs::movb2d_move4_rows_is_bit_13_on_blackhole, crates/tt-tests/tests/probe_src.rs::mov_to_dst_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["BroadcastCol0", "Broadcast1RowTo8", "Move4Rows", "AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOVB2D.md",
    );

    /// `MOVD2A_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MOVD2A.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MOVD2A: InstructionDef = InstructionDef::new(
        "MOVD2A_BH",
        "MOVD2A",
        0x08,
        &[
            Field::new("UseDst32bLo", 23, 1, false, None),
            Field::new("SrcRow", 17, 6, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Move4Rows", 13, 1, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOVD2A.md",
    );

    /// `MOVD2B_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MOVD2B.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MOVD2B: InstructionDef = InstructionDef::new(
        "MOVD2B_BH",
        "MOVD2B",
        0x0a,
        &[
            Field::new("UseDst32bLo", 23, 1, false, None),
            Field::new("SrcRow", 17, 6, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Move4Rows", 13, 1, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOVD2B.md",
    );

    /// `MOVB2A_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MOVB2A.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MOVB2A: InstructionDef = InstructionDef::new(
        "MOVB2A_BH",
        "MOVB2A",
        0x0b,
        &[
            Field::new("SrcARow", 17, 6, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Move4Rows", 13, 1, false, None),
            Field::new("SrcBRow", 0, 6, false, None),
        ],
        &[],
        0x00801fc0,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOVB2A.md",
    );

    /// `ELWADD_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole, crates/tt-tests/tests/step9_matmul.rs::elw_broadcast_assignment_and_destination_fields, crates/tt-tests/tests/step90_matrix_eltwise.rs::resident_matrix_arithmetic_and_rhs_broadcasts`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/ELWADD.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Other fields are carried from that diagram. Step9 and step90 validate broadcast, assignment, Dst addressing and repeated bank release on ttsim and both Blackhole cards; floating arithmetic is not IEEE754.
    pub static ELWADD: InstructionDef = InstructionDef::new(
        "ELWADD_BH",
        "ELWADD",
        0x28,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("AddDst", 21, 1, false, None),
            Field::new("BroadcastSrcBRow", 20, 1, false, None),
            Field::new("BroadcastSrcBCol0", 19, 1, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00063c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole, crates/tt-tests/tests/step9_matmul.rs::elw_broadcast_assignment_and_destination_fields, crates/tt-tests/tests/step90_matrix_eltwise.rs::resident_matrix_arithmetic_and_rhs_broadcasts",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/ELWADD.md",
    );

    /// `ELWSUB_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole, crates/tt-tests/tests/step9_matmul.rs::elw_broadcast_assignment_and_destination_fields, crates/tt-tests/tests/step90_matrix_eltwise.rs::resident_matrix_arithmetic_and_rhs_broadcasts`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/ELWSUB.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Other fields are carried from that diagram. Step9 and step90 validate broadcast, assignment, Dst addressing and repeated bank release on ttsim and both Blackhole cards; floating arithmetic is not IEEE754.
    pub static ELWSUB: InstructionDef = InstructionDef::new(
        "ELWSUB_BH",
        "ELWSUB",
        0x30,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("AddDst", 21, 1, false, None),
            Field::new("BroadcastSrcBRow", 20, 1, false, None),
            Field::new("BroadcastSrcBCol0", 19, 1, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00063c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole, crates/tt-tests/tests/step9_matmul.rs::elw_broadcast_assignment_and_destination_fields, crates/tt-tests/tests/step90_matrix_eltwise.rs::resident_matrix_arithmetic_and_rhs_broadcasts",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/ELWSUB.md",
    );

    /// `ELWMUL_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole, crates/tt-tests/tests/step9_matmul.rs::elw_broadcast_assignment_and_destination_fields, crates/tt-tests/tests/step90_matrix_eltwise.rs::resident_matrix_arithmetic_and_rhs_broadcasts`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/ELWMUL.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Other fields are carried from that diagram. Step9 and step90 validate broadcast, assignment, Dst addressing and repeated bank release on ttsim and both Blackhole cards; floating arithmetic is not IEEE754.
    pub static ELWMUL: InstructionDef = InstructionDef::new(
        "ELWMUL_BH",
        "ELWMUL",
        0x27,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("BroadcastSrcBRow", 20, 1, false, None),
            Field::new("BroadcastSrcBCol0", 19, 1, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00263c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole, crates/tt-tests/tests/step9_matmul.rs::elw_broadcast_assignment_and_destination_fields, crates/tt-tests/tests/step90_matrix_eltwise.rs::resident_matrix_arithmetic_and_rhs_broadcasts",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/ELWMUL.md",
    );

    /// `DOTPV_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/DOTPV.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static DOTPV: InstructionDef = InstructionDef::new(
        "DOTPV_BH",
        "DOTPV",
        0x29,
        &[
            Field::new("FlipSrcB", 23, 1, false, None),
            Field::new("FlipSrcA", 22, 1, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x003e3c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/DOTPV.md",
    );

    /// `MOVDBGA2D_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/MOVDBGA2D.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static MOVDBGA2D: InstructionDef = InstructionDef::new(
        "MOVDBGA2D_BH",
        "MOVDBGA2D",
        0x09,
        &[
            Field::new("UseDst32bLo", 23, 1, false, None),
            Field::new("SrcRow", 17, 6, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Move8Rows", 13, 1, false, None),
            Field::new("DstRow", 0, 10, false, None),
        ],
        &[],
        0x00001c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/MOVDBGA2D.md",
    );

    /// `SHIFTXB_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::shiftxb_addr_mod_sits_one_bit_lower_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/SHIFTXB.md`), and on Blackhole `AddrMod` sits elsewhere. `AddrMod` has a different width on Blackhole. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static SHIFTXB: InstructionDef = InstructionDef::new(
        "SHIFTXB_BH",
        "SHIFTXB",
        0x18,
        &[
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("ShiftInZero", 10, 1, false, None),
            Field::new("SrcRow", 0, 6, false, None),
        ],
        &[],
        0x00fe3bc0,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::shiftxb_addr_mod_sits_one_bit_lower_on_blackhole",
            moved: &["AddrMod"],
            dropped: &[],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/SHIFTXB.md",
    );

    /// `ZEROACC_BH`. **`MEASURED`** on ttsim or silicon by `crates/tt-tests/tests/step9_matmul.rs::zeroacc_addr_mod_and_use_dst32b_on_blackhole`, not documented: the only diagram is Wormhole's (`WormholeB0/TensixTile/TensixCoprocessor/ZEROACC.md`), and on Blackhole `AddrMod`, `UseDst32b` sit elsewhere. `AddrMod` has a different width on Blackhole. `Revert` is not carried: its bits hold something else on Blackhole and its own position is unknown. Every other field is carried from that diagram and is as unverified as it was. Re-derive on silicon.
    pub static ZEROACC: InstructionDef = InstructionDef::new(
        "ZEROACC_BH",
        "ZEROACC",
        0x10,
        &[
            Field::new("Mode", 19, 2, false, None),
            Field::new("UseDst32b", 18, 1, false, None),
            Field::new("AddrMod", 14, 3, false, None),
            Field::new("Imm10", 0, 10, false, None),
        ],
        &[],
        0x00e23c00,
        Provenance::Measured {
            evidence: "crates/tt-tests/tests/step9_matmul.rs::zeroacc_addr_mod_and_use_dst32b_on_blackhole",
            moved: &["AddrMod", "UseDst32b"],
            dropped: &["Revert"],
            widened: &["AddrMod"],
        },
        "WormholeB0/TensixTile/TensixCoprocessor/ZEROACC.md",
    );

    /// Encodings Blackhole replaces. Present so that the difference is
    /// visible and testable, not so that they can be used here.
    pub mod wormhole {
        use super::super::*;

        /// `STALLWAIT`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/STALLWAIT.md`. Blackhole replaces it with `STALLWAIT_BH`; use that instead.
        pub static STALLWAIT: InstructionDef = InstructionDef::new(
            "STALLWAIT",
            "STALLWAIT",
            0xa2,
            &[
                Field::new("BlockMask", 15, 9, false, None),
                Field::new("ConditionMask", 0, 15, false, None),
            ],
            &[],
            0x00000000,
            Provenance::SupersededOnBlackhole { by: "STALLWAIT_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/STALLWAIT.md",
        );

        /// `PACR`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/PACR.md`. Blackhole replaces it with `PACR_BH`; use that instead.
        pub static PACR: InstructionDef = InstructionDef::new(
            "PACR",
            "PACR",
            0x41,
            &[
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("ZeroWrite", 12, 1, false, None),
                Field::new("PackerMask", 8, 4, false, None),
                Field::new("OvrdThreadId", 7, 1, false, None),
                Field::new("Concat", 4, 1, false, None),
                Field::new("Flush", 1, 1, false, None),
                Field::new("Last", 0, 1, false, None),
            ],
            &[(Field::new("", 23, 1, false, None), 0)],
            0x007e606c,
            Provenance::SupersededOnBlackhole { by: "PACR_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/PACR.md",
        );

        /// `SFPLUTFP32`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPLUTFP32.md`. Blackhole replaces it with `SFPLUTFP32_BH`; use that instead.
        pub static SFPLUTFP32: InstructionDef = InstructionDef::new(
            "SFPLUTFP32",
            "SFPLUTFP32",
            0x95,
            &[
                Field::new("VD", 4, 4, false, None),
                Field::new("Mod1", 0, 4, false, None),
            ],
            &[],
            0x00ffff00,
            Provenance::SupersededOnBlackhole {
                by: "SFPLUTFP32_BH",
            },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPLUTFP32.md",
        );

        /// `MOVD2A`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MOVD2A.md`. Blackhole replaces it with `MOVD2A_BH`; use that instead.
        pub static MOVD2A: InstructionDef = InstructionDef::new(
            "MOVD2A",
            "MOVD2A",
            0x08,
            &[
                Field::new("UseDst32bLo", 23, 1, false, None),
                Field::new("SrcRow", 17, 6, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Move4Rows", 13, 1, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00005c00,
            Provenance::SupersededOnBlackhole { by: "MOVD2A_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MOVD2A.md",
        );

        /// `MOVD2B`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MOVD2B.md`. Blackhole replaces it with `MOVD2B_BH`; use that instead.
        pub static MOVD2B: InstructionDef = InstructionDef::new(
            "MOVD2B",
            "MOVD2B",
            0x0a,
            &[
                Field::new("UseDst32bLo", 23, 1, false, None),
                Field::new("SrcRow", 17, 6, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Move4Rows", 13, 1, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00005c00,
            Provenance::SupersededOnBlackhole { by: "MOVD2B_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MOVD2B.md",
        );

        /// `MOVB2A`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MOVB2A.md`. Blackhole replaces it with `MOVB2A_BH`; use that instead.
        pub static MOVB2A: InstructionDef = InstructionDef::new(
            "MOVB2A",
            "MOVB2A",
            0x0b,
            &[
                Field::new("SrcARow", 17, 6, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Move4Rows", 13, 1, false, None),
                Field::new("SrcBRow", 0, 6, false, None),
            ],
            &[],
            0x00805fc0,
            Provenance::SupersededOnBlackhole { by: "MOVB2A_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MOVB2A.md",
        );

        /// `MOVDBGA2D`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MOVDBGA2D.md`. Blackhole replaces it with `MOVDBGA2D_BH`; use that instead.
        pub static MOVDBGA2D: InstructionDef = InstructionDef::new(
            "MOVDBGA2D",
            "MOVDBGA2D",
            0x09,
            &[
                Field::new("UseDst32bLo", 23, 1, false, None),
                Field::new("SrcRow", 17, 6, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Move8Rows", 13, 1, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00005c00,
            Provenance::SupersededOnBlackhole { by: "MOVDBGA2D_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MOVDBGA2D.md",
        );

        /// `MOVA2D`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MOVA2D.md`. Blackhole replaces it with `MOVA2D_BH`; use that instead.
        pub static MOVA2D: InstructionDef = InstructionDef::new(
            "MOVA2D",
            "MOVA2D",
            0x12,
            &[
                Field::new("UseDst32bLo", 23, 1, false, None),
                Field::new("SrcRow", 17, 6, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Move8Rows", 13, 1, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00005c00,
            Provenance::SupersededOnBlackhole { by: "MOVA2D_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MOVA2D.md",
        );

        /// `MOVB2D`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MOVB2D.md`. Blackhole replaces it with `MOVB2D_BH`; use that instead.
        pub static MOVB2D: InstructionDef = InstructionDef::new(
            "MOVB2D",
            "MOVB2D",
            0x13,
            &[
                Field::new("UseDst32bLo", 23, 1, false, None),
                Field::new("SrcRow", 17, 6, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Move4Rows", 14, 1, false, None),
                Field::new("Broadcast1RowTo8", 13, 1, false, None),
                Field::new("BroadcastCol0", 12, 1, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00000c00,
            Provenance::SupersededOnBlackhole { by: "MOVB2D_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MOVB2D.md",
        );

        /// `ELWMUL`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/ELWMUL.md`. Blackhole replaces it with `ELWMUL_BH`; use that instead.
        pub static ELWMUL: InstructionDef = InstructionDef::new(
            "ELWMUL",
            "ELWMUL",
            0x27,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("BroadcastSrcBRow", 20, 1, false, None),
                Field::new("BroadcastSrcBCol0", 19, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00267c00,
            Provenance::SupersededOnBlackhole { by: "ELWMUL_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/ELWMUL.md",
        );

        /// `ELWADD`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/ELWADD.md`. Blackhole replaces it with `ELWADD_BH`; use that instead.
        pub static ELWADD: InstructionDef = InstructionDef::new(
            "ELWADD",
            "ELWADD",
            0x28,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("AddDst", 21, 1, false, None),
                Field::new("BroadcastSrcBRow", 20, 1, false, None),
                Field::new("BroadcastSrcBCol0", 19, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00067c00,
            Provenance::SupersededOnBlackhole { by: "ELWADD_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/ELWADD.md",
        );

        /// `ELWSUB`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/ELWSUB.md`. Blackhole replaces it with `ELWSUB_BH`; use that instead.
        pub static ELWSUB: InstructionDef = InstructionDef::new(
            "ELWSUB",
            "ELWSUB",
            0x30,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("AddDst", 21, 1, false, None),
                Field::new("BroadcastSrcBRow", 20, 1, false, None),
                Field::new("BroadcastSrcBCol0", 19, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00067c00,
            Provenance::SupersededOnBlackhole { by: "ELWSUB_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/ELWSUB.md",
        );

        /// `GMPOOL`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/GMPOOL.md`. Blackhole replaces it with `GMPOOL_BH`; use that instead.
        pub static GMPOOL: InstructionDef = InstructionDef::new(
            "GMPOOL",
            "GMPOOL",
            0x33,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("ArgMax", 14, 1, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x003e3c00,
            Provenance::SupersededOnBlackhole { by: "GMPOOL_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/GMPOOL.md",
        );

        /// `ZEROACC`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/ZEROACC.md`. Blackhole replaces it with `ZEROACC_BH`; use that instead.
        pub static ZEROACC: InstructionDef = InstructionDef::new(
            "ZEROACC",
            "ZEROACC",
            0x10,
            &[
                Field::new("UseDst32b", 21, 1, false, None),
                Field::new("Mode", 19, 2, false, None),
                Field::new("Revert", 18, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("Imm10", 0, 10, false, None),
            ],
            &[],
            0x00c27c00,
            Provenance::SupersededOnBlackhole { by: "ZEROACC_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/ZEROACC.md",
        );

        /// `SHIFTXB`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SHIFTXB.md`. Blackhole replaces it with `SHIFTXB_BH`; use that instead.
        pub static SHIFTXB: InstructionDef = InstructionDef::new(
            "SHIFTXB",
            "SHIFTXB",
            0x18,
            &[
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("ShiftInZero", 10, 1, false, None),
                Field::new("SrcRow", 0, 6, false, None),
            ],
            &[],
            0x00fe7bc0,
            Provenance::SupersededOnBlackhole { by: "SHIFTXB_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/SHIFTXB.md",
        );

        /// `MVMUL`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/MVMUL.md`. Blackhole replaces it with `MVMUL_BH`; use that instead.
        pub static MVMUL: InstructionDef = InstructionDef::new(
            "MVMUL",
            "MVMUL",
            0x26,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("BroadcastSrcBRow", 19, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x00367c00,
            Provenance::SupersededOnBlackhole { by: "MVMUL_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/MVMUL.md",
        );

        /// `DOTPV`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/DOTPV.md`. Blackhole replaces it with `DOTPV_BH`; use that instead.
        pub static DOTPV: InstructionDef = InstructionDef::new(
            "DOTPV",
            "DOTPV",
            0x29,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x003e7c00,
            Provenance::SupersededOnBlackhole { by: "DOTPV_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/DOTPV.md",
        );

        /// `GAPOOL`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/GAPOOL.md`. Blackhole replaces it with `GAPOOL_BH`; use that instead.
        pub static GAPOOL: InstructionDef = InstructionDef::new(
            "GAPOOL",
            "GAPOOL",
            0x34,
            &[
                Field::new("FlipSrcB", 23, 1, false, None),
                Field::new("FlipSrcA", 22, 1, false, None),
                Field::new("AddrMod", 15, 2, false, None),
                Field::new("DstRow", 0, 10, false, None),
            ],
            &[],
            0x003e7c00,
            Provenance::SupersededOnBlackhole { by: "GAPOOL_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/GAPOOL.md",
        );

        /// `SFPLOAD`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPLOAD.md`. Blackhole replaces it with `SFPLOAD_BH`; use that instead.
        pub static SFPLOAD: InstructionDef = InstructionDef::new(
            "SFPLOAD",
            "SFPLOAD",
            0x70,
            &[
                Field::new("VD", 20, 4, false, None),
                Field::new("Mod0", 16, 4, false, None),
                Field::new("AddrMod", 14, 2, false, None),
                Field::new("Imm10", 0, 10, false, None),
            ],
            &[],
            0x00003c00,
            Provenance::SupersededOnBlackhole { by: "SFPLOAD_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPLOAD.md",
        );

        /// `SFPLOADMACRO`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPLOADMACRO.md`. Blackhole replaces it with `SFPLOADMACRO_BH`; use that instead.
        pub static SFPLOADMACRO: InstructionDef = InstructionDef::new(
            "SFPLOADMACRO",
            "SFPLOADMACRO",
            0x93,
            &[
                Field::new("MacroIndex", 22, 2, false, None),
                Field::new("VDLo", 20, 2, false, None),
                Field::new("Mod0", 16, 4, false, None),
                Field::new("AddrMod", 14, 2, false, None),
                Field::new("Imm9", 1, 9, false, None),
                Field::new("VDHi", 0, 1, false, None),
            ],
            &[],
            0x00003c00,
            Provenance::SupersededOnBlackhole {
                by: "SFPLOADMACRO_BH",
            },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPLOADMACRO.md",
        );

        /// `SFPSTORE`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPSTORE.md`. Blackhole replaces it with `SFPSTORE_BH`; use that instead.
        pub static SFPSTORE: InstructionDef = InstructionDef::new(
            "SFPSTORE",
            "SFPSTORE",
            0x72,
            &[
                Field::new("VD", 20, 4, false, None),
                Field::new("Mod0", 16, 4, false, None),
                Field::new("AddrMod", 14, 2, false, None),
                Field::new("Imm10", 0, 10, false, None),
            ],
            &[],
            0x00003c00,
            Provenance::SupersededOnBlackhole { by: "SFPSTORE_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPSTORE.md",
        );

        /// `SFPAND`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPAND.md`. Blackhole replaces it with `SFPAND_BH`; use that instead.
        pub static SFPAND: InstructionDef = InstructionDef::new(
            "SFPAND",
            "SFPAND",
            0x7e,
            &[
                Field::new("VC", 8, 4, false, None),
                Field::new("VD", 4, 4, false, None),
            ],
            &[],
            0x00fff00f,
            Provenance::SupersededOnBlackhole { by: "SFPAND_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPAND.md",
        );

        /// `SFPOR`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPOR.md`. Blackhole replaces it with `SFPOR_BH`; use that instead.
        pub static SFPOR: InstructionDef = InstructionDef::new(
            "SFPOR",
            "SFPOR",
            0x7f,
            &[
                Field::new("VC", 8, 4, false, None),
                Field::new("VD", 4, 4, false, None),
            ],
            &[],
            0x00fff00f,
            Provenance::SupersededOnBlackhole { by: "SFPOR_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPOR.md",
        );

        /// `SFPPUSHC`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPPUSHC.md`. Blackhole replaces it with `SFPPUSHC_BH`; use that instead.
        pub static SFPPUSHC: InstructionDef = InstructionDef::new(
            "SFPPUSHC",
            "SFPPUSHC",
            0x87,
            &[Field::new("VD", 4, 4, false, None)],
            &[(Field::new("", 0, 4, false, None), 0)],
            0x00ffff00,
            Provenance::SupersededOnBlackhole { by: "SFPPUSHC_BH" },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPPUSHC.md",
        );

        /// `SFPSTOCHRND`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPSTOCHRND_FloatFloat.md`. Blackhole replaces it with `SFPSTOCHRND_BH`; use that instead.
        pub static SFPSTOCHRND: InstructionDef = InstructionDef::new(
            "SFPSTOCHRND",
            "SFP_STOCH_RND",
            0x8e,
            &[
                Field::new("StochasticRounding", 21, 1, false, None),
                Field::new("VC", 8, 4, false, None),
                Field::new("VD", 4, 4, false, None),
                Field::new("Mod1", 0, 3, false, None),
            ],
            &[],
            0x00dff008,
            Provenance::SupersededOnBlackhole {
                by: "SFPSTOCHRND_BH",
            },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPSTOCHRND_FloatFloat.md",
        );

        /// `SFPSTOCHRNDi`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/SFPSTOCHRND_IntInt.md`. Blackhole replaces it with `SFPSTOCHRNDi_BH`; use that instead.
        pub static SFPSTOCHRNDi: InstructionDef = InstructionDef::new(
            "SFPSTOCHRNDi",
            "SFP_STOCH_RND",
            0x8e,
            &[
                Field::new("StochasticRounding", 21, 1, false, None),
                Field::new("Imm5", 16, 5, false, None),
                Field::new("VB", 12, 4, false, None),
                Field::new("VC", 8, 4, false, None),
                Field::new("VD", 4, 4, false, None),
                Field::new("UseImm5", 3, 1, false, None),
                Field::new("Mod1", 0, 3, false, None),
            ],
            &[],
            0x00c00000,
            Provenance::SupersededOnBlackhole {
                by: "SFPSTOCHRNDi_BH",
            },
            "WormholeB0/TensixTile/TensixCoprocessor/SFPSTOCHRND_IntInt.md",
        );

        /// `UNPACR_NOP_ZEROSRC`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_ZEROSRC.md`. Blackhole replaces it with `UNPACR_NOP_ZEROSRC_BH`; use that instead.
        pub static UNPACR_NOP_ZEROSRC: InstructionDef = InstructionDef::new(
            "UNPACR_NOP_ZEROSRC",
            "UNPACR_NOP",
            0x43,
            &[
                Field::new("WhichUnpacker", 23, 1, false, None),
                Field::new("WaitLikeUnpacr", 4, 1, false, None),
                Field::new("BothBanks", 3, 1, false, None),
                Field::new("NegativeInfSrcA", 2, 1, false, None),
            ],
            &[
                (Field::new("", 0, 2, false, None), 1),
                (Field::new("", 6, 1, false, None), 0),
            ],
            0x007fffa0,
            Provenance::SupersededOnBlackhole {
                by: "UNPACR_NOP_ZEROSRC_BH",
            },
            "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_ZEROSRC.md",
        );

        /// `UNPACR_NOP_SETDVALID`. **Wormhole's encoding**, from `WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_SETDVALID.md`. Blackhole replaces it with `UNPACR_NOP_SETDVALID_BH`; use that instead.
        pub static UNPACR_NOP_SETDVALID: InstructionDef = InstructionDef::new(
            "UNPACR_NOP_SETDVALID",
            "UNPACR_NOP",
            0x43,
            &[Field::new("WhichUnpacker", 23, 1, false, None)],
            &[(Field::new("", 0, 3, false, None), 7)],
            0x007ffff8,
            Provenance::SupersededOnBlackhole {
                by: "UNPACR_NOP_SETDVALID_BH",
            },
            "WormholeB0/TensixTile/TensixCoprocessor/UNPACR_NOP_SETDVALID.md",
        );
    }
}

/// Every instruction encoding, for table-wide checks.
pub static ALL: &[&InstructionDef] = &[
    &defs::ATCAS,
    &defs::ATSWAP,
    &defs::RMWCIB0,
    &defs::RMWCIB1,
    &defs::RMWCIB2,
    &defs::RMWCIB3,
    &defs::SETDMAREG_Immediate,
    &defs::SETDMAREG_Special,
    &defs::wormhole::STALLWAIT,
    &defs::STALLWAIT,
    &defs::SEMWAIT,
    &defs::STREAMWAIT,
    &defs::SEMINIT,
    &defs::REPLAY,
    &defs::wormhole::PACR,
    &defs::PACR,
    &defs::PACR_SETREG,
    &defs::SFPSHFT2,
    &defs::SFPSHFT2b,
    &defs::wormhole::SFPLUTFP32,
    &defs::SFPLUTFP32,
    &defs::REG2FLOP_Configuration,
    &defs::REG2FLOP_ADC,
    &defs::XMOV,
    &defs::WRCFG,
    &defs::RDCFG,
    &defs::SETC16,
    &defs::STREAMWRCFG,
    &defs::CFGSHIFTMASK,
    &defs::SETADC,
    &defs::SETADCXY,
    &defs::INCADCXY,
    &defs::ADDRCRXY,
    &defs::SETADCZW,
    &defs::INCADCZW,
    &defs::ADDRCRZW,
    &defs::SETADCXX,
    &defs::NOP,
    &defs::SEMPOST,
    &defs::SEMGET,
    &defs::SETDVALID,
    &defs::GATESRCRST,
    &defs::wormhole::MOVD2A,
    &defs::wormhole::MOVD2B,
    &defs::wormhole::MOVB2A,
    &defs::wormhole::MOVDBGA2D,
    &defs::ZEROSRC,
    &defs::wormhole::MOVA2D,
    &defs::wormhole::MOVB2D,
    &defs::wormhole::ELWMUL,
    &defs::wormhole::ELWADD,
    &defs::wormhole::ELWSUB,
    &defs::wormhole::GMPOOL,
    &defs::wormhole::ZEROACC,
    &defs::TRNSPSRCB,
    &defs::SHIFTXA,
    &defs::wormhole::SHIFTXB,
    &defs::CLREXPHIST,
    &defs::wormhole::MVMUL,
    &defs::wormhole::DOTPV,
    &defs::wormhole::GAPOOL,
    &defs::CLEARDVALID,
    &defs::SETRWC,
    &defs::INCRWC,
    &defs::LOADIND,
    &defs::STOREIND_L1,
    &defs::STOREIND_MMIO,
    &defs::STOREIND_Src,
    &defs::LOADREG,
    &defs::STOREREG,
    &defs::FLUSHDMA,
    &defs::DMANOP,
    &defs::ADDDMAREG,
    &defs::ADDDMAREGi,
    &defs::SUBDMAREG,
    &defs::SUBDMAREGi,
    &defs::MULDMAREG,
    &defs::MULDMAREGi,
    &defs::BITWOPDMAREG,
    &defs::BITWOPDMAREGi,
    &defs::SHIFTDMAREG,
    &defs::SHIFTDMAREGi,
    &defs::CMPDMAREG,
    &defs::CMPDMAREGi,
    &defs::ATGETM,
    &defs::ATRELM,
    &defs::ATINCGET,
    &defs::ATINCGETPTR,
    &defs::wormhole::SFPLOAD,
    &defs::SFPLOAD,
    &defs::wormhole::SFPLOADMACRO,
    &defs::SFPLOADMACRO,
    &defs::wormhole::SFPSTORE,
    &defs::SFPSTORE,
    &defs::MOP,
    &defs::MOP_CFG,
    &defs::SFPLOADI,
    &defs::SFPIADD,
    &defs::SFPSWAP,
    &defs::SFPCONFIG,
    &defs::SFPMAD,
    &defs::SFPADD,
    &defs::SFPMUL,
    &defs::SFPLUT,
    &defs::SFPMULI,
    &defs::SFPADDI,
    &defs::SFPDIVP2,
    &defs::SFPEXEXP,
    &defs::SFPEXMAN,
    &defs::SFPSHFT,
    &defs::SFPSETCC,
    &defs::SFPMOV,
    &defs::SFPABS,
    &defs::wormhole::SFPAND,
    &defs::SFPAND,
    &defs::wormhole::SFPOR,
    &defs::SFPOR,
    &defs::SFPNOT,
    &defs::SFPLZ,
    &defs::SFPSETEXP,
    &defs::SFPSETMAN,
    &defs::wormhole::SFPPUSHC,
    &defs::SFPPUSHC,
    &defs::SFPPOPC,
    &defs::SFPSETSGN,
    &defs::SFPENCC,
    &defs::SFPCOMPC,
    &defs::SFPTRANSP,
    &defs::SFPXOR,
    &defs::wormhole::SFPSTOCHRND,
    &defs::wormhole::SFPSTOCHRNDi,
    &defs::SFPSTOCHRND,
    &defs::SFPSTOCHRNDi,
    &defs::SFPNOP,
    &defs::SFPCAST,
    &defs::SFPLE,
    &defs::SFPGT,
    &defs::SFPMUL24,
    &defs::SFPARECIP,
    &defs::UNPACR_Regular,
    &defs::UNPACR_IncrementContextCounter,
    &defs::UNPACR_FlushCache,
    &defs::UNPACR_NOP_OverlayClear0,
    &defs::UNPACR_NOP_OverlayClear3,
    &defs::wormhole::UNPACR_NOP_ZEROSRC,
    &defs::UNPACR_NOP_Nop,
    &defs::UNPACR_NOP_SETREG,
    &defs::wormhole::UNPACR_NOP_SETDVALID,
    &defs::UNPACR_NOP_SETDVALID,
    &defs::UNPACR_NOP_ZEROSRC,
    &defs::GMPOOL,
    &defs::GAPOOL,
    &defs::MVMUL,
    &defs::MOVA2D,
    &defs::MOVB2D,
    &defs::MOVD2A,
    &defs::MOVD2B,
    &defs::MOVB2A,
    &defs::ELWADD,
    &defs::ELWSUB,
    &defs::ELWMUL,
    &defs::DOTPV,
    &defs::MOVDBGA2D,
    &defs::SHIFTXB,
    &defs::ZEROACC,
];

/// The documented bit layout of each datum type in `Src` and `Dst`.
///
/// The coprocessor does not entirely follow IEEE 754, so conversions are
/// built against these rather than against assumptions.
pub mod datum {
    use super::*;

    /// `Src_TF32`, from `WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md`.
    pub static SRC_TF32: DatumLayout = DatumLayout::new(
        "Src_TF32",
        19,
        &[
            Field::new("Sign", 18, 1, false, None),
            Field::new("Mantissa", 8, 10, false, None),
            Field::new("Exponent", 0, 8, false, None),
        ],
        &[],
        "WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md",
    );

    /// `Src_BF16`, from `WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md`.
    pub static SRC_BF16: DatumLayout = DatumLayout::new(
        "Src_BF16",
        19,
        &[
            Field::new("Sign", 18, 1, false, None),
            Field::new("Mantissa", 11, 7, false, None),
            Field::new("Exponent", 0, 8, false, None),
        ],
        &[(Field::new("", 8, 3, false, None), 0)],
        "WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md",
    );

    /// `Src_FP16`, from `WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md`.
    pub static SRC_FP16: DatumLayout = DatumLayout::new(
        "Src_FP16",
        19,
        &[
            Field::new("Sign", 18, 1, false, None),
            Field::new("Mantissa", 8, 10, false, None),
            Field::new("Exponent", 0, 5, false, None),
        ],
        &[(Field::new("", 5, 3, false, None), 0)],
        "WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md",
    );

    /// `Src_INT8`, from `WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md`.
    pub static SRC_INT8: DatumLayout = DatumLayout::new(
        "Src_INT8",
        19,
        &[
            Field::new("Sign", 18, 1, false, None),
            Field::new("Magnitude", 8, 10, false, None),
            Field::new("Exponent", 0, 5, false, Some("16 or 0")),
        ],
        &[(Field::new("", 5, 3, false, None), 0)],
        "WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md",
    );

    /// `Src_INT16`, from `WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md`.
    pub static SRC_INT16: DatumLayout = DatumLayout::new(
        "Src_INT16",
        19,
        &[
            Field::new("Sign", 18, 1, false, None),
            Field::new("Magnitude", 11, 7, false, Some("high")),
            Field::new("Magnitude", 0, 8, false, Some("low")),
        ],
        &[(Field::new("", 8, 3, false, None), 0)],
        "WormholeB0/TensixTile/TensixCoprocessor/SrcASrcB.md",
    );

    /// `Dst16_BF16`, from `BlackholeA0/TensixTile/TensixCoprocessor/Dst.md`.
    pub static DST16_BF16: DatumLayout = DatumLayout::new(
        "Dst16_BF16",
        16,
        &[
            Field::new("Sign", 15, 1, false, None),
            Field::new("Mantissa", 8, 7, false, None),
            Field::new("Exponent", 0, 8, false, None),
        ],
        &[],
        "BlackholeA0/TensixTile/TensixCoprocessor/Dst.md",
    );

    /// `Dst16_FP16`, from `BlackholeA0/TensixTile/TensixCoprocessor/Dst.md`.
    pub static DST16_FP16: DatumLayout = DatumLayout::new(
        "Dst16_FP16",
        16,
        &[
            Field::new("Sign", 15, 1, false, None),
            Field::new("Mantissa", 5, 10, false, None),
            Field::new("Exponent", 0, 5, false, None),
        ],
        &[],
        "BlackholeA0/TensixTile/TensixCoprocessor/Dst.md",
    );

    /// `Dst16_INT8`, from `BlackholeA0/TensixTile/TensixCoprocessor/Dst.md`.
    pub static DST16_INT8: DatumLayout = DatumLayout::new(
        "Dst16_INT8",
        16,
        &[
            Field::new("Sign", 15, 1, false, None),
            Field::new("Magnitude", 5, 10, false, None),
            Field::new("Exponent", 0, 5, false, Some("16 or 0")),
        ],
        &[],
        "BlackholeA0/TensixTile/TensixCoprocessor/Dst.md",
    );

    /// `Dst16_INT16`, from `BlackholeA0/TensixTile/TensixCoprocessor/Dst.md`.
    pub static DST16_INT16: DatumLayout = DatumLayout::new(
        "Dst16_INT16",
        16,
        &[
            Field::new("Sign", 15, 1, false, None),
            Field::new("Magnitude", 0, 15, false, None),
        ],
        &[],
        "BlackholeA0/TensixTile/TensixCoprocessor/Dst.md",
    );

    /// `Dst32_FP32`, from `BlackholeA0/TensixTile/TensixCoprocessor/Dst.md`.
    pub static DST32_FP32: DatumLayout = DatumLayout::new(
        "Dst32_FP32",
        32,
        &[
            Field::new("Sign", 31, 1, false, None),
            Field::new("Mantissa", 24, 7, false, Some("high")),
            Field::new("Exponent", 16, 8, false, None),
            Field::new("Mantissa", 0, 16, false, Some("low")),
        ],
        &[],
        "BlackholeA0/TensixTile/TensixCoprocessor/Dst.md",
    );

    /// `Dst32_INT32`, from `BlackholeA0/TensixTile/TensixCoprocessor/Dst.md`.
    pub static DST32_INT32: DatumLayout = DatumLayout::new(
        "Dst32_INT32",
        32,
        &[
            Field::new("Sign", 31, 1, false, None),
            Field::new("Magnitude", 24, 7, false, Some("middle")),
            Field::new("Magnitude", 16, 8, false, Some("high")),
            Field::new("Magnitude", 0, 16, false, Some("low")),
        ],
        &[],
        "BlackholeA0/TensixTile/TensixCoprocessor/Dst.md",
    );

    /// `NOC_AT_LEN_BE_Increment`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_INCREMENT: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_Increment",
        32,
        &[
            Field::new("IntWidth", 2, 5, false, None),
            Field::new("Ofs", 0, 2, false, None),
        ],
        &[(Field::new("", 12, 4, false, None), 1)],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_CAS`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_CAS: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_CAS",
        32,
        &[
            Field::new("SetVal", 6, 4, false, None),
            Field::new("CmpVal", 2, 4, false, None),
            Field::new("Ofs", 0, 2, false, None),
        ],
        &[(Field::new("", 12, 4, false, None), 4)],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_SwapMask`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_SWAPMASK: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_SwapMask",
        32,
        &[Field::new("Mask", 2, 8, false, None)],
        &[(Field::new("", 12, 4, false, None), 3)],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_SwapIndex6`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_SWAPINDEX6: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_SwapIndex6",
        32,
        &[Field::new("Ofs", 0, 2, false, None)],
        &[
            (Field::new("", 2, 1, false, None), 1),
            (Field::new("", 12, 4, false, None), 6),
        ],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_SwapIndex7`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_SWAPINDEX7: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_SwapIndex7",
        32,
        &[Field::new("Ofs", 2, 2, false, None)],
        &[(Field::new("", 12, 4, false, None), 7)],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_SwapIndex10`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_SWAPINDEX10: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_SwapIndex10",
        32,
        &[Field::new("Ofs", 0, 2, false, None)],
        &[
            (Field::new("", 8, 4, false, None), 3),
            (Field::new("", 12, 4, false, None), 10),
        ],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_Zaamo`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_ZAAMO: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_Zaamo",
        32,
        &[
            Field::new("Op", 8, 3, false, None),
            Field::new("Ofs", 0, 2, false, None),
        ],
        &[
            (Field::new("", 11, 1, false, None), 1),
            (Field::new("", 12, 4, false, None), 10),
        ],
        "BlackholeA0/NoC/Atomics.md",
    );

    /// `NOC_AT_LEN_BE_Acc`, from `BlackholeA0/NoC/Atomics.md`.
    pub static NOC_AT_LEN_BE_ACC: DatumLayout = DatumLayout::new(
        "NOC_AT_LEN_BE_Acc",
        32,
        &[Field::new("Fmt", 0, 4, false, None)],
        &[(Field::new("", 12, 4, false, None), 9)],
        "BlackholeA0/NoC/Atomics.md",
    );
}

/// Every datum layout, for table-wide checks.
pub static ALL_LAYOUTS: &[&DatumLayout] = &[
    &datum::SRC_TF32,
    &datum::SRC_BF16,
    &datum::SRC_FP16,
    &datum::SRC_INT8,
    &datum::SRC_INT16,
    &datum::DST16_BF16,
    &datum::DST16_FP16,
    &datum::DST16_INT8,
    &datum::DST16_INT16,
    &datum::DST32_FP32,
    &datum::DST32_INT32,
    &datum::NOC_AT_LEN_BE_INCREMENT,
    &datum::NOC_AT_LEN_BE_CAS,
    &datum::NOC_AT_LEN_BE_SWAPMASK,
    &datum::NOC_AT_LEN_BE_SWAPINDEX6,
    &datum::NOC_AT_LEN_BE_SWAPINDEX7,
    &datum::NOC_AT_LEN_BE_SWAPINDEX10,
    &datum::NOC_AT_LEN_BE_ZAAMO,
    &datum::NOC_AT_LEN_BE_ACC,
];

/// One encoder per instruction, taking its operands from the high bits of
/// the word down — the order the specification lists them in.
pub mod encode {
    use super::*;

    /// An encoder whose operands are too many to pass positionally without inviting a
    /// transposition: `Ty::ZERO.a(1).b(2).encode()`.
    macro_rules! builder {
        ($(#[$meta:meta])* $ty:ident, $def:path, [$($field:ident),* $(,)?]) => {
            $(#[$meta])*
            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub struct $ty { $($field: u32,)* }
            impl $ty {
                /// Every operand zero. Fixed bits are added by [`Self::encode`].
                pub const ZERO: Self = $ty { $($field: 0,)* };
                $(pub const fn $field(mut self, value: u32) -> Self { self.$field = value; self })
*
                pub const fn encode(self) -> Result<Instruction, EncodeError> {
                    Instruction::with_operands(&$def, &[$(self.$field),*])
                }
            }
        };
    }

    /// `ATCAS`.
    pub const fn atcas(
        set_val: u32,
        cmp_val: u32,
        ofs: u32,
        addr_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATCAS, &[set_val, cmp_val, ofs, addr_reg])
    }

    /// `ATSWAP`.
    pub const fn atswap(
        single_data_reg: u32,
        mask: u32,
        data_reg: u32,
        addr_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATSWAP, &[single_data_reg, mask, data_reg, addr_reg])
    }

    /// `RMWCIB0`.
    pub const fn rmwcib0(
        mask: u32,
        new_value: u32,
        index4: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB0, &[mask, new_value, index4])
    }

    /// `RMWCIB1`.
    pub const fn rmwcib1(
        mask: u32,
        new_value: u32,
        index4: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB1, &[mask, new_value, index4])
    }

    /// `RMWCIB2`.
    pub const fn rmwcib2(
        mask: u32,
        new_value: u32,
        index4: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB2, &[mask, new_value, index4])
    }

    /// `RMWCIB3`.
    pub const fn rmwcib3(
        mask: u32,
        new_value: u32,
        index4: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB3, &[mask, new_value, index4])
    }

    /// `SETDMAREG_Immediate`.
    pub const fn setdmareg_immediate(
        new_value: u32,
        result_half_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SETDMAREG_Immediate, &[new_value, result_half_reg])
    }

    builder!(
        #[doc = "`SETDMAREG_Special`."]
        SetdmaregSpecial,
        defs::SETDMAREG_Special,
        [
            result_size,
            which_packers,
            input_source,
            input_half_reg,
            result_half_reg
        ]
    );

    /// `STALLWAIT_BH`.
    pub const fn stallwait(
        block_mask: u32,
        condition_mask: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::STALLWAIT, &[block_mask, condition_mask])
    }

    /// `SEMWAIT`.
    pub const fn semwait(
        block_mask: u32,
        semaphore_mask: u32,
        condition_mask: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SEMWAIT,
            &[block_mask, semaphore_mask, condition_mask],
        )
    }

    /// `STREAMWAIT`.
    pub const fn streamwait(
        block_mask: u32,
        target_value_lo: u32,
        condition_index: u32,
        stream_select: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::STREAMWAIT,
            &[block_mask, target_value_lo, condition_index, stream_select],
        )
    }

    /// `SEMINIT`.
    pub const fn seminit(
        new_max: u32,
        new_value: u32,
        semaphore_mask: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SEMINIT, &[new_max, new_value, semaphore_mask])
    }

    /// `REPLAY`.
    pub const fn replay(
        index: u32,
        count: u32,
        exec: u32,
        load: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::REPLAY, &[index, count, exec, load])
    }

    builder!(
        #[doc = "`PACR_BH`."]
        Pacr,
        defs::PACR,
        [
            cfg_context,
            row_pad_zero,
            dst_access_mode,
            addr_mod,
            addr_cnt_context,
            zero_write,
            read_intf_sel,
            ovrd_thread_id,
            concat,
            ctxt_ctrl,
            flush,
            last
        ]
    );

    /// `PACR_SETREG`.
    pub const fn pacr_setreg(
        addr_sel: u32,
        value10: u32,
        addr_mid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::PACR_SETREG, &[addr_sel, value10, addr_mid])
    }

    /// `SFPSHFT2`.
    pub const fn sfpshft2(
        vb: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSHFT2, &[vb, vc, vd, mod1])
    }

    /// `SFPSHFT2b`.
    pub const fn sfpshft2b(imm12: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSHFT2b, &[imm12, vd, mod1])
    }

    /// `SFPLUTFP32_BH`.
    pub const fn sfplutfp32(
        mod1_mirror: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLUTFP32, &[mod1_mirror, vd, mod1])
    }

    /// `REG2FLOP_Configuration`.
    pub const fn reg2_flop_configuration(
        size_sel: u32,
        th_con_cfg_index: u32,
        input_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::REG2FLOP_Configuration,
            &[size_sel, th_con_cfg_index, input_reg],
        )
    }

    builder!(
        #[doc = "`REG2FLOP_ADC`."]
        Reg2FlopAdc,
        defs::REG2FLOP_ADC,
        [
            size_sel,
            override_thread,
            shift8,
            thread_sel,
            channel,
            adc_sel,
            cr,
            xyzw,
            input_reg
        ]
    );

    /// `XMOV`.
    pub const fn xmov() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::XMOV, &[])
    }

    /// `WRCFG`.
    pub const fn wrcfg(
        input_reg: u32,
        is128_bit: u32,
        cfg_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::WRCFG, &[input_reg, is128_bit, cfg_index])
    }

    /// `RDCFG`.
    pub const fn rdcfg(result_reg: u32, cfg_index: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RDCFG, &[result_reg, cfg_index])
    }

    /// `SETC16`.
    pub const fn setc16(cfg_index: u32, new_value: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SETC16, &[cfg_index, new_value])
    }

    /// `STREAMWRCFG`.
    pub const fn streamwrcfg(
        stream_select: u32,
        reg_index: u32,
        cfg_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::STREAMWRCFG, &[stream_select, reg_index, cfg_index])
    }

    builder!(
        #[doc = "`CFGSHIFTMASK`."]
        Cfgshiftmask,
        defs::CFGSHIFTMASK,
        [
            mask_mode,
            alu_mode,
            mask_width,
            rotate_amt,
            scratch_index,
            cfg_index
        ]
    );

    builder!(
        #[doc = "`SETADC`."]
        Setadc,
        defs::SETADC,
        [pk, u1, u0, channel, xyzw, new_value]
    );

    builder!(
        #[doc = "`SETADCXY`."]
        Setadcxy,
        defs::SETADCXY,
        [
            pk,
            u1,
            u0,
            thread_override,
            y1_val,
            x1_val,
            y0_val,
            x0_val,
            y1,
            x1,
            y0,
            x0
        ]
    );

    builder!(
        #[doc = "`INCADCXY`."]
        Incadcxy,
        defs::INCADCXY,
        [pk, u1, u0, thread_override, y1_inc, x1_inc, y0_inc, x0_inc]
    );

    builder!(
        #[doc = "`ADDRCRXY`."]
        Addrcrxy,
        defs::ADDRCRXY,
        [
            pk,
            u1,
            u0,
            thread_override,
            y1_inc,
            x1_inc,
            y0_inc,
            x0_inc,
            y1,
            x1,
            y0,
            x0
        ]
    );

    builder!(
        #[doc = "`SETADCZW`."]
        Setadczw,
        defs::SETADCZW,
        [
            pk,
            u1,
            u0,
            thread_override,
            w1_val,
            z1_val,
            w0_val,
            z0_val,
            w1,
            z1,
            w0,
            z0
        ]
    );

    builder!(
        #[doc = "`INCADCZW`."]
        Incadczw,
        defs::INCADCZW,
        [pk, u1, u0, thread_override, w1_inc, z1_inc, w0_inc, z0_inc]
    );

    builder!(
        #[doc = "`ADDRCRZW`."]
        Addrcrzw,
        defs::ADDRCRZW,
        [
            pk,
            u1,
            u0,
            thread_override,
            w1_inc,
            z1_inc,
            w0_inc,
            z0_inc,
            w1,
            z1,
            w0,
            z0
        ]
    );

    builder!(
        #[doc = "`SETADCXX`."]
        Setadcxx,
        defs::SETADCXX,
        [pk, u1, u0, x1_val, x0_val]
    );

    /// `NOP`.
    pub const fn nop() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::NOP, &[])
    }

    /// `SEMPOST`.
    pub const fn sempost(semaphore_mask: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SEMPOST, &[semaphore_mask])
    }

    /// `SEMGET`.
    pub const fn semget(semaphore_mask: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SEMGET, &[semaphore_mask])
    }

    /// `SETDVALID`.
    pub const fn setdvalid(flip_src_b: u32, flip_src_a: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SETDVALID, &[flip_src_b, flip_src_a])
    }

    /// `GATESRCRST`.
    pub const fn gatesrcrst(invalidate_src_b_cache: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::GATESRCRST, &[invalidate_src_b_cache])
    }

    builder!(
        #[doc = "`ZEROSRC`."]
        Zerosrc,
        defs::ZEROSRC,
        [
            negative_inf_src_a,
            single_bank_matrix_unit,
            both_banks,
            clear_src_b,
            clear_src_a
        ]
    );

    /// `TRNSPSRCB`.
    pub const fn trnspsrcb() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::TRNSPSRCB, &[])
    }

    /// `SHIFTXA`.
    pub const fn shiftxa(direction: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SHIFTXA, &[direction])
    }

    /// `CLREXPHIST`.
    pub const fn clrexphist() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::CLREXPHIST, &[])
    }

    /// `CLEARDVALID`.
    pub const fn cleardvalid(
        flip_src_b: u32,
        flip_src_a: u32,
        keep_reading_same_src: u32,
        reset: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::CLEARDVALID,
            &[flip_src_b, flip_src_a, keep_reading_same_src, reset],
        )
    }

    builder!(
        #[doc = "`SETRWC`."]
        Setrwc,
        defs::SETRWC,
        [
            flip_src_b, flip_src_a, dst_cto_cr, dst_cr, src_b_cr, src_a_cr, dst_val, src_b_val,
            src_a_val, fidelity, dst, src_b, src_a
        ]
    );

    builder!(
        #[doc = "`INCRWC`."]
        Incrwc,
        defs::INCRWC,
        [dst_cr, src_b_cr, src_a_cr, dst_inc, src_b_inc, src_a_inc]
    );

    builder!(
        #[doc = "`LOADIND`."]
        Loadind,
        defs::LOADIND,
        [
            size,
            offset_half_reg,
            offset_increment,
            result_reg,
            addr_reg
        ]
    );

    builder!(
        #[doc = "`STOREIND_L1`."]
        StoreindL1,
        defs::STOREIND_L1,
        [size, offset_half_reg, offset_increment, data_reg, addr_reg]
    );

    /// `STOREIND_MMIO`.
    pub const fn storeind_mmio(
        offset_half_reg: u32,
        offset_increment: u32,
        data_reg: u32,
        addr_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::STOREIND_MMIO,
            &[offset_half_reg, offset_increment, data_reg, addr_reg],
        )
    }

    builder!(
        #[doc = "`STOREIND_Src`."]
        StoreindSrc,
        defs::STOREIND_Src,
        [
            store_to_src_b,
            offset_half_reg,
            offset_increment,
            data_reg,
            addr_reg
        ]
    );

    /// `LOADREG`.
    pub const fn loadreg(result_reg: u32, addr_lo: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::LOADREG, &[result_reg, addr_lo])
    }

    /// `STOREREG`.
    pub const fn storereg(data_reg: u32, addr_lo: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::STOREREG, &[data_reg, addr_lo])
    }

    /// `FLUSHDMA`.
    pub const fn flushdma(condition_mask: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::FLUSHDMA, &[condition_mask])
    }

    /// `DMANOP`.
    pub const fn dmanop() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::DMANOP, &[])
    }

    /// `ADDDMAREG`.
    pub const fn adddmareg(
        result_reg: u32,
        right_reg: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ADDDMAREG, &[result_reg, right_reg, left_reg])
    }

    /// `ADDDMAREGi`.
    pub const fn adddmare_gi(
        result_reg: u32,
        right_imm6: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ADDDMAREGi, &[result_reg, right_imm6, left_reg])
    }

    /// `SUBDMAREG`.
    pub const fn subdmareg(
        result_reg: u32,
        right_reg: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SUBDMAREG, &[result_reg, right_reg, left_reg])
    }

    /// `SUBDMAREGi`.
    pub const fn subdmare_gi(
        result_reg: u32,
        right_imm6: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SUBDMAREGi, &[result_reg, right_imm6, left_reg])
    }

    /// `MULDMAREG`.
    pub const fn muldmareg(
        result_reg: u32,
        right_reg: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MULDMAREG, &[result_reg, right_reg, left_reg])
    }

    /// `MULDMAREGi`.
    pub const fn muldmare_gi(
        result_reg: u32,
        right_imm6: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MULDMAREGi, &[result_reg, right_imm6, left_reg])
    }

    /// `BITWOPDMAREG`.
    pub const fn bitwopdmareg(
        mode: u32,
        result_reg: u32,
        right_reg: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::BITWOPDMAREG,
            &[mode, result_reg, right_reg, left_reg],
        )
    }

    /// `BITWOPDMAREGi`.
    pub const fn bitwopdmare_gi(
        mode: u32,
        result_reg: u32,
        right_imm6: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::BITWOPDMAREGi,
            &[mode, result_reg, right_imm6, left_reg],
        )
    }

    /// `SHIFTDMAREG`.
    pub const fn shiftdmareg(
        mode: u32,
        result_reg: u32,
        right_reg: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SHIFTDMAREG, &[mode, result_reg, right_reg, left_reg])
    }

    /// `SHIFTDMAREGi`.
    pub const fn shiftdmare_gi(
        mode: u32,
        result_reg: u32,
        right_imm5: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SHIFTDMAREGi,
            &[mode, result_reg, right_imm5, left_reg],
        )
    }

    /// `CMPDMAREG`.
    pub const fn cmpdmareg(
        mode: u32,
        result_reg: u32,
        right_reg: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::CMPDMAREG, &[mode, result_reg, right_reg, left_reg])
    }

    /// `CMPDMAREGi`.
    pub const fn cmpdmare_gi(
        mode: u32,
        result_reg: u32,
        right_imm6: u32,
        left_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::CMPDMAREGi, &[mode, result_reg, right_imm6, left_reg])
    }

    /// `ATGETM`.
    pub const fn atgetm(index: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATGETM, &[index])
    }

    /// `ATRELM`.
    pub const fn atrelm(index: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATRELM, &[index])
    }

    /// `ATINCGET`.
    pub const fn atincget(
        int_width: u32,
        ofs: u32,
        in_out_reg: u32,
        addr_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATINCGET, &[int_width, ofs, in_out_reg, addr_reg])
    }

    builder!(
        #[doc = "`ATINCGETPTR`."]
        Atincgetptr,
        defs::ATINCGETPTR,
        [no_incr, incr_log2, int_width, ofs, result_reg, addr_reg]
    );

    /// `SFPLOAD_BH`.
    pub const fn sfpload(
        vd: u32,
        mod0: u32,
        addr_mod: u32,
        imm10: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLOAD, &[vd, mod0, addr_mod, imm10])
    }

    builder!(
        #[doc = "`SFPLOADMACRO_BH`."]
        Sfploadmacro,
        defs::SFPLOADMACRO,
        [macro_index, vd_lo, mod0, addr_mod, imm9, vd_hi]
    );

    /// `SFPSTORE_BH`.
    pub const fn sfpstore(
        vd: u32,
        mod0: u32,
        addr_mod: u32,
        imm10: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSTORE, &[vd, mod0, addr_mod, imm10])
    }

    /// `MOP`.
    pub const fn mop(template: u32, count1: u32, mask_lo: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MOP, &[template, count1, mask_lo])
    }

    /// `MOP_CFG`.
    pub const fn mop_cfg(mask_hi: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MOP_CFG, &[mask_hi])
    }

    /// `SFPLOADI`.
    pub const fn sfploadi(vd: u32, mod0: u32, imm16: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLOADI, &[vd, mod0, imm16])
    }

    /// `SFPIADD`.
    pub const fn sfpiadd(
        imm12: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPIADD, &[imm12, vc, vd, mod1])
    }

    /// `SFPSWAP`.
    pub const fn sfpswap(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSWAP, &[vc, vd, mod1])
    }

    /// `SFPCONFIG`.
    pub const fn sfpconfig(imm16: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPCONFIG, &[imm16, vd, mod1])
    }

    builder!(
        #[doc = "`SFPMAD`."]
        Sfpmad,
        defs::SFPMAD,
        [va, vb, vc, vd, mod1]
    );

    builder!(
        #[doc = "`SFPADD`."]
        Sfpadd,
        defs::SFPADD,
        [va, vb, vc, vd, mod1]
    );

    builder!(
        #[doc = "`SFPMUL`."]
        Sfpmul,
        defs::SFPMUL,
        [va, vb, vc, vd, mod1]
    );

    /// `SFPLUT`.
    pub const fn sfplut(vd: u32, mod0: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLUT, &[vd, mod0])
    }

    /// `SFPMULI`.
    pub const fn sfpmuli(imm16: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPMULI, &[imm16, vd, mod1])
    }

    /// `SFPADDI`.
    pub const fn sfpaddi(imm16: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPADDI, &[imm16, vd, mod1])
    }

    /// `SFPDIVP2`.
    pub const fn sfpdivp2(
        imm8: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPDIVP2, &[imm8, vc, vd, mod1])
    }

    /// `SFPEXEXP`.
    pub const fn sfpexexp(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPEXEXP, &[vc, vd, mod1])
    }

    /// `SFPEXMAN`.
    pub const fn sfpexman(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPEXMAN, &[vc, vd, mod1])
    }

    /// `SFPSHFT`.
    pub const fn sfpshft(
        imm12: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSHFT, &[imm12, vc, vd, mod1])
    }

    /// `SFPSETCC`.
    pub const fn sfpsetcc(
        imm1: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSETCC, &[imm1, vc, vd, mod1])
    }

    /// `SFPMOV`.
    pub const fn sfpmov(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPMOV, &[vc, vd, mod1])
    }

    /// `SFPABS`.
    pub const fn sfpabs(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPABS, &[vc, vd, mod1])
    }

    /// `SFPAND_BH`.
    pub const fn sfpand(vb: u32, vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPAND, &[vb, vc, vd, mod1])
    }

    /// `SFPOR_BH`.
    pub const fn sfpor(vb: u32, vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPOR, &[vb, vc, vd, mod1])
    }

    /// `SFPNOT`.
    pub const fn sfpnot(vc: u32, vd: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPNOT, &[vc, vd])
    }

    /// `SFPLZ`.
    pub const fn sfplz(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLZ, &[vc, vd, mod1])
    }

    /// `SFPSETEXP`.
    pub const fn sfpsetexp(
        imm8: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSETEXP, &[imm8, vc, vd, mod1])
    }

    /// `SFPSETMAN`.
    pub const fn sfpsetman(
        imm12: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSETMAN, &[imm12, vc, vd, mod1])
    }

    /// `SFPPUSHC_BH`.
    pub const fn sfppushc(vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPPUSHC, &[vd, mod1])
    }

    /// `SFPPOPC`.
    pub const fn sfppopc(vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPPOPC, &[vd, mod1])
    }

    /// `SFPSETSGN`.
    pub const fn sfpsetsgn(
        imm1: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSETSGN, &[imm1, vc, vd, mod1])
    }

    /// `SFPENCC`.
    pub const fn sfpencc(imm2: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPENCC, &[imm2, vd, mod1])
    }

    /// `SFPCOMPC`.
    pub const fn sfpcompc(vd: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPCOMPC, &[vd])
    }

    /// `SFPTRANSP`.
    pub const fn sfptransp(vd: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPTRANSP, &[vd])
    }

    /// `SFPXOR`.
    pub const fn sfpxor(vc: u32, vd: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPXOR, &[vc, vd])
    }

    builder!(
        #[doc = "`SFPSTOCHRND_BH`."]
        Sfpstochrnd,
        defs::SFPSTOCHRND,
        [rounding_mode, vb, vc, vd, mod1]
    );

    builder!(
        #[doc = "`SFPSTOCHRNDi_BH`."]
        SfpstochrnDi,
        defs::SFPSTOCHRNDi,
        [rounding_mode, imm5, vb, vc, vd, use_imm5, mod1]
    );

    /// `SFPNOP`.
    pub const fn sfpnop() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPNOP, &[])
    }

    /// `SFPCAST`.
    pub const fn sfpcast(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPCAST, &[vc, vd, mod1])
    }

    /// `SFPLE`.
    pub const fn sfple(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLE, &[vc, vd, mod1])
    }

    /// `SFPGT`.
    pub const fn sfpgt(vc: u32, vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPGT, &[vc, vd, mod1])
    }

    builder!(
        #[doc = "`SFPMUL24`."]
        Sfpmul24,
        defs::SFPMUL24,
        [va, vb, vc, vd, mod1]
    );

    /// `SFPARECIP`.
    pub const fn sfparecip(
        vb: u32,
        vc: u32,
        vd: u32,
        mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPARECIP, &[vb, vc, vd, mod1])
    }

    builder!(
        #[doc = "`UNPACR_Regular`."]
        UnpacrRegular,
        defs::UNPACR_Regular,
        [
            which_unpacker,
            ch1_y_inc,
            ch1_z_inc,
            ch0_y_inc,
            ch0_z_inc,
            context_number,
            context_adc,
            multi_context_mode,
            flip_src,
            all_datums_are_zero,
            use_context_counter,
            row_search
        ]
    );

    /// `UNPACR_IncrementContextCounter`.
    pub const fn unpacr_increment_context_counter(
        which_unpacker: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::UNPACR_IncrementContextCounter, &[which_unpacker])
    }

    /// `UNPACR_FlushCache`.
    pub const fn unpacr_flush_cache(
        which_unpacker: u32,
        multi_context_mode: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR_FlushCache,
            &[which_unpacker, multi_context_mode],
        )
    }

    /// `UNPACR_NOP_OverlayClear0`.
    pub const fn unpacr_nop_overlay_clear0(
        which_unpacker: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::UNPACR_NOP_OverlayClear0, &[which_unpacker])
    }

    /// `UNPACR_NOP_OverlayClear3`.
    pub const fn unpacr_nop_overlay_clear3(
        which_unpacker: u32,
        which_stream: u32,
        clear_count: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR_NOP_OverlayClear3,
            &[which_unpacker, which_stream, clear_count],
        )
    }

    /// `UNPACR_NOP_Nop`.
    pub const fn unpacr_nop_nop(which_unpacker: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::UNPACR_NOP_Nop, &[which_unpacker])
    }

    builder!(
        #[doc = "`UNPACR_NOP_SETREG`."]
        UnpacrNopSetreg,
        defs::UNPACR_NOP_SETREG,
        [which_unpacker, addr_sel, addr_mid, value11, accumulate]
    );

    /// `UNPACR_NOP_SETDVALID_BH`.
    pub const fn unpacr_nop_setdvalid(which_unpacker: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::UNPACR_NOP_SETDVALID, &[which_unpacker])
    }

    /// `UNPACR_NOP_ZEROSRC_BH`.
    pub const fn unpacr_nop_zerosrc(
        which_unpacker: u32,
        wait_like_unpacr: u32,
        both_banks: u32,
        negative_inf_src_a: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR_NOP_ZEROSRC,
            &[
                which_unpacker,
                wait_like_unpacr,
                both_banks,
                negative_inf_src_a,
            ],
        )
    }

    builder!(
        #[doc = "`GMPOOL_BH`."]
        Gmpool,
        defs::GMPOOL,
        [flip_src_b, flip_src_a, addr_mod, arg_max, dst_row]
    );

    /// `GAPOOL_BH`.
    pub const fn gapool(
        flip_src_b: u32,
        flip_src_a: u32,
        addr_mod: u32,
        dst_row: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::GAPOOL, &[flip_src_b, flip_src_a, addr_mod, dst_row])
    }

    builder!(
        #[doc = "`MVMUL_BH`."]
        Mvmul,
        defs::MVMUL,
        [
            flip_src_b,
            flip_src_a,
            broadcast_src_b_row,
            addr_mod,
            dst_row
        ]
    );

    builder!(
        #[doc = "`MOVA2D_BH`."]
        Mova2D,
        defs::MOVA2D,
        [use_dst32b_lo, src_row, addr_mod, move8_rows, dst_row]
    );

    builder!(
        #[doc = "`MOVB2D_BH`."]
        Movb2D,
        defs::MOVB2D,
        [
            use_dst32b_lo,
            src_row,
            addr_mod,
            move4_rows,
            broadcast1_row_to8,
            broadcast_col0,
            dst_row
        ]
    );

    builder!(
        #[doc = "`MOVD2A_BH`."]
        Movd2A,
        defs::MOVD2A,
        [use_dst32b_lo, src_row, addr_mod, move4_rows, dst_row]
    );

    builder!(
        #[doc = "`MOVD2B_BH`."]
        Movd2B,
        defs::MOVD2B,
        [use_dst32b_lo, src_row, addr_mod, move4_rows, dst_row]
    );

    /// `MOVB2A_BH`.
    pub const fn movb2_a(
        src_a_row: u32,
        addr_mod: u32,
        move4_rows: u32,
        src_b_row: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MOVB2A, &[src_a_row, addr_mod, move4_rows, src_b_row])
    }

    builder!(
        #[doc = "`ELWADD_BH`."]
        Elwadd,
        defs::ELWADD,
        [
            flip_src_b,
            flip_src_a,
            add_dst,
            broadcast_src_b_row,
            broadcast_src_b_col0,
            addr_mod,
            dst_row
        ]
    );

    builder!(
        #[doc = "`ELWSUB_BH`."]
        Elwsub,
        defs::ELWSUB,
        [
            flip_src_b,
            flip_src_a,
            add_dst,
            broadcast_src_b_row,
            broadcast_src_b_col0,
            addr_mod,
            dst_row
        ]
    );

    builder!(
        #[doc = "`ELWMUL_BH`."]
        Elwmul,
        defs::ELWMUL,
        [
            flip_src_b,
            flip_src_a,
            broadcast_src_b_row,
            broadcast_src_b_col0,
            addr_mod,
            dst_row
        ]
    );

    /// `DOTPV_BH`.
    pub const fn dotpv(
        flip_src_b: u32,
        flip_src_a: u32,
        addr_mod: u32,
        dst_row: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::DOTPV, &[flip_src_b, flip_src_a, addr_mod, dst_row])
    }

    builder!(
        #[doc = "`MOVDBGA2D_BH`."]
        Movdbga2D,
        defs::MOVDBGA2D,
        [use_dst32b_lo, src_row, addr_mod, move8_rows, dst_row]
    );

    /// `SHIFTXB_BH`.
    pub const fn shiftxb(
        addr_mod: u32,
        shift_in_zero: u32,
        src_row: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SHIFTXB, &[addr_mod, shift_in_zero, src_row])
    }

    /// `ZEROACC_BH`.
    pub const fn zeroacc(
        mode: u32,
        use_dst32b: u32,
        addr_mod: u32,
        imm10: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ZEROACC, &[mode, use_dst32b, addr_mod, imm10])
    }

    /// Encoders for the forms Blackhole replaces. Reaching one has to be
    /// deliberate.
    pub mod wormhole {
        use super::super::*;

        /// `STALLWAIT`.
        pub const fn stallwait(
            block_mask: u32,
            condition_mask: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::STALLWAIT, &[block_mask, condition_mask])
        }

        builder!(
            #[doc = "`PACR`."]
            Pacr,
            defs::wormhole::PACR,
            [
                addr_mod,
                zero_write,
                packer_mask,
                ovrd_thread_id,
                concat,
                flush,
                last
            ]
        );

        /// `SFPLUTFP32`.
        pub const fn sfplutfp32(vd: u32, mod1: u32) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::SFPLUTFP32, &[vd, mod1])
        }

        builder!(
            #[doc = "`MOVD2A`."]
            Movd2A,
            defs::wormhole::MOVD2A,
            [use_dst32b_lo, src_row, addr_mod, move4_rows, dst_row]
        );

        builder!(
            #[doc = "`MOVD2B`."]
            Movd2B,
            defs::wormhole::MOVD2B,
            [use_dst32b_lo, src_row, addr_mod, move4_rows, dst_row]
        );

        /// `MOVB2A`.
        pub const fn movb2_a(
            src_a_row: u32,
            addr_mod: u32,
            move4_rows: u32,
            src_b_row: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(
                &defs::wormhole::MOVB2A,
                &[src_a_row, addr_mod, move4_rows, src_b_row],
            )
        }

        builder!(
            #[doc = "`MOVDBGA2D`."]
            Movdbga2D,
            defs::wormhole::MOVDBGA2D,
            [use_dst32b_lo, src_row, addr_mod, move8_rows, dst_row]
        );

        builder!(
            #[doc = "`MOVA2D`."]
            Mova2D,
            defs::wormhole::MOVA2D,
            [use_dst32b_lo, src_row, addr_mod, move8_rows, dst_row]
        );

        builder!(
            #[doc = "`MOVB2D`."]
            Movb2D,
            defs::wormhole::MOVB2D,
            [
                use_dst32b_lo,
                src_row,
                addr_mod,
                move4_rows,
                broadcast1_row_to8,
                broadcast_col0,
                dst_row
            ]
        );

        builder!(
            #[doc = "`ELWMUL`."]
            Elwmul,
            defs::wormhole::ELWMUL,
            [
                flip_src_b,
                flip_src_a,
                broadcast_src_b_row,
                broadcast_src_b_col0,
                addr_mod,
                dst_row
            ]
        );

        builder!(
            #[doc = "`ELWADD`."]
            Elwadd,
            defs::wormhole::ELWADD,
            [
                flip_src_b,
                flip_src_a,
                add_dst,
                broadcast_src_b_row,
                broadcast_src_b_col0,
                addr_mod,
                dst_row
            ]
        );

        builder!(
            #[doc = "`ELWSUB`."]
            Elwsub,
            defs::wormhole::ELWSUB,
            [
                flip_src_b,
                flip_src_a,
                add_dst,
                broadcast_src_b_row,
                broadcast_src_b_col0,
                addr_mod,
                dst_row
            ]
        );

        builder!(
            #[doc = "`GMPOOL`."]
            Gmpool,
            defs::wormhole::GMPOOL,
            [flip_src_b, flip_src_a, addr_mod, arg_max, dst_row]
        );

        builder!(
            #[doc = "`ZEROACC`."]
            Zeroacc,
            defs::wormhole::ZEROACC,
            [use_dst32b, mode, revert, addr_mod, imm10]
        );

        /// `SHIFTXB`.
        pub const fn shiftxb(
            addr_mod: u32,
            shift_in_zero: u32,
            src_row: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(
                &defs::wormhole::SHIFTXB,
                &[addr_mod, shift_in_zero, src_row],
            )
        }

        builder!(
            #[doc = "`MVMUL`."]
            Mvmul,
            defs::wormhole::MVMUL,
            [
                flip_src_b,
                flip_src_a,
                broadcast_src_b_row,
                addr_mod,
                dst_row
            ]
        );

        /// `DOTPV`.
        pub const fn dotpv(
            flip_src_b: u32,
            flip_src_a: u32,
            addr_mod: u32,
            dst_row: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(
                &defs::wormhole::DOTPV,
                &[flip_src_b, flip_src_a, addr_mod, dst_row],
            )
        }

        /// `GAPOOL`.
        pub const fn gapool(
            flip_src_b: u32,
            flip_src_a: u32,
            addr_mod: u32,
            dst_row: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(
                &defs::wormhole::GAPOOL,
                &[flip_src_b, flip_src_a, addr_mod, dst_row],
            )
        }

        /// `SFPLOAD`.
        pub const fn sfpload(
            vd: u32,
            mod0: u32,
            addr_mod: u32,
            imm10: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::SFPLOAD, &[vd, mod0, addr_mod, imm10])
        }

        builder!(
            #[doc = "`SFPLOADMACRO`."]
            Sfploadmacro,
            defs::wormhole::SFPLOADMACRO,
            [macro_index, vd_lo, mod0, addr_mod, imm9, vd_hi]
        );

        /// `SFPSTORE`.
        pub const fn sfpstore(
            vd: u32,
            mod0: u32,
            addr_mod: u32,
            imm10: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::SFPSTORE, &[vd, mod0, addr_mod, imm10])
        }

        /// `SFPAND`.
        pub const fn sfpand(vc: u32, vd: u32) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::SFPAND, &[vc, vd])
        }

        /// `SFPOR`.
        pub const fn sfpor(vc: u32, vd: u32) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::SFPOR, &[vc, vd])
        }

        /// `SFPPUSHC`.
        pub const fn sfppushc(vd: u32) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::SFPPUSHC, &[vd])
        }

        /// `SFPSTOCHRND`.
        pub const fn sfpstochrnd(
            stochastic_rounding: u32,
            vc: u32,
            vd: u32,
            mod1: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(
                &defs::wormhole::SFPSTOCHRND,
                &[stochastic_rounding, vc, vd, mod1],
            )
        }

        builder!(
            #[doc = "`SFPSTOCHRNDi`."]
            SfpstochrnDi,
            defs::wormhole::SFPSTOCHRNDi,
            [stochastic_rounding, imm5, vb, vc, vd, use_imm5, mod1]
        );

        /// `UNPACR_NOP_ZEROSRC`.
        pub const fn unpacr_nop_zerosrc(
            which_unpacker: u32,
            wait_like_unpacr: u32,
            both_banks: u32,
            negative_inf_src_a: u32,
        ) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(
                &defs::wormhole::UNPACR_NOP_ZEROSRC,
                &[
                    which_unpacker,
                    wait_like_unpacr,
                    both_banks,
                    negative_inf_src_a,
                ],
            )
        }

        /// `UNPACR_NOP_SETDVALID`.
        pub const fn unpacr_nop_setdvalid(which_unpacker: u32) -> Result<Instruction, EncodeError> {
            Instruction::with_operands(&defs::wormhole::UNPACR_NOP_SETDVALID, &[which_unpacker])
        }
    }
}
