//! Tensix backend-configuration fields, generated from `cfg_defines.h`.
//!
//! **Do not edit.** Regenerate with `cargo xtask gen-cfg`, which fetches the
//! pinned header itself; `cargo xtask gen-cfg --check` fails when this file is
//! out of date.
//!
//! Source: tt-metal `ba9340e3a45ac5ba51c752a49341f2def28d0514`, `tt_metal/hw/inc/internal/tt-2xx/quasar/cfg_defines.h`
//! (1123 fields across 3 sections).
//!
//! Field names are kept exactly as `cfg_defines.h` spells them, so that a name
//! in the specification can be searched for here without translation.
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]

use super::{ConfigField, ConfigSpan, ThreadConfigField};

/// `uint32_t Config[2][CFG_STATE_SIZE * 4]`
pub const CFG_STATE_SIZE: u32 = 94;

/// `// Registers for ALU` — 43 fields, indexing `Config` (write with `WRCFG`).
pub mod alu {
    use super::*;

    /// First word index of this section, from `ALU_CFGREG_BASE_ADDR32`.
    pub const CFGREG_BASE: u16 = 0;

    pub const ALU_ACC_CTRL_Fp32_enabled: ConfigField = ConfigField::new(2, 8, 0x100);
    pub const ALU_ACC_CTRL_INT8_math_enabled: ConfigField = ConfigField::new(2, 10, 0x400);
    pub const ALU_ACC_CTRL_SFPU_Fp32_enabled: ConfigField = ConfigField::new(2, 9, 0x200);
    pub const ALU_ACC_CTRL_Zero_Flag_disabled_dst: ConfigField = ConfigField::new(3, 1, 0x2);
    pub const ALU_ACC_CTRL_Zero_Flag_disabled_src: ConfigField = ConfigField::new(3, 0, 0x1);
    pub const ALU_FORMAT_SPEC_REG0_SrcA: ConfigField = ConfigField::new(1, 15, 0x7f8000);
    pub const ALU_FORMAT_SPEC_REG1_SrcB: ConfigField = ConfigField::new(1, 23, 0x7f800000);
    pub const ALU_FORMAT_SPEC_REG2_Dstacc: ConfigField = ConfigField::new(2, 0, 0xff);
    pub const ALU_FORMAT_SPEC_REG_Dstacc_override: ConfigField = ConfigField::new(0, 26, 0x4000000);
    pub const ALU_FORMAT_SPEC_REG_Dstacc_val: ConfigField = ConfigField::new(0, 18, 0x3fc0000);
    pub const ALU_FORMAT_SPEC_REG_SrcA_override: ConfigField = ConfigField::new(0, 8, 0x100);
    pub const ALU_FORMAT_SPEC_REG_SrcA_val: ConfigField = ConfigField::new(0, 0, 0xff);
    pub const ALU_FORMAT_SPEC_REG_SrcB_override: ConfigField = ConfigField::new(0, 17, 0x20000);
    pub const ALU_FORMAT_SPEC_REG_SrcB_val: ConfigField = ConfigField::new(0, 9, 0x1fe00);
    pub const ALU_ROUNDING_MODE_Bfp8_HF: ConfigField = ConfigField::new(1, 14, 0x4000);
    pub const ALU_ROUNDING_MODE_Fpu_srnd_en: ConfigField = ConfigField::new(1, 0, 0x1);
    pub const ALU_ROUNDING_MODE_GS_LF: ConfigField = ConfigField::new(1, 13, 0x2000);
    pub const ALU_ROUNDING_MODE_Padding: ConfigField = ConfigField::new(1, 3, 0x1ff8);
    pub const DEST_REGW_BASE_Base: ConfigField = ConfigField::new(6, 0, 0xffff);
    pub const DEST_SP_BASE_Base: ConfigField = ConfigField::new(7, 0, 0xffff);
    pub const DEST_TARGET_REG_CFG_MATH_SEC0_Offset: ConfigField = ConfigField::new(8, 0, 0xfff);
    pub const DEST_TARGET_REG_CFG_MATH_SEC1_Offset: ConfigField = ConfigField::new(9, 0, 0xfff);
    pub const DEST_TARGET_REG_CFG_MATH_SEC2_Offset: ConfigField = ConfigField::new(10, 0, 0xfff);
    pub const DEST_TARGET_REG_CFG_MATH_SEC3_Offset: ConfigField = ConfigField::new(11, 0, 0xfff);
    pub const DISABLE_IMPLIED_SRCS_FORMAT: ConfigField = ConfigField::new(7, 19, 0x80000);
    pub const RISC_DEST_ACCESS_CTRL_SEC0_fmt: ConfigField = ConfigField::new(3, 16, 0x70000);
    pub const RISC_DEST_ACCESS_CTRL_SEC0_no_swizzle: ConfigField = ConfigField::new(3, 14, 0x4000);
    pub const RISC_DEST_ACCESS_CTRL_SEC0_unsigned_int: ConfigField =
        ConfigField::new(3, 15, 0x8000);
    pub const RISC_DEST_ACCESS_CTRL_SEC1_fmt: ConfigField = ConfigField::new(3, 21, 0xe00000);
    pub const RISC_DEST_ACCESS_CTRL_SEC1_no_swizzle: ConfigField = ConfigField::new(3, 19, 0x80000);
    pub const RISC_DEST_ACCESS_CTRL_SEC1_unsigned_int: ConfigField =
        ConfigField::new(3, 20, 0x100000);
    pub const RISC_DEST_ACCESS_CTRL_SEC2_fmt: ConfigField = ConfigField::new(3, 26, 0x1c000000);
    pub const RISC_DEST_ACCESS_CTRL_SEC2_no_swizzle: ConfigField =
        ConfigField::new(3, 24, 0x1000000);
    pub const RISC_DEST_ACCESS_CTRL_SEC2_unsigned_int: ConfigField =
        ConfigField::new(3, 25, 0x2000000);
    pub const RISC_DEST_ACCESS_CTRL_SEC3_fmt: ConfigField = ConfigField::new(7, 16, 0x70000);
    pub const RISC_DEST_ACCESS_CTRL_SEC3_no_swizzle: ConfigField =
        ConfigField::new(3, 29, 0x20000000);
    pub const RISC_DEST_ACCESS_CTRL_SEC3_unsigned_int: ConfigField =
        ConfigField::new(3, 30, 0x40000000);
    pub const SFPU_SRCS_BANK0_FORMAT: ConfigField = ConfigField::new(7, 20, 0xf00000);
    pub const SFPU_SRCS_BANK1_FORMAT: ConfigField = ConfigField::new(7, 24, 0xf000000);
    pub const STATE_RESET_EN: ConfigField = ConfigField::new(4, 0, 0x1);
    pub const UNUSED0: ConfigField = ConfigField::new(1, 1, 0x6);
    pub const UNUSED1: ConfigField = ConfigField::new(2, 11, 0xfffff800);
    pub const UNUSED2: ConfigField = ConfigField::new(3, 2, 0x3ffc);
}

/// `// Registers for GLOBAL` — 906 fields, indexing `Config` (write with `WRCFG`).
pub mod global {
    use super::*;

    /// First word index of this section, from `GLOBAL_CFGREG_BASE_ADDR32`.
    pub const CFGREG_BASE: u16 = 80;

    pub const ADDR_MOD_AB2_SEC0_SrcAIncr: ConfigField = ConfigField::new(296, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC0_SrcBIncr: ConfigField = ConfigField::new(296, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC10_SrcAIncr: ConfigField = ConfigField::new(306, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC10_SrcBIncr: ConfigField = ConfigField::new(306, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC11_SrcAIncr: ConfigField = ConfigField::new(307, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC11_SrcBIncr: ConfigField = ConfigField::new(307, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC12_SrcAIncr: ConfigField = ConfigField::new(308, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC12_SrcBIncr: ConfigField = ConfigField::new(308, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC13_SrcAIncr: ConfigField = ConfigField::new(309, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC13_SrcBIncr: ConfigField = ConfigField::new(309, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC14_SrcAIncr: ConfigField = ConfigField::new(310, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC14_SrcBIncr: ConfigField = ConfigField::new(310, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC15_SrcAIncr: ConfigField = ConfigField::new(311, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC15_SrcBIncr: ConfigField = ConfigField::new(311, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC16_SrcAIncr: ConfigField = ConfigField::new(312, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC16_SrcBIncr: ConfigField = ConfigField::new(312, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC17_SrcAIncr: ConfigField = ConfigField::new(313, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC17_SrcBIncr: ConfigField = ConfigField::new(313, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC18_SrcAIncr: ConfigField = ConfigField::new(314, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC18_SrcBIncr: ConfigField = ConfigField::new(314, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC19_SrcAIncr: ConfigField = ConfigField::new(315, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC19_SrcBIncr: ConfigField = ConfigField::new(315, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC1_SrcAIncr: ConfigField = ConfigField::new(297, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC1_SrcBIncr: ConfigField = ConfigField::new(297, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC20_SrcAIncr: ConfigField = ConfigField::new(316, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC20_SrcBIncr: ConfigField = ConfigField::new(316, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC21_SrcAIncr: ConfigField = ConfigField::new(317, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC21_SrcBIncr: ConfigField = ConfigField::new(317, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC22_SrcAIncr: ConfigField = ConfigField::new(318, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC22_SrcBIncr: ConfigField = ConfigField::new(318, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC23_SrcAIncr: ConfigField = ConfigField::new(319, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC23_SrcBIncr: ConfigField = ConfigField::new(319, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC24_SrcAIncr: ConfigField = ConfigField::new(320, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC24_SrcBIncr: ConfigField = ConfigField::new(320, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC25_SrcAIncr: ConfigField = ConfigField::new(321, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC25_SrcBIncr: ConfigField = ConfigField::new(321, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC26_SrcAIncr: ConfigField = ConfigField::new(322, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC26_SrcBIncr: ConfigField = ConfigField::new(322, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC27_SrcAIncr: ConfigField = ConfigField::new(323, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC27_SrcBIncr: ConfigField = ConfigField::new(323, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC28_SrcAIncr: ConfigField = ConfigField::new(324, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC28_SrcBIncr: ConfigField = ConfigField::new(324, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC29_SrcAIncr: ConfigField = ConfigField::new(325, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC29_SrcBIncr: ConfigField = ConfigField::new(325, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC2_SrcAIncr: ConfigField = ConfigField::new(298, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC2_SrcBIncr: ConfigField = ConfigField::new(298, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC30_SrcAIncr: ConfigField = ConfigField::new(326, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC30_SrcBIncr: ConfigField = ConfigField::new(326, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC31_SrcAIncr: ConfigField = ConfigField::new(327, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC31_SrcBIncr: ConfigField = ConfigField::new(327, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC3_SrcAIncr: ConfigField = ConfigField::new(299, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC3_SrcBIncr: ConfigField = ConfigField::new(299, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC4_SrcAIncr: ConfigField = ConfigField::new(300, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC4_SrcBIncr: ConfigField = ConfigField::new(300, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC5_SrcAIncr: ConfigField = ConfigField::new(301, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC5_SrcBIncr: ConfigField = ConfigField::new(301, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC6_SrcAIncr: ConfigField = ConfigField::new(302, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC6_SrcBIncr: ConfigField = ConfigField::new(302, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC7_SrcAIncr: ConfigField = ConfigField::new(303, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC7_SrcBIncr: ConfigField = ConfigField::new(303, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC8_SrcAIncr: ConfigField = ConfigField::new(304, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC8_SrcBIncr: ConfigField = ConfigField::new(304, 1, 0x2);
    pub const ADDR_MOD_AB2_SEC9_SrcAIncr: ConfigField = ConfigField::new(305, 0, 0x1);
    pub const ADDR_MOD_AB2_SEC9_SrcBIncr: ConfigField = ConfigField::new(305, 1, 0x2);
    pub const ADDR_MOD_AB_SEC0_SrcACR: ConfigField = ConfigField::new(264, 6, 0x40);
    pub const ADDR_MOD_AB_SEC0_SrcAClear: ConfigField = ConfigField::new(264, 7, 0x80);
    pub const ADDR_MOD_AB_SEC0_SrcAIncr: ConfigField = ConfigField::new(264, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC0_SrcBCR: ConfigField = ConfigField::new(264, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC0_SrcBClear: ConfigField = ConfigField::new(264, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC0_SrcBIncr: ConfigField = ConfigField::new(264, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC10_SrcACR: ConfigField = ConfigField::new(274, 6, 0x40);
    pub const ADDR_MOD_AB_SEC10_SrcAClear: ConfigField = ConfigField::new(274, 7, 0x80);
    pub const ADDR_MOD_AB_SEC10_SrcAIncr: ConfigField = ConfigField::new(274, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC10_SrcBCR: ConfigField = ConfigField::new(274, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC10_SrcBClear: ConfigField = ConfigField::new(274, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC10_SrcBIncr: ConfigField = ConfigField::new(274, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC11_SrcACR: ConfigField = ConfigField::new(275, 6, 0x40);
    pub const ADDR_MOD_AB_SEC11_SrcAClear: ConfigField = ConfigField::new(275, 7, 0x80);
    pub const ADDR_MOD_AB_SEC11_SrcAIncr: ConfigField = ConfigField::new(275, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC11_SrcBCR: ConfigField = ConfigField::new(275, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC11_SrcBClear: ConfigField = ConfigField::new(275, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC11_SrcBIncr: ConfigField = ConfigField::new(275, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC12_SrcACR: ConfigField = ConfigField::new(276, 6, 0x40);
    pub const ADDR_MOD_AB_SEC12_SrcAClear: ConfigField = ConfigField::new(276, 7, 0x80);
    pub const ADDR_MOD_AB_SEC12_SrcAIncr: ConfigField = ConfigField::new(276, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC12_SrcBCR: ConfigField = ConfigField::new(276, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC12_SrcBClear: ConfigField = ConfigField::new(276, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC12_SrcBIncr: ConfigField = ConfigField::new(276, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC13_SrcACR: ConfigField = ConfigField::new(277, 6, 0x40);
    pub const ADDR_MOD_AB_SEC13_SrcAClear: ConfigField = ConfigField::new(277, 7, 0x80);
    pub const ADDR_MOD_AB_SEC13_SrcAIncr: ConfigField = ConfigField::new(277, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC13_SrcBCR: ConfigField = ConfigField::new(277, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC13_SrcBClear: ConfigField = ConfigField::new(277, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC13_SrcBIncr: ConfigField = ConfigField::new(277, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC14_SrcACR: ConfigField = ConfigField::new(278, 6, 0x40);
    pub const ADDR_MOD_AB_SEC14_SrcAClear: ConfigField = ConfigField::new(278, 7, 0x80);
    pub const ADDR_MOD_AB_SEC14_SrcAIncr: ConfigField = ConfigField::new(278, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC14_SrcBCR: ConfigField = ConfigField::new(278, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC14_SrcBClear: ConfigField = ConfigField::new(278, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC14_SrcBIncr: ConfigField = ConfigField::new(278, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC15_SrcACR: ConfigField = ConfigField::new(279, 6, 0x40);
    pub const ADDR_MOD_AB_SEC15_SrcAClear: ConfigField = ConfigField::new(279, 7, 0x80);
    pub const ADDR_MOD_AB_SEC15_SrcAIncr: ConfigField = ConfigField::new(279, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC15_SrcBCR: ConfigField = ConfigField::new(279, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC15_SrcBClear: ConfigField = ConfigField::new(279, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC15_SrcBIncr: ConfigField = ConfigField::new(279, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC16_SrcACR: ConfigField = ConfigField::new(280, 6, 0x40);
    pub const ADDR_MOD_AB_SEC16_SrcAClear: ConfigField = ConfigField::new(280, 7, 0x80);
    pub const ADDR_MOD_AB_SEC16_SrcAIncr: ConfigField = ConfigField::new(280, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC16_SrcBCR: ConfigField = ConfigField::new(280, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC16_SrcBClear: ConfigField = ConfigField::new(280, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC16_SrcBIncr: ConfigField = ConfigField::new(280, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC17_SrcACR: ConfigField = ConfigField::new(281, 6, 0x40);
    pub const ADDR_MOD_AB_SEC17_SrcAClear: ConfigField = ConfigField::new(281, 7, 0x80);
    pub const ADDR_MOD_AB_SEC17_SrcAIncr: ConfigField = ConfigField::new(281, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC17_SrcBCR: ConfigField = ConfigField::new(281, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC17_SrcBClear: ConfigField = ConfigField::new(281, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC17_SrcBIncr: ConfigField = ConfigField::new(281, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC18_SrcACR: ConfigField = ConfigField::new(282, 6, 0x40);
    pub const ADDR_MOD_AB_SEC18_SrcAClear: ConfigField = ConfigField::new(282, 7, 0x80);
    pub const ADDR_MOD_AB_SEC18_SrcAIncr: ConfigField = ConfigField::new(282, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC18_SrcBCR: ConfigField = ConfigField::new(282, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC18_SrcBClear: ConfigField = ConfigField::new(282, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC18_SrcBIncr: ConfigField = ConfigField::new(282, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC19_SrcACR: ConfigField = ConfigField::new(283, 6, 0x40);
    pub const ADDR_MOD_AB_SEC19_SrcAClear: ConfigField = ConfigField::new(283, 7, 0x80);
    pub const ADDR_MOD_AB_SEC19_SrcAIncr: ConfigField = ConfigField::new(283, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC19_SrcBCR: ConfigField = ConfigField::new(283, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC19_SrcBClear: ConfigField = ConfigField::new(283, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC19_SrcBIncr: ConfigField = ConfigField::new(283, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC1_SrcACR: ConfigField = ConfigField::new(265, 6, 0x40);
    pub const ADDR_MOD_AB_SEC1_SrcAClear: ConfigField = ConfigField::new(265, 7, 0x80);
    pub const ADDR_MOD_AB_SEC1_SrcAIncr: ConfigField = ConfigField::new(265, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC1_SrcBCR: ConfigField = ConfigField::new(265, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC1_SrcBClear: ConfigField = ConfigField::new(265, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC1_SrcBIncr: ConfigField = ConfigField::new(265, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC20_SrcACR: ConfigField = ConfigField::new(284, 6, 0x40);
    pub const ADDR_MOD_AB_SEC20_SrcAClear: ConfigField = ConfigField::new(284, 7, 0x80);
    pub const ADDR_MOD_AB_SEC20_SrcAIncr: ConfigField = ConfigField::new(284, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC20_SrcBCR: ConfigField = ConfigField::new(284, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC20_SrcBClear: ConfigField = ConfigField::new(284, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC20_SrcBIncr: ConfigField = ConfigField::new(284, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC21_SrcACR: ConfigField = ConfigField::new(285, 6, 0x40);
    pub const ADDR_MOD_AB_SEC21_SrcAClear: ConfigField = ConfigField::new(285, 7, 0x80);
    pub const ADDR_MOD_AB_SEC21_SrcAIncr: ConfigField = ConfigField::new(285, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC21_SrcBCR: ConfigField = ConfigField::new(285, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC21_SrcBClear: ConfigField = ConfigField::new(285, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC21_SrcBIncr: ConfigField = ConfigField::new(285, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC22_SrcACR: ConfigField = ConfigField::new(286, 6, 0x40);
    pub const ADDR_MOD_AB_SEC22_SrcAClear: ConfigField = ConfigField::new(286, 7, 0x80);
    pub const ADDR_MOD_AB_SEC22_SrcAIncr: ConfigField = ConfigField::new(286, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC22_SrcBCR: ConfigField = ConfigField::new(286, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC22_SrcBClear: ConfigField = ConfigField::new(286, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC22_SrcBIncr: ConfigField = ConfigField::new(286, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC23_SrcACR: ConfigField = ConfigField::new(287, 6, 0x40);
    pub const ADDR_MOD_AB_SEC23_SrcAClear: ConfigField = ConfigField::new(287, 7, 0x80);
    pub const ADDR_MOD_AB_SEC23_SrcAIncr: ConfigField = ConfigField::new(287, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC23_SrcBCR: ConfigField = ConfigField::new(287, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC23_SrcBClear: ConfigField = ConfigField::new(287, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC23_SrcBIncr: ConfigField = ConfigField::new(287, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC24_SrcACR: ConfigField = ConfigField::new(288, 6, 0x40);
    pub const ADDR_MOD_AB_SEC24_SrcAClear: ConfigField = ConfigField::new(288, 7, 0x80);
    pub const ADDR_MOD_AB_SEC24_SrcAIncr: ConfigField = ConfigField::new(288, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC24_SrcBCR: ConfigField = ConfigField::new(288, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC24_SrcBClear: ConfigField = ConfigField::new(288, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC24_SrcBIncr: ConfigField = ConfigField::new(288, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC25_SrcACR: ConfigField = ConfigField::new(289, 6, 0x40);
    pub const ADDR_MOD_AB_SEC25_SrcAClear: ConfigField = ConfigField::new(289, 7, 0x80);
    pub const ADDR_MOD_AB_SEC25_SrcAIncr: ConfigField = ConfigField::new(289, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC25_SrcBCR: ConfigField = ConfigField::new(289, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC25_SrcBClear: ConfigField = ConfigField::new(289, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC25_SrcBIncr: ConfigField = ConfigField::new(289, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC26_SrcACR: ConfigField = ConfigField::new(290, 6, 0x40);
    pub const ADDR_MOD_AB_SEC26_SrcAClear: ConfigField = ConfigField::new(290, 7, 0x80);
    pub const ADDR_MOD_AB_SEC26_SrcAIncr: ConfigField = ConfigField::new(290, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC26_SrcBCR: ConfigField = ConfigField::new(290, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC26_SrcBClear: ConfigField = ConfigField::new(290, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC26_SrcBIncr: ConfigField = ConfigField::new(290, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC27_SrcACR: ConfigField = ConfigField::new(291, 6, 0x40);
    pub const ADDR_MOD_AB_SEC27_SrcAClear: ConfigField = ConfigField::new(291, 7, 0x80);
    pub const ADDR_MOD_AB_SEC27_SrcAIncr: ConfigField = ConfigField::new(291, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC27_SrcBCR: ConfigField = ConfigField::new(291, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC27_SrcBClear: ConfigField = ConfigField::new(291, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC27_SrcBIncr: ConfigField = ConfigField::new(291, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC28_SrcACR: ConfigField = ConfigField::new(292, 6, 0x40);
    pub const ADDR_MOD_AB_SEC28_SrcAClear: ConfigField = ConfigField::new(292, 7, 0x80);
    pub const ADDR_MOD_AB_SEC28_SrcAIncr: ConfigField = ConfigField::new(292, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC28_SrcBCR: ConfigField = ConfigField::new(292, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC28_SrcBClear: ConfigField = ConfigField::new(292, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC28_SrcBIncr: ConfigField = ConfigField::new(292, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC29_SrcACR: ConfigField = ConfigField::new(293, 6, 0x40);
    pub const ADDR_MOD_AB_SEC29_SrcAClear: ConfigField = ConfigField::new(293, 7, 0x80);
    pub const ADDR_MOD_AB_SEC29_SrcAIncr: ConfigField = ConfigField::new(293, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC29_SrcBCR: ConfigField = ConfigField::new(293, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC29_SrcBClear: ConfigField = ConfigField::new(293, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC29_SrcBIncr: ConfigField = ConfigField::new(293, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC2_SrcACR: ConfigField = ConfigField::new(266, 6, 0x40);
    pub const ADDR_MOD_AB_SEC2_SrcAClear: ConfigField = ConfigField::new(266, 7, 0x80);
    pub const ADDR_MOD_AB_SEC2_SrcAIncr: ConfigField = ConfigField::new(266, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC2_SrcBCR: ConfigField = ConfigField::new(266, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC2_SrcBClear: ConfigField = ConfigField::new(266, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC2_SrcBIncr: ConfigField = ConfigField::new(266, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC30_SrcACR: ConfigField = ConfigField::new(294, 6, 0x40);
    pub const ADDR_MOD_AB_SEC30_SrcAClear: ConfigField = ConfigField::new(294, 7, 0x80);
    pub const ADDR_MOD_AB_SEC30_SrcAIncr: ConfigField = ConfigField::new(294, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC30_SrcBCR: ConfigField = ConfigField::new(294, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC30_SrcBClear: ConfigField = ConfigField::new(294, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC30_SrcBIncr: ConfigField = ConfigField::new(294, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC31_SrcACR: ConfigField = ConfigField::new(295, 6, 0x40);
    pub const ADDR_MOD_AB_SEC31_SrcAClear: ConfigField = ConfigField::new(295, 7, 0x80);
    pub const ADDR_MOD_AB_SEC31_SrcAIncr: ConfigField = ConfigField::new(295, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC31_SrcBCR: ConfigField = ConfigField::new(295, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC31_SrcBClear: ConfigField = ConfigField::new(295, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC31_SrcBIncr: ConfigField = ConfigField::new(295, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC3_SrcACR: ConfigField = ConfigField::new(267, 6, 0x40);
    pub const ADDR_MOD_AB_SEC3_SrcAClear: ConfigField = ConfigField::new(267, 7, 0x80);
    pub const ADDR_MOD_AB_SEC3_SrcAIncr: ConfigField = ConfigField::new(267, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC3_SrcBCR: ConfigField = ConfigField::new(267, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC3_SrcBClear: ConfigField = ConfigField::new(267, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC3_SrcBIncr: ConfigField = ConfigField::new(267, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC4_SrcACR: ConfigField = ConfigField::new(268, 6, 0x40);
    pub const ADDR_MOD_AB_SEC4_SrcAClear: ConfigField = ConfigField::new(268, 7, 0x80);
    pub const ADDR_MOD_AB_SEC4_SrcAIncr: ConfigField = ConfigField::new(268, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC4_SrcBCR: ConfigField = ConfigField::new(268, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC4_SrcBClear: ConfigField = ConfigField::new(268, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC4_SrcBIncr: ConfigField = ConfigField::new(268, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC5_SrcACR: ConfigField = ConfigField::new(269, 6, 0x40);
    pub const ADDR_MOD_AB_SEC5_SrcAClear: ConfigField = ConfigField::new(269, 7, 0x80);
    pub const ADDR_MOD_AB_SEC5_SrcAIncr: ConfigField = ConfigField::new(269, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC5_SrcBCR: ConfigField = ConfigField::new(269, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC5_SrcBClear: ConfigField = ConfigField::new(269, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC5_SrcBIncr: ConfigField = ConfigField::new(269, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC6_SrcACR: ConfigField = ConfigField::new(270, 6, 0x40);
    pub const ADDR_MOD_AB_SEC6_SrcAClear: ConfigField = ConfigField::new(270, 7, 0x80);
    pub const ADDR_MOD_AB_SEC6_SrcAIncr: ConfigField = ConfigField::new(270, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC6_SrcBCR: ConfigField = ConfigField::new(270, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC6_SrcBClear: ConfigField = ConfigField::new(270, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC6_SrcBIncr: ConfigField = ConfigField::new(270, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC7_SrcACR: ConfigField = ConfigField::new(271, 6, 0x40);
    pub const ADDR_MOD_AB_SEC7_SrcAClear: ConfigField = ConfigField::new(271, 7, 0x80);
    pub const ADDR_MOD_AB_SEC7_SrcAIncr: ConfigField = ConfigField::new(271, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC7_SrcBCR: ConfigField = ConfigField::new(271, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC7_SrcBClear: ConfigField = ConfigField::new(271, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC7_SrcBIncr: ConfigField = ConfigField::new(271, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC8_SrcACR: ConfigField = ConfigField::new(272, 6, 0x40);
    pub const ADDR_MOD_AB_SEC8_SrcAClear: ConfigField = ConfigField::new(272, 7, 0x80);
    pub const ADDR_MOD_AB_SEC8_SrcAIncr: ConfigField = ConfigField::new(272, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC8_SrcBCR: ConfigField = ConfigField::new(272, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC8_SrcBClear: ConfigField = ConfigField::new(272, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC8_SrcBIncr: ConfigField = ConfigField::new(272, 8, 0x3f00);
    pub const ADDR_MOD_AB_SEC9_SrcACR: ConfigField = ConfigField::new(273, 6, 0x40);
    pub const ADDR_MOD_AB_SEC9_SrcAClear: ConfigField = ConfigField::new(273, 7, 0x80);
    pub const ADDR_MOD_AB_SEC9_SrcAIncr: ConfigField = ConfigField::new(273, 0, 0x3f);
    pub const ADDR_MOD_AB_SEC9_SrcBCR: ConfigField = ConfigField::new(273, 14, 0x4000);
    pub const ADDR_MOD_AB_SEC9_SrcBClear: ConfigField = ConfigField::new(273, 15, 0x8000);
    pub const ADDR_MOD_AB_SEC9_SrcBIncr: ConfigField = ConfigField::new(273, 8, 0x3f00);
    pub const ADDR_MOD_DST_SEC0_DestCR: ConfigField = ConfigField::new(328, 10, 0x400);
    pub const ADDR_MOD_DST_SEC0_DestCToCR: ConfigField = ConfigField::new(328, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC0_DestClear: ConfigField = ConfigField::new(328, 11, 0x800);
    pub const ADDR_MOD_DST_SEC0_DestIncr: ConfigField = ConfigField::new(328, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC0_FidelityClear: ConfigField = ConfigField::new(328, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC0_FidelityIncr: ConfigField = ConfigField::new(328, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC10_DestCR: ConfigField = ConfigField::new(338, 10, 0x400);
    pub const ADDR_MOD_DST_SEC10_DestCToCR: ConfigField = ConfigField::new(338, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC10_DestClear: ConfigField = ConfigField::new(338, 11, 0x800);
    pub const ADDR_MOD_DST_SEC10_DestIncr: ConfigField = ConfigField::new(338, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC10_FidelityClear: ConfigField = ConfigField::new(338, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC10_FidelityIncr: ConfigField = ConfigField::new(338, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC11_DestCR: ConfigField = ConfigField::new(339, 10, 0x400);
    pub const ADDR_MOD_DST_SEC11_DestCToCR: ConfigField = ConfigField::new(339, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC11_DestClear: ConfigField = ConfigField::new(339, 11, 0x800);
    pub const ADDR_MOD_DST_SEC11_DestIncr: ConfigField = ConfigField::new(339, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC11_FidelityClear: ConfigField = ConfigField::new(339, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC11_FidelityIncr: ConfigField = ConfigField::new(339, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC12_DestCR: ConfigField = ConfigField::new(340, 10, 0x400);
    pub const ADDR_MOD_DST_SEC12_DestCToCR: ConfigField = ConfigField::new(340, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC12_DestClear: ConfigField = ConfigField::new(340, 11, 0x800);
    pub const ADDR_MOD_DST_SEC12_DestIncr: ConfigField = ConfigField::new(340, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC12_FidelityClear: ConfigField = ConfigField::new(340, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC12_FidelityIncr: ConfigField = ConfigField::new(340, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC13_DestCR: ConfigField = ConfigField::new(341, 10, 0x400);
    pub const ADDR_MOD_DST_SEC13_DestCToCR: ConfigField = ConfigField::new(341, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC13_DestClear: ConfigField = ConfigField::new(341, 11, 0x800);
    pub const ADDR_MOD_DST_SEC13_DestIncr: ConfigField = ConfigField::new(341, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC13_FidelityClear: ConfigField = ConfigField::new(341, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC13_FidelityIncr: ConfigField = ConfigField::new(341, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC14_DestCR: ConfigField = ConfigField::new(342, 10, 0x400);
    pub const ADDR_MOD_DST_SEC14_DestCToCR: ConfigField = ConfigField::new(342, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC14_DestClear: ConfigField = ConfigField::new(342, 11, 0x800);
    pub const ADDR_MOD_DST_SEC14_DestIncr: ConfigField = ConfigField::new(342, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC14_FidelityClear: ConfigField = ConfigField::new(342, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC14_FidelityIncr: ConfigField = ConfigField::new(342, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC15_DestCR: ConfigField = ConfigField::new(343, 10, 0x400);
    pub const ADDR_MOD_DST_SEC15_DestCToCR: ConfigField = ConfigField::new(343, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC15_DestClear: ConfigField = ConfigField::new(343, 11, 0x800);
    pub const ADDR_MOD_DST_SEC15_DestIncr: ConfigField = ConfigField::new(343, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC15_FidelityClear: ConfigField = ConfigField::new(343, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC15_FidelityIncr: ConfigField = ConfigField::new(343, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC16_DestCR: ConfigField = ConfigField::new(344, 10, 0x400);
    pub const ADDR_MOD_DST_SEC16_DestCToCR: ConfigField = ConfigField::new(344, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC16_DestClear: ConfigField = ConfigField::new(344, 11, 0x800);
    pub const ADDR_MOD_DST_SEC16_DestIncr: ConfigField = ConfigField::new(344, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC16_FidelityClear: ConfigField = ConfigField::new(344, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC16_FidelityIncr: ConfigField = ConfigField::new(344, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC17_DestCR: ConfigField = ConfigField::new(345, 10, 0x400);
    pub const ADDR_MOD_DST_SEC17_DestCToCR: ConfigField = ConfigField::new(345, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC17_DestClear: ConfigField = ConfigField::new(345, 11, 0x800);
    pub const ADDR_MOD_DST_SEC17_DestIncr: ConfigField = ConfigField::new(345, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC17_FidelityClear: ConfigField = ConfigField::new(345, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC17_FidelityIncr: ConfigField = ConfigField::new(345, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC18_DestCR: ConfigField = ConfigField::new(346, 10, 0x400);
    pub const ADDR_MOD_DST_SEC18_DestCToCR: ConfigField = ConfigField::new(346, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC18_DestClear: ConfigField = ConfigField::new(346, 11, 0x800);
    pub const ADDR_MOD_DST_SEC18_DestIncr: ConfigField = ConfigField::new(346, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC18_FidelityClear: ConfigField = ConfigField::new(346, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC18_FidelityIncr: ConfigField = ConfigField::new(346, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC19_DestCR: ConfigField = ConfigField::new(347, 10, 0x400);
    pub const ADDR_MOD_DST_SEC19_DestCToCR: ConfigField = ConfigField::new(347, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC19_DestClear: ConfigField = ConfigField::new(347, 11, 0x800);
    pub const ADDR_MOD_DST_SEC19_DestIncr: ConfigField = ConfigField::new(347, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC19_FidelityClear: ConfigField = ConfigField::new(347, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC19_FidelityIncr: ConfigField = ConfigField::new(347, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC1_DestCR: ConfigField = ConfigField::new(329, 10, 0x400);
    pub const ADDR_MOD_DST_SEC1_DestCToCR: ConfigField = ConfigField::new(329, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC1_DestClear: ConfigField = ConfigField::new(329, 11, 0x800);
    pub const ADDR_MOD_DST_SEC1_DestIncr: ConfigField = ConfigField::new(329, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC1_FidelityClear: ConfigField = ConfigField::new(329, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC1_FidelityIncr: ConfigField = ConfigField::new(329, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC20_DestCR: ConfigField = ConfigField::new(348, 10, 0x400);
    pub const ADDR_MOD_DST_SEC20_DestCToCR: ConfigField = ConfigField::new(348, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC20_DestClear: ConfigField = ConfigField::new(348, 11, 0x800);
    pub const ADDR_MOD_DST_SEC20_DestIncr: ConfigField = ConfigField::new(348, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC20_FidelityClear: ConfigField = ConfigField::new(348, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC20_FidelityIncr: ConfigField = ConfigField::new(348, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC21_DestCR: ConfigField = ConfigField::new(349, 10, 0x400);
    pub const ADDR_MOD_DST_SEC21_DestCToCR: ConfigField = ConfigField::new(349, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC21_DestClear: ConfigField = ConfigField::new(349, 11, 0x800);
    pub const ADDR_MOD_DST_SEC21_DestIncr: ConfigField = ConfigField::new(349, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC21_FidelityClear: ConfigField = ConfigField::new(349, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC21_FidelityIncr: ConfigField = ConfigField::new(349, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC22_DestCR: ConfigField = ConfigField::new(350, 10, 0x400);
    pub const ADDR_MOD_DST_SEC22_DestCToCR: ConfigField = ConfigField::new(350, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC22_DestClear: ConfigField = ConfigField::new(350, 11, 0x800);
    pub const ADDR_MOD_DST_SEC22_DestIncr: ConfigField = ConfigField::new(350, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC22_FidelityClear: ConfigField = ConfigField::new(350, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC22_FidelityIncr: ConfigField = ConfigField::new(350, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC23_DestCR: ConfigField = ConfigField::new(351, 10, 0x400);
    pub const ADDR_MOD_DST_SEC23_DestCToCR: ConfigField = ConfigField::new(351, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC23_DestClear: ConfigField = ConfigField::new(351, 11, 0x800);
    pub const ADDR_MOD_DST_SEC23_DestIncr: ConfigField = ConfigField::new(351, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC23_FidelityClear: ConfigField = ConfigField::new(351, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC23_FidelityIncr: ConfigField = ConfigField::new(351, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC24_DestCR: ConfigField = ConfigField::new(352, 10, 0x400);
    pub const ADDR_MOD_DST_SEC24_DestCToCR: ConfigField = ConfigField::new(352, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC24_DestClear: ConfigField = ConfigField::new(352, 11, 0x800);
    pub const ADDR_MOD_DST_SEC24_DestIncr: ConfigField = ConfigField::new(352, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC24_FidelityClear: ConfigField = ConfigField::new(352, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC24_FidelityIncr: ConfigField = ConfigField::new(352, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC25_DestCR: ConfigField = ConfigField::new(353, 10, 0x400);
    pub const ADDR_MOD_DST_SEC25_DestCToCR: ConfigField = ConfigField::new(353, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC25_DestClear: ConfigField = ConfigField::new(353, 11, 0x800);
    pub const ADDR_MOD_DST_SEC25_DestIncr: ConfigField = ConfigField::new(353, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC25_FidelityClear: ConfigField = ConfigField::new(353, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC25_FidelityIncr: ConfigField = ConfigField::new(353, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC26_DestCR: ConfigField = ConfigField::new(354, 10, 0x400);
    pub const ADDR_MOD_DST_SEC26_DestCToCR: ConfigField = ConfigField::new(354, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC26_DestClear: ConfigField = ConfigField::new(354, 11, 0x800);
    pub const ADDR_MOD_DST_SEC26_DestIncr: ConfigField = ConfigField::new(354, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC26_FidelityClear: ConfigField = ConfigField::new(354, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC26_FidelityIncr: ConfigField = ConfigField::new(354, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC27_DestCR: ConfigField = ConfigField::new(355, 10, 0x400);
    pub const ADDR_MOD_DST_SEC27_DestCToCR: ConfigField = ConfigField::new(355, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC27_DestClear: ConfigField = ConfigField::new(355, 11, 0x800);
    pub const ADDR_MOD_DST_SEC27_DestIncr: ConfigField = ConfigField::new(355, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC27_FidelityClear: ConfigField = ConfigField::new(355, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC27_FidelityIncr: ConfigField = ConfigField::new(355, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC28_DestCR: ConfigField = ConfigField::new(356, 10, 0x400);
    pub const ADDR_MOD_DST_SEC28_DestCToCR: ConfigField = ConfigField::new(356, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC28_DestClear: ConfigField = ConfigField::new(356, 11, 0x800);
    pub const ADDR_MOD_DST_SEC28_DestIncr: ConfigField = ConfigField::new(356, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC28_FidelityClear: ConfigField = ConfigField::new(356, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC28_FidelityIncr: ConfigField = ConfigField::new(356, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC29_DestCR: ConfigField = ConfigField::new(357, 10, 0x400);
    pub const ADDR_MOD_DST_SEC29_DestCToCR: ConfigField = ConfigField::new(357, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC29_DestClear: ConfigField = ConfigField::new(357, 11, 0x800);
    pub const ADDR_MOD_DST_SEC29_DestIncr: ConfigField = ConfigField::new(357, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC29_FidelityClear: ConfigField = ConfigField::new(357, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC29_FidelityIncr: ConfigField = ConfigField::new(357, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC2_DestCR: ConfigField = ConfigField::new(330, 10, 0x400);
    pub const ADDR_MOD_DST_SEC2_DestCToCR: ConfigField = ConfigField::new(330, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC2_DestClear: ConfigField = ConfigField::new(330, 11, 0x800);
    pub const ADDR_MOD_DST_SEC2_DestIncr: ConfigField = ConfigField::new(330, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC2_FidelityClear: ConfigField = ConfigField::new(330, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC2_FidelityIncr: ConfigField = ConfigField::new(330, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC30_DestCR: ConfigField = ConfigField::new(358, 10, 0x400);
    pub const ADDR_MOD_DST_SEC30_DestCToCR: ConfigField = ConfigField::new(358, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC30_DestClear: ConfigField = ConfigField::new(358, 11, 0x800);
    pub const ADDR_MOD_DST_SEC30_DestIncr: ConfigField = ConfigField::new(358, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC30_FidelityClear: ConfigField = ConfigField::new(358, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC30_FidelityIncr: ConfigField = ConfigField::new(358, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC31_DestCR: ConfigField = ConfigField::new(359, 10, 0x400);
    pub const ADDR_MOD_DST_SEC31_DestCToCR: ConfigField = ConfigField::new(359, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC31_DestClear: ConfigField = ConfigField::new(359, 11, 0x800);
    pub const ADDR_MOD_DST_SEC31_DestIncr: ConfigField = ConfigField::new(359, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC31_FidelityClear: ConfigField = ConfigField::new(359, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC31_FidelityIncr: ConfigField = ConfigField::new(359, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC3_DestCR: ConfigField = ConfigField::new(331, 10, 0x400);
    pub const ADDR_MOD_DST_SEC3_DestCToCR: ConfigField = ConfigField::new(331, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC3_DestClear: ConfigField = ConfigField::new(331, 11, 0x800);
    pub const ADDR_MOD_DST_SEC3_DestIncr: ConfigField = ConfigField::new(331, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC3_FidelityClear: ConfigField = ConfigField::new(331, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC3_FidelityIncr: ConfigField = ConfigField::new(331, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC4_DestCR: ConfigField = ConfigField::new(332, 10, 0x400);
    pub const ADDR_MOD_DST_SEC4_DestCToCR: ConfigField = ConfigField::new(332, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC4_DestClear: ConfigField = ConfigField::new(332, 11, 0x800);
    pub const ADDR_MOD_DST_SEC4_DestIncr: ConfigField = ConfigField::new(332, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC4_FidelityClear: ConfigField = ConfigField::new(332, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC4_FidelityIncr: ConfigField = ConfigField::new(332, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC5_DestCR: ConfigField = ConfigField::new(333, 10, 0x400);
    pub const ADDR_MOD_DST_SEC5_DestCToCR: ConfigField = ConfigField::new(333, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC5_DestClear: ConfigField = ConfigField::new(333, 11, 0x800);
    pub const ADDR_MOD_DST_SEC5_DestIncr: ConfigField = ConfigField::new(333, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC5_FidelityClear: ConfigField = ConfigField::new(333, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC5_FidelityIncr: ConfigField = ConfigField::new(333, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC6_DestCR: ConfigField = ConfigField::new(334, 10, 0x400);
    pub const ADDR_MOD_DST_SEC6_DestCToCR: ConfigField = ConfigField::new(334, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC6_DestClear: ConfigField = ConfigField::new(334, 11, 0x800);
    pub const ADDR_MOD_DST_SEC6_DestIncr: ConfigField = ConfigField::new(334, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC6_FidelityClear: ConfigField = ConfigField::new(334, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC6_FidelityIncr: ConfigField = ConfigField::new(334, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC7_DestCR: ConfigField = ConfigField::new(335, 10, 0x400);
    pub const ADDR_MOD_DST_SEC7_DestCToCR: ConfigField = ConfigField::new(335, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC7_DestClear: ConfigField = ConfigField::new(335, 11, 0x800);
    pub const ADDR_MOD_DST_SEC7_DestIncr: ConfigField = ConfigField::new(335, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC7_FidelityClear: ConfigField = ConfigField::new(335, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC7_FidelityIncr: ConfigField = ConfigField::new(335, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC8_DestCR: ConfigField = ConfigField::new(336, 10, 0x400);
    pub const ADDR_MOD_DST_SEC8_DestCToCR: ConfigField = ConfigField::new(336, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC8_DestClear: ConfigField = ConfigField::new(336, 11, 0x800);
    pub const ADDR_MOD_DST_SEC8_DestIncr: ConfigField = ConfigField::new(336, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC8_FidelityClear: ConfigField = ConfigField::new(336, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC8_FidelityIncr: ConfigField = ConfigField::new(336, 13, 0x6000);
    pub const ADDR_MOD_DST_SEC9_DestCR: ConfigField = ConfigField::new(337, 10, 0x400);
    pub const ADDR_MOD_DST_SEC9_DestCToCR: ConfigField = ConfigField::new(337, 12, 0x1000);
    pub const ADDR_MOD_DST_SEC9_DestClear: ConfigField = ConfigField::new(337, 11, 0x800);
    pub const ADDR_MOD_DST_SEC9_DestIncr: ConfigField = ConfigField::new(337, 0, 0x3ff);
    pub const ADDR_MOD_DST_SEC9_FidelityClear: ConfigField = ConfigField::new(337, 15, 0x8000);
    pub const ADDR_MOD_DST_SEC9_FidelityIncr: ConfigField = ConfigField::new(337, 13, 0x6000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_L1_BASE_ADDR: ConfigField =
        ConfigField::new(108, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(109, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_TILE_FORMAT: ConfigField =
        ConfigField::new(108, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_TILE_X_DIM: ConfigField =
        ConfigField::new(109, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Y_DIM: ConfigField = ConfigField::new(110, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Z_DIM: ConfigField =
        ConfigField::new(110, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED0: ConfigField =
        ConfigField::new(108, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED1: ConfigField =
        ConfigField::new(109, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED2: ConfigField =
        ConfigField::new(110, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED3: ConfigField =
        ConfigField::new(111, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_L1_BASE_ADDR: ConfigField =
        ConfigField::new(148, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(149, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_TILE_FORMAT: ConfigField =
        ConfigField::new(148, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_TILE_X_DIM: ConfigField =
        ConfigField::new(149, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Y_DIM: ConfigField =
        ConfigField::new(150, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Z_DIM: ConfigField =
        ConfigField::new(150, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED0: ConfigField =
        ConfigField::new(148, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED1: ConfigField =
        ConfigField::new(149, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED2: ConfigField =
        ConfigField::new(150, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED3: ConfigField =
        ConfigField::new(151, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_L1_BASE_ADDR: ConfigField =
        ConfigField::new(152, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(153, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_TILE_FORMAT: ConfigField =
        ConfigField::new(152, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_TILE_X_DIM: ConfigField =
        ConfigField::new(153, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Y_DIM: ConfigField =
        ConfigField::new(154, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Z_DIM: ConfigField =
        ConfigField::new(154, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED0: ConfigField =
        ConfigField::new(152, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED1: ConfigField =
        ConfigField::new(153, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED2: ConfigField =
        ConfigField::new(154, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED3: ConfigField =
        ConfigField::new(155, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_L1_BASE_ADDR: ConfigField =
        ConfigField::new(156, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(157, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_TILE_FORMAT: ConfigField =
        ConfigField::new(156, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_TILE_X_DIM: ConfigField =
        ConfigField::new(157, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Y_DIM: ConfigField =
        ConfigField::new(158, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Z_DIM: ConfigField =
        ConfigField::new(158, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED0: ConfigField =
        ConfigField::new(156, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED1: ConfigField =
        ConfigField::new(157, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED2: ConfigField =
        ConfigField::new(158, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED3: ConfigField =
        ConfigField::new(159, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_L1_BASE_ADDR: ConfigField =
        ConfigField::new(160, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(161, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_TILE_FORMAT: ConfigField =
        ConfigField::new(160, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_TILE_X_DIM: ConfigField =
        ConfigField::new(161, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Y_DIM: ConfigField =
        ConfigField::new(162, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Z_DIM: ConfigField =
        ConfigField::new(162, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED0: ConfigField =
        ConfigField::new(160, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED1: ConfigField =
        ConfigField::new(161, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED2: ConfigField =
        ConfigField::new(162, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED3: ConfigField =
        ConfigField::new(163, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_L1_BASE_ADDR: ConfigField =
        ConfigField::new(164, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(165, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_TILE_FORMAT: ConfigField =
        ConfigField::new(164, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_TILE_X_DIM: ConfigField =
        ConfigField::new(165, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Y_DIM: ConfigField =
        ConfigField::new(166, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Z_DIM: ConfigField =
        ConfigField::new(166, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED0: ConfigField =
        ConfigField::new(164, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED1: ConfigField =
        ConfigField::new(165, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED2: ConfigField =
        ConfigField::new(166, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED3: ConfigField =
        ConfigField::new(167, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_L1_BASE_ADDR: ConfigField =
        ConfigField::new(168, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(169, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_TILE_FORMAT: ConfigField =
        ConfigField::new(168, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_TILE_X_DIM: ConfigField =
        ConfigField::new(169, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Y_DIM: ConfigField =
        ConfigField::new(170, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Z_DIM: ConfigField =
        ConfigField::new(170, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED0: ConfigField =
        ConfigField::new(168, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED1: ConfigField =
        ConfigField::new(169, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED2: ConfigField =
        ConfigField::new(170, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED3: ConfigField =
        ConfigField::new(171, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_L1_BASE_ADDR: ConfigField =
        ConfigField::new(172, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(173, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_TILE_FORMAT: ConfigField =
        ConfigField::new(172, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_TILE_X_DIM: ConfigField =
        ConfigField::new(173, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Y_DIM: ConfigField =
        ConfigField::new(174, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Z_DIM: ConfigField =
        ConfigField::new(174, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED0: ConfigField =
        ConfigField::new(172, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED1: ConfigField =
        ConfigField::new(173, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED2: ConfigField =
        ConfigField::new(174, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED3: ConfigField =
        ConfigField::new(175, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_L1_BASE_ADDR: ConfigField =
        ConfigField::new(176, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(177, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_TILE_FORMAT: ConfigField =
        ConfigField::new(176, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_TILE_X_DIM: ConfigField =
        ConfigField::new(177, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Y_DIM: ConfigField =
        ConfigField::new(178, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Z_DIM: ConfigField =
        ConfigField::new(178, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED0: ConfigField =
        ConfigField::new(176, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED1: ConfigField =
        ConfigField::new(177, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED2: ConfigField =
        ConfigField::new(178, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED3: ConfigField =
        ConfigField::new(179, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_L1_BASE_ADDR: ConfigField =
        ConfigField::new(180, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(181, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_TILE_FORMAT: ConfigField =
        ConfigField::new(180, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_TILE_X_DIM: ConfigField =
        ConfigField::new(181, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Y_DIM: ConfigField =
        ConfigField::new(182, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Z_DIM: ConfigField =
        ConfigField::new(182, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED0: ConfigField =
        ConfigField::new(180, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED1: ConfigField =
        ConfigField::new(181, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED2: ConfigField =
        ConfigField::new(182, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED3: ConfigField =
        ConfigField::new(183, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_L1_BASE_ADDR: ConfigField =
        ConfigField::new(184, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(185, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_TILE_FORMAT: ConfigField =
        ConfigField::new(184, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_TILE_X_DIM: ConfigField =
        ConfigField::new(185, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Y_DIM: ConfigField =
        ConfigField::new(186, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Z_DIM: ConfigField =
        ConfigField::new(186, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED0: ConfigField =
        ConfigField::new(184, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED1: ConfigField =
        ConfigField::new(185, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED2: ConfigField =
        ConfigField::new(186, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED3: ConfigField =
        ConfigField::new(187, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_L1_BASE_ADDR: ConfigField =
        ConfigField::new(112, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(113, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_TILE_FORMAT: ConfigField =
        ConfigField::new(112, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_TILE_X_DIM: ConfigField =
        ConfigField::new(113, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Y_DIM: ConfigField = ConfigField::new(114, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Z_DIM: ConfigField =
        ConfigField::new(114, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED0: ConfigField =
        ConfigField::new(112, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED1: ConfigField =
        ConfigField::new(113, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED2: ConfigField =
        ConfigField::new(114, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED3: ConfigField =
        ConfigField::new(115, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_L1_BASE_ADDR: ConfigField =
        ConfigField::new(188, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(189, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_TILE_FORMAT: ConfigField =
        ConfigField::new(188, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_TILE_X_DIM: ConfigField =
        ConfigField::new(189, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Y_DIM: ConfigField =
        ConfigField::new(190, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Z_DIM: ConfigField =
        ConfigField::new(190, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED0: ConfigField =
        ConfigField::new(188, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED1: ConfigField =
        ConfigField::new(189, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED2: ConfigField =
        ConfigField::new(190, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED3: ConfigField =
        ConfigField::new(191, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_L1_BASE_ADDR: ConfigField =
        ConfigField::new(192, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(193, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_TILE_FORMAT: ConfigField =
        ConfigField::new(192, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_TILE_X_DIM: ConfigField =
        ConfigField::new(193, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Y_DIM: ConfigField =
        ConfigField::new(194, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Z_DIM: ConfigField =
        ConfigField::new(194, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED0: ConfigField =
        ConfigField::new(192, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED1: ConfigField =
        ConfigField::new(193, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED2: ConfigField =
        ConfigField::new(194, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED3: ConfigField =
        ConfigField::new(195, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_L1_BASE_ADDR: ConfigField =
        ConfigField::new(196, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(197, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_TILE_FORMAT: ConfigField =
        ConfigField::new(196, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_TILE_X_DIM: ConfigField =
        ConfigField::new(197, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Y_DIM: ConfigField =
        ConfigField::new(198, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Z_DIM: ConfigField =
        ConfigField::new(198, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED0: ConfigField =
        ConfigField::new(196, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED1: ConfigField =
        ConfigField::new(197, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED2: ConfigField =
        ConfigField::new(198, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED3: ConfigField =
        ConfigField::new(199, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_L1_BASE_ADDR: ConfigField =
        ConfigField::new(200, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(201, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_TILE_FORMAT: ConfigField =
        ConfigField::new(200, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_TILE_X_DIM: ConfigField =
        ConfigField::new(201, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Y_DIM: ConfigField =
        ConfigField::new(202, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Z_DIM: ConfigField =
        ConfigField::new(202, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED0: ConfigField =
        ConfigField::new(200, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED1: ConfigField =
        ConfigField::new(201, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED2: ConfigField =
        ConfigField::new(202, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED3: ConfigField =
        ConfigField::new(203, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_L1_BASE_ADDR: ConfigField =
        ConfigField::new(204, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(205, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_TILE_FORMAT: ConfigField =
        ConfigField::new(204, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_TILE_X_DIM: ConfigField =
        ConfigField::new(205, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Y_DIM: ConfigField =
        ConfigField::new(206, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Z_DIM: ConfigField =
        ConfigField::new(206, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED0: ConfigField =
        ConfigField::new(204, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED1: ConfigField =
        ConfigField::new(205, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED2: ConfigField =
        ConfigField::new(206, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED3: ConfigField =
        ConfigField::new(207, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_L1_BASE_ADDR: ConfigField =
        ConfigField::new(208, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(209, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_TILE_FORMAT: ConfigField =
        ConfigField::new(208, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_TILE_X_DIM: ConfigField =
        ConfigField::new(209, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Y_DIM: ConfigField =
        ConfigField::new(210, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Z_DIM: ConfigField =
        ConfigField::new(210, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED0: ConfigField =
        ConfigField::new(208, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED1: ConfigField =
        ConfigField::new(209, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED2: ConfigField =
        ConfigField::new(210, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED3: ConfigField =
        ConfigField::new(211, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_L1_BASE_ADDR: ConfigField =
        ConfigField::new(212, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(213, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_TILE_FORMAT: ConfigField =
        ConfigField::new(212, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_TILE_X_DIM: ConfigField =
        ConfigField::new(213, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Y_DIM: ConfigField =
        ConfigField::new(214, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Z_DIM: ConfigField =
        ConfigField::new(214, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED0: ConfigField =
        ConfigField::new(212, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED1: ConfigField =
        ConfigField::new(213, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED2: ConfigField =
        ConfigField::new(214, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED3: ConfigField =
        ConfigField::new(215, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_L1_BASE_ADDR: ConfigField =
        ConfigField::new(216, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(217, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_TILE_FORMAT: ConfigField =
        ConfigField::new(216, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_TILE_X_DIM: ConfigField =
        ConfigField::new(217, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Y_DIM: ConfigField =
        ConfigField::new(218, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Z_DIM: ConfigField =
        ConfigField::new(218, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED0: ConfigField =
        ConfigField::new(216, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED1: ConfigField =
        ConfigField::new(217, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED2: ConfigField =
        ConfigField::new(218, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED3: ConfigField =
        ConfigField::new(219, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_L1_BASE_ADDR: ConfigField =
        ConfigField::new(220, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(221, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_TILE_FORMAT: ConfigField =
        ConfigField::new(220, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_TILE_X_DIM: ConfigField =
        ConfigField::new(221, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Y_DIM: ConfigField =
        ConfigField::new(222, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Z_DIM: ConfigField =
        ConfigField::new(222, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED0: ConfigField =
        ConfigField::new(220, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED1: ConfigField =
        ConfigField::new(221, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED2: ConfigField =
        ConfigField::new(222, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED3: ConfigField =
        ConfigField::new(223, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_L1_BASE_ADDR: ConfigField =
        ConfigField::new(224, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(225, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_TILE_FORMAT: ConfigField =
        ConfigField::new(224, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_TILE_X_DIM: ConfigField =
        ConfigField::new(225, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Y_DIM: ConfigField =
        ConfigField::new(226, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Z_DIM: ConfigField =
        ConfigField::new(226, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED0: ConfigField =
        ConfigField::new(224, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED1: ConfigField =
        ConfigField::new(225, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED2: ConfigField =
        ConfigField::new(226, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED3: ConfigField =
        ConfigField::new(227, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_L1_BASE_ADDR: ConfigField =
        ConfigField::new(116, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(117, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_TILE_FORMAT: ConfigField =
        ConfigField::new(116, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_TILE_X_DIM: ConfigField =
        ConfigField::new(117, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Y_DIM: ConfigField = ConfigField::new(118, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Z_DIM: ConfigField =
        ConfigField::new(118, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED0: ConfigField =
        ConfigField::new(116, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED1: ConfigField =
        ConfigField::new(117, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED2: ConfigField =
        ConfigField::new(118, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED3: ConfigField =
        ConfigField::new(119, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_L1_BASE_ADDR: ConfigField =
        ConfigField::new(228, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(229, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_TILE_FORMAT: ConfigField =
        ConfigField::new(228, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_TILE_X_DIM: ConfigField =
        ConfigField::new(229, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Y_DIM: ConfigField =
        ConfigField::new(230, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Z_DIM: ConfigField =
        ConfigField::new(230, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED0: ConfigField =
        ConfigField::new(228, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED1: ConfigField =
        ConfigField::new(229, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED2: ConfigField =
        ConfigField::new(230, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED3: ConfigField =
        ConfigField::new(231, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_L1_BASE_ADDR: ConfigField =
        ConfigField::new(232, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(233, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_TILE_FORMAT: ConfigField =
        ConfigField::new(232, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_TILE_X_DIM: ConfigField =
        ConfigField::new(233, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Y_DIM: ConfigField =
        ConfigField::new(234, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Z_DIM: ConfigField =
        ConfigField::new(234, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED0: ConfigField =
        ConfigField::new(232, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED1: ConfigField =
        ConfigField::new(233, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED2: ConfigField =
        ConfigField::new(234, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED3: ConfigField =
        ConfigField::new(235, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_L1_BASE_ADDR: ConfigField =
        ConfigField::new(120, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(121, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_TILE_FORMAT: ConfigField =
        ConfigField::new(120, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_TILE_X_DIM: ConfigField =
        ConfigField::new(121, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Y_DIM: ConfigField = ConfigField::new(122, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Z_DIM: ConfigField =
        ConfigField::new(122, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED0: ConfigField =
        ConfigField::new(120, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED1: ConfigField =
        ConfigField::new(121, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED2: ConfigField =
        ConfigField::new(122, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED3: ConfigField =
        ConfigField::new(123, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_L1_BASE_ADDR: ConfigField =
        ConfigField::new(124, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(125, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_TILE_FORMAT: ConfigField =
        ConfigField::new(124, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_TILE_X_DIM: ConfigField =
        ConfigField::new(125, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Y_DIM: ConfigField = ConfigField::new(126, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Z_DIM: ConfigField =
        ConfigField::new(126, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED0: ConfigField =
        ConfigField::new(124, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED1: ConfigField =
        ConfigField::new(125, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED2: ConfigField =
        ConfigField::new(126, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED3: ConfigField =
        ConfigField::new(127, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_L1_BASE_ADDR: ConfigField =
        ConfigField::new(128, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(129, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_TILE_FORMAT: ConfigField =
        ConfigField::new(128, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_TILE_X_DIM: ConfigField =
        ConfigField::new(129, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Y_DIM: ConfigField = ConfigField::new(130, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Z_DIM: ConfigField =
        ConfigField::new(130, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED0: ConfigField =
        ConfigField::new(128, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED1: ConfigField =
        ConfigField::new(129, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED2: ConfigField =
        ConfigField::new(130, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED3: ConfigField =
        ConfigField::new(131, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_L1_BASE_ADDR: ConfigField =
        ConfigField::new(132, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(133, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_TILE_FORMAT: ConfigField =
        ConfigField::new(132, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_TILE_X_DIM: ConfigField =
        ConfigField::new(133, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Y_DIM: ConfigField = ConfigField::new(134, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Z_DIM: ConfigField =
        ConfigField::new(134, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED0: ConfigField =
        ConfigField::new(132, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED1: ConfigField =
        ConfigField::new(133, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED2: ConfigField =
        ConfigField::new(134, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED3: ConfigField =
        ConfigField::new(135, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_L1_BASE_ADDR: ConfigField =
        ConfigField::new(136, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(137, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_TILE_FORMAT: ConfigField =
        ConfigField::new(136, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_TILE_X_DIM: ConfigField =
        ConfigField::new(137, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Y_DIM: ConfigField = ConfigField::new(138, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Z_DIM: ConfigField =
        ConfigField::new(138, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED0: ConfigField =
        ConfigField::new(136, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED1: ConfigField =
        ConfigField::new(137, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED2: ConfigField =
        ConfigField::new(138, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED3: ConfigField =
        ConfigField::new(139, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_L1_BASE_ADDR: ConfigField =
        ConfigField::new(140, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(141, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_TILE_FORMAT: ConfigField =
        ConfigField::new(140, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_TILE_X_DIM: ConfigField =
        ConfigField::new(141, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Y_DIM: ConfigField = ConfigField::new(142, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Z_DIM: ConfigField =
        ConfigField::new(142, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED0: ConfigField =
        ConfigField::new(140, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED1: ConfigField =
        ConfigField::new(141, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED2: ConfigField =
        ConfigField::new(142, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED3: ConfigField =
        ConfigField::new(143, 0, 0xffffffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_L1_BASE_ADDR: ConfigField =
        ConfigField::new(144, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_L1_LIMIT_ADDR: ConfigField =
        ConfigField::new(145, 0, 0xfffff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_TILE_FORMAT: ConfigField =
        ConfigField::new(144, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_TILE_X_DIM: ConfigField =
        ConfigField::new(145, 20, 0xff00000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Y_DIM: ConfigField = ConfigField::new(146, 0, 0xff);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Z_DIM: ConfigField =
        ConfigField::new(146, 8, 0xff00);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED0: ConfigField =
        ConfigField::new(144, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED1: ConfigField =
        ConfigField::new(145, 28, 0xf0000000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED2: ConfigField =
        ConfigField::new(146, 16, 0xffff0000);
    pub const BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED3: ConfigField =
        ConfigField::new(147, 0, 0xffffffff);
    pub const CFG_WATCHPOINTS_0_enable: ConfigField = ConfigField::new(240, 31, 0x80000000);
    pub const CFG_WATCHPOINTS_0_hi_addr: ConfigField = ConfigField::new(240, 11, 0x3ff800);
    pub const CFG_WATCHPOINTS_0_lo_addr: ConfigField = ConfigField::new(240, 0, 0x7ff);
    pub const CFG_WATCHPOINTS_0_rsvd: ConfigField = ConfigField::new(240, 22, 0x7fc00000);
    pub const CFG_WATCHPOINTS_1_enable: ConfigField = ConfigField::new(241, 31, 0x80000000);
    pub const CFG_WATCHPOINTS_1_hi_addr: ConfigField = ConfigField::new(241, 11, 0x3ff800);
    pub const CFG_WATCHPOINTS_1_lo_addr: ConfigField = ConfigField::new(241, 0, 0x7ff);
    pub const CFG_WATCHPOINTS_1_rsvd: ConfigField = ConfigField::new(241, 22, 0x7fc00000);
    pub const CFG_WATCHPOINTS_2_enable: ConfigField = ConfigField::new(242, 31, 0x80000000);
    pub const CFG_WATCHPOINTS_2_hi_addr: ConfigField = ConfigField::new(242, 11, 0x3ff800);
    pub const CFG_WATCHPOINTS_2_lo_addr: ConfigField = ConfigField::new(242, 0, 0x7ff);
    pub const CFG_WATCHPOINTS_2_rsvd: ConfigField = ConfigField::new(242, 22, 0x7fc00000);
    pub const CFG_WATCHPOINTS_3_enable: ConfigField = ConfigField::new(243, 31, 0x80000000);
    pub const CFG_WATCHPOINTS_3_hi_addr: ConfigField = ConfigField::new(243, 11, 0x3ff800);
    pub const CFG_WATCHPOINTS_3_lo_addr: ConfigField = ConfigField::new(243, 0, 0x7ff);
    pub const CFG_WATCHPOINTS_3_rsvd: ConfigField = ConfigField::new(243, 22, 0x7fc00000);
    pub const CLR_DVALID_SEC0_SrcA_Disable: ConfigField = ConfigField::new(256, 0, 0x1);
    pub const CLR_DVALID_SEC0_SrcB_Disable: ConfigField = ConfigField::new(256, 1, 0x2);
    pub const CLR_DVALID_SEC1_SrcA_Disable: ConfigField = ConfigField::new(257, 0, 0x1);
    pub const CLR_DVALID_SEC1_SrcB_Disable: ConfigField = ConfigField::new(257, 1, 0x2);
    pub const CLR_DVALID_SEC2_SrcA_Disable: ConfigField = ConfigField::new(258, 0, 0x1);
    pub const CLR_DVALID_SEC2_SrcB_Disable: ConfigField = ConfigField::new(258, 1, 0x2);
    pub const CLR_DVALID_SEC3_SrcA_Disable: ConfigField = ConfigField::new(259, 0, 0x1);
    pub const CLR_DVALID_SEC3_SrcB_Disable: ConfigField = ConfigField::new(259, 1, 0x2);
    pub const DEST_ACCESS_CFG_zeroacc_absolute_tile_mode: ConfigField =
        ConfigField::new(104, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCA_FMT_SEC0_Base: ConfigField = ConfigField::new(244, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCA_FMT_SEC1_Base: ConfigField = ConfigField::new(245, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCA_FMT_SEC2_Base: ConfigField = ConfigField::new(246, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCA_FMT_SEC3_Base: ConfigField = ConfigField::new(247, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCB_FMT_SEC0_Base: ConfigField = ConfigField::new(248, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCB_FMT_SEC1_Base: ConfigField = ConfigField::new(249, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCB_FMT_SEC2_Base: ConfigField = ConfigField::new(250, 0, 0x1);
    pub const DISABLE_IMPLIED_SRCB_FMT_SEC3_Base: ConfigField = ConfigField::new(251, 0, 0x1);
    pub const ENABLE_ACC_STATS_SEC0_Enable: ConfigField = ConfigField::new(368, 0, 0x1);
    pub const ENABLE_ACC_STATS_SEC1_Enable: ConfigField = ConfigField::new(369, 0, 0x1);
    pub const ENABLE_ACC_STATS_SEC2_Enable: ConfigField = ConfigField::new(370, 0, 0x1);
    pub const ENABLE_ACC_STATS_SEC3_Enable: ConfigField = ConfigField::new(371, 0, 0x1);
    pub const FIDELITY_BASE_SEC0_Phase: ConfigField = ConfigField::new(260, 0, 0x3);
    pub const FIDELITY_BASE_SEC1_Phase: ConfigField = ConfigField::new(261, 0, 0x3);
    pub const FIDELITY_BASE_SEC2_Phase: ConfigField = ConfigField::new(262, 0, 0x3);
    pub const FIDELITY_BASE_SEC3_Phase: ConfigField = ConfigField::new(263, 0, 0x3);
    pub const FMT_CTRL_FP8_OVF_EN: ConfigField = ConfigField::new(106, 1, 0x2);
    pub const FMT_CTRL_MXINT_NEGZERO_EQ_NEGINF: ConfigField = ConfigField::new(106, 0, 0x1);
    pub const FMT_CTRL_MX_BLOCK_EXP_RND_TO_INF: ConfigField = ConfigField::new(106, 4, 0x10);
    pub const FMT_CTRL_RESET_PACKER0_EXP_STATS: ConfigField = ConfigField::new(106, 2, 0x4);
    pub const FMT_CTRL_RESET_PACKER1_EXP_STATS: ConfigField = ConfigField::new(106, 3, 0x8);
    pub const FP16A_FORCE_SEC0_Enable: ConfigField = ConfigField::new(372, 0, 0x1);
    pub const FP16A_FORCE_SEC1_Enable: ConfigField = ConfigField::new(373, 0, 0x1);
    pub const FP16A_FORCE_SEC2_Enable: ConfigField = ConfigField::new(374, 0, 0x1);
    pub const FP16A_FORCE_SEC3_Enable: ConfigField = ConfigField::new(375, 0, 0x1);
    pub const INT_DESCALE_VALUES_SEC0_Value: ConfigField = ConfigField::new(84, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC10_Value: ConfigField = ConfigField::new(94, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC11_Value: ConfigField = ConfigField::new(95, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC12_Value: ConfigField = ConfigField::new(96, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC13_Value: ConfigField = ConfigField::new(97, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC14_Value: ConfigField = ConfigField::new(98, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC15_Value: ConfigField = ConfigField::new(99, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC1_Value: ConfigField = ConfigField::new(85, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC2_Value: ConfigField = ConfigField::new(86, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC3_Value: ConfigField = ConfigField::new(87, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC4_Value: ConfigField = ConfigField::new(88, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC5_Value: ConfigField = ConfigField::new(89, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC6_Value: ConfigField = ConfigField::new(90, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC7_Value: ConfigField = ConfigField::new(91, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC8_Value: ConfigField = ConfigField::new(92, 0, 0xffffffff);
    pub const INT_DESCALE_VALUES_SEC9_Value: ConfigField = ConfigField::new(93, 0, 0xffffffff);
    pub const MATH_DEST_DVALID_CTRL_disable_auto_bank_id_toggle: ConfigField =
        ConfigField::new(237, 12, 0x1000);
    pub const MATH_DEST_DVALID_CTRL_toggle_mask: ConfigField = ConfigField::new(237, 8, 0xf00);
    pub const MATH_DEST_DVALID_CTRL_wait_mask: ConfigField = ConfigField::new(237, 0, 0xf);
    pub const MATH_DEST_DVALID_CTRL_wait_polarity: ConfigField = ConfigField::new(237, 4, 0xf0);
    pub const PACK_DEST_DVALID_CTRL_disable_auto_bank_id_toggle: ConfigField =
        ConfigField::new(239, 12, 0x1000);
    pub const PACK_DEST_DVALID_CTRL_toggle_mask: ConfigField = ConfigField::new(239, 8, 0xf00);
    pub const PACK_DEST_DVALID_CTRL_wait_mask: ConfigField = ConfigField::new(239, 0, 0xf);
    pub const PACK_DEST_DVALID_CTRL_wait_polarity: ConfigField = ConfigField::new(239, 4, 0xf0);
    pub const PERF_CNT_CMD_SEC0_Cmd0Start: ConfigField = ConfigField::new(364, 0, 0x1);
    pub const PERF_CNT_CMD_SEC0_Cmd0Stop: ConfigField = ConfigField::new(364, 1, 0x2);
    pub const PERF_CNT_CMD_SEC0_Cmd1Start: ConfigField = ConfigField::new(364, 2, 0x4);
    pub const PERF_CNT_CMD_SEC0_Cmd1Stop: ConfigField = ConfigField::new(364, 3, 0x8);
    pub const PERF_CNT_CMD_SEC0_Cmd2Start: ConfigField = ConfigField::new(364, 4, 0x10);
    pub const PERF_CNT_CMD_SEC0_Cmd2Stop: ConfigField = ConfigField::new(364, 5, 0x20);
    pub const PERF_CNT_CMD_SEC0_Cmd3Start: ConfigField = ConfigField::new(364, 6, 0x40);
    pub const PERF_CNT_CMD_SEC0_Cmd3Stop: ConfigField = ConfigField::new(364, 7, 0x80);
    pub const PERF_CNT_CMD_SEC1_Cmd0Start: ConfigField = ConfigField::new(365, 0, 0x1);
    pub const PERF_CNT_CMD_SEC1_Cmd0Stop: ConfigField = ConfigField::new(365, 1, 0x2);
    pub const PERF_CNT_CMD_SEC1_Cmd1Start: ConfigField = ConfigField::new(365, 2, 0x4);
    pub const PERF_CNT_CMD_SEC1_Cmd1Stop: ConfigField = ConfigField::new(365, 3, 0x8);
    pub const PERF_CNT_CMD_SEC1_Cmd2Start: ConfigField = ConfigField::new(365, 4, 0x10);
    pub const PERF_CNT_CMD_SEC1_Cmd2Stop: ConfigField = ConfigField::new(365, 5, 0x20);
    pub const PERF_CNT_CMD_SEC1_Cmd3Start: ConfigField = ConfigField::new(365, 6, 0x40);
    pub const PERF_CNT_CMD_SEC1_Cmd3Stop: ConfigField = ConfigField::new(365, 7, 0x80);
    pub const PERF_CNT_CMD_SEC2_Cmd0Start: ConfigField = ConfigField::new(366, 0, 0x1);
    pub const PERF_CNT_CMD_SEC2_Cmd0Stop: ConfigField = ConfigField::new(366, 1, 0x2);
    pub const PERF_CNT_CMD_SEC2_Cmd1Start: ConfigField = ConfigField::new(366, 2, 0x4);
    pub const PERF_CNT_CMD_SEC2_Cmd1Stop: ConfigField = ConfigField::new(366, 3, 0x8);
    pub const PERF_CNT_CMD_SEC2_Cmd2Start: ConfigField = ConfigField::new(366, 4, 0x10);
    pub const PERF_CNT_CMD_SEC2_Cmd2Stop: ConfigField = ConfigField::new(366, 5, 0x20);
    pub const PERF_CNT_CMD_SEC2_Cmd3Start: ConfigField = ConfigField::new(366, 6, 0x40);
    pub const PERF_CNT_CMD_SEC2_Cmd3Stop: ConfigField = ConfigField::new(366, 7, 0x80);
    pub const PERF_CNT_CMD_SEC3_Cmd0Start: ConfigField = ConfigField::new(367, 0, 0x1);
    pub const PERF_CNT_CMD_SEC3_Cmd0Stop: ConfigField = ConfigField::new(367, 1, 0x2);
    pub const PERF_CNT_CMD_SEC3_Cmd1Start: ConfigField = ConfigField::new(367, 2, 0x4);
    pub const PERF_CNT_CMD_SEC3_Cmd1Stop: ConfigField = ConfigField::new(367, 3, 0x8);
    pub const PERF_CNT_CMD_SEC3_Cmd2Start: ConfigField = ConfigField::new(367, 4, 0x10);
    pub const PERF_CNT_CMD_SEC3_Cmd2Stop: ConfigField = ConfigField::new(367, 5, 0x20);
    pub const PERF_CNT_CMD_SEC3_Cmd3Start: ConfigField = ConfigField::new(367, 6, 0x40);
    pub const PERF_CNT_CMD_SEC3_Cmd3Stop: ConfigField = ConfigField::new(367, 7, 0x80);
    pub const PRNG_SEED_OVERRIDE_CTRL_override_en: ConfigField = ConfigField::new(82, 16, 0x10000);
    pub const PRNG_SEED_OVERRIDE_CTRL_prng_id: ConfigField = ConfigField::new(82, 0, 0xffff);
    pub const PRNG_SEED_OVERRIDE_DATA_seed_override: ConfigField =
        ConfigField::new(83, 0, 0xffffffff);
    pub const PRNG_SEED_Seed_Val: ConfigField = ConfigField::new(80, 0, 0xffffffff);
    pub const SCRATCH_SEC0_val: ConfigField = ConfigField::new(100, 0, 0xffffffff);
    pub const SCRATCH_SEC1_val: ConfigField = ConfigField::new(101, 0, 0xffffffff);
    pub const SCRATCH_SEC2_val: ConfigField = ConfigField::new(102, 0, 0xffffffff);
    pub const SCRATCH_SEC3_val: ConfigField = ConfigField::new(103, 0, 0xffffffff);
    pub const SFPU_DEST_DVALID_CTRL_disable_auto_bank_id_toggle: ConfigField =
        ConfigField::new(238, 12, 0x1000);
    pub const SFPU_DEST_DVALID_CTRL_toggle_mask: ConfigField = ConfigField::new(238, 8, 0xf00);
    pub const SFPU_DEST_DVALID_CTRL_wait_mask: ConfigField = ConfigField::new(238, 0, 0xf);
    pub const SFPU_DEST_DVALID_CTRL_wait_polarity: ConfigField = ConfigField::new(238, 4, 0xf0);
    pub const SFPU_DEST_FMT_SEC0_Base: ConfigField = ConfigField::new(252, 1, 0x1e);
    pub const SFPU_DEST_FMT_SEC0_Enable: ConfigField = ConfigField::new(252, 0, 0x1);
    pub const SFPU_DEST_FMT_SEC1_Base: ConfigField = ConfigField::new(253, 1, 0x1e);
    pub const SFPU_DEST_FMT_SEC1_Enable: ConfigField = ConfigField::new(253, 0, 0x1);
    pub const SFPU_DEST_FMT_SEC2_Base: ConfigField = ConfigField::new(254, 1, 0x1e);
    pub const SFPU_DEST_FMT_SEC2_Enable: ConfigField = ConfigField::new(254, 0, 0x1);
    pub const SFPU_DEST_FMT_SEC3_Base: ConfigField = ConfigField::new(255, 1, 0x1e);
    pub const SFPU_DEST_FMT_SEC3_Enable: ConfigField = ConfigField::new(255, 0, 0x1);
    pub const SFPU_STACK_SEC0_Incr: ConfigField = ConfigField::new(360, 0, 0x3ff);
    pub const SFPU_STACK_SEC1_Incr: ConfigField = ConfigField::new(361, 0, 0x3ff);
    pub const SFPU_STACK_SEC2_Incr: ConfigField = ConfigField::new(362, 0, 0x3ff);
    pub const SFPU_STACK_SEC3_Incr: ConfigField = ConfigField::new(363, 0, 0x3ff);
    pub const SRC_ACCESS_CFG_disable_contig_srca_dvalid_phase: ConfigField =
        ConfigField::new(105, 2, 0x4);
    pub const SRC_ACCESS_CFG_disable_contig_srcb_dvalid_phase: ConfigField =
        ConfigField::new(105, 3, 0x8);
    pub const SRC_ACCESS_CFG_math_view_srca_as_one_bank: ConfigField =
        ConfigField::new(105, 0, 0x1);
    pub const SRC_ACCESS_CFG_math_view_srcb_as_one_bank: ConfigField =
        ConfigField::new(105, 1, 0x2);
    pub const UNPACK_TO_DEST_DVALID_CTRL_disable_auto_bank_id_toggle: ConfigField =
        ConfigField::new(236, 12, 0x1000);
    pub const UNPACK_TO_DEST_DVALID_CTRL_toggle_mask: ConfigField = ConfigField::new(236, 8, 0xf00);
    pub const UNPACK_TO_DEST_DVALID_CTRL_wait_mask: ConfigField = ConfigField::new(236, 0, 0xf);
    pub const UNPACK_TO_DEST_DVALID_CTRL_wait_polarity: ConfigField =
        ConfigField::new(236, 4, 0xf0);
}

/// `// Registers for THCON` — 174 fields, indexing `Config` (write with `WRCFG`).
pub mod thcon {
    use super::*;

    /// First word index of this section, from `THCON_CFGREG_BASE_ADDR32`.
    pub const CFGREG_BASE: u16 = 12;

    pub const THCON_PACKER0_REG0_DEST_REG_WRAP_MODE: ConfigField = ConfigField::new(48, 10, 0xc00);
    pub const THCON_PACKER0_REG0_EXP_THRESHOLD: ConfigField = ConfigField::new(48, 15, 0x7f8000);
    pub const THCON_PACKER0_REG0_EXP_THRESHOLD_EN: ConfigField = ConfigField::new(48, 14, 0x4000);
    pub const THCON_PACKER0_REG0_INT_DESCALE_ENABLE: ConfigField =
        ConfigField::new(48, 23, 0x800000);
    pub const THCON_PACKER0_REG0_INT_DESCALE_MODE: ConfigField =
        ConfigField::new(48, 25, 0xe000000);
    pub const THCON_PACKER0_REG0_IN_DATA_FORMAT: ConfigField = ConfigField::new(48, 0, 0xff);
    pub const THCON_PACKER0_REG0_L1_ACC: ConfigField = ConfigField::new(48, 12, 0x1000);
    pub const THCON_PACKER0_REG0_READ_MODE: ConfigField = ConfigField::new(48, 8, 0x300);
    pub const THCON_PACKER0_REG0_SELF_CHECK_EN: ConfigField = ConfigField::new(48, 29, 0x20000000);
    pub const THCON_PACKER0_REG0_SRC_ADDR_OFFSET: ConfigField = ConfigField::new(49, 0, 0xfffff);
    pub const THCON_PACKER0_REG0_STOCH_RND_EN: ConfigField = ConfigField::new(48, 28, 0x10000000);
    pub const THCON_PACKER0_REG0_UNTILIZE_DST_ADDR_OFFSET: ConfigField =
        ConfigField::new(50, 0, 0xfffff);
    pub const THCON_PACKER0_REG0_UNUSED0: ConfigField = ConfigField::new(48, 30, 0xc0000000);
    pub const THCON_PACKER0_REG0_UNUSED1: ConfigField = ConfigField::new(49, 20, 0xfff00000);
    pub const THCON_PACKER0_REG0_UNUSED2: ConfigField = ConfigField::new(50, 20, 0xfff00000);
    pub const THCON_PACKER0_REG0_WRAP_TILE_DIM: ConfigField = ConfigField::new(51, 0, 0x3ffff);
    pub const THCON_PACKER0_REG0_ZERO_WRITE: ConfigField = ConfigField::new(48, 13, 0x2000);
    pub const THCON_PACKER0_REG1_EDGE_MASK0: ConfigField = ConfigField::new(52, 0, 0xffff);
    pub const THCON_PACKER0_REG1_EDGE_MASK1: ConfigField = ConfigField::new(52, 16, 0xffff0000);
    pub const THCON_PACKER0_REG1_EDGE_MASK2: ConfigField = ConfigField::new(53, 0, 0xffff);
    pub const THCON_PACKER0_REG1_EDGE_MASK3: ConfigField = ConfigField::new(53, 16, 0xffff0000);
    pub const THCON_PACKER0_REG1_EDGE_MASK_MODE: ConfigField = ConfigField::new(54, 0, 0x3);
    pub const THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_0: ConfigField =
        ConfigField::new(55, 0, 0xffff);
    pub const THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_1: ConfigField =
        ConfigField::new(55, 16, 0xffff0000);
    pub const THCON_PACKER0_REG1_PACK_UNTILIZE_DST_Z_STRIDE: ConfigField =
        ConfigField::new(54, 10, 0x3fffc00);
    pub const THCON_PACKER0_REG1_PACK_UNTILIZE_SRC_Z_STRIDE: ConfigField =
        ConfigField::new(54, 2, 0x3fc);
    pub const THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE0: ConfigField =
        ConfigField::new(56, 0, 0xffffffff);
    pub const THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE1: ConfigField =
        ConfigField::new(57, 0, 0xffffffff);
    pub const THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE2: ConfigField =
        ConfigField::new(58, 0, 0xffffffff);
    pub const THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE3: ConfigField =
        ConfigField::new(59, 0, 0xffffffff);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_COL_MASK: ConfigField =
        ConfigField::new(60, 8, 0xffff00);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_MASK_VAL: ConfigField =
        ConfigField::new(62, 0, 0xffffffff);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_NO_WRITE: ConfigField = ConfigField::new(60, 3, 0x8);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_2: ConfigField =
        ConfigField::new(63, 0, 0xffff);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_3: ConfigField =
        ConfigField::new(63, 16, 0xffff0000);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_ROW_MASK: ConfigField = ConfigField::new(60, 4, 0xf0);
    pub const THCON_PACKER0_REG3_PACK_STRIDE_VAL_SOURCE: ConfigField = ConfigField::new(60, 2, 0x4);
    pub const THCON_PACKER0_REG3_RELU_MODE: ConfigField = ConfigField::new(60, 0, 0x3);
    pub const THCON_PACKER0_REG3_RELU_THRESHOLD: ConfigField = ConfigField::new(61, 0, 0xffffffff);
    pub const THCON_PACKER0_REG3_UNUSED0: ConfigField = ConfigField::new(60, 24, 0xff000000);
    pub const THCON_PACKER1_REG0_DISABLE_SRCS_OFFSET: ConfigField =
        ConfigField::new(65, 30, 0x40000000);
    pub const THCON_PACKER1_REG0_EXP_THRESHOLD: ConfigField = ConfigField::new(64, 15, 0x7f8000);
    pub const THCON_PACKER1_REG0_EXP_THRESHOLD_EN: ConfigField = ConfigField::new(64, 14, 0x4000);
    pub const THCON_PACKER1_REG0_INSTRN_COUNT: ConfigField = ConfigField::new(65, 28, 0x30000000);
    pub const THCON_PACKER1_REG0_INSTRN_LOOP_COUNT: ConfigField =
        ConfigField::new(65, 20, 0xff00000);
    pub const THCON_PACKER1_REG0_INT_DESCALE_ENABLE: ConfigField =
        ConfigField::new(64, 23, 0x800000);
    pub const THCON_PACKER1_REG0_INT_DESCALE_MODE: ConfigField =
        ConfigField::new(64, 25, 0xe000000);
    pub const THCON_PACKER1_REG0_IN_DATA_FORMAT: ConfigField = ConfigField::new(64, 0, 0xff);
    pub const THCON_PACKER1_REG0_L1_ACC: ConfigField = ConfigField::new(64, 12, 0x1000);
    pub const THCON_PACKER1_REG0_SRC_ADDR_OFFSET: ConfigField = ConfigField::new(65, 0, 0xfffff);
    pub const THCON_PACKER1_REG0_STOCH_RND_EN: ConfigField = ConfigField::new(64, 28, 0x10000000);
    pub const THCON_PACKER1_REG0_UNUSED0: ConfigField = ConfigField::new(64, 8, 0xf00);
    pub const THCON_PACKER1_REG0_UNUSED1: ConfigField = ConfigField::new(64, 29, 0xe0000000);
    pub const THCON_PACKER1_REG0_UNUSED2: ConfigField = ConfigField::new(65, 31, 0x80000000);
    pub const THCON_PACKER1_REG0_WRAP_TILE_DIM: ConfigField = ConfigField::new(66, 0, 0x3ffff);
    pub const THCON_PACKER1_REG0_ZERO_WRITE: ConfigField = ConfigField::new(64, 13, 0x2000);
    pub const THCON_PACKER1_REG1_EDGE_MASK0: ConfigField = ConfigField::new(68, 0, 0xffff);
    pub const THCON_PACKER1_REG1_EDGE_MASK1: ConfigField = ConfigField::new(68, 16, 0xffff0000);
    pub const THCON_PACKER1_REG1_EDGE_MASK2: ConfigField = ConfigField::new(69, 0, 0xffff);
    pub const THCON_PACKER1_REG1_EDGE_MASK3: ConfigField = ConfigField::new(69, 16, 0xffff0000);
    pub const THCON_PACKER1_REG1_EDGE_MASK_MODE: ConfigField = ConfigField::new(70, 0, 0x3);
    pub const THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_0: ConfigField =
        ConfigField::new(71, 0, 0xffff);
    pub const THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_1: ConfigField =
        ConfigField::new(71, 16, 0xffff0000);
    pub const THCON_PACKER1_REG1_UNUSED0: ConfigField = ConfigField::new(70, 2, 0xfffffffc);
    pub const THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE0: ConfigField =
        ConfigField::new(72, 0, 0xffffffff);
    pub const THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE1: ConfigField =
        ConfigField::new(73, 0, 0xffffffff);
    pub const THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE2: ConfigField =
        ConfigField::new(74, 0, 0xffffffff);
    pub const THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE3: ConfigField =
        ConfigField::new(75, 0, 0xffffffff);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_COL_MASK: ConfigField =
        ConfigField::new(76, 8, 0xffff00);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_MASK_VAL: ConfigField =
        ConfigField::new(78, 0, 0xffffffff);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_NO_WRITE: ConfigField = ConfigField::new(76, 3, 0x8);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_2: ConfigField =
        ConfigField::new(79, 0, 0xffff);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_3: ConfigField =
        ConfigField::new(79, 16, 0xffff0000);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_ROW_MASK: ConfigField = ConfigField::new(76, 4, 0xf0);
    pub const THCON_PACKER1_REG3_PACK_STRIDE_VAL_SOURCE: ConfigField = ConfigField::new(76, 2, 0x4);
    pub const THCON_PACKER1_REG3_RELU_MODE: ConfigField = ConfigField::new(76, 0, 0x3);
    pub const THCON_PACKER1_REG3_RELU_THRESHOLD: ConfigField = ConfigField::new(77, 0, 0xffffffff);
    pub const THCON_PACKER1_REG3_UNUSED0: ConfigField = ConfigField::new(76, 24, 0xff000000);
    pub const THCON_UNPACKER0_REG0_ARG_FIFO_MODE: ConfigField =
        ConfigField::new(12, 30, 0x40000000);
    pub const THCON_UNPACKER0_REG0_ENABLE_ARG_FIFO: ConfigField =
        ConfigField::new(12, 29, 0x20000000);
    pub const THCON_UNPACKER0_REG0_OUT_DATA_FORMAT: ConfigField = ConfigField::new(12, 0, 0xff);
    pub const THCON_UNPACKER0_REG0_SELF_CHECK_EN: ConfigField =
        ConfigField::new(12, 31, 0x80000000);
    pub const THCON_UNPACKER0_REG0_SET_STRIDE_MODE: ConfigField = ConfigField::new(13, 19, 0x80000);
    pub const THCON_UNPACKER0_REG0_TILIZE_SRC_ADDR_OFFSET: ConfigField =
        ConfigField::new(12, 9, 0x1ffffe00);
    pub const THCON_UNPACKER0_REG0_TRANSPOSE: ConfigField = ConfigField::new(12, 8, 0x100);
    pub const THCON_UNPACKER0_REG0_UNPACK_STRIDE_COL_MASK: ConfigField =
        ConfigField::new(14, 0, 0xffff);
    pub const THCON_UNPACKER0_REG0_UNPACR_STRIDE_SET_STRIDE: ConfigField =
        ConfigField::new(13, 18, 0x40000);
    pub const THCON_UNPACKER0_REG0_UNUSED1: ConfigField = ConfigField::new(13, 20, 0xfff00000);
    pub const THCON_UNPACKER0_REG0_WRAP_TILE_DIM: ConfigField = ConfigField::new(13, 0, 0x3ffff);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_MASK_VAL: ConfigField =
        ConfigField::new(19, 0, 0xffffffff);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_NO_WRITE: ConfigField =
        ConfigField::new(18, 25, 0x2000000);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK0: ConfigField =
        ConfigField::new(16, 0, 0xff);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK1: ConfigField =
        ConfigField::new(16, 8, 0xff00);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK2: ConfigField =
        ConfigField::new(16, 16, 0xff0000);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK3: ConfigField =
        ConfigField::new(16, 24, 0xff000000);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK4: ConfigField =
        ConfigField::new(17, 0, 0xff);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK5: ConfigField =
        ConfigField::new(17, 8, 0xff00);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK6: ConfigField =
        ConfigField::new(17, 16, 0xff0000);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK7: ConfigField =
        ConfigField::new(17, 24, 0xff000000);
    pub const THCON_UNPACKER0_REG1_UNPACK_STRIDE_VAL_SOURCE: ConfigField =
        ConfigField::new(18, 24, 0x1000000);
    pub const THCON_UNPACKER0_REG1_UNPACK_TILIZE_DST_Z_STRIDE: ConfigField =
        ConfigField::new(18, 16, 0xff0000);
    pub const THCON_UNPACKER0_REG1_UNPACK_TILIZE_SRC_Z_STRIDE: ConfigField =
        ConfigField::new(18, 0, 0xffff);
    pub const THCON_UNPACKER0_REG1_UNUSED0: ConfigField = ConfigField::new(18, 26, 0xfc000000);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_0: ConfigField =
        ConfigField::new(20, 0, 0xffff);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_1: ConfigField =
        ConfigField::new(20, 16, 0xffff0000);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_2: ConfigField =
        ConfigField::new(21, 0, 0xffff);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_3: ConfigField =
        ConfigField::new(21, 16, 0xffff0000);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_4: ConfigField =
        ConfigField::new(22, 0, 0xffff);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_5: ConfigField =
        ConfigField::new(22, 16, 0xffff0000);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_6: ConfigField =
        ConfigField::new(23, 0, 0xffff);
    pub const THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_7: ConfigField =
        ConfigField::new(23, 16, 0xffff0000);
    pub const THCON_UNPACKER1_REG0_ARG_FIFO_MODE: ConfigField =
        ConfigField::new(24, 30, 0x40000000);
    pub const THCON_UNPACKER1_REG0_ENABLE_ARG_FIFO: ConfigField =
        ConfigField::new(24, 29, 0x20000000);
    pub const THCON_UNPACKER1_REG0_OUT_DATA_FORMAT: ConfigField = ConfigField::new(24, 0, 0xff);
    pub const THCON_UNPACKER1_REG0_SET_STRIDE_MODE: ConfigField = ConfigField::new(25, 19, 0x80000);
    pub const THCON_UNPACKER1_REG0_TILIZE_SRC_ADDR_OFFSET: ConfigField =
        ConfigField::new(24, 9, 0x1ffffe00);
    pub const THCON_UNPACKER1_REG0_TRANSPOSE: ConfigField = ConfigField::new(24, 8, 0x100);
    pub const THCON_UNPACKER1_REG0_UNPACK_STRIDE_COL_MASK: ConfigField =
        ConfigField::new(26, 0, 0xffff);
    pub const THCON_UNPACKER1_REG0_UNPACR_STRIDE_SET_STRIDE: ConfigField =
        ConfigField::new(25, 18, 0x40000);
    pub const THCON_UNPACKER1_REG0_UNUSED0: ConfigField = ConfigField::new(24, 31, 0x80000000);
    pub const THCON_UNPACKER1_REG0_UNUSED1: ConfigField = ConfigField::new(25, 20, 0xfff00000);
    pub const THCON_UNPACKER1_REG0_WRAP_TILE_DIM: ConfigField = ConfigField::new(25, 0, 0x3ffff);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_MASK_VAL: ConfigField =
        ConfigField::new(31, 0, 0xffffffff);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_NO_WRITE: ConfigField =
        ConfigField::new(30, 25, 0x2000000);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK0: ConfigField =
        ConfigField::new(28, 0, 0xff);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK1: ConfigField =
        ConfigField::new(28, 8, 0xff00);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK2: ConfigField =
        ConfigField::new(28, 16, 0xff0000);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK3: ConfigField =
        ConfigField::new(28, 24, 0xff000000);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK4: ConfigField =
        ConfigField::new(29, 0, 0xff);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK5: ConfigField =
        ConfigField::new(29, 8, 0xff00);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK6: ConfigField =
        ConfigField::new(29, 16, 0xff0000);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK7: ConfigField =
        ConfigField::new(29, 24, 0xff000000);
    pub const THCON_UNPACKER1_REG1_UNPACK_STRIDE_VAL_SOURCE: ConfigField =
        ConfigField::new(30, 24, 0x1000000);
    pub const THCON_UNPACKER1_REG1_UNPACK_TILIZE_DST_Z_STRIDE: ConfigField =
        ConfigField::new(30, 16, 0xff0000);
    pub const THCON_UNPACKER1_REG1_UNPACK_TILIZE_SRC_Z_STRIDE: ConfigField =
        ConfigField::new(30, 0, 0xffff);
    pub const THCON_UNPACKER1_REG1_UNUSED0: ConfigField = ConfigField::new(30, 26, 0xfc000000);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_0: ConfigField =
        ConfigField::new(32, 0, 0xffff);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_1: ConfigField =
        ConfigField::new(32, 16, 0xffff0000);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_2: ConfigField =
        ConfigField::new(33, 0, 0xffff);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_3: ConfigField =
        ConfigField::new(33, 16, 0xffff0000);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_4: ConfigField =
        ConfigField::new(34, 0, 0xffff);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_5: ConfigField =
        ConfigField::new(34, 16, 0xffff0000);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_6: ConfigField =
        ConfigField::new(35, 0, 0xffff);
    pub const THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_7: ConfigField =
        ConfigField::new(35, 16, 0xffff0000);
    pub const THCON_UNPACKER2_REG0_ARG_FIFO_MODE: ConfigField = ConfigField::new(36, 10, 0x400);
    pub const THCON_UNPACKER2_REG0_ENABLE_ARG_FIFO: ConfigField = ConfigField::new(36, 9, 0x200);
    pub const THCON_UNPACKER2_REG0_INSTRN_COUNT: ConfigField = ConfigField::new(37, 26, 0xc000000);
    pub const THCON_UNPACKER2_REG0_INSTRN_LOOP_COUNT: ConfigField =
        ConfigField::new(37, 18, 0x3fc0000);
    pub const THCON_UNPACKER2_REG0_OUT_DATA_FORMAT: ConfigField = ConfigField::new(36, 0, 0xff);
    pub const THCON_UNPACKER2_REG0_UNPACK_STRIDE_COL_MASK: ConfigField =
        ConfigField::new(38, 0, 0xffff);
    pub const THCON_UNPACKER2_REG0_UNUSED0: ConfigField = ConfigField::new(36, 8, 0x100);
    pub const THCON_UNPACKER2_REG0_UNUSED1: ConfigField = ConfigField::new(36, 11, 0xfffff800);
    pub const THCON_UNPACKER2_REG0_UNUSED2: ConfigField = ConfigField::new(37, 28, 0xf0000000);
    pub const THCON_UNPACKER2_REG0_WRAP_TILE_DIM: ConfigField = ConfigField::new(37, 0, 0x3ffff);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_MASK_VAL: ConfigField =
        ConfigField::new(43, 0, 0xffffffff);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_NO_WRITE: ConfigField =
        ConfigField::new(42, 1, 0x2);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK0: ConfigField =
        ConfigField::new(40, 0, 0xff);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK1: ConfigField =
        ConfigField::new(40, 8, 0xff00);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK2: ConfigField =
        ConfigField::new(40, 16, 0xff0000);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK3: ConfigField =
        ConfigField::new(40, 24, 0xff000000);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK4: ConfigField =
        ConfigField::new(41, 0, 0xff);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK5: ConfigField =
        ConfigField::new(41, 8, 0xff00);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK6: ConfigField =
        ConfigField::new(41, 16, 0xff0000);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK7: ConfigField =
        ConfigField::new(41, 24, 0xff000000);
    pub const THCON_UNPACKER2_REG1_UNPACK_STRIDE_VAL_SOURCE: ConfigField =
        ConfigField::new(42, 0, 0x1);
    pub const THCON_UNPACKER2_REG1_UNUSED0: ConfigField = ConfigField::new(42, 2, 0x3ffffc);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_0: ConfigField =
        ConfigField::new(44, 0, 0xffff);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_1: ConfigField =
        ConfigField::new(44, 16, 0xffff0000);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_2: ConfigField =
        ConfigField::new(45, 0, 0xffff);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_3: ConfigField =
        ConfigField::new(45, 16, 0xffff0000);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_4: ConfigField =
        ConfigField::new(46, 0, 0xffff);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_5: ConfigField =
        ConfigField::new(46, 16, 0xffff0000);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_6: ConfigField =
        ConfigField::new(47, 0, 0xffff);
    pub const THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_7: ConfigField =
        ConfigField::new(47, 16, 0xffff0000);
}

pub static ALL_CONFIG_FIELDS: &[(&str, ConfigField)] = &[
    ("ALU_ACC_CTRL_Fp32_enabled", alu::ALU_ACC_CTRL_Fp32_enabled),
    (
        "ALU_ACC_CTRL_INT8_math_enabled",
        alu::ALU_ACC_CTRL_INT8_math_enabled,
    ),
    (
        "ALU_ACC_CTRL_SFPU_Fp32_enabled",
        alu::ALU_ACC_CTRL_SFPU_Fp32_enabled,
    ),
    (
        "ALU_ACC_CTRL_Zero_Flag_disabled_dst",
        alu::ALU_ACC_CTRL_Zero_Flag_disabled_dst,
    ),
    (
        "ALU_ACC_CTRL_Zero_Flag_disabled_src",
        alu::ALU_ACC_CTRL_Zero_Flag_disabled_src,
    ),
    ("ALU_FORMAT_SPEC_REG0_SrcA", alu::ALU_FORMAT_SPEC_REG0_SrcA),
    ("ALU_FORMAT_SPEC_REG1_SrcB", alu::ALU_FORMAT_SPEC_REG1_SrcB),
    (
        "ALU_FORMAT_SPEC_REG2_Dstacc",
        alu::ALU_FORMAT_SPEC_REG2_Dstacc,
    ),
    (
        "ALU_FORMAT_SPEC_REG_Dstacc_override",
        alu::ALU_FORMAT_SPEC_REG_Dstacc_override,
    ),
    (
        "ALU_FORMAT_SPEC_REG_Dstacc_val",
        alu::ALU_FORMAT_SPEC_REG_Dstacc_val,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcA_override",
        alu::ALU_FORMAT_SPEC_REG_SrcA_override,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcA_val",
        alu::ALU_FORMAT_SPEC_REG_SrcA_val,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcB_override",
        alu::ALU_FORMAT_SPEC_REG_SrcB_override,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcB_val",
        alu::ALU_FORMAT_SPEC_REG_SrcB_val,
    ),
    ("ALU_ROUNDING_MODE_Bfp8_HF", alu::ALU_ROUNDING_MODE_Bfp8_HF),
    (
        "ALU_ROUNDING_MODE_Fpu_srnd_en",
        alu::ALU_ROUNDING_MODE_Fpu_srnd_en,
    ),
    ("ALU_ROUNDING_MODE_GS_LF", alu::ALU_ROUNDING_MODE_GS_LF),
    ("ALU_ROUNDING_MODE_Padding", alu::ALU_ROUNDING_MODE_Padding),
    ("DEST_REGW_BASE_Base", alu::DEST_REGW_BASE_Base),
    ("DEST_SP_BASE_Base", alu::DEST_SP_BASE_Base),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC0_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC0_Offset,
    ),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC1_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC1_Offset,
    ),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC2_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC2_Offset,
    ),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC3_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC3_Offset,
    ),
    (
        "DISABLE_IMPLIED_SRCS_FORMAT",
        alu::DISABLE_IMPLIED_SRCS_FORMAT,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC0_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC0_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC0_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC0_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC0_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC0_unsigned_int,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC1_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC1_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC1_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC1_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC1_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC1_unsigned_int,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC2_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC2_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC2_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC2_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC2_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC2_unsigned_int,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC3_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC3_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC3_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC3_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC3_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC3_unsigned_int,
    ),
    ("SFPU_SRCS_BANK0_FORMAT", alu::SFPU_SRCS_BANK0_FORMAT),
    ("SFPU_SRCS_BANK1_FORMAT", alu::SFPU_SRCS_BANK1_FORMAT),
    ("STATE_RESET_EN", alu::STATE_RESET_EN),
    ("UNUSED0", alu::UNUSED0),
    ("UNUSED1", alu::UNUSED1),
    ("UNUSED2", alu::UNUSED2),
    (
        "ADDR_MOD_AB2_SEC0_SrcAIncr",
        global::ADDR_MOD_AB2_SEC0_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC0_SrcBIncr",
        global::ADDR_MOD_AB2_SEC0_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC10_SrcAIncr",
        global::ADDR_MOD_AB2_SEC10_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC10_SrcBIncr",
        global::ADDR_MOD_AB2_SEC10_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC11_SrcAIncr",
        global::ADDR_MOD_AB2_SEC11_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC11_SrcBIncr",
        global::ADDR_MOD_AB2_SEC11_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC12_SrcAIncr",
        global::ADDR_MOD_AB2_SEC12_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC12_SrcBIncr",
        global::ADDR_MOD_AB2_SEC12_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC13_SrcAIncr",
        global::ADDR_MOD_AB2_SEC13_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC13_SrcBIncr",
        global::ADDR_MOD_AB2_SEC13_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC14_SrcAIncr",
        global::ADDR_MOD_AB2_SEC14_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC14_SrcBIncr",
        global::ADDR_MOD_AB2_SEC14_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC15_SrcAIncr",
        global::ADDR_MOD_AB2_SEC15_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC15_SrcBIncr",
        global::ADDR_MOD_AB2_SEC15_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC16_SrcAIncr",
        global::ADDR_MOD_AB2_SEC16_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC16_SrcBIncr",
        global::ADDR_MOD_AB2_SEC16_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC17_SrcAIncr",
        global::ADDR_MOD_AB2_SEC17_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC17_SrcBIncr",
        global::ADDR_MOD_AB2_SEC17_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC18_SrcAIncr",
        global::ADDR_MOD_AB2_SEC18_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC18_SrcBIncr",
        global::ADDR_MOD_AB2_SEC18_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC19_SrcAIncr",
        global::ADDR_MOD_AB2_SEC19_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC19_SrcBIncr",
        global::ADDR_MOD_AB2_SEC19_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC1_SrcAIncr",
        global::ADDR_MOD_AB2_SEC1_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC1_SrcBIncr",
        global::ADDR_MOD_AB2_SEC1_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC20_SrcAIncr",
        global::ADDR_MOD_AB2_SEC20_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC20_SrcBIncr",
        global::ADDR_MOD_AB2_SEC20_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC21_SrcAIncr",
        global::ADDR_MOD_AB2_SEC21_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC21_SrcBIncr",
        global::ADDR_MOD_AB2_SEC21_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC22_SrcAIncr",
        global::ADDR_MOD_AB2_SEC22_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC22_SrcBIncr",
        global::ADDR_MOD_AB2_SEC22_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC23_SrcAIncr",
        global::ADDR_MOD_AB2_SEC23_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC23_SrcBIncr",
        global::ADDR_MOD_AB2_SEC23_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC24_SrcAIncr",
        global::ADDR_MOD_AB2_SEC24_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC24_SrcBIncr",
        global::ADDR_MOD_AB2_SEC24_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC25_SrcAIncr",
        global::ADDR_MOD_AB2_SEC25_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC25_SrcBIncr",
        global::ADDR_MOD_AB2_SEC25_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC26_SrcAIncr",
        global::ADDR_MOD_AB2_SEC26_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC26_SrcBIncr",
        global::ADDR_MOD_AB2_SEC26_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC27_SrcAIncr",
        global::ADDR_MOD_AB2_SEC27_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC27_SrcBIncr",
        global::ADDR_MOD_AB2_SEC27_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC28_SrcAIncr",
        global::ADDR_MOD_AB2_SEC28_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC28_SrcBIncr",
        global::ADDR_MOD_AB2_SEC28_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC29_SrcAIncr",
        global::ADDR_MOD_AB2_SEC29_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC29_SrcBIncr",
        global::ADDR_MOD_AB2_SEC29_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC2_SrcAIncr",
        global::ADDR_MOD_AB2_SEC2_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC2_SrcBIncr",
        global::ADDR_MOD_AB2_SEC2_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC30_SrcAIncr",
        global::ADDR_MOD_AB2_SEC30_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC30_SrcBIncr",
        global::ADDR_MOD_AB2_SEC30_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC31_SrcAIncr",
        global::ADDR_MOD_AB2_SEC31_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC31_SrcBIncr",
        global::ADDR_MOD_AB2_SEC31_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC3_SrcAIncr",
        global::ADDR_MOD_AB2_SEC3_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC3_SrcBIncr",
        global::ADDR_MOD_AB2_SEC3_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC4_SrcAIncr",
        global::ADDR_MOD_AB2_SEC4_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC4_SrcBIncr",
        global::ADDR_MOD_AB2_SEC4_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC5_SrcAIncr",
        global::ADDR_MOD_AB2_SEC5_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC5_SrcBIncr",
        global::ADDR_MOD_AB2_SEC5_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC6_SrcAIncr",
        global::ADDR_MOD_AB2_SEC6_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC6_SrcBIncr",
        global::ADDR_MOD_AB2_SEC6_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC7_SrcAIncr",
        global::ADDR_MOD_AB2_SEC7_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC7_SrcBIncr",
        global::ADDR_MOD_AB2_SEC7_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC8_SrcAIncr",
        global::ADDR_MOD_AB2_SEC8_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC8_SrcBIncr",
        global::ADDR_MOD_AB2_SEC8_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC9_SrcAIncr",
        global::ADDR_MOD_AB2_SEC9_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC9_SrcBIncr",
        global::ADDR_MOD_AB2_SEC9_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC0_SrcACR", global::ADDR_MOD_AB_SEC0_SrcACR),
    (
        "ADDR_MOD_AB_SEC0_SrcAClear",
        global::ADDR_MOD_AB_SEC0_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC0_SrcAIncr",
        global::ADDR_MOD_AB_SEC0_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC0_SrcBCR", global::ADDR_MOD_AB_SEC0_SrcBCR),
    (
        "ADDR_MOD_AB_SEC0_SrcBClear",
        global::ADDR_MOD_AB_SEC0_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC0_SrcBIncr",
        global::ADDR_MOD_AB_SEC0_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC10_SrcACR", global::ADDR_MOD_AB_SEC10_SrcACR),
    (
        "ADDR_MOD_AB_SEC10_SrcAClear",
        global::ADDR_MOD_AB_SEC10_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC10_SrcAIncr",
        global::ADDR_MOD_AB_SEC10_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC10_SrcBCR", global::ADDR_MOD_AB_SEC10_SrcBCR),
    (
        "ADDR_MOD_AB_SEC10_SrcBClear",
        global::ADDR_MOD_AB_SEC10_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC10_SrcBIncr",
        global::ADDR_MOD_AB_SEC10_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC11_SrcACR", global::ADDR_MOD_AB_SEC11_SrcACR),
    (
        "ADDR_MOD_AB_SEC11_SrcAClear",
        global::ADDR_MOD_AB_SEC11_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC11_SrcAIncr",
        global::ADDR_MOD_AB_SEC11_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC11_SrcBCR", global::ADDR_MOD_AB_SEC11_SrcBCR),
    (
        "ADDR_MOD_AB_SEC11_SrcBClear",
        global::ADDR_MOD_AB_SEC11_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC11_SrcBIncr",
        global::ADDR_MOD_AB_SEC11_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC12_SrcACR", global::ADDR_MOD_AB_SEC12_SrcACR),
    (
        "ADDR_MOD_AB_SEC12_SrcAClear",
        global::ADDR_MOD_AB_SEC12_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC12_SrcAIncr",
        global::ADDR_MOD_AB_SEC12_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC12_SrcBCR", global::ADDR_MOD_AB_SEC12_SrcBCR),
    (
        "ADDR_MOD_AB_SEC12_SrcBClear",
        global::ADDR_MOD_AB_SEC12_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC12_SrcBIncr",
        global::ADDR_MOD_AB_SEC12_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC13_SrcACR", global::ADDR_MOD_AB_SEC13_SrcACR),
    (
        "ADDR_MOD_AB_SEC13_SrcAClear",
        global::ADDR_MOD_AB_SEC13_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC13_SrcAIncr",
        global::ADDR_MOD_AB_SEC13_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC13_SrcBCR", global::ADDR_MOD_AB_SEC13_SrcBCR),
    (
        "ADDR_MOD_AB_SEC13_SrcBClear",
        global::ADDR_MOD_AB_SEC13_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC13_SrcBIncr",
        global::ADDR_MOD_AB_SEC13_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC14_SrcACR", global::ADDR_MOD_AB_SEC14_SrcACR),
    (
        "ADDR_MOD_AB_SEC14_SrcAClear",
        global::ADDR_MOD_AB_SEC14_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC14_SrcAIncr",
        global::ADDR_MOD_AB_SEC14_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC14_SrcBCR", global::ADDR_MOD_AB_SEC14_SrcBCR),
    (
        "ADDR_MOD_AB_SEC14_SrcBClear",
        global::ADDR_MOD_AB_SEC14_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC14_SrcBIncr",
        global::ADDR_MOD_AB_SEC14_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC15_SrcACR", global::ADDR_MOD_AB_SEC15_SrcACR),
    (
        "ADDR_MOD_AB_SEC15_SrcAClear",
        global::ADDR_MOD_AB_SEC15_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC15_SrcAIncr",
        global::ADDR_MOD_AB_SEC15_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC15_SrcBCR", global::ADDR_MOD_AB_SEC15_SrcBCR),
    (
        "ADDR_MOD_AB_SEC15_SrcBClear",
        global::ADDR_MOD_AB_SEC15_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC15_SrcBIncr",
        global::ADDR_MOD_AB_SEC15_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC16_SrcACR", global::ADDR_MOD_AB_SEC16_SrcACR),
    (
        "ADDR_MOD_AB_SEC16_SrcAClear",
        global::ADDR_MOD_AB_SEC16_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC16_SrcAIncr",
        global::ADDR_MOD_AB_SEC16_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC16_SrcBCR", global::ADDR_MOD_AB_SEC16_SrcBCR),
    (
        "ADDR_MOD_AB_SEC16_SrcBClear",
        global::ADDR_MOD_AB_SEC16_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC16_SrcBIncr",
        global::ADDR_MOD_AB_SEC16_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC17_SrcACR", global::ADDR_MOD_AB_SEC17_SrcACR),
    (
        "ADDR_MOD_AB_SEC17_SrcAClear",
        global::ADDR_MOD_AB_SEC17_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC17_SrcAIncr",
        global::ADDR_MOD_AB_SEC17_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC17_SrcBCR", global::ADDR_MOD_AB_SEC17_SrcBCR),
    (
        "ADDR_MOD_AB_SEC17_SrcBClear",
        global::ADDR_MOD_AB_SEC17_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC17_SrcBIncr",
        global::ADDR_MOD_AB_SEC17_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC18_SrcACR", global::ADDR_MOD_AB_SEC18_SrcACR),
    (
        "ADDR_MOD_AB_SEC18_SrcAClear",
        global::ADDR_MOD_AB_SEC18_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC18_SrcAIncr",
        global::ADDR_MOD_AB_SEC18_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC18_SrcBCR", global::ADDR_MOD_AB_SEC18_SrcBCR),
    (
        "ADDR_MOD_AB_SEC18_SrcBClear",
        global::ADDR_MOD_AB_SEC18_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC18_SrcBIncr",
        global::ADDR_MOD_AB_SEC18_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC19_SrcACR", global::ADDR_MOD_AB_SEC19_SrcACR),
    (
        "ADDR_MOD_AB_SEC19_SrcAClear",
        global::ADDR_MOD_AB_SEC19_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC19_SrcAIncr",
        global::ADDR_MOD_AB_SEC19_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC19_SrcBCR", global::ADDR_MOD_AB_SEC19_SrcBCR),
    (
        "ADDR_MOD_AB_SEC19_SrcBClear",
        global::ADDR_MOD_AB_SEC19_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC19_SrcBIncr",
        global::ADDR_MOD_AB_SEC19_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC1_SrcACR", global::ADDR_MOD_AB_SEC1_SrcACR),
    (
        "ADDR_MOD_AB_SEC1_SrcAClear",
        global::ADDR_MOD_AB_SEC1_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC1_SrcAIncr",
        global::ADDR_MOD_AB_SEC1_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC1_SrcBCR", global::ADDR_MOD_AB_SEC1_SrcBCR),
    (
        "ADDR_MOD_AB_SEC1_SrcBClear",
        global::ADDR_MOD_AB_SEC1_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC1_SrcBIncr",
        global::ADDR_MOD_AB_SEC1_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC20_SrcACR", global::ADDR_MOD_AB_SEC20_SrcACR),
    (
        "ADDR_MOD_AB_SEC20_SrcAClear",
        global::ADDR_MOD_AB_SEC20_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC20_SrcAIncr",
        global::ADDR_MOD_AB_SEC20_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC20_SrcBCR", global::ADDR_MOD_AB_SEC20_SrcBCR),
    (
        "ADDR_MOD_AB_SEC20_SrcBClear",
        global::ADDR_MOD_AB_SEC20_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC20_SrcBIncr",
        global::ADDR_MOD_AB_SEC20_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC21_SrcACR", global::ADDR_MOD_AB_SEC21_SrcACR),
    (
        "ADDR_MOD_AB_SEC21_SrcAClear",
        global::ADDR_MOD_AB_SEC21_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC21_SrcAIncr",
        global::ADDR_MOD_AB_SEC21_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC21_SrcBCR", global::ADDR_MOD_AB_SEC21_SrcBCR),
    (
        "ADDR_MOD_AB_SEC21_SrcBClear",
        global::ADDR_MOD_AB_SEC21_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC21_SrcBIncr",
        global::ADDR_MOD_AB_SEC21_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC22_SrcACR", global::ADDR_MOD_AB_SEC22_SrcACR),
    (
        "ADDR_MOD_AB_SEC22_SrcAClear",
        global::ADDR_MOD_AB_SEC22_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC22_SrcAIncr",
        global::ADDR_MOD_AB_SEC22_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC22_SrcBCR", global::ADDR_MOD_AB_SEC22_SrcBCR),
    (
        "ADDR_MOD_AB_SEC22_SrcBClear",
        global::ADDR_MOD_AB_SEC22_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC22_SrcBIncr",
        global::ADDR_MOD_AB_SEC22_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC23_SrcACR", global::ADDR_MOD_AB_SEC23_SrcACR),
    (
        "ADDR_MOD_AB_SEC23_SrcAClear",
        global::ADDR_MOD_AB_SEC23_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC23_SrcAIncr",
        global::ADDR_MOD_AB_SEC23_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC23_SrcBCR", global::ADDR_MOD_AB_SEC23_SrcBCR),
    (
        "ADDR_MOD_AB_SEC23_SrcBClear",
        global::ADDR_MOD_AB_SEC23_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC23_SrcBIncr",
        global::ADDR_MOD_AB_SEC23_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC24_SrcACR", global::ADDR_MOD_AB_SEC24_SrcACR),
    (
        "ADDR_MOD_AB_SEC24_SrcAClear",
        global::ADDR_MOD_AB_SEC24_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC24_SrcAIncr",
        global::ADDR_MOD_AB_SEC24_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC24_SrcBCR", global::ADDR_MOD_AB_SEC24_SrcBCR),
    (
        "ADDR_MOD_AB_SEC24_SrcBClear",
        global::ADDR_MOD_AB_SEC24_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC24_SrcBIncr",
        global::ADDR_MOD_AB_SEC24_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC25_SrcACR", global::ADDR_MOD_AB_SEC25_SrcACR),
    (
        "ADDR_MOD_AB_SEC25_SrcAClear",
        global::ADDR_MOD_AB_SEC25_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC25_SrcAIncr",
        global::ADDR_MOD_AB_SEC25_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC25_SrcBCR", global::ADDR_MOD_AB_SEC25_SrcBCR),
    (
        "ADDR_MOD_AB_SEC25_SrcBClear",
        global::ADDR_MOD_AB_SEC25_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC25_SrcBIncr",
        global::ADDR_MOD_AB_SEC25_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC26_SrcACR", global::ADDR_MOD_AB_SEC26_SrcACR),
    (
        "ADDR_MOD_AB_SEC26_SrcAClear",
        global::ADDR_MOD_AB_SEC26_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC26_SrcAIncr",
        global::ADDR_MOD_AB_SEC26_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC26_SrcBCR", global::ADDR_MOD_AB_SEC26_SrcBCR),
    (
        "ADDR_MOD_AB_SEC26_SrcBClear",
        global::ADDR_MOD_AB_SEC26_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC26_SrcBIncr",
        global::ADDR_MOD_AB_SEC26_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC27_SrcACR", global::ADDR_MOD_AB_SEC27_SrcACR),
    (
        "ADDR_MOD_AB_SEC27_SrcAClear",
        global::ADDR_MOD_AB_SEC27_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC27_SrcAIncr",
        global::ADDR_MOD_AB_SEC27_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC27_SrcBCR", global::ADDR_MOD_AB_SEC27_SrcBCR),
    (
        "ADDR_MOD_AB_SEC27_SrcBClear",
        global::ADDR_MOD_AB_SEC27_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC27_SrcBIncr",
        global::ADDR_MOD_AB_SEC27_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC28_SrcACR", global::ADDR_MOD_AB_SEC28_SrcACR),
    (
        "ADDR_MOD_AB_SEC28_SrcAClear",
        global::ADDR_MOD_AB_SEC28_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC28_SrcAIncr",
        global::ADDR_MOD_AB_SEC28_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC28_SrcBCR", global::ADDR_MOD_AB_SEC28_SrcBCR),
    (
        "ADDR_MOD_AB_SEC28_SrcBClear",
        global::ADDR_MOD_AB_SEC28_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC28_SrcBIncr",
        global::ADDR_MOD_AB_SEC28_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC29_SrcACR", global::ADDR_MOD_AB_SEC29_SrcACR),
    (
        "ADDR_MOD_AB_SEC29_SrcAClear",
        global::ADDR_MOD_AB_SEC29_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC29_SrcAIncr",
        global::ADDR_MOD_AB_SEC29_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC29_SrcBCR", global::ADDR_MOD_AB_SEC29_SrcBCR),
    (
        "ADDR_MOD_AB_SEC29_SrcBClear",
        global::ADDR_MOD_AB_SEC29_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC29_SrcBIncr",
        global::ADDR_MOD_AB_SEC29_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC2_SrcACR", global::ADDR_MOD_AB_SEC2_SrcACR),
    (
        "ADDR_MOD_AB_SEC2_SrcAClear",
        global::ADDR_MOD_AB_SEC2_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC2_SrcAIncr",
        global::ADDR_MOD_AB_SEC2_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC2_SrcBCR", global::ADDR_MOD_AB_SEC2_SrcBCR),
    (
        "ADDR_MOD_AB_SEC2_SrcBClear",
        global::ADDR_MOD_AB_SEC2_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC2_SrcBIncr",
        global::ADDR_MOD_AB_SEC2_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC30_SrcACR", global::ADDR_MOD_AB_SEC30_SrcACR),
    (
        "ADDR_MOD_AB_SEC30_SrcAClear",
        global::ADDR_MOD_AB_SEC30_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC30_SrcAIncr",
        global::ADDR_MOD_AB_SEC30_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC30_SrcBCR", global::ADDR_MOD_AB_SEC30_SrcBCR),
    (
        "ADDR_MOD_AB_SEC30_SrcBClear",
        global::ADDR_MOD_AB_SEC30_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC30_SrcBIncr",
        global::ADDR_MOD_AB_SEC30_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC31_SrcACR", global::ADDR_MOD_AB_SEC31_SrcACR),
    (
        "ADDR_MOD_AB_SEC31_SrcAClear",
        global::ADDR_MOD_AB_SEC31_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC31_SrcAIncr",
        global::ADDR_MOD_AB_SEC31_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC31_SrcBCR", global::ADDR_MOD_AB_SEC31_SrcBCR),
    (
        "ADDR_MOD_AB_SEC31_SrcBClear",
        global::ADDR_MOD_AB_SEC31_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC31_SrcBIncr",
        global::ADDR_MOD_AB_SEC31_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC3_SrcACR", global::ADDR_MOD_AB_SEC3_SrcACR),
    (
        "ADDR_MOD_AB_SEC3_SrcAClear",
        global::ADDR_MOD_AB_SEC3_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC3_SrcAIncr",
        global::ADDR_MOD_AB_SEC3_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC3_SrcBCR", global::ADDR_MOD_AB_SEC3_SrcBCR),
    (
        "ADDR_MOD_AB_SEC3_SrcBClear",
        global::ADDR_MOD_AB_SEC3_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC3_SrcBIncr",
        global::ADDR_MOD_AB_SEC3_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC4_SrcACR", global::ADDR_MOD_AB_SEC4_SrcACR),
    (
        "ADDR_MOD_AB_SEC4_SrcAClear",
        global::ADDR_MOD_AB_SEC4_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC4_SrcAIncr",
        global::ADDR_MOD_AB_SEC4_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC4_SrcBCR", global::ADDR_MOD_AB_SEC4_SrcBCR),
    (
        "ADDR_MOD_AB_SEC4_SrcBClear",
        global::ADDR_MOD_AB_SEC4_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC4_SrcBIncr",
        global::ADDR_MOD_AB_SEC4_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC5_SrcACR", global::ADDR_MOD_AB_SEC5_SrcACR),
    (
        "ADDR_MOD_AB_SEC5_SrcAClear",
        global::ADDR_MOD_AB_SEC5_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC5_SrcAIncr",
        global::ADDR_MOD_AB_SEC5_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC5_SrcBCR", global::ADDR_MOD_AB_SEC5_SrcBCR),
    (
        "ADDR_MOD_AB_SEC5_SrcBClear",
        global::ADDR_MOD_AB_SEC5_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC5_SrcBIncr",
        global::ADDR_MOD_AB_SEC5_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC6_SrcACR", global::ADDR_MOD_AB_SEC6_SrcACR),
    (
        "ADDR_MOD_AB_SEC6_SrcAClear",
        global::ADDR_MOD_AB_SEC6_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC6_SrcAIncr",
        global::ADDR_MOD_AB_SEC6_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC6_SrcBCR", global::ADDR_MOD_AB_SEC6_SrcBCR),
    (
        "ADDR_MOD_AB_SEC6_SrcBClear",
        global::ADDR_MOD_AB_SEC6_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC6_SrcBIncr",
        global::ADDR_MOD_AB_SEC6_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC7_SrcACR", global::ADDR_MOD_AB_SEC7_SrcACR),
    (
        "ADDR_MOD_AB_SEC7_SrcAClear",
        global::ADDR_MOD_AB_SEC7_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC7_SrcAIncr",
        global::ADDR_MOD_AB_SEC7_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC7_SrcBCR", global::ADDR_MOD_AB_SEC7_SrcBCR),
    (
        "ADDR_MOD_AB_SEC7_SrcBClear",
        global::ADDR_MOD_AB_SEC7_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC7_SrcBIncr",
        global::ADDR_MOD_AB_SEC7_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC8_SrcACR", global::ADDR_MOD_AB_SEC8_SrcACR),
    (
        "ADDR_MOD_AB_SEC8_SrcAClear",
        global::ADDR_MOD_AB_SEC8_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC8_SrcAIncr",
        global::ADDR_MOD_AB_SEC8_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC8_SrcBCR", global::ADDR_MOD_AB_SEC8_SrcBCR),
    (
        "ADDR_MOD_AB_SEC8_SrcBClear",
        global::ADDR_MOD_AB_SEC8_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC8_SrcBIncr",
        global::ADDR_MOD_AB_SEC8_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC9_SrcACR", global::ADDR_MOD_AB_SEC9_SrcACR),
    (
        "ADDR_MOD_AB_SEC9_SrcAClear",
        global::ADDR_MOD_AB_SEC9_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC9_SrcAIncr",
        global::ADDR_MOD_AB_SEC9_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC9_SrcBCR", global::ADDR_MOD_AB_SEC9_SrcBCR),
    (
        "ADDR_MOD_AB_SEC9_SrcBClear",
        global::ADDR_MOD_AB_SEC9_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC9_SrcBIncr",
        global::ADDR_MOD_AB_SEC9_SrcBIncr,
    ),
    ("ADDR_MOD_DST_SEC0_DestCR", global::ADDR_MOD_DST_SEC0_DestCR),
    (
        "ADDR_MOD_DST_SEC0_DestCToCR",
        global::ADDR_MOD_DST_SEC0_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC0_DestClear",
        global::ADDR_MOD_DST_SEC0_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC0_DestIncr",
        global::ADDR_MOD_DST_SEC0_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC0_FidelityClear",
        global::ADDR_MOD_DST_SEC0_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC0_FidelityIncr",
        global::ADDR_MOD_DST_SEC0_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestCR",
        global::ADDR_MOD_DST_SEC10_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestCToCR",
        global::ADDR_MOD_DST_SEC10_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestClear",
        global::ADDR_MOD_DST_SEC10_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestIncr",
        global::ADDR_MOD_DST_SEC10_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC10_FidelityClear",
        global::ADDR_MOD_DST_SEC10_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC10_FidelityIncr",
        global::ADDR_MOD_DST_SEC10_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestCR",
        global::ADDR_MOD_DST_SEC11_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestCToCR",
        global::ADDR_MOD_DST_SEC11_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestClear",
        global::ADDR_MOD_DST_SEC11_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestIncr",
        global::ADDR_MOD_DST_SEC11_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC11_FidelityClear",
        global::ADDR_MOD_DST_SEC11_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC11_FidelityIncr",
        global::ADDR_MOD_DST_SEC11_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestCR",
        global::ADDR_MOD_DST_SEC12_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestCToCR",
        global::ADDR_MOD_DST_SEC12_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestClear",
        global::ADDR_MOD_DST_SEC12_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestIncr",
        global::ADDR_MOD_DST_SEC12_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC12_FidelityClear",
        global::ADDR_MOD_DST_SEC12_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC12_FidelityIncr",
        global::ADDR_MOD_DST_SEC12_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestCR",
        global::ADDR_MOD_DST_SEC13_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestCToCR",
        global::ADDR_MOD_DST_SEC13_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestClear",
        global::ADDR_MOD_DST_SEC13_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestIncr",
        global::ADDR_MOD_DST_SEC13_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC13_FidelityClear",
        global::ADDR_MOD_DST_SEC13_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC13_FidelityIncr",
        global::ADDR_MOD_DST_SEC13_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestCR",
        global::ADDR_MOD_DST_SEC14_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestCToCR",
        global::ADDR_MOD_DST_SEC14_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestClear",
        global::ADDR_MOD_DST_SEC14_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestIncr",
        global::ADDR_MOD_DST_SEC14_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC14_FidelityClear",
        global::ADDR_MOD_DST_SEC14_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC14_FidelityIncr",
        global::ADDR_MOD_DST_SEC14_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestCR",
        global::ADDR_MOD_DST_SEC15_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestCToCR",
        global::ADDR_MOD_DST_SEC15_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestClear",
        global::ADDR_MOD_DST_SEC15_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestIncr",
        global::ADDR_MOD_DST_SEC15_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC15_FidelityClear",
        global::ADDR_MOD_DST_SEC15_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC15_FidelityIncr",
        global::ADDR_MOD_DST_SEC15_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestCR",
        global::ADDR_MOD_DST_SEC16_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestCToCR",
        global::ADDR_MOD_DST_SEC16_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestClear",
        global::ADDR_MOD_DST_SEC16_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestIncr",
        global::ADDR_MOD_DST_SEC16_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC16_FidelityClear",
        global::ADDR_MOD_DST_SEC16_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC16_FidelityIncr",
        global::ADDR_MOD_DST_SEC16_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestCR",
        global::ADDR_MOD_DST_SEC17_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestCToCR",
        global::ADDR_MOD_DST_SEC17_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestClear",
        global::ADDR_MOD_DST_SEC17_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestIncr",
        global::ADDR_MOD_DST_SEC17_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC17_FidelityClear",
        global::ADDR_MOD_DST_SEC17_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC17_FidelityIncr",
        global::ADDR_MOD_DST_SEC17_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestCR",
        global::ADDR_MOD_DST_SEC18_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestCToCR",
        global::ADDR_MOD_DST_SEC18_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestClear",
        global::ADDR_MOD_DST_SEC18_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestIncr",
        global::ADDR_MOD_DST_SEC18_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC18_FidelityClear",
        global::ADDR_MOD_DST_SEC18_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC18_FidelityIncr",
        global::ADDR_MOD_DST_SEC18_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestCR",
        global::ADDR_MOD_DST_SEC19_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestCToCR",
        global::ADDR_MOD_DST_SEC19_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestClear",
        global::ADDR_MOD_DST_SEC19_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestIncr",
        global::ADDR_MOD_DST_SEC19_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC19_FidelityClear",
        global::ADDR_MOD_DST_SEC19_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC19_FidelityIncr",
        global::ADDR_MOD_DST_SEC19_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC1_DestCR", global::ADDR_MOD_DST_SEC1_DestCR),
    (
        "ADDR_MOD_DST_SEC1_DestCToCR",
        global::ADDR_MOD_DST_SEC1_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC1_DestClear",
        global::ADDR_MOD_DST_SEC1_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC1_DestIncr",
        global::ADDR_MOD_DST_SEC1_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC1_FidelityClear",
        global::ADDR_MOD_DST_SEC1_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC1_FidelityIncr",
        global::ADDR_MOD_DST_SEC1_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestCR",
        global::ADDR_MOD_DST_SEC20_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestCToCR",
        global::ADDR_MOD_DST_SEC20_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestClear",
        global::ADDR_MOD_DST_SEC20_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestIncr",
        global::ADDR_MOD_DST_SEC20_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC20_FidelityClear",
        global::ADDR_MOD_DST_SEC20_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC20_FidelityIncr",
        global::ADDR_MOD_DST_SEC20_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestCR",
        global::ADDR_MOD_DST_SEC21_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestCToCR",
        global::ADDR_MOD_DST_SEC21_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestClear",
        global::ADDR_MOD_DST_SEC21_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestIncr",
        global::ADDR_MOD_DST_SEC21_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC21_FidelityClear",
        global::ADDR_MOD_DST_SEC21_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC21_FidelityIncr",
        global::ADDR_MOD_DST_SEC21_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestCR",
        global::ADDR_MOD_DST_SEC22_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestCToCR",
        global::ADDR_MOD_DST_SEC22_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestClear",
        global::ADDR_MOD_DST_SEC22_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestIncr",
        global::ADDR_MOD_DST_SEC22_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC22_FidelityClear",
        global::ADDR_MOD_DST_SEC22_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC22_FidelityIncr",
        global::ADDR_MOD_DST_SEC22_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestCR",
        global::ADDR_MOD_DST_SEC23_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestCToCR",
        global::ADDR_MOD_DST_SEC23_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestClear",
        global::ADDR_MOD_DST_SEC23_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestIncr",
        global::ADDR_MOD_DST_SEC23_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC23_FidelityClear",
        global::ADDR_MOD_DST_SEC23_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC23_FidelityIncr",
        global::ADDR_MOD_DST_SEC23_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestCR",
        global::ADDR_MOD_DST_SEC24_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestCToCR",
        global::ADDR_MOD_DST_SEC24_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestClear",
        global::ADDR_MOD_DST_SEC24_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestIncr",
        global::ADDR_MOD_DST_SEC24_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC24_FidelityClear",
        global::ADDR_MOD_DST_SEC24_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC24_FidelityIncr",
        global::ADDR_MOD_DST_SEC24_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestCR",
        global::ADDR_MOD_DST_SEC25_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestCToCR",
        global::ADDR_MOD_DST_SEC25_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestClear",
        global::ADDR_MOD_DST_SEC25_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestIncr",
        global::ADDR_MOD_DST_SEC25_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC25_FidelityClear",
        global::ADDR_MOD_DST_SEC25_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC25_FidelityIncr",
        global::ADDR_MOD_DST_SEC25_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestCR",
        global::ADDR_MOD_DST_SEC26_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestCToCR",
        global::ADDR_MOD_DST_SEC26_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestClear",
        global::ADDR_MOD_DST_SEC26_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestIncr",
        global::ADDR_MOD_DST_SEC26_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC26_FidelityClear",
        global::ADDR_MOD_DST_SEC26_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC26_FidelityIncr",
        global::ADDR_MOD_DST_SEC26_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestCR",
        global::ADDR_MOD_DST_SEC27_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestCToCR",
        global::ADDR_MOD_DST_SEC27_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestClear",
        global::ADDR_MOD_DST_SEC27_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestIncr",
        global::ADDR_MOD_DST_SEC27_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC27_FidelityClear",
        global::ADDR_MOD_DST_SEC27_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC27_FidelityIncr",
        global::ADDR_MOD_DST_SEC27_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestCR",
        global::ADDR_MOD_DST_SEC28_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestCToCR",
        global::ADDR_MOD_DST_SEC28_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestClear",
        global::ADDR_MOD_DST_SEC28_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestIncr",
        global::ADDR_MOD_DST_SEC28_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC28_FidelityClear",
        global::ADDR_MOD_DST_SEC28_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC28_FidelityIncr",
        global::ADDR_MOD_DST_SEC28_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestCR",
        global::ADDR_MOD_DST_SEC29_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestCToCR",
        global::ADDR_MOD_DST_SEC29_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestClear",
        global::ADDR_MOD_DST_SEC29_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestIncr",
        global::ADDR_MOD_DST_SEC29_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC29_FidelityClear",
        global::ADDR_MOD_DST_SEC29_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC29_FidelityIncr",
        global::ADDR_MOD_DST_SEC29_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC2_DestCR", global::ADDR_MOD_DST_SEC2_DestCR),
    (
        "ADDR_MOD_DST_SEC2_DestCToCR",
        global::ADDR_MOD_DST_SEC2_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC2_DestClear",
        global::ADDR_MOD_DST_SEC2_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC2_DestIncr",
        global::ADDR_MOD_DST_SEC2_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC2_FidelityClear",
        global::ADDR_MOD_DST_SEC2_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC2_FidelityIncr",
        global::ADDR_MOD_DST_SEC2_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestCR",
        global::ADDR_MOD_DST_SEC30_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestCToCR",
        global::ADDR_MOD_DST_SEC30_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestClear",
        global::ADDR_MOD_DST_SEC30_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestIncr",
        global::ADDR_MOD_DST_SEC30_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC30_FidelityClear",
        global::ADDR_MOD_DST_SEC30_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC30_FidelityIncr",
        global::ADDR_MOD_DST_SEC30_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestCR",
        global::ADDR_MOD_DST_SEC31_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestCToCR",
        global::ADDR_MOD_DST_SEC31_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestClear",
        global::ADDR_MOD_DST_SEC31_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestIncr",
        global::ADDR_MOD_DST_SEC31_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC31_FidelityClear",
        global::ADDR_MOD_DST_SEC31_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC31_FidelityIncr",
        global::ADDR_MOD_DST_SEC31_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC3_DestCR", global::ADDR_MOD_DST_SEC3_DestCR),
    (
        "ADDR_MOD_DST_SEC3_DestCToCR",
        global::ADDR_MOD_DST_SEC3_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC3_DestClear",
        global::ADDR_MOD_DST_SEC3_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC3_DestIncr",
        global::ADDR_MOD_DST_SEC3_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC3_FidelityClear",
        global::ADDR_MOD_DST_SEC3_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC3_FidelityIncr",
        global::ADDR_MOD_DST_SEC3_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC4_DestCR", global::ADDR_MOD_DST_SEC4_DestCR),
    (
        "ADDR_MOD_DST_SEC4_DestCToCR",
        global::ADDR_MOD_DST_SEC4_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC4_DestClear",
        global::ADDR_MOD_DST_SEC4_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC4_DestIncr",
        global::ADDR_MOD_DST_SEC4_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC4_FidelityClear",
        global::ADDR_MOD_DST_SEC4_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC4_FidelityIncr",
        global::ADDR_MOD_DST_SEC4_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC5_DestCR", global::ADDR_MOD_DST_SEC5_DestCR),
    (
        "ADDR_MOD_DST_SEC5_DestCToCR",
        global::ADDR_MOD_DST_SEC5_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC5_DestClear",
        global::ADDR_MOD_DST_SEC5_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC5_DestIncr",
        global::ADDR_MOD_DST_SEC5_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC5_FidelityClear",
        global::ADDR_MOD_DST_SEC5_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC5_FidelityIncr",
        global::ADDR_MOD_DST_SEC5_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC6_DestCR", global::ADDR_MOD_DST_SEC6_DestCR),
    (
        "ADDR_MOD_DST_SEC6_DestCToCR",
        global::ADDR_MOD_DST_SEC6_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC6_DestClear",
        global::ADDR_MOD_DST_SEC6_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC6_DestIncr",
        global::ADDR_MOD_DST_SEC6_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC6_FidelityClear",
        global::ADDR_MOD_DST_SEC6_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC6_FidelityIncr",
        global::ADDR_MOD_DST_SEC6_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC7_DestCR", global::ADDR_MOD_DST_SEC7_DestCR),
    (
        "ADDR_MOD_DST_SEC7_DestCToCR",
        global::ADDR_MOD_DST_SEC7_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC7_DestClear",
        global::ADDR_MOD_DST_SEC7_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC7_DestIncr",
        global::ADDR_MOD_DST_SEC7_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC7_FidelityClear",
        global::ADDR_MOD_DST_SEC7_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC7_FidelityIncr",
        global::ADDR_MOD_DST_SEC7_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC8_DestCR", global::ADDR_MOD_DST_SEC8_DestCR),
    (
        "ADDR_MOD_DST_SEC8_DestCToCR",
        global::ADDR_MOD_DST_SEC8_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC8_DestClear",
        global::ADDR_MOD_DST_SEC8_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC8_DestIncr",
        global::ADDR_MOD_DST_SEC8_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC8_FidelityClear",
        global::ADDR_MOD_DST_SEC8_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC8_FidelityIncr",
        global::ADDR_MOD_DST_SEC8_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC9_DestCR", global::ADDR_MOD_DST_SEC9_DestCR),
    (
        "ADDR_MOD_DST_SEC9_DestCToCR",
        global::ADDR_MOD_DST_SEC9_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC9_DestClear",
        global::ADDR_MOD_DST_SEC9_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC9_DestIncr",
        global::ADDR_MOD_DST_SEC9_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC9_FidelityClear",
        global::ADDR_MOD_DST_SEC9_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC9_FidelityIncr",
        global::ADDR_MOD_DST_SEC9_FidelityIncr,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED3,
    ),
    ("CFG_WATCHPOINTS_0_enable", global::CFG_WATCHPOINTS_0_enable),
    (
        "CFG_WATCHPOINTS_0_hi_addr",
        global::CFG_WATCHPOINTS_0_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_0_lo_addr",
        global::CFG_WATCHPOINTS_0_lo_addr,
    ),
    ("CFG_WATCHPOINTS_0_rsvd", global::CFG_WATCHPOINTS_0_rsvd),
    ("CFG_WATCHPOINTS_1_enable", global::CFG_WATCHPOINTS_1_enable),
    (
        "CFG_WATCHPOINTS_1_hi_addr",
        global::CFG_WATCHPOINTS_1_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_1_lo_addr",
        global::CFG_WATCHPOINTS_1_lo_addr,
    ),
    ("CFG_WATCHPOINTS_1_rsvd", global::CFG_WATCHPOINTS_1_rsvd),
    ("CFG_WATCHPOINTS_2_enable", global::CFG_WATCHPOINTS_2_enable),
    (
        "CFG_WATCHPOINTS_2_hi_addr",
        global::CFG_WATCHPOINTS_2_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_2_lo_addr",
        global::CFG_WATCHPOINTS_2_lo_addr,
    ),
    ("CFG_WATCHPOINTS_2_rsvd", global::CFG_WATCHPOINTS_2_rsvd),
    ("CFG_WATCHPOINTS_3_enable", global::CFG_WATCHPOINTS_3_enable),
    (
        "CFG_WATCHPOINTS_3_hi_addr",
        global::CFG_WATCHPOINTS_3_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_3_lo_addr",
        global::CFG_WATCHPOINTS_3_lo_addr,
    ),
    ("CFG_WATCHPOINTS_3_rsvd", global::CFG_WATCHPOINTS_3_rsvd),
    (
        "CLR_DVALID_SEC0_SrcA_Disable",
        global::CLR_DVALID_SEC0_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC0_SrcB_Disable",
        global::CLR_DVALID_SEC0_SrcB_Disable,
    ),
    (
        "CLR_DVALID_SEC1_SrcA_Disable",
        global::CLR_DVALID_SEC1_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC1_SrcB_Disable",
        global::CLR_DVALID_SEC1_SrcB_Disable,
    ),
    (
        "CLR_DVALID_SEC2_SrcA_Disable",
        global::CLR_DVALID_SEC2_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC2_SrcB_Disable",
        global::CLR_DVALID_SEC2_SrcB_Disable,
    ),
    (
        "CLR_DVALID_SEC3_SrcA_Disable",
        global::CLR_DVALID_SEC3_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC3_SrcB_Disable",
        global::CLR_DVALID_SEC3_SrcB_Disable,
    ),
    (
        "DEST_ACCESS_CFG_zeroacc_absolute_tile_mode",
        global::DEST_ACCESS_CFG_zeroacc_absolute_tile_mode,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC0_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC0_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC1_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC1_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC2_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC2_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC3_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC3_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC0_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC0_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC1_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC1_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC2_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC2_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC3_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC3_Base,
    ),
    (
        "ENABLE_ACC_STATS_SEC0_Enable",
        global::ENABLE_ACC_STATS_SEC0_Enable,
    ),
    (
        "ENABLE_ACC_STATS_SEC1_Enable",
        global::ENABLE_ACC_STATS_SEC1_Enable,
    ),
    (
        "ENABLE_ACC_STATS_SEC2_Enable",
        global::ENABLE_ACC_STATS_SEC2_Enable,
    ),
    (
        "ENABLE_ACC_STATS_SEC3_Enable",
        global::ENABLE_ACC_STATS_SEC3_Enable,
    ),
    ("FIDELITY_BASE_SEC0_Phase", global::FIDELITY_BASE_SEC0_Phase),
    ("FIDELITY_BASE_SEC1_Phase", global::FIDELITY_BASE_SEC1_Phase),
    ("FIDELITY_BASE_SEC2_Phase", global::FIDELITY_BASE_SEC2_Phase),
    ("FIDELITY_BASE_SEC3_Phase", global::FIDELITY_BASE_SEC3_Phase),
    ("FMT_CTRL_FP8_OVF_EN", global::FMT_CTRL_FP8_OVF_EN),
    (
        "FMT_CTRL_MXINT_NEGZERO_EQ_NEGINF",
        global::FMT_CTRL_MXINT_NEGZERO_EQ_NEGINF,
    ),
    (
        "FMT_CTRL_MX_BLOCK_EXP_RND_TO_INF",
        global::FMT_CTRL_MX_BLOCK_EXP_RND_TO_INF,
    ),
    (
        "FMT_CTRL_RESET_PACKER0_EXP_STATS",
        global::FMT_CTRL_RESET_PACKER0_EXP_STATS,
    ),
    (
        "FMT_CTRL_RESET_PACKER1_EXP_STATS",
        global::FMT_CTRL_RESET_PACKER1_EXP_STATS,
    ),
    ("FP16A_FORCE_SEC0_Enable", global::FP16A_FORCE_SEC0_Enable),
    ("FP16A_FORCE_SEC1_Enable", global::FP16A_FORCE_SEC1_Enable),
    ("FP16A_FORCE_SEC2_Enable", global::FP16A_FORCE_SEC2_Enable),
    ("FP16A_FORCE_SEC3_Enable", global::FP16A_FORCE_SEC3_Enable),
    (
        "INT_DESCALE_VALUES_SEC0_Value",
        global::INT_DESCALE_VALUES_SEC0_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC10_Value",
        global::INT_DESCALE_VALUES_SEC10_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC11_Value",
        global::INT_DESCALE_VALUES_SEC11_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC12_Value",
        global::INT_DESCALE_VALUES_SEC12_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC13_Value",
        global::INT_DESCALE_VALUES_SEC13_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC14_Value",
        global::INT_DESCALE_VALUES_SEC14_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC15_Value",
        global::INT_DESCALE_VALUES_SEC15_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC1_Value",
        global::INT_DESCALE_VALUES_SEC1_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC2_Value",
        global::INT_DESCALE_VALUES_SEC2_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC3_Value",
        global::INT_DESCALE_VALUES_SEC3_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC4_Value",
        global::INT_DESCALE_VALUES_SEC4_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC5_Value",
        global::INT_DESCALE_VALUES_SEC5_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC6_Value",
        global::INT_DESCALE_VALUES_SEC6_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC7_Value",
        global::INT_DESCALE_VALUES_SEC7_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC8_Value",
        global::INT_DESCALE_VALUES_SEC8_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC9_Value",
        global::INT_DESCALE_VALUES_SEC9_Value,
    ),
    (
        "MATH_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::MATH_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "MATH_DEST_DVALID_CTRL_toggle_mask",
        global::MATH_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "MATH_DEST_DVALID_CTRL_wait_mask",
        global::MATH_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "MATH_DEST_DVALID_CTRL_wait_polarity",
        global::MATH_DEST_DVALID_CTRL_wait_polarity,
    ),
    (
        "PACK_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::PACK_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "PACK_DEST_DVALID_CTRL_toggle_mask",
        global::PACK_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "PACK_DEST_DVALID_CTRL_wait_mask",
        global::PACK_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "PACK_DEST_DVALID_CTRL_wait_polarity",
        global::PACK_DEST_DVALID_CTRL_wait_polarity,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd0Start",
        global::PERF_CNT_CMD_SEC0_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd0Stop",
        global::PERF_CNT_CMD_SEC0_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd1Start",
        global::PERF_CNT_CMD_SEC0_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd1Stop",
        global::PERF_CNT_CMD_SEC0_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd2Start",
        global::PERF_CNT_CMD_SEC0_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd2Stop",
        global::PERF_CNT_CMD_SEC0_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd3Start",
        global::PERF_CNT_CMD_SEC0_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd3Stop",
        global::PERF_CNT_CMD_SEC0_Cmd3Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd0Start",
        global::PERF_CNT_CMD_SEC1_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd0Stop",
        global::PERF_CNT_CMD_SEC1_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd1Start",
        global::PERF_CNT_CMD_SEC1_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd1Stop",
        global::PERF_CNT_CMD_SEC1_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd2Start",
        global::PERF_CNT_CMD_SEC1_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd2Stop",
        global::PERF_CNT_CMD_SEC1_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd3Start",
        global::PERF_CNT_CMD_SEC1_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd3Stop",
        global::PERF_CNT_CMD_SEC1_Cmd3Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd0Start",
        global::PERF_CNT_CMD_SEC2_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd0Stop",
        global::PERF_CNT_CMD_SEC2_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd1Start",
        global::PERF_CNT_CMD_SEC2_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd1Stop",
        global::PERF_CNT_CMD_SEC2_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd2Start",
        global::PERF_CNT_CMD_SEC2_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd2Stop",
        global::PERF_CNT_CMD_SEC2_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd3Start",
        global::PERF_CNT_CMD_SEC2_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd3Stop",
        global::PERF_CNT_CMD_SEC2_Cmd3Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd0Start",
        global::PERF_CNT_CMD_SEC3_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd0Stop",
        global::PERF_CNT_CMD_SEC3_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd1Start",
        global::PERF_CNT_CMD_SEC3_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd1Stop",
        global::PERF_CNT_CMD_SEC3_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd2Start",
        global::PERF_CNT_CMD_SEC3_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd2Stop",
        global::PERF_CNT_CMD_SEC3_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd3Start",
        global::PERF_CNT_CMD_SEC3_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd3Stop",
        global::PERF_CNT_CMD_SEC3_Cmd3Stop,
    ),
    (
        "PRNG_SEED_OVERRIDE_CTRL_override_en",
        global::PRNG_SEED_OVERRIDE_CTRL_override_en,
    ),
    (
        "PRNG_SEED_OVERRIDE_CTRL_prng_id",
        global::PRNG_SEED_OVERRIDE_CTRL_prng_id,
    ),
    (
        "PRNG_SEED_OVERRIDE_DATA_seed_override",
        global::PRNG_SEED_OVERRIDE_DATA_seed_override,
    ),
    ("PRNG_SEED_Seed_Val", global::PRNG_SEED_Seed_Val),
    ("SCRATCH_SEC0_val", global::SCRATCH_SEC0_val),
    ("SCRATCH_SEC1_val", global::SCRATCH_SEC1_val),
    ("SCRATCH_SEC2_val", global::SCRATCH_SEC2_val),
    ("SCRATCH_SEC3_val", global::SCRATCH_SEC3_val),
    (
        "SFPU_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::SFPU_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "SFPU_DEST_DVALID_CTRL_toggle_mask",
        global::SFPU_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "SFPU_DEST_DVALID_CTRL_wait_mask",
        global::SFPU_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "SFPU_DEST_DVALID_CTRL_wait_polarity",
        global::SFPU_DEST_DVALID_CTRL_wait_polarity,
    ),
    ("SFPU_DEST_FMT_SEC0_Base", global::SFPU_DEST_FMT_SEC0_Base),
    (
        "SFPU_DEST_FMT_SEC0_Enable",
        global::SFPU_DEST_FMT_SEC0_Enable,
    ),
    ("SFPU_DEST_FMT_SEC1_Base", global::SFPU_DEST_FMT_SEC1_Base),
    (
        "SFPU_DEST_FMT_SEC1_Enable",
        global::SFPU_DEST_FMT_SEC1_Enable,
    ),
    ("SFPU_DEST_FMT_SEC2_Base", global::SFPU_DEST_FMT_SEC2_Base),
    (
        "SFPU_DEST_FMT_SEC2_Enable",
        global::SFPU_DEST_FMT_SEC2_Enable,
    ),
    ("SFPU_DEST_FMT_SEC3_Base", global::SFPU_DEST_FMT_SEC3_Base),
    (
        "SFPU_DEST_FMT_SEC3_Enable",
        global::SFPU_DEST_FMT_SEC3_Enable,
    ),
    ("SFPU_STACK_SEC0_Incr", global::SFPU_STACK_SEC0_Incr),
    ("SFPU_STACK_SEC1_Incr", global::SFPU_STACK_SEC1_Incr),
    ("SFPU_STACK_SEC2_Incr", global::SFPU_STACK_SEC2_Incr),
    ("SFPU_STACK_SEC3_Incr", global::SFPU_STACK_SEC3_Incr),
    (
        "SRC_ACCESS_CFG_disable_contig_srca_dvalid_phase",
        global::SRC_ACCESS_CFG_disable_contig_srca_dvalid_phase,
    ),
    (
        "SRC_ACCESS_CFG_disable_contig_srcb_dvalid_phase",
        global::SRC_ACCESS_CFG_disable_contig_srcb_dvalid_phase,
    ),
    (
        "SRC_ACCESS_CFG_math_view_srca_as_one_bank",
        global::SRC_ACCESS_CFG_math_view_srca_as_one_bank,
    ),
    (
        "SRC_ACCESS_CFG_math_view_srcb_as_one_bank",
        global::SRC_ACCESS_CFG_math_view_srcb_as_one_bank,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::UNPACK_TO_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_toggle_mask",
        global::UNPACK_TO_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_wait_mask",
        global::UNPACK_TO_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_wait_polarity",
        global::UNPACK_TO_DEST_DVALID_CTRL_wait_polarity,
    ),
    (
        "THCON_PACKER0_REG0_DEST_REG_WRAP_MODE",
        thcon::THCON_PACKER0_REG0_DEST_REG_WRAP_MODE,
    ),
    (
        "THCON_PACKER0_REG0_EXP_THRESHOLD",
        thcon::THCON_PACKER0_REG0_EXP_THRESHOLD,
    ),
    (
        "THCON_PACKER0_REG0_EXP_THRESHOLD_EN",
        thcon::THCON_PACKER0_REG0_EXP_THRESHOLD_EN,
    ),
    (
        "THCON_PACKER0_REG0_INT_DESCALE_ENABLE",
        thcon::THCON_PACKER0_REG0_INT_DESCALE_ENABLE,
    ),
    (
        "THCON_PACKER0_REG0_INT_DESCALE_MODE",
        thcon::THCON_PACKER0_REG0_INT_DESCALE_MODE,
    ),
    (
        "THCON_PACKER0_REG0_IN_DATA_FORMAT",
        thcon::THCON_PACKER0_REG0_IN_DATA_FORMAT,
    ),
    (
        "THCON_PACKER0_REG0_L1_ACC",
        thcon::THCON_PACKER0_REG0_L1_ACC,
    ),
    (
        "THCON_PACKER0_REG0_READ_MODE",
        thcon::THCON_PACKER0_REG0_READ_MODE,
    ),
    (
        "THCON_PACKER0_REG0_SELF_CHECK_EN",
        thcon::THCON_PACKER0_REG0_SELF_CHECK_EN,
    ),
    (
        "THCON_PACKER0_REG0_SRC_ADDR_OFFSET",
        thcon::THCON_PACKER0_REG0_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_PACKER0_REG0_STOCH_RND_EN",
        thcon::THCON_PACKER0_REG0_STOCH_RND_EN,
    ),
    (
        "THCON_PACKER0_REG0_UNTILIZE_DST_ADDR_OFFSET",
        thcon::THCON_PACKER0_REG0_UNTILIZE_DST_ADDR_OFFSET,
    ),
    (
        "THCON_PACKER0_REG0_UNUSED0",
        thcon::THCON_PACKER0_REG0_UNUSED0,
    ),
    (
        "THCON_PACKER0_REG0_UNUSED1",
        thcon::THCON_PACKER0_REG0_UNUSED1,
    ),
    (
        "THCON_PACKER0_REG0_UNUSED2",
        thcon::THCON_PACKER0_REG0_UNUSED2,
    ),
    (
        "THCON_PACKER0_REG0_WRAP_TILE_DIM",
        thcon::THCON_PACKER0_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_PACKER0_REG0_ZERO_WRITE",
        thcon::THCON_PACKER0_REG0_ZERO_WRITE,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK0",
        thcon::THCON_PACKER0_REG1_EDGE_MASK0,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK1",
        thcon::THCON_PACKER0_REG1_EDGE_MASK1,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK2",
        thcon::THCON_PACKER0_REG1_EDGE_MASK2,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK3",
        thcon::THCON_PACKER0_REG1_EDGE_MASK3,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK_MODE",
        thcon::THCON_PACKER0_REG1_EDGE_MASK_MODE,
    ),
    (
        "THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_0",
        thcon::THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_1",
        thcon::THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_PACKER0_REG1_PACK_UNTILIZE_DST_Z_STRIDE",
        thcon::THCON_PACKER0_REG1_PACK_UNTILIZE_DST_Z_STRIDE,
    ),
    (
        "THCON_PACKER0_REG1_PACK_UNTILIZE_SRC_Z_STRIDE",
        thcon::THCON_PACKER0_REG1_PACK_UNTILIZE_SRC_Z_STRIDE,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE0",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE0,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE1",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE1,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE2",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE2,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE3",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE3,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_COL_MASK",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_MASK_VAL",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_NO_WRITE",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_2",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_3",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_ROW_MASK",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_ROW_MASK,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_VAL_SOURCE",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_PACKER0_REG3_RELU_MODE",
        thcon::THCON_PACKER0_REG3_RELU_MODE,
    ),
    (
        "THCON_PACKER0_REG3_RELU_THRESHOLD",
        thcon::THCON_PACKER0_REG3_RELU_THRESHOLD,
    ),
    (
        "THCON_PACKER0_REG3_UNUSED0",
        thcon::THCON_PACKER0_REG3_UNUSED0,
    ),
    (
        "THCON_PACKER1_REG0_DISABLE_SRCS_OFFSET",
        thcon::THCON_PACKER1_REG0_DISABLE_SRCS_OFFSET,
    ),
    (
        "THCON_PACKER1_REG0_EXP_THRESHOLD",
        thcon::THCON_PACKER1_REG0_EXP_THRESHOLD,
    ),
    (
        "THCON_PACKER1_REG0_EXP_THRESHOLD_EN",
        thcon::THCON_PACKER1_REG0_EXP_THRESHOLD_EN,
    ),
    (
        "THCON_PACKER1_REG0_INSTRN_COUNT",
        thcon::THCON_PACKER1_REG0_INSTRN_COUNT,
    ),
    (
        "THCON_PACKER1_REG0_INSTRN_LOOP_COUNT",
        thcon::THCON_PACKER1_REG0_INSTRN_LOOP_COUNT,
    ),
    (
        "THCON_PACKER1_REG0_INT_DESCALE_ENABLE",
        thcon::THCON_PACKER1_REG0_INT_DESCALE_ENABLE,
    ),
    (
        "THCON_PACKER1_REG0_INT_DESCALE_MODE",
        thcon::THCON_PACKER1_REG0_INT_DESCALE_MODE,
    ),
    (
        "THCON_PACKER1_REG0_IN_DATA_FORMAT",
        thcon::THCON_PACKER1_REG0_IN_DATA_FORMAT,
    ),
    (
        "THCON_PACKER1_REG0_L1_ACC",
        thcon::THCON_PACKER1_REG0_L1_ACC,
    ),
    (
        "THCON_PACKER1_REG0_SRC_ADDR_OFFSET",
        thcon::THCON_PACKER1_REG0_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_PACKER1_REG0_STOCH_RND_EN",
        thcon::THCON_PACKER1_REG0_STOCH_RND_EN,
    ),
    (
        "THCON_PACKER1_REG0_UNUSED0",
        thcon::THCON_PACKER1_REG0_UNUSED0,
    ),
    (
        "THCON_PACKER1_REG0_UNUSED1",
        thcon::THCON_PACKER1_REG0_UNUSED1,
    ),
    (
        "THCON_PACKER1_REG0_UNUSED2",
        thcon::THCON_PACKER1_REG0_UNUSED2,
    ),
    (
        "THCON_PACKER1_REG0_WRAP_TILE_DIM",
        thcon::THCON_PACKER1_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_PACKER1_REG0_ZERO_WRITE",
        thcon::THCON_PACKER1_REG0_ZERO_WRITE,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK0",
        thcon::THCON_PACKER1_REG1_EDGE_MASK0,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK1",
        thcon::THCON_PACKER1_REG1_EDGE_MASK1,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK2",
        thcon::THCON_PACKER1_REG1_EDGE_MASK2,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK3",
        thcon::THCON_PACKER1_REG1_EDGE_MASK3,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK_MODE",
        thcon::THCON_PACKER1_REG1_EDGE_MASK_MODE,
    ),
    (
        "THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_0",
        thcon::THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_1",
        thcon::THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_PACKER1_REG1_UNUSED0",
        thcon::THCON_PACKER1_REG1_UNUSED0,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE0",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE0,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE1",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE1,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE2",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE2,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE3",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE3,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_COL_MASK",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_MASK_VAL",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_NO_WRITE",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_2",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_3",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_ROW_MASK",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_ROW_MASK,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_VAL_SOURCE",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_PACKER1_REG3_RELU_MODE",
        thcon::THCON_PACKER1_REG3_RELU_MODE,
    ),
    (
        "THCON_PACKER1_REG3_RELU_THRESHOLD",
        thcon::THCON_PACKER1_REG3_RELU_THRESHOLD,
    ),
    (
        "THCON_PACKER1_REG3_UNUSED0",
        thcon::THCON_PACKER1_REG3_UNUSED0,
    ),
    (
        "THCON_UNPACKER0_REG0_ARG_FIFO_MODE",
        thcon::THCON_UNPACKER0_REG0_ARG_FIFO_MODE,
    ),
    (
        "THCON_UNPACKER0_REG0_ENABLE_ARG_FIFO",
        thcon::THCON_UNPACKER0_REG0_ENABLE_ARG_FIFO,
    ),
    (
        "THCON_UNPACKER0_REG0_OUT_DATA_FORMAT",
        thcon::THCON_UNPACKER0_REG0_OUT_DATA_FORMAT,
    ),
    (
        "THCON_UNPACKER0_REG0_SELF_CHECK_EN",
        thcon::THCON_UNPACKER0_REG0_SELF_CHECK_EN,
    ),
    (
        "THCON_UNPACKER0_REG0_SET_STRIDE_MODE",
        thcon::THCON_UNPACKER0_REG0_SET_STRIDE_MODE,
    ),
    (
        "THCON_UNPACKER0_REG0_TILIZE_SRC_ADDR_OFFSET",
        thcon::THCON_UNPACKER0_REG0_TILIZE_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_UNPACKER0_REG0_TRANSPOSE",
        thcon::THCON_UNPACKER0_REG0_TRANSPOSE,
    ),
    (
        "THCON_UNPACKER0_REG0_UNPACK_STRIDE_COL_MASK",
        thcon::THCON_UNPACKER0_REG0_UNPACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_UNPACKER0_REG0_UNPACR_STRIDE_SET_STRIDE",
        thcon::THCON_UNPACKER0_REG0_UNPACR_STRIDE_SET_STRIDE,
    ),
    (
        "THCON_UNPACKER0_REG0_UNUSED1",
        thcon::THCON_UNPACKER0_REG0_UNUSED1,
    ),
    (
        "THCON_UNPACKER0_REG0_WRAP_TILE_DIM",
        thcon::THCON_UNPACKER0_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_MASK_VAL",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_NO_WRITE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK0",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK0,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK1",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK1,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK2",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK2,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK3",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK3,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK4",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK4,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK5",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK5,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK6",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK6,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK7",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK7,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_VAL_SOURCE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_TILIZE_DST_Z_STRIDE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_TILIZE_DST_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_TILIZE_SRC_Z_STRIDE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_TILIZE_SRC_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNUSED0",
        thcon::THCON_UNPACKER0_REG1_UNUSED0,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_0",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_1",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_2",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_3",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_4",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_4,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_5",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_5,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_6",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_6,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_7",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_7,
    ),
    (
        "THCON_UNPACKER1_REG0_ARG_FIFO_MODE",
        thcon::THCON_UNPACKER1_REG0_ARG_FIFO_MODE,
    ),
    (
        "THCON_UNPACKER1_REG0_ENABLE_ARG_FIFO",
        thcon::THCON_UNPACKER1_REG0_ENABLE_ARG_FIFO,
    ),
    (
        "THCON_UNPACKER1_REG0_OUT_DATA_FORMAT",
        thcon::THCON_UNPACKER1_REG0_OUT_DATA_FORMAT,
    ),
    (
        "THCON_UNPACKER1_REG0_SET_STRIDE_MODE",
        thcon::THCON_UNPACKER1_REG0_SET_STRIDE_MODE,
    ),
    (
        "THCON_UNPACKER1_REG0_TILIZE_SRC_ADDR_OFFSET",
        thcon::THCON_UNPACKER1_REG0_TILIZE_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_UNPACKER1_REG0_TRANSPOSE",
        thcon::THCON_UNPACKER1_REG0_TRANSPOSE,
    ),
    (
        "THCON_UNPACKER1_REG0_UNPACK_STRIDE_COL_MASK",
        thcon::THCON_UNPACKER1_REG0_UNPACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_UNPACKER1_REG0_UNPACR_STRIDE_SET_STRIDE",
        thcon::THCON_UNPACKER1_REG0_UNPACR_STRIDE_SET_STRIDE,
    ),
    (
        "THCON_UNPACKER1_REG0_UNUSED0",
        thcon::THCON_UNPACKER1_REG0_UNUSED0,
    ),
    (
        "THCON_UNPACKER1_REG0_UNUSED1",
        thcon::THCON_UNPACKER1_REG0_UNUSED1,
    ),
    (
        "THCON_UNPACKER1_REG0_WRAP_TILE_DIM",
        thcon::THCON_UNPACKER1_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_MASK_VAL",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_NO_WRITE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK0",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK0,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK1",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK1,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK2",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK2,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK3",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK3,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK4",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK4,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK5",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK5,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK6",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK6,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK7",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK7,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_VAL_SOURCE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_TILIZE_DST_Z_STRIDE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_TILIZE_DST_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_TILIZE_SRC_Z_STRIDE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_TILIZE_SRC_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNUSED0",
        thcon::THCON_UNPACKER1_REG1_UNUSED0,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_0",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_1",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_2",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_3",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_4",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_4,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_5",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_5,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_6",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_6,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_7",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_7,
    ),
    (
        "THCON_UNPACKER2_REG0_ARG_FIFO_MODE",
        thcon::THCON_UNPACKER2_REG0_ARG_FIFO_MODE,
    ),
    (
        "THCON_UNPACKER2_REG0_ENABLE_ARG_FIFO",
        thcon::THCON_UNPACKER2_REG0_ENABLE_ARG_FIFO,
    ),
    (
        "THCON_UNPACKER2_REG0_INSTRN_COUNT",
        thcon::THCON_UNPACKER2_REG0_INSTRN_COUNT,
    ),
    (
        "THCON_UNPACKER2_REG0_INSTRN_LOOP_COUNT",
        thcon::THCON_UNPACKER2_REG0_INSTRN_LOOP_COUNT,
    ),
    (
        "THCON_UNPACKER2_REG0_OUT_DATA_FORMAT",
        thcon::THCON_UNPACKER2_REG0_OUT_DATA_FORMAT,
    ),
    (
        "THCON_UNPACKER2_REG0_UNPACK_STRIDE_COL_MASK",
        thcon::THCON_UNPACKER2_REG0_UNPACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_UNPACKER2_REG0_UNUSED0",
        thcon::THCON_UNPACKER2_REG0_UNUSED0,
    ),
    (
        "THCON_UNPACKER2_REG0_UNUSED1",
        thcon::THCON_UNPACKER2_REG0_UNUSED1,
    ),
    (
        "THCON_UNPACKER2_REG0_UNUSED2",
        thcon::THCON_UNPACKER2_REG0_UNUSED2,
    ),
    (
        "THCON_UNPACKER2_REG0_WRAP_TILE_DIM",
        thcon::THCON_UNPACKER2_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_MASK_VAL",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_NO_WRITE",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK0",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK0,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK1",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK1,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK2",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK2,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK3",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK3,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK4",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK4,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK5",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK5,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK6",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK6,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK7",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK7,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_VAL_SOURCE",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_UNPACKER2_REG1_UNUSED0",
        thcon::THCON_UNPACKER2_REG1_UNUSED0,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_0",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_1",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_2",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_3",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_4",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_4,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_5",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_5,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_6",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_6,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_7",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_7,
    ),
];

pub static ALL_THREAD_CONFIG_FIELDS: &[(&str, ThreadConfigField)] = &[];

pub static ALL_CONFIG_SPANS: &[(&str, ConfigSpan)] = &[];

pub static ALU_FIELDS: &[(&str, ConfigField)] = &[
    ("ALU_ACC_CTRL_Fp32_enabled", alu::ALU_ACC_CTRL_Fp32_enabled),
    (
        "ALU_ACC_CTRL_INT8_math_enabled",
        alu::ALU_ACC_CTRL_INT8_math_enabled,
    ),
    (
        "ALU_ACC_CTRL_SFPU_Fp32_enabled",
        alu::ALU_ACC_CTRL_SFPU_Fp32_enabled,
    ),
    (
        "ALU_ACC_CTRL_Zero_Flag_disabled_dst",
        alu::ALU_ACC_CTRL_Zero_Flag_disabled_dst,
    ),
    (
        "ALU_ACC_CTRL_Zero_Flag_disabled_src",
        alu::ALU_ACC_CTRL_Zero_Flag_disabled_src,
    ),
    ("ALU_FORMAT_SPEC_REG0_SrcA", alu::ALU_FORMAT_SPEC_REG0_SrcA),
    ("ALU_FORMAT_SPEC_REG1_SrcB", alu::ALU_FORMAT_SPEC_REG1_SrcB),
    (
        "ALU_FORMAT_SPEC_REG2_Dstacc",
        alu::ALU_FORMAT_SPEC_REG2_Dstacc,
    ),
    (
        "ALU_FORMAT_SPEC_REG_Dstacc_override",
        alu::ALU_FORMAT_SPEC_REG_Dstacc_override,
    ),
    (
        "ALU_FORMAT_SPEC_REG_Dstacc_val",
        alu::ALU_FORMAT_SPEC_REG_Dstacc_val,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcA_override",
        alu::ALU_FORMAT_SPEC_REG_SrcA_override,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcA_val",
        alu::ALU_FORMAT_SPEC_REG_SrcA_val,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcB_override",
        alu::ALU_FORMAT_SPEC_REG_SrcB_override,
    ),
    (
        "ALU_FORMAT_SPEC_REG_SrcB_val",
        alu::ALU_FORMAT_SPEC_REG_SrcB_val,
    ),
    ("ALU_ROUNDING_MODE_Bfp8_HF", alu::ALU_ROUNDING_MODE_Bfp8_HF),
    (
        "ALU_ROUNDING_MODE_Fpu_srnd_en",
        alu::ALU_ROUNDING_MODE_Fpu_srnd_en,
    ),
    ("ALU_ROUNDING_MODE_GS_LF", alu::ALU_ROUNDING_MODE_GS_LF),
    ("ALU_ROUNDING_MODE_Padding", alu::ALU_ROUNDING_MODE_Padding),
    ("DEST_REGW_BASE_Base", alu::DEST_REGW_BASE_Base),
    ("DEST_SP_BASE_Base", alu::DEST_SP_BASE_Base),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC0_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC0_Offset,
    ),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC1_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC1_Offset,
    ),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC2_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC2_Offset,
    ),
    (
        "DEST_TARGET_REG_CFG_MATH_SEC3_Offset",
        alu::DEST_TARGET_REG_CFG_MATH_SEC3_Offset,
    ),
    (
        "DISABLE_IMPLIED_SRCS_FORMAT",
        alu::DISABLE_IMPLIED_SRCS_FORMAT,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC0_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC0_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC0_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC0_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC0_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC0_unsigned_int,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC1_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC1_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC1_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC1_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC1_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC1_unsigned_int,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC2_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC2_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC2_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC2_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC2_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC2_unsigned_int,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC3_fmt",
        alu::RISC_DEST_ACCESS_CTRL_SEC3_fmt,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC3_no_swizzle",
        alu::RISC_DEST_ACCESS_CTRL_SEC3_no_swizzle,
    ),
    (
        "RISC_DEST_ACCESS_CTRL_SEC3_unsigned_int",
        alu::RISC_DEST_ACCESS_CTRL_SEC3_unsigned_int,
    ),
    ("SFPU_SRCS_BANK0_FORMAT", alu::SFPU_SRCS_BANK0_FORMAT),
    ("SFPU_SRCS_BANK1_FORMAT", alu::SFPU_SRCS_BANK1_FORMAT),
    ("STATE_RESET_EN", alu::STATE_RESET_EN),
    ("UNUSED0", alu::UNUSED0),
    ("UNUSED1", alu::UNUSED1),
    ("UNUSED2", alu::UNUSED2),
];

pub static GLOBAL_FIELDS: &[(&str, ConfigField)] = &[
    (
        "ADDR_MOD_AB2_SEC0_SrcAIncr",
        global::ADDR_MOD_AB2_SEC0_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC0_SrcBIncr",
        global::ADDR_MOD_AB2_SEC0_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC10_SrcAIncr",
        global::ADDR_MOD_AB2_SEC10_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC10_SrcBIncr",
        global::ADDR_MOD_AB2_SEC10_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC11_SrcAIncr",
        global::ADDR_MOD_AB2_SEC11_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC11_SrcBIncr",
        global::ADDR_MOD_AB2_SEC11_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC12_SrcAIncr",
        global::ADDR_MOD_AB2_SEC12_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC12_SrcBIncr",
        global::ADDR_MOD_AB2_SEC12_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC13_SrcAIncr",
        global::ADDR_MOD_AB2_SEC13_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC13_SrcBIncr",
        global::ADDR_MOD_AB2_SEC13_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC14_SrcAIncr",
        global::ADDR_MOD_AB2_SEC14_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC14_SrcBIncr",
        global::ADDR_MOD_AB2_SEC14_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC15_SrcAIncr",
        global::ADDR_MOD_AB2_SEC15_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC15_SrcBIncr",
        global::ADDR_MOD_AB2_SEC15_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC16_SrcAIncr",
        global::ADDR_MOD_AB2_SEC16_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC16_SrcBIncr",
        global::ADDR_MOD_AB2_SEC16_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC17_SrcAIncr",
        global::ADDR_MOD_AB2_SEC17_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC17_SrcBIncr",
        global::ADDR_MOD_AB2_SEC17_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC18_SrcAIncr",
        global::ADDR_MOD_AB2_SEC18_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC18_SrcBIncr",
        global::ADDR_MOD_AB2_SEC18_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC19_SrcAIncr",
        global::ADDR_MOD_AB2_SEC19_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC19_SrcBIncr",
        global::ADDR_MOD_AB2_SEC19_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC1_SrcAIncr",
        global::ADDR_MOD_AB2_SEC1_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC1_SrcBIncr",
        global::ADDR_MOD_AB2_SEC1_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC20_SrcAIncr",
        global::ADDR_MOD_AB2_SEC20_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC20_SrcBIncr",
        global::ADDR_MOD_AB2_SEC20_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC21_SrcAIncr",
        global::ADDR_MOD_AB2_SEC21_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC21_SrcBIncr",
        global::ADDR_MOD_AB2_SEC21_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC22_SrcAIncr",
        global::ADDR_MOD_AB2_SEC22_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC22_SrcBIncr",
        global::ADDR_MOD_AB2_SEC22_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC23_SrcAIncr",
        global::ADDR_MOD_AB2_SEC23_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC23_SrcBIncr",
        global::ADDR_MOD_AB2_SEC23_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC24_SrcAIncr",
        global::ADDR_MOD_AB2_SEC24_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC24_SrcBIncr",
        global::ADDR_MOD_AB2_SEC24_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC25_SrcAIncr",
        global::ADDR_MOD_AB2_SEC25_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC25_SrcBIncr",
        global::ADDR_MOD_AB2_SEC25_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC26_SrcAIncr",
        global::ADDR_MOD_AB2_SEC26_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC26_SrcBIncr",
        global::ADDR_MOD_AB2_SEC26_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC27_SrcAIncr",
        global::ADDR_MOD_AB2_SEC27_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC27_SrcBIncr",
        global::ADDR_MOD_AB2_SEC27_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC28_SrcAIncr",
        global::ADDR_MOD_AB2_SEC28_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC28_SrcBIncr",
        global::ADDR_MOD_AB2_SEC28_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC29_SrcAIncr",
        global::ADDR_MOD_AB2_SEC29_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC29_SrcBIncr",
        global::ADDR_MOD_AB2_SEC29_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC2_SrcAIncr",
        global::ADDR_MOD_AB2_SEC2_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC2_SrcBIncr",
        global::ADDR_MOD_AB2_SEC2_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC30_SrcAIncr",
        global::ADDR_MOD_AB2_SEC30_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC30_SrcBIncr",
        global::ADDR_MOD_AB2_SEC30_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC31_SrcAIncr",
        global::ADDR_MOD_AB2_SEC31_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC31_SrcBIncr",
        global::ADDR_MOD_AB2_SEC31_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC3_SrcAIncr",
        global::ADDR_MOD_AB2_SEC3_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC3_SrcBIncr",
        global::ADDR_MOD_AB2_SEC3_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC4_SrcAIncr",
        global::ADDR_MOD_AB2_SEC4_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC4_SrcBIncr",
        global::ADDR_MOD_AB2_SEC4_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC5_SrcAIncr",
        global::ADDR_MOD_AB2_SEC5_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC5_SrcBIncr",
        global::ADDR_MOD_AB2_SEC5_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC6_SrcAIncr",
        global::ADDR_MOD_AB2_SEC6_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC6_SrcBIncr",
        global::ADDR_MOD_AB2_SEC6_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC7_SrcAIncr",
        global::ADDR_MOD_AB2_SEC7_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC7_SrcBIncr",
        global::ADDR_MOD_AB2_SEC7_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC8_SrcAIncr",
        global::ADDR_MOD_AB2_SEC8_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC8_SrcBIncr",
        global::ADDR_MOD_AB2_SEC8_SrcBIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC9_SrcAIncr",
        global::ADDR_MOD_AB2_SEC9_SrcAIncr,
    ),
    (
        "ADDR_MOD_AB2_SEC9_SrcBIncr",
        global::ADDR_MOD_AB2_SEC9_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC0_SrcACR", global::ADDR_MOD_AB_SEC0_SrcACR),
    (
        "ADDR_MOD_AB_SEC0_SrcAClear",
        global::ADDR_MOD_AB_SEC0_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC0_SrcAIncr",
        global::ADDR_MOD_AB_SEC0_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC0_SrcBCR", global::ADDR_MOD_AB_SEC0_SrcBCR),
    (
        "ADDR_MOD_AB_SEC0_SrcBClear",
        global::ADDR_MOD_AB_SEC0_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC0_SrcBIncr",
        global::ADDR_MOD_AB_SEC0_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC10_SrcACR", global::ADDR_MOD_AB_SEC10_SrcACR),
    (
        "ADDR_MOD_AB_SEC10_SrcAClear",
        global::ADDR_MOD_AB_SEC10_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC10_SrcAIncr",
        global::ADDR_MOD_AB_SEC10_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC10_SrcBCR", global::ADDR_MOD_AB_SEC10_SrcBCR),
    (
        "ADDR_MOD_AB_SEC10_SrcBClear",
        global::ADDR_MOD_AB_SEC10_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC10_SrcBIncr",
        global::ADDR_MOD_AB_SEC10_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC11_SrcACR", global::ADDR_MOD_AB_SEC11_SrcACR),
    (
        "ADDR_MOD_AB_SEC11_SrcAClear",
        global::ADDR_MOD_AB_SEC11_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC11_SrcAIncr",
        global::ADDR_MOD_AB_SEC11_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC11_SrcBCR", global::ADDR_MOD_AB_SEC11_SrcBCR),
    (
        "ADDR_MOD_AB_SEC11_SrcBClear",
        global::ADDR_MOD_AB_SEC11_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC11_SrcBIncr",
        global::ADDR_MOD_AB_SEC11_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC12_SrcACR", global::ADDR_MOD_AB_SEC12_SrcACR),
    (
        "ADDR_MOD_AB_SEC12_SrcAClear",
        global::ADDR_MOD_AB_SEC12_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC12_SrcAIncr",
        global::ADDR_MOD_AB_SEC12_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC12_SrcBCR", global::ADDR_MOD_AB_SEC12_SrcBCR),
    (
        "ADDR_MOD_AB_SEC12_SrcBClear",
        global::ADDR_MOD_AB_SEC12_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC12_SrcBIncr",
        global::ADDR_MOD_AB_SEC12_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC13_SrcACR", global::ADDR_MOD_AB_SEC13_SrcACR),
    (
        "ADDR_MOD_AB_SEC13_SrcAClear",
        global::ADDR_MOD_AB_SEC13_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC13_SrcAIncr",
        global::ADDR_MOD_AB_SEC13_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC13_SrcBCR", global::ADDR_MOD_AB_SEC13_SrcBCR),
    (
        "ADDR_MOD_AB_SEC13_SrcBClear",
        global::ADDR_MOD_AB_SEC13_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC13_SrcBIncr",
        global::ADDR_MOD_AB_SEC13_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC14_SrcACR", global::ADDR_MOD_AB_SEC14_SrcACR),
    (
        "ADDR_MOD_AB_SEC14_SrcAClear",
        global::ADDR_MOD_AB_SEC14_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC14_SrcAIncr",
        global::ADDR_MOD_AB_SEC14_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC14_SrcBCR", global::ADDR_MOD_AB_SEC14_SrcBCR),
    (
        "ADDR_MOD_AB_SEC14_SrcBClear",
        global::ADDR_MOD_AB_SEC14_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC14_SrcBIncr",
        global::ADDR_MOD_AB_SEC14_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC15_SrcACR", global::ADDR_MOD_AB_SEC15_SrcACR),
    (
        "ADDR_MOD_AB_SEC15_SrcAClear",
        global::ADDR_MOD_AB_SEC15_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC15_SrcAIncr",
        global::ADDR_MOD_AB_SEC15_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC15_SrcBCR", global::ADDR_MOD_AB_SEC15_SrcBCR),
    (
        "ADDR_MOD_AB_SEC15_SrcBClear",
        global::ADDR_MOD_AB_SEC15_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC15_SrcBIncr",
        global::ADDR_MOD_AB_SEC15_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC16_SrcACR", global::ADDR_MOD_AB_SEC16_SrcACR),
    (
        "ADDR_MOD_AB_SEC16_SrcAClear",
        global::ADDR_MOD_AB_SEC16_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC16_SrcAIncr",
        global::ADDR_MOD_AB_SEC16_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC16_SrcBCR", global::ADDR_MOD_AB_SEC16_SrcBCR),
    (
        "ADDR_MOD_AB_SEC16_SrcBClear",
        global::ADDR_MOD_AB_SEC16_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC16_SrcBIncr",
        global::ADDR_MOD_AB_SEC16_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC17_SrcACR", global::ADDR_MOD_AB_SEC17_SrcACR),
    (
        "ADDR_MOD_AB_SEC17_SrcAClear",
        global::ADDR_MOD_AB_SEC17_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC17_SrcAIncr",
        global::ADDR_MOD_AB_SEC17_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC17_SrcBCR", global::ADDR_MOD_AB_SEC17_SrcBCR),
    (
        "ADDR_MOD_AB_SEC17_SrcBClear",
        global::ADDR_MOD_AB_SEC17_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC17_SrcBIncr",
        global::ADDR_MOD_AB_SEC17_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC18_SrcACR", global::ADDR_MOD_AB_SEC18_SrcACR),
    (
        "ADDR_MOD_AB_SEC18_SrcAClear",
        global::ADDR_MOD_AB_SEC18_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC18_SrcAIncr",
        global::ADDR_MOD_AB_SEC18_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC18_SrcBCR", global::ADDR_MOD_AB_SEC18_SrcBCR),
    (
        "ADDR_MOD_AB_SEC18_SrcBClear",
        global::ADDR_MOD_AB_SEC18_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC18_SrcBIncr",
        global::ADDR_MOD_AB_SEC18_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC19_SrcACR", global::ADDR_MOD_AB_SEC19_SrcACR),
    (
        "ADDR_MOD_AB_SEC19_SrcAClear",
        global::ADDR_MOD_AB_SEC19_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC19_SrcAIncr",
        global::ADDR_MOD_AB_SEC19_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC19_SrcBCR", global::ADDR_MOD_AB_SEC19_SrcBCR),
    (
        "ADDR_MOD_AB_SEC19_SrcBClear",
        global::ADDR_MOD_AB_SEC19_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC19_SrcBIncr",
        global::ADDR_MOD_AB_SEC19_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC1_SrcACR", global::ADDR_MOD_AB_SEC1_SrcACR),
    (
        "ADDR_MOD_AB_SEC1_SrcAClear",
        global::ADDR_MOD_AB_SEC1_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC1_SrcAIncr",
        global::ADDR_MOD_AB_SEC1_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC1_SrcBCR", global::ADDR_MOD_AB_SEC1_SrcBCR),
    (
        "ADDR_MOD_AB_SEC1_SrcBClear",
        global::ADDR_MOD_AB_SEC1_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC1_SrcBIncr",
        global::ADDR_MOD_AB_SEC1_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC20_SrcACR", global::ADDR_MOD_AB_SEC20_SrcACR),
    (
        "ADDR_MOD_AB_SEC20_SrcAClear",
        global::ADDR_MOD_AB_SEC20_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC20_SrcAIncr",
        global::ADDR_MOD_AB_SEC20_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC20_SrcBCR", global::ADDR_MOD_AB_SEC20_SrcBCR),
    (
        "ADDR_MOD_AB_SEC20_SrcBClear",
        global::ADDR_MOD_AB_SEC20_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC20_SrcBIncr",
        global::ADDR_MOD_AB_SEC20_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC21_SrcACR", global::ADDR_MOD_AB_SEC21_SrcACR),
    (
        "ADDR_MOD_AB_SEC21_SrcAClear",
        global::ADDR_MOD_AB_SEC21_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC21_SrcAIncr",
        global::ADDR_MOD_AB_SEC21_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC21_SrcBCR", global::ADDR_MOD_AB_SEC21_SrcBCR),
    (
        "ADDR_MOD_AB_SEC21_SrcBClear",
        global::ADDR_MOD_AB_SEC21_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC21_SrcBIncr",
        global::ADDR_MOD_AB_SEC21_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC22_SrcACR", global::ADDR_MOD_AB_SEC22_SrcACR),
    (
        "ADDR_MOD_AB_SEC22_SrcAClear",
        global::ADDR_MOD_AB_SEC22_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC22_SrcAIncr",
        global::ADDR_MOD_AB_SEC22_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC22_SrcBCR", global::ADDR_MOD_AB_SEC22_SrcBCR),
    (
        "ADDR_MOD_AB_SEC22_SrcBClear",
        global::ADDR_MOD_AB_SEC22_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC22_SrcBIncr",
        global::ADDR_MOD_AB_SEC22_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC23_SrcACR", global::ADDR_MOD_AB_SEC23_SrcACR),
    (
        "ADDR_MOD_AB_SEC23_SrcAClear",
        global::ADDR_MOD_AB_SEC23_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC23_SrcAIncr",
        global::ADDR_MOD_AB_SEC23_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC23_SrcBCR", global::ADDR_MOD_AB_SEC23_SrcBCR),
    (
        "ADDR_MOD_AB_SEC23_SrcBClear",
        global::ADDR_MOD_AB_SEC23_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC23_SrcBIncr",
        global::ADDR_MOD_AB_SEC23_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC24_SrcACR", global::ADDR_MOD_AB_SEC24_SrcACR),
    (
        "ADDR_MOD_AB_SEC24_SrcAClear",
        global::ADDR_MOD_AB_SEC24_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC24_SrcAIncr",
        global::ADDR_MOD_AB_SEC24_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC24_SrcBCR", global::ADDR_MOD_AB_SEC24_SrcBCR),
    (
        "ADDR_MOD_AB_SEC24_SrcBClear",
        global::ADDR_MOD_AB_SEC24_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC24_SrcBIncr",
        global::ADDR_MOD_AB_SEC24_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC25_SrcACR", global::ADDR_MOD_AB_SEC25_SrcACR),
    (
        "ADDR_MOD_AB_SEC25_SrcAClear",
        global::ADDR_MOD_AB_SEC25_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC25_SrcAIncr",
        global::ADDR_MOD_AB_SEC25_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC25_SrcBCR", global::ADDR_MOD_AB_SEC25_SrcBCR),
    (
        "ADDR_MOD_AB_SEC25_SrcBClear",
        global::ADDR_MOD_AB_SEC25_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC25_SrcBIncr",
        global::ADDR_MOD_AB_SEC25_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC26_SrcACR", global::ADDR_MOD_AB_SEC26_SrcACR),
    (
        "ADDR_MOD_AB_SEC26_SrcAClear",
        global::ADDR_MOD_AB_SEC26_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC26_SrcAIncr",
        global::ADDR_MOD_AB_SEC26_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC26_SrcBCR", global::ADDR_MOD_AB_SEC26_SrcBCR),
    (
        "ADDR_MOD_AB_SEC26_SrcBClear",
        global::ADDR_MOD_AB_SEC26_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC26_SrcBIncr",
        global::ADDR_MOD_AB_SEC26_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC27_SrcACR", global::ADDR_MOD_AB_SEC27_SrcACR),
    (
        "ADDR_MOD_AB_SEC27_SrcAClear",
        global::ADDR_MOD_AB_SEC27_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC27_SrcAIncr",
        global::ADDR_MOD_AB_SEC27_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC27_SrcBCR", global::ADDR_MOD_AB_SEC27_SrcBCR),
    (
        "ADDR_MOD_AB_SEC27_SrcBClear",
        global::ADDR_MOD_AB_SEC27_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC27_SrcBIncr",
        global::ADDR_MOD_AB_SEC27_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC28_SrcACR", global::ADDR_MOD_AB_SEC28_SrcACR),
    (
        "ADDR_MOD_AB_SEC28_SrcAClear",
        global::ADDR_MOD_AB_SEC28_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC28_SrcAIncr",
        global::ADDR_MOD_AB_SEC28_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC28_SrcBCR", global::ADDR_MOD_AB_SEC28_SrcBCR),
    (
        "ADDR_MOD_AB_SEC28_SrcBClear",
        global::ADDR_MOD_AB_SEC28_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC28_SrcBIncr",
        global::ADDR_MOD_AB_SEC28_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC29_SrcACR", global::ADDR_MOD_AB_SEC29_SrcACR),
    (
        "ADDR_MOD_AB_SEC29_SrcAClear",
        global::ADDR_MOD_AB_SEC29_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC29_SrcAIncr",
        global::ADDR_MOD_AB_SEC29_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC29_SrcBCR", global::ADDR_MOD_AB_SEC29_SrcBCR),
    (
        "ADDR_MOD_AB_SEC29_SrcBClear",
        global::ADDR_MOD_AB_SEC29_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC29_SrcBIncr",
        global::ADDR_MOD_AB_SEC29_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC2_SrcACR", global::ADDR_MOD_AB_SEC2_SrcACR),
    (
        "ADDR_MOD_AB_SEC2_SrcAClear",
        global::ADDR_MOD_AB_SEC2_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC2_SrcAIncr",
        global::ADDR_MOD_AB_SEC2_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC2_SrcBCR", global::ADDR_MOD_AB_SEC2_SrcBCR),
    (
        "ADDR_MOD_AB_SEC2_SrcBClear",
        global::ADDR_MOD_AB_SEC2_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC2_SrcBIncr",
        global::ADDR_MOD_AB_SEC2_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC30_SrcACR", global::ADDR_MOD_AB_SEC30_SrcACR),
    (
        "ADDR_MOD_AB_SEC30_SrcAClear",
        global::ADDR_MOD_AB_SEC30_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC30_SrcAIncr",
        global::ADDR_MOD_AB_SEC30_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC30_SrcBCR", global::ADDR_MOD_AB_SEC30_SrcBCR),
    (
        "ADDR_MOD_AB_SEC30_SrcBClear",
        global::ADDR_MOD_AB_SEC30_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC30_SrcBIncr",
        global::ADDR_MOD_AB_SEC30_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC31_SrcACR", global::ADDR_MOD_AB_SEC31_SrcACR),
    (
        "ADDR_MOD_AB_SEC31_SrcAClear",
        global::ADDR_MOD_AB_SEC31_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC31_SrcAIncr",
        global::ADDR_MOD_AB_SEC31_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC31_SrcBCR", global::ADDR_MOD_AB_SEC31_SrcBCR),
    (
        "ADDR_MOD_AB_SEC31_SrcBClear",
        global::ADDR_MOD_AB_SEC31_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC31_SrcBIncr",
        global::ADDR_MOD_AB_SEC31_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC3_SrcACR", global::ADDR_MOD_AB_SEC3_SrcACR),
    (
        "ADDR_MOD_AB_SEC3_SrcAClear",
        global::ADDR_MOD_AB_SEC3_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC3_SrcAIncr",
        global::ADDR_MOD_AB_SEC3_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC3_SrcBCR", global::ADDR_MOD_AB_SEC3_SrcBCR),
    (
        "ADDR_MOD_AB_SEC3_SrcBClear",
        global::ADDR_MOD_AB_SEC3_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC3_SrcBIncr",
        global::ADDR_MOD_AB_SEC3_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC4_SrcACR", global::ADDR_MOD_AB_SEC4_SrcACR),
    (
        "ADDR_MOD_AB_SEC4_SrcAClear",
        global::ADDR_MOD_AB_SEC4_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC4_SrcAIncr",
        global::ADDR_MOD_AB_SEC4_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC4_SrcBCR", global::ADDR_MOD_AB_SEC4_SrcBCR),
    (
        "ADDR_MOD_AB_SEC4_SrcBClear",
        global::ADDR_MOD_AB_SEC4_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC4_SrcBIncr",
        global::ADDR_MOD_AB_SEC4_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC5_SrcACR", global::ADDR_MOD_AB_SEC5_SrcACR),
    (
        "ADDR_MOD_AB_SEC5_SrcAClear",
        global::ADDR_MOD_AB_SEC5_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC5_SrcAIncr",
        global::ADDR_MOD_AB_SEC5_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC5_SrcBCR", global::ADDR_MOD_AB_SEC5_SrcBCR),
    (
        "ADDR_MOD_AB_SEC5_SrcBClear",
        global::ADDR_MOD_AB_SEC5_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC5_SrcBIncr",
        global::ADDR_MOD_AB_SEC5_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC6_SrcACR", global::ADDR_MOD_AB_SEC6_SrcACR),
    (
        "ADDR_MOD_AB_SEC6_SrcAClear",
        global::ADDR_MOD_AB_SEC6_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC6_SrcAIncr",
        global::ADDR_MOD_AB_SEC6_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC6_SrcBCR", global::ADDR_MOD_AB_SEC6_SrcBCR),
    (
        "ADDR_MOD_AB_SEC6_SrcBClear",
        global::ADDR_MOD_AB_SEC6_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC6_SrcBIncr",
        global::ADDR_MOD_AB_SEC6_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC7_SrcACR", global::ADDR_MOD_AB_SEC7_SrcACR),
    (
        "ADDR_MOD_AB_SEC7_SrcAClear",
        global::ADDR_MOD_AB_SEC7_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC7_SrcAIncr",
        global::ADDR_MOD_AB_SEC7_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC7_SrcBCR", global::ADDR_MOD_AB_SEC7_SrcBCR),
    (
        "ADDR_MOD_AB_SEC7_SrcBClear",
        global::ADDR_MOD_AB_SEC7_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC7_SrcBIncr",
        global::ADDR_MOD_AB_SEC7_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC8_SrcACR", global::ADDR_MOD_AB_SEC8_SrcACR),
    (
        "ADDR_MOD_AB_SEC8_SrcAClear",
        global::ADDR_MOD_AB_SEC8_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC8_SrcAIncr",
        global::ADDR_MOD_AB_SEC8_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC8_SrcBCR", global::ADDR_MOD_AB_SEC8_SrcBCR),
    (
        "ADDR_MOD_AB_SEC8_SrcBClear",
        global::ADDR_MOD_AB_SEC8_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC8_SrcBIncr",
        global::ADDR_MOD_AB_SEC8_SrcBIncr,
    ),
    ("ADDR_MOD_AB_SEC9_SrcACR", global::ADDR_MOD_AB_SEC9_SrcACR),
    (
        "ADDR_MOD_AB_SEC9_SrcAClear",
        global::ADDR_MOD_AB_SEC9_SrcAClear,
    ),
    (
        "ADDR_MOD_AB_SEC9_SrcAIncr",
        global::ADDR_MOD_AB_SEC9_SrcAIncr,
    ),
    ("ADDR_MOD_AB_SEC9_SrcBCR", global::ADDR_MOD_AB_SEC9_SrcBCR),
    (
        "ADDR_MOD_AB_SEC9_SrcBClear",
        global::ADDR_MOD_AB_SEC9_SrcBClear,
    ),
    (
        "ADDR_MOD_AB_SEC9_SrcBIncr",
        global::ADDR_MOD_AB_SEC9_SrcBIncr,
    ),
    ("ADDR_MOD_DST_SEC0_DestCR", global::ADDR_MOD_DST_SEC0_DestCR),
    (
        "ADDR_MOD_DST_SEC0_DestCToCR",
        global::ADDR_MOD_DST_SEC0_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC0_DestClear",
        global::ADDR_MOD_DST_SEC0_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC0_DestIncr",
        global::ADDR_MOD_DST_SEC0_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC0_FidelityClear",
        global::ADDR_MOD_DST_SEC0_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC0_FidelityIncr",
        global::ADDR_MOD_DST_SEC0_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestCR",
        global::ADDR_MOD_DST_SEC10_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestCToCR",
        global::ADDR_MOD_DST_SEC10_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestClear",
        global::ADDR_MOD_DST_SEC10_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC10_DestIncr",
        global::ADDR_MOD_DST_SEC10_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC10_FidelityClear",
        global::ADDR_MOD_DST_SEC10_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC10_FidelityIncr",
        global::ADDR_MOD_DST_SEC10_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestCR",
        global::ADDR_MOD_DST_SEC11_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestCToCR",
        global::ADDR_MOD_DST_SEC11_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestClear",
        global::ADDR_MOD_DST_SEC11_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC11_DestIncr",
        global::ADDR_MOD_DST_SEC11_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC11_FidelityClear",
        global::ADDR_MOD_DST_SEC11_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC11_FidelityIncr",
        global::ADDR_MOD_DST_SEC11_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestCR",
        global::ADDR_MOD_DST_SEC12_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestCToCR",
        global::ADDR_MOD_DST_SEC12_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestClear",
        global::ADDR_MOD_DST_SEC12_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC12_DestIncr",
        global::ADDR_MOD_DST_SEC12_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC12_FidelityClear",
        global::ADDR_MOD_DST_SEC12_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC12_FidelityIncr",
        global::ADDR_MOD_DST_SEC12_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestCR",
        global::ADDR_MOD_DST_SEC13_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestCToCR",
        global::ADDR_MOD_DST_SEC13_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestClear",
        global::ADDR_MOD_DST_SEC13_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC13_DestIncr",
        global::ADDR_MOD_DST_SEC13_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC13_FidelityClear",
        global::ADDR_MOD_DST_SEC13_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC13_FidelityIncr",
        global::ADDR_MOD_DST_SEC13_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestCR",
        global::ADDR_MOD_DST_SEC14_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestCToCR",
        global::ADDR_MOD_DST_SEC14_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestClear",
        global::ADDR_MOD_DST_SEC14_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC14_DestIncr",
        global::ADDR_MOD_DST_SEC14_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC14_FidelityClear",
        global::ADDR_MOD_DST_SEC14_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC14_FidelityIncr",
        global::ADDR_MOD_DST_SEC14_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestCR",
        global::ADDR_MOD_DST_SEC15_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestCToCR",
        global::ADDR_MOD_DST_SEC15_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestClear",
        global::ADDR_MOD_DST_SEC15_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC15_DestIncr",
        global::ADDR_MOD_DST_SEC15_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC15_FidelityClear",
        global::ADDR_MOD_DST_SEC15_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC15_FidelityIncr",
        global::ADDR_MOD_DST_SEC15_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestCR",
        global::ADDR_MOD_DST_SEC16_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestCToCR",
        global::ADDR_MOD_DST_SEC16_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestClear",
        global::ADDR_MOD_DST_SEC16_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC16_DestIncr",
        global::ADDR_MOD_DST_SEC16_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC16_FidelityClear",
        global::ADDR_MOD_DST_SEC16_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC16_FidelityIncr",
        global::ADDR_MOD_DST_SEC16_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestCR",
        global::ADDR_MOD_DST_SEC17_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestCToCR",
        global::ADDR_MOD_DST_SEC17_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestClear",
        global::ADDR_MOD_DST_SEC17_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC17_DestIncr",
        global::ADDR_MOD_DST_SEC17_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC17_FidelityClear",
        global::ADDR_MOD_DST_SEC17_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC17_FidelityIncr",
        global::ADDR_MOD_DST_SEC17_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestCR",
        global::ADDR_MOD_DST_SEC18_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestCToCR",
        global::ADDR_MOD_DST_SEC18_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestClear",
        global::ADDR_MOD_DST_SEC18_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC18_DestIncr",
        global::ADDR_MOD_DST_SEC18_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC18_FidelityClear",
        global::ADDR_MOD_DST_SEC18_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC18_FidelityIncr",
        global::ADDR_MOD_DST_SEC18_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestCR",
        global::ADDR_MOD_DST_SEC19_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestCToCR",
        global::ADDR_MOD_DST_SEC19_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestClear",
        global::ADDR_MOD_DST_SEC19_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC19_DestIncr",
        global::ADDR_MOD_DST_SEC19_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC19_FidelityClear",
        global::ADDR_MOD_DST_SEC19_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC19_FidelityIncr",
        global::ADDR_MOD_DST_SEC19_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC1_DestCR", global::ADDR_MOD_DST_SEC1_DestCR),
    (
        "ADDR_MOD_DST_SEC1_DestCToCR",
        global::ADDR_MOD_DST_SEC1_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC1_DestClear",
        global::ADDR_MOD_DST_SEC1_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC1_DestIncr",
        global::ADDR_MOD_DST_SEC1_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC1_FidelityClear",
        global::ADDR_MOD_DST_SEC1_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC1_FidelityIncr",
        global::ADDR_MOD_DST_SEC1_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestCR",
        global::ADDR_MOD_DST_SEC20_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestCToCR",
        global::ADDR_MOD_DST_SEC20_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestClear",
        global::ADDR_MOD_DST_SEC20_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC20_DestIncr",
        global::ADDR_MOD_DST_SEC20_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC20_FidelityClear",
        global::ADDR_MOD_DST_SEC20_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC20_FidelityIncr",
        global::ADDR_MOD_DST_SEC20_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestCR",
        global::ADDR_MOD_DST_SEC21_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestCToCR",
        global::ADDR_MOD_DST_SEC21_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestClear",
        global::ADDR_MOD_DST_SEC21_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC21_DestIncr",
        global::ADDR_MOD_DST_SEC21_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC21_FidelityClear",
        global::ADDR_MOD_DST_SEC21_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC21_FidelityIncr",
        global::ADDR_MOD_DST_SEC21_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestCR",
        global::ADDR_MOD_DST_SEC22_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestCToCR",
        global::ADDR_MOD_DST_SEC22_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestClear",
        global::ADDR_MOD_DST_SEC22_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC22_DestIncr",
        global::ADDR_MOD_DST_SEC22_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC22_FidelityClear",
        global::ADDR_MOD_DST_SEC22_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC22_FidelityIncr",
        global::ADDR_MOD_DST_SEC22_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestCR",
        global::ADDR_MOD_DST_SEC23_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestCToCR",
        global::ADDR_MOD_DST_SEC23_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestClear",
        global::ADDR_MOD_DST_SEC23_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC23_DestIncr",
        global::ADDR_MOD_DST_SEC23_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC23_FidelityClear",
        global::ADDR_MOD_DST_SEC23_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC23_FidelityIncr",
        global::ADDR_MOD_DST_SEC23_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestCR",
        global::ADDR_MOD_DST_SEC24_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestCToCR",
        global::ADDR_MOD_DST_SEC24_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestClear",
        global::ADDR_MOD_DST_SEC24_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC24_DestIncr",
        global::ADDR_MOD_DST_SEC24_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC24_FidelityClear",
        global::ADDR_MOD_DST_SEC24_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC24_FidelityIncr",
        global::ADDR_MOD_DST_SEC24_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestCR",
        global::ADDR_MOD_DST_SEC25_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestCToCR",
        global::ADDR_MOD_DST_SEC25_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestClear",
        global::ADDR_MOD_DST_SEC25_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC25_DestIncr",
        global::ADDR_MOD_DST_SEC25_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC25_FidelityClear",
        global::ADDR_MOD_DST_SEC25_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC25_FidelityIncr",
        global::ADDR_MOD_DST_SEC25_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestCR",
        global::ADDR_MOD_DST_SEC26_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestCToCR",
        global::ADDR_MOD_DST_SEC26_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestClear",
        global::ADDR_MOD_DST_SEC26_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC26_DestIncr",
        global::ADDR_MOD_DST_SEC26_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC26_FidelityClear",
        global::ADDR_MOD_DST_SEC26_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC26_FidelityIncr",
        global::ADDR_MOD_DST_SEC26_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestCR",
        global::ADDR_MOD_DST_SEC27_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestCToCR",
        global::ADDR_MOD_DST_SEC27_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestClear",
        global::ADDR_MOD_DST_SEC27_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC27_DestIncr",
        global::ADDR_MOD_DST_SEC27_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC27_FidelityClear",
        global::ADDR_MOD_DST_SEC27_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC27_FidelityIncr",
        global::ADDR_MOD_DST_SEC27_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestCR",
        global::ADDR_MOD_DST_SEC28_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestCToCR",
        global::ADDR_MOD_DST_SEC28_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestClear",
        global::ADDR_MOD_DST_SEC28_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC28_DestIncr",
        global::ADDR_MOD_DST_SEC28_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC28_FidelityClear",
        global::ADDR_MOD_DST_SEC28_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC28_FidelityIncr",
        global::ADDR_MOD_DST_SEC28_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestCR",
        global::ADDR_MOD_DST_SEC29_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestCToCR",
        global::ADDR_MOD_DST_SEC29_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestClear",
        global::ADDR_MOD_DST_SEC29_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC29_DestIncr",
        global::ADDR_MOD_DST_SEC29_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC29_FidelityClear",
        global::ADDR_MOD_DST_SEC29_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC29_FidelityIncr",
        global::ADDR_MOD_DST_SEC29_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC2_DestCR", global::ADDR_MOD_DST_SEC2_DestCR),
    (
        "ADDR_MOD_DST_SEC2_DestCToCR",
        global::ADDR_MOD_DST_SEC2_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC2_DestClear",
        global::ADDR_MOD_DST_SEC2_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC2_DestIncr",
        global::ADDR_MOD_DST_SEC2_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC2_FidelityClear",
        global::ADDR_MOD_DST_SEC2_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC2_FidelityIncr",
        global::ADDR_MOD_DST_SEC2_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestCR",
        global::ADDR_MOD_DST_SEC30_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestCToCR",
        global::ADDR_MOD_DST_SEC30_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestClear",
        global::ADDR_MOD_DST_SEC30_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC30_DestIncr",
        global::ADDR_MOD_DST_SEC30_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC30_FidelityClear",
        global::ADDR_MOD_DST_SEC30_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC30_FidelityIncr",
        global::ADDR_MOD_DST_SEC30_FidelityIncr,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestCR",
        global::ADDR_MOD_DST_SEC31_DestCR,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestCToCR",
        global::ADDR_MOD_DST_SEC31_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestClear",
        global::ADDR_MOD_DST_SEC31_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC31_DestIncr",
        global::ADDR_MOD_DST_SEC31_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC31_FidelityClear",
        global::ADDR_MOD_DST_SEC31_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC31_FidelityIncr",
        global::ADDR_MOD_DST_SEC31_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC3_DestCR", global::ADDR_MOD_DST_SEC3_DestCR),
    (
        "ADDR_MOD_DST_SEC3_DestCToCR",
        global::ADDR_MOD_DST_SEC3_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC3_DestClear",
        global::ADDR_MOD_DST_SEC3_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC3_DestIncr",
        global::ADDR_MOD_DST_SEC3_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC3_FidelityClear",
        global::ADDR_MOD_DST_SEC3_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC3_FidelityIncr",
        global::ADDR_MOD_DST_SEC3_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC4_DestCR", global::ADDR_MOD_DST_SEC4_DestCR),
    (
        "ADDR_MOD_DST_SEC4_DestCToCR",
        global::ADDR_MOD_DST_SEC4_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC4_DestClear",
        global::ADDR_MOD_DST_SEC4_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC4_DestIncr",
        global::ADDR_MOD_DST_SEC4_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC4_FidelityClear",
        global::ADDR_MOD_DST_SEC4_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC4_FidelityIncr",
        global::ADDR_MOD_DST_SEC4_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC5_DestCR", global::ADDR_MOD_DST_SEC5_DestCR),
    (
        "ADDR_MOD_DST_SEC5_DestCToCR",
        global::ADDR_MOD_DST_SEC5_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC5_DestClear",
        global::ADDR_MOD_DST_SEC5_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC5_DestIncr",
        global::ADDR_MOD_DST_SEC5_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC5_FidelityClear",
        global::ADDR_MOD_DST_SEC5_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC5_FidelityIncr",
        global::ADDR_MOD_DST_SEC5_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC6_DestCR", global::ADDR_MOD_DST_SEC6_DestCR),
    (
        "ADDR_MOD_DST_SEC6_DestCToCR",
        global::ADDR_MOD_DST_SEC6_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC6_DestClear",
        global::ADDR_MOD_DST_SEC6_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC6_DestIncr",
        global::ADDR_MOD_DST_SEC6_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC6_FidelityClear",
        global::ADDR_MOD_DST_SEC6_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC6_FidelityIncr",
        global::ADDR_MOD_DST_SEC6_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC7_DestCR", global::ADDR_MOD_DST_SEC7_DestCR),
    (
        "ADDR_MOD_DST_SEC7_DestCToCR",
        global::ADDR_MOD_DST_SEC7_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC7_DestClear",
        global::ADDR_MOD_DST_SEC7_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC7_DestIncr",
        global::ADDR_MOD_DST_SEC7_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC7_FidelityClear",
        global::ADDR_MOD_DST_SEC7_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC7_FidelityIncr",
        global::ADDR_MOD_DST_SEC7_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC8_DestCR", global::ADDR_MOD_DST_SEC8_DestCR),
    (
        "ADDR_MOD_DST_SEC8_DestCToCR",
        global::ADDR_MOD_DST_SEC8_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC8_DestClear",
        global::ADDR_MOD_DST_SEC8_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC8_DestIncr",
        global::ADDR_MOD_DST_SEC8_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC8_FidelityClear",
        global::ADDR_MOD_DST_SEC8_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC8_FidelityIncr",
        global::ADDR_MOD_DST_SEC8_FidelityIncr,
    ),
    ("ADDR_MOD_DST_SEC9_DestCR", global::ADDR_MOD_DST_SEC9_DestCR),
    (
        "ADDR_MOD_DST_SEC9_DestCToCR",
        global::ADDR_MOD_DST_SEC9_DestCToCR,
    ),
    (
        "ADDR_MOD_DST_SEC9_DestClear",
        global::ADDR_MOD_DST_SEC9_DestClear,
    ),
    (
        "ADDR_MOD_DST_SEC9_DestIncr",
        global::ADDR_MOD_DST_SEC9_DestIncr,
    ),
    (
        "ADDR_MOD_DST_SEC9_FidelityClear",
        global::ADDR_MOD_DST_SEC9_FidelityClear,
    ),
    (
        "ADDR_MOD_DST_SEC9_FidelityIncr",
        global::ADDR_MOD_DST_SEC9_FidelityIncr,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG0_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG10_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG11_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG12_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG13_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG14_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG15_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG16_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG17_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG18_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG19_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG1_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG20_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG21_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG22_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG23_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG24_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG25_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG26_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG27_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG28_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG29_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG2_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG30_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG31_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG3_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG4_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG5_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG6_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG7_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG8_UNUSED3,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_L1_BASE_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_L1_BASE_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_L1_LIMIT_ADDR",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_L1_LIMIT_ADDR,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_FORMAT",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_FORMAT,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_X_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_X_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Y_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Y_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Z_DIM",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_TILE_Z_DIM,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED0",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED0,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED1",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED1,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED2",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED2,
    ),
    (
        "BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED3",
        global::BUFFER_DESCRIPTOR_TABLE_REG9_UNUSED3,
    ),
    ("CFG_WATCHPOINTS_0_enable", global::CFG_WATCHPOINTS_0_enable),
    (
        "CFG_WATCHPOINTS_0_hi_addr",
        global::CFG_WATCHPOINTS_0_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_0_lo_addr",
        global::CFG_WATCHPOINTS_0_lo_addr,
    ),
    ("CFG_WATCHPOINTS_0_rsvd", global::CFG_WATCHPOINTS_0_rsvd),
    ("CFG_WATCHPOINTS_1_enable", global::CFG_WATCHPOINTS_1_enable),
    (
        "CFG_WATCHPOINTS_1_hi_addr",
        global::CFG_WATCHPOINTS_1_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_1_lo_addr",
        global::CFG_WATCHPOINTS_1_lo_addr,
    ),
    ("CFG_WATCHPOINTS_1_rsvd", global::CFG_WATCHPOINTS_1_rsvd),
    ("CFG_WATCHPOINTS_2_enable", global::CFG_WATCHPOINTS_2_enable),
    (
        "CFG_WATCHPOINTS_2_hi_addr",
        global::CFG_WATCHPOINTS_2_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_2_lo_addr",
        global::CFG_WATCHPOINTS_2_lo_addr,
    ),
    ("CFG_WATCHPOINTS_2_rsvd", global::CFG_WATCHPOINTS_2_rsvd),
    ("CFG_WATCHPOINTS_3_enable", global::CFG_WATCHPOINTS_3_enable),
    (
        "CFG_WATCHPOINTS_3_hi_addr",
        global::CFG_WATCHPOINTS_3_hi_addr,
    ),
    (
        "CFG_WATCHPOINTS_3_lo_addr",
        global::CFG_WATCHPOINTS_3_lo_addr,
    ),
    ("CFG_WATCHPOINTS_3_rsvd", global::CFG_WATCHPOINTS_3_rsvd),
    (
        "CLR_DVALID_SEC0_SrcA_Disable",
        global::CLR_DVALID_SEC0_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC0_SrcB_Disable",
        global::CLR_DVALID_SEC0_SrcB_Disable,
    ),
    (
        "CLR_DVALID_SEC1_SrcA_Disable",
        global::CLR_DVALID_SEC1_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC1_SrcB_Disable",
        global::CLR_DVALID_SEC1_SrcB_Disable,
    ),
    (
        "CLR_DVALID_SEC2_SrcA_Disable",
        global::CLR_DVALID_SEC2_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC2_SrcB_Disable",
        global::CLR_DVALID_SEC2_SrcB_Disable,
    ),
    (
        "CLR_DVALID_SEC3_SrcA_Disable",
        global::CLR_DVALID_SEC3_SrcA_Disable,
    ),
    (
        "CLR_DVALID_SEC3_SrcB_Disable",
        global::CLR_DVALID_SEC3_SrcB_Disable,
    ),
    (
        "DEST_ACCESS_CFG_zeroacc_absolute_tile_mode",
        global::DEST_ACCESS_CFG_zeroacc_absolute_tile_mode,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC0_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC0_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC1_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC1_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC2_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC2_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCA_FMT_SEC3_Base",
        global::DISABLE_IMPLIED_SRCA_FMT_SEC3_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC0_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC0_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC1_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC1_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC2_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC2_Base,
    ),
    (
        "DISABLE_IMPLIED_SRCB_FMT_SEC3_Base",
        global::DISABLE_IMPLIED_SRCB_FMT_SEC3_Base,
    ),
    (
        "ENABLE_ACC_STATS_SEC0_Enable",
        global::ENABLE_ACC_STATS_SEC0_Enable,
    ),
    (
        "ENABLE_ACC_STATS_SEC1_Enable",
        global::ENABLE_ACC_STATS_SEC1_Enable,
    ),
    (
        "ENABLE_ACC_STATS_SEC2_Enable",
        global::ENABLE_ACC_STATS_SEC2_Enable,
    ),
    (
        "ENABLE_ACC_STATS_SEC3_Enable",
        global::ENABLE_ACC_STATS_SEC3_Enable,
    ),
    ("FIDELITY_BASE_SEC0_Phase", global::FIDELITY_BASE_SEC0_Phase),
    ("FIDELITY_BASE_SEC1_Phase", global::FIDELITY_BASE_SEC1_Phase),
    ("FIDELITY_BASE_SEC2_Phase", global::FIDELITY_BASE_SEC2_Phase),
    ("FIDELITY_BASE_SEC3_Phase", global::FIDELITY_BASE_SEC3_Phase),
    ("FMT_CTRL_FP8_OVF_EN", global::FMT_CTRL_FP8_OVF_EN),
    (
        "FMT_CTRL_MXINT_NEGZERO_EQ_NEGINF",
        global::FMT_CTRL_MXINT_NEGZERO_EQ_NEGINF,
    ),
    (
        "FMT_CTRL_MX_BLOCK_EXP_RND_TO_INF",
        global::FMT_CTRL_MX_BLOCK_EXP_RND_TO_INF,
    ),
    (
        "FMT_CTRL_RESET_PACKER0_EXP_STATS",
        global::FMT_CTRL_RESET_PACKER0_EXP_STATS,
    ),
    (
        "FMT_CTRL_RESET_PACKER1_EXP_STATS",
        global::FMT_CTRL_RESET_PACKER1_EXP_STATS,
    ),
    ("FP16A_FORCE_SEC0_Enable", global::FP16A_FORCE_SEC0_Enable),
    ("FP16A_FORCE_SEC1_Enable", global::FP16A_FORCE_SEC1_Enable),
    ("FP16A_FORCE_SEC2_Enable", global::FP16A_FORCE_SEC2_Enable),
    ("FP16A_FORCE_SEC3_Enable", global::FP16A_FORCE_SEC3_Enable),
    (
        "INT_DESCALE_VALUES_SEC0_Value",
        global::INT_DESCALE_VALUES_SEC0_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC10_Value",
        global::INT_DESCALE_VALUES_SEC10_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC11_Value",
        global::INT_DESCALE_VALUES_SEC11_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC12_Value",
        global::INT_DESCALE_VALUES_SEC12_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC13_Value",
        global::INT_DESCALE_VALUES_SEC13_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC14_Value",
        global::INT_DESCALE_VALUES_SEC14_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC15_Value",
        global::INT_DESCALE_VALUES_SEC15_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC1_Value",
        global::INT_DESCALE_VALUES_SEC1_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC2_Value",
        global::INT_DESCALE_VALUES_SEC2_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC3_Value",
        global::INT_DESCALE_VALUES_SEC3_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC4_Value",
        global::INT_DESCALE_VALUES_SEC4_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC5_Value",
        global::INT_DESCALE_VALUES_SEC5_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC6_Value",
        global::INT_DESCALE_VALUES_SEC6_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC7_Value",
        global::INT_DESCALE_VALUES_SEC7_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC8_Value",
        global::INT_DESCALE_VALUES_SEC8_Value,
    ),
    (
        "INT_DESCALE_VALUES_SEC9_Value",
        global::INT_DESCALE_VALUES_SEC9_Value,
    ),
    (
        "MATH_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::MATH_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "MATH_DEST_DVALID_CTRL_toggle_mask",
        global::MATH_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "MATH_DEST_DVALID_CTRL_wait_mask",
        global::MATH_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "MATH_DEST_DVALID_CTRL_wait_polarity",
        global::MATH_DEST_DVALID_CTRL_wait_polarity,
    ),
    (
        "PACK_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::PACK_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "PACK_DEST_DVALID_CTRL_toggle_mask",
        global::PACK_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "PACK_DEST_DVALID_CTRL_wait_mask",
        global::PACK_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "PACK_DEST_DVALID_CTRL_wait_polarity",
        global::PACK_DEST_DVALID_CTRL_wait_polarity,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd0Start",
        global::PERF_CNT_CMD_SEC0_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd0Stop",
        global::PERF_CNT_CMD_SEC0_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd1Start",
        global::PERF_CNT_CMD_SEC0_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd1Stop",
        global::PERF_CNT_CMD_SEC0_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd2Start",
        global::PERF_CNT_CMD_SEC0_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd2Stop",
        global::PERF_CNT_CMD_SEC0_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd3Start",
        global::PERF_CNT_CMD_SEC0_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC0_Cmd3Stop",
        global::PERF_CNT_CMD_SEC0_Cmd3Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd0Start",
        global::PERF_CNT_CMD_SEC1_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd0Stop",
        global::PERF_CNT_CMD_SEC1_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd1Start",
        global::PERF_CNT_CMD_SEC1_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd1Stop",
        global::PERF_CNT_CMD_SEC1_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd2Start",
        global::PERF_CNT_CMD_SEC1_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd2Stop",
        global::PERF_CNT_CMD_SEC1_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd3Start",
        global::PERF_CNT_CMD_SEC1_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC1_Cmd3Stop",
        global::PERF_CNT_CMD_SEC1_Cmd3Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd0Start",
        global::PERF_CNT_CMD_SEC2_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd0Stop",
        global::PERF_CNT_CMD_SEC2_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd1Start",
        global::PERF_CNT_CMD_SEC2_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd1Stop",
        global::PERF_CNT_CMD_SEC2_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd2Start",
        global::PERF_CNT_CMD_SEC2_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd2Stop",
        global::PERF_CNT_CMD_SEC2_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd3Start",
        global::PERF_CNT_CMD_SEC2_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC2_Cmd3Stop",
        global::PERF_CNT_CMD_SEC2_Cmd3Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd0Start",
        global::PERF_CNT_CMD_SEC3_Cmd0Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd0Stop",
        global::PERF_CNT_CMD_SEC3_Cmd0Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd1Start",
        global::PERF_CNT_CMD_SEC3_Cmd1Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd1Stop",
        global::PERF_CNT_CMD_SEC3_Cmd1Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd2Start",
        global::PERF_CNT_CMD_SEC3_Cmd2Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd2Stop",
        global::PERF_CNT_CMD_SEC3_Cmd2Stop,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd3Start",
        global::PERF_CNT_CMD_SEC3_Cmd3Start,
    ),
    (
        "PERF_CNT_CMD_SEC3_Cmd3Stop",
        global::PERF_CNT_CMD_SEC3_Cmd3Stop,
    ),
    (
        "PRNG_SEED_OVERRIDE_CTRL_override_en",
        global::PRNG_SEED_OVERRIDE_CTRL_override_en,
    ),
    (
        "PRNG_SEED_OVERRIDE_CTRL_prng_id",
        global::PRNG_SEED_OVERRIDE_CTRL_prng_id,
    ),
    (
        "PRNG_SEED_OVERRIDE_DATA_seed_override",
        global::PRNG_SEED_OVERRIDE_DATA_seed_override,
    ),
    ("PRNG_SEED_Seed_Val", global::PRNG_SEED_Seed_Val),
    ("SCRATCH_SEC0_val", global::SCRATCH_SEC0_val),
    ("SCRATCH_SEC1_val", global::SCRATCH_SEC1_val),
    ("SCRATCH_SEC2_val", global::SCRATCH_SEC2_val),
    ("SCRATCH_SEC3_val", global::SCRATCH_SEC3_val),
    (
        "SFPU_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::SFPU_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "SFPU_DEST_DVALID_CTRL_toggle_mask",
        global::SFPU_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "SFPU_DEST_DVALID_CTRL_wait_mask",
        global::SFPU_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "SFPU_DEST_DVALID_CTRL_wait_polarity",
        global::SFPU_DEST_DVALID_CTRL_wait_polarity,
    ),
    ("SFPU_DEST_FMT_SEC0_Base", global::SFPU_DEST_FMT_SEC0_Base),
    (
        "SFPU_DEST_FMT_SEC0_Enable",
        global::SFPU_DEST_FMT_SEC0_Enable,
    ),
    ("SFPU_DEST_FMT_SEC1_Base", global::SFPU_DEST_FMT_SEC1_Base),
    (
        "SFPU_DEST_FMT_SEC1_Enable",
        global::SFPU_DEST_FMT_SEC1_Enable,
    ),
    ("SFPU_DEST_FMT_SEC2_Base", global::SFPU_DEST_FMT_SEC2_Base),
    (
        "SFPU_DEST_FMT_SEC2_Enable",
        global::SFPU_DEST_FMT_SEC2_Enable,
    ),
    ("SFPU_DEST_FMT_SEC3_Base", global::SFPU_DEST_FMT_SEC3_Base),
    (
        "SFPU_DEST_FMT_SEC3_Enable",
        global::SFPU_DEST_FMT_SEC3_Enable,
    ),
    ("SFPU_STACK_SEC0_Incr", global::SFPU_STACK_SEC0_Incr),
    ("SFPU_STACK_SEC1_Incr", global::SFPU_STACK_SEC1_Incr),
    ("SFPU_STACK_SEC2_Incr", global::SFPU_STACK_SEC2_Incr),
    ("SFPU_STACK_SEC3_Incr", global::SFPU_STACK_SEC3_Incr),
    (
        "SRC_ACCESS_CFG_disable_contig_srca_dvalid_phase",
        global::SRC_ACCESS_CFG_disable_contig_srca_dvalid_phase,
    ),
    (
        "SRC_ACCESS_CFG_disable_contig_srcb_dvalid_phase",
        global::SRC_ACCESS_CFG_disable_contig_srcb_dvalid_phase,
    ),
    (
        "SRC_ACCESS_CFG_math_view_srca_as_one_bank",
        global::SRC_ACCESS_CFG_math_view_srca_as_one_bank,
    ),
    (
        "SRC_ACCESS_CFG_math_view_srcb_as_one_bank",
        global::SRC_ACCESS_CFG_math_view_srcb_as_one_bank,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_disable_auto_bank_id_toggle",
        global::UNPACK_TO_DEST_DVALID_CTRL_disable_auto_bank_id_toggle,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_toggle_mask",
        global::UNPACK_TO_DEST_DVALID_CTRL_toggle_mask,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_wait_mask",
        global::UNPACK_TO_DEST_DVALID_CTRL_wait_mask,
    ),
    (
        "UNPACK_TO_DEST_DVALID_CTRL_wait_polarity",
        global::UNPACK_TO_DEST_DVALID_CTRL_wait_polarity,
    ),
];

pub static THCON_FIELDS: &[(&str, ConfigField)] = &[
    (
        "THCON_PACKER0_REG0_DEST_REG_WRAP_MODE",
        thcon::THCON_PACKER0_REG0_DEST_REG_WRAP_MODE,
    ),
    (
        "THCON_PACKER0_REG0_EXP_THRESHOLD",
        thcon::THCON_PACKER0_REG0_EXP_THRESHOLD,
    ),
    (
        "THCON_PACKER0_REG0_EXP_THRESHOLD_EN",
        thcon::THCON_PACKER0_REG0_EXP_THRESHOLD_EN,
    ),
    (
        "THCON_PACKER0_REG0_INT_DESCALE_ENABLE",
        thcon::THCON_PACKER0_REG0_INT_DESCALE_ENABLE,
    ),
    (
        "THCON_PACKER0_REG0_INT_DESCALE_MODE",
        thcon::THCON_PACKER0_REG0_INT_DESCALE_MODE,
    ),
    (
        "THCON_PACKER0_REG0_IN_DATA_FORMAT",
        thcon::THCON_PACKER0_REG0_IN_DATA_FORMAT,
    ),
    (
        "THCON_PACKER0_REG0_L1_ACC",
        thcon::THCON_PACKER0_REG0_L1_ACC,
    ),
    (
        "THCON_PACKER0_REG0_READ_MODE",
        thcon::THCON_PACKER0_REG0_READ_MODE,
    ),
    (
        "THCON_PACKER0_REG0_SELF_CHECK_EN",
        thcon::THCON_PACKER0_REG0_SELF_CHECK_EN,
    ),
    (
        "THCON_PACKER0_REG0_SRC_ADDR_OFFSET",
        thcon::THCON_PACKER0_REG0_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_PACKER0_REG0_STOCH_RND_EN",
        thcon::THCON_PACKER0_REG0_STOCH_RND_EN,
    ),
    (
        "THCON_PACKER0_REG0_UNTILIZE_DST_ADDR_OFFSET",
        thcon::THCON_PACKER0_REG0_UNTILIZE_DST_ADDR_OFFSET,
    ),
    (
        "THCON_PACKER0_REG0_UNUSED0",
        thcon::THCON_PACKER0_REG0_UNUSED0,
    ),
    (
        "THCON_PACKER0_REG0_UNUSED1",
        thcon::THCON_PACKER0_REG0_UNUSED1,
    ),
    (
        "THCON_PACKER0_REG0_UNUSED2",
        thcon::THCON_PACKER0_REG0_UNUSED2,
    ),
    (
        "THCON_PACKER0_REG0_WRAP_TILE_DIM",
        thcon::THCON_PACKER0_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_PACKER0_REG0_ZERO_WRITE",
        thcon::THCON_PACKER0_REG0_ZERO_WRITE,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK0",
        thcon::THCON_PACKER0_REG1_EDGE_MASK0,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK1",
        thcon::THCON_PACKER0_REG1_EDGE_MASK1,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK2",
        thcon::THCON_PACKER0_REG1_EDGE_MASK2,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK3",
        thcon::THCON_PACKER0_REG1_EDGE_MASK3,
    ),
    (
        "THCON_PACKER0_REG1_EDGE_MASK_MODE",
        thcon::THCON_PACKER0_REG1_EDGE_MASK_MODE,
    ),
    (
        "THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_0",
        thcon::THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_1",
        thcon::THCON_PACKER0_REG1_PACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_PACKER0_REG1_PACK_UNTILIZE_DST_Z_STRIDE",
        thcon::THCON_PACKER0_REG1_PACK_UNTILIZE_DST_Z_STRIDE,
    ),
    (
        "THCON_PACKER0_REG1_PACK_UNTILIZE_SRC_Z_STRIDE",
        thcon::THCON_PACKER0_REG1_PACK_UNTILIZE_SRC_Z_STRIDE,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE0",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE0,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE1",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE1,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE2",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE2,
    ),
    (
        "THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE3",
        thcon::THCON_PACKER0_REG2_EDGE_MASK_SELECT_FACE3,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_COL_MASK",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_MASK_VAL",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_NO_WRITE",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_2",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_3",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_ROW_MASK",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_ROW_MASK,
    ),
    (
        "THCON_PACKER0_REG3_PACK_STRIDE_VAL_SOURCE",
        thcon::THCON_PACKER0_REG3_PACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_PACKER0_REG3_RELU_MODE",
        thcon::THCON_PACKER0_REG3_RELU_MODE,
    ),
    (
        "THCON_PACKER0_REG3_RELU_THRESHOLD",
        thcon::THCON_PACKER0_REG3_RELU_THRESHOLD,
    ),
    (
        "THCON_PACKER0_REG3_UNUSED0",
        thcon::THCON_PACKER0_REG3_UNUSED0,
    ),
    (
        "THCON_PACKER1_REG0_DISABLE_SRCS_OFFSET",
        thcon::THCON_PACKER1_REG0_DISABLE_SRCS_OFFSET,
    ),
    (
        "THCON_PACKER1_REG0_EXP_THRESHOLD",
        thcon::THCON_PACKER1_REG0_EXP_THRESHOLD,
    ),
    (
        "THCON_PACKER1_REG0_EXP_THRESHOLD_EN",
        thcon::THCON_PACKER1_REG0_EXP_THRESHOLD_EN,
    ),
    (
        "THCON_PACKER1_REG0_INSTRN_COUNT",
        thcon::THCON_PACKER1_REG0_INSTRN_COUNT,
    ),
    (
        "THCON_PACKER1_REG0_INSTRN_LOOP_COUNT",
        thcon::THCON_PACKER1_REG0_INSTRN_LOOP_COUNT,
    ),
    (
        "THCON_PACKER1_REG0_INT_DESCALE_ENABLE",
        thcon::THCON_PACKER1_REG0_INT_DESCALE_ENABLE,
    ),
    (
        "THCON_PACKER1_REG0_INT_DESCALE_MODE",
        thcon::THCON_PACKER1_REG0_INT_DESCALE_MODE,
    ),
    (
        "THCON_PACKER1_REG0_IN_DATA_FORMAT",
        thcon::THCON_PACKER1_REG0_IN_DATA_FORMAT,
    ),
    (
        "THCON_PACKER1_REG0_L1_ACC",
        thcon::THCON_PACKER1_REG0_L1_ACC,
    ),
    (
        "THCON_PACKER1_REG0_SRC_ADDR_OFFSET",
        thcon::THCON_PACKER1_REG0_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_PACKER1_REG0_STOCH_RND_EN",
        thcon::THCON_PACKER1_REG0_STOCH_RND_EN,
    ),
    (
        "THCON_PACKER1_REG0_UNUSED0",
        thcon::THCON_PACKER1_REG0_UNUSED0,
    ),
    (
        "THCON_PACKER1_REG0_UNUSED1",
        thcon::THCON_PACKER1_REG0_UNUSED1,
    ),
    (
        "THCON_PACKER1_REG0_UNUSED2",
        thcon::THCON_PACKER1_REG0_UNUSED2,
    ),
    (
        "THCON_PACKER1_REG0_WRAP_TILE_DIM",
        thcon::THCON_PACKER1_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_PACKER1_REG0_ZERO_WRITE",
        thcon::THCON_PACKER1_REG0_ZERO_WRITE,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK0",
        thcon::THCON_PACKER1_REG1_EDGE_MASK0,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK1",
        thcon::THCON_PACKER1_REG1_EDGE_MASK1,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK2",
        thcon::THCON_PACKER1_REG1_EDGE_MASK2,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK3",
        thcon::THCON_PACKER1_REG1_EDGE_MASK3,
    ),
    (
        "THCON_PACKER1_REG1_EDGE_MASK_MODE",
        thcon::THCON_PACKER1_REG1_EDGE_MASK_MODE,
    ),
    (
        "THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_0",
        thcon::THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_1",
        thcon::THCON_PACKER1_REG1_PACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_PACKER1_REG1_UNUSED0",
        thcon::THCON_PACKER1_REG1_UNUSED0,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE0",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE0,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE1",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE1,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE2",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE2,
    ),
    (
        "THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE3",
        thcon::THCON_PACKER1_REG2_EDGE_MASK_SELECT_FACE3,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_COL_MASK",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_MASK_VAL",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_NO_WRITE",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_2",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_3",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_ROW_MASK",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_ROW_MASK,
    ),
    (
        "THCON_PACKER1_REG3_PACK_STRIDE_VAL_SOURCE",
        thcon::THCON_PACKER1_REG3_PACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_PACKER1_REG3_RELU_MODE",
        thcon::THCON_PACKER1_REG3_RELU_MODE,
    ),
    (
        "THCON_PACKER1_REG3_RELU_THRESHOLD",
        thcon::THCON_PACKER1_REG3_RELU_THRESHOLD,
    ),
    (
        "THCON_PACKER1_REG3_UNUSED0",
        thcon::THCON_PACKER1_REG3_UNUSED0,
    ),
    (
        "THCON_UNPACKER0_REG0_ARG_FIFO_MODE",
        thcon::THCON_UNPACKER0_REG0_ARG_FIFO_MODE,
    ),
    (
        "THCON_UNPACKER0_REG0_ENABLE_ARG_FIFO",
        thcon::THCON_UNPACKER0_REG0_ENABLE_ARG_FIFO,
    ),
    (
        "THCON_UNPACKER0_REG0_OUT_DATA_FORMAT",
        thcon::THCON_UNPACKER0_REG0_OUT_DATA_FORMAT,
    ),
    (
        "THCON_UNPACKER0_REG0_SELF_CHECK_EN",
        thcon::THCON_UNPACKER0_REG0_SELF_CHECK_EN,
    ),
    (
        "THCON_UNPACKER0_REG0_SET_STRIDE_MODE",
        thcon::THCON_UNPACKER0_REG0_SET_STRIDE_MODE,
    ),
    (
        "THCON_UNPACKER0_REG0_TILIZE_SRC_ADDR_OFFSET",
        thcon::THCON_UNPACKER0_REG0_TILIZE_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_UNPACKER0_REG0_TRANSPOSE",
        thcon::THCON_UNPACKER0_REG0_TRANSPOSE,
    ),
    (
        "THCON_UNPACKER0_REG0_UNPACK_STRIDE_COL_MASK",
        thcon::THCON_UNPACKER0_REG0_UNPACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_UNPACKER0_REG0_UNPACR_STRIDE_SET_STRIDE",
        thcon::THCON_UNPACKER0_REG0_UNPACR_STRIDE_SET_STRIDE,
    ),
    (
        "THCON_UNPACKER0_REG0_UNUSED1",
        thcon::THCON_UNPACKER0_REG0_UNUSED1,
    ),
    (
        "THCON_UNPACKER0_REG0_WRAP_TILE_DIM",
        thcon::THCON_UNPACKER0_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_MASK_VAL",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_NO_WRITE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK0",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK0,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK1",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK1,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK2",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK2,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK3",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK3,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK4",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK4,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK5",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK5,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK6",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK6,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK7",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_ROW_MASK7,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_STRIDE_VAL_SOURCE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_TILIZE_DST_Z_STRIDE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_TILIZE_DST_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNPACK_TILIZE_SRC_Z_STRIDE",
        thcon::THCON_UNPACKER0_REG1_UNPACK_TILIZE_SRC_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER0_REG1_UNUSED0",
        thcon::THCON_UNPACKER0_REG1_UNUSED0,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_0",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_1",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_2",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_3",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_4",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_4,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_5",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_5,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_6",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_6,
    ),
    (
        "THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_7",
        thcon::THCON_UNPACKER0_REG2_UNPACK_STRIDE_OFFSET_7,
    ),
    (
        "THCON_UNPACKER1_REG0_ARG_FIFO_MODE",
        thcon::THCON_UNPACKER1_REG0_ARG_FIFO_MODE,
    ),
    (
        "THCON_UNPACKER1_REG0_ENABLE_ARG_FIFO",
        thcon::THCON_UNPACKER1_REG0_ENABLE_ARG_FIFO,
    ),
    (
        "THCON_UNPACKER1_REG0_OUT_DATA_FORMAT",
        thcon::THCON_UNPACKER1_REG0_OUT_DATA_FORMAT,
    ),
    (
        "THCON_UNPACKER1_REG0_SET_STRIDE_MODE",
        thcon::THCON_UNPACKER1_REG0_SET_STRIDE_MODE,
    ),
    (
        "THCON_UNPACKER1_REG0_TILIZE_SRC_ADDR_OFFSET",
        thcon::THCON_UNPACKER1_REG0_TILIZE_SRC_ADDR_OFFSET,
    ),
    (
        "THCON_UNPACKER1_REG0_TRANSPOSE",
        thcon::THCON_UNPACKER1_REG0_TRANSPOSE,
    ),
    (
        "THCON_UNPACKER1_REG0_UNPACK_STRIDE_COL_MASK",
        thcon::THCON_UNPACKER1_REG0_UNPACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_UNPACKER1_REG0_UNPACR_STRIDE_SET_STRIDE",
        thcon::THCON_UNPACKER1_REG0_UNPACR_STRIDE_SET_STRIDE,
    ),
    (
        "THCON_UNPACKER1_REG0_UNUSED0",
        thcon::THCON_UNPACKER1_REG0_UNUSED0,
    ),
    (
        "THCON_UNPACKER1_REG0_UNUSED1",
        thcon::THCON_UNPACKER1_REG0_UNUSED1,
    ),
    (
        "THCON_UNPACKER1_REG0_WRAP_TILE_DIM",
        thcon::THCON_UNPACKER1_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_MASK_VAL",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_NO_WRITE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK0",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK0,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK1",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK1,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK2",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK2,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK3",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK3,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK4",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK4,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK5",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK5,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK6",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK6,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK7",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_ROW_MASK7,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_STRIDE_VAL_SOURCE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_TILIZE_DST_Z_STRIDE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_TILIZE_DST_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNPACK_TILIZE_SRC_Z_STRIDE",
        thcon::THCON_UNPACKER1_REG1_UNPACK_TILIZE_SRC_Z_STRIDE,
    ),
    (
        "THCON_UNPACKER1_REG1_UNUSED0",
        thcon::THCON_UNPACKER1_REG1_UNUSED0,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_0",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_1",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_2",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_3",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_4",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_4,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_5",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_5,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_6",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_6,
    ),
    (
        "THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_7",
        thcon::THCON_UNPACKER1_REG2_UNPACK_STRIDE_OFFSET_7,
    ),
    (
        "THCON_UNPACKER2_REG0_ARG_FIFO_MODE",
        thcon::THCON_UNPACKER2_REG0_ARG_FIFO_MODE,
    ),
    (
        "THCON_UNPACKER2_REG0_ENABLE_ARG_FIFO",
        thcon::THCON_UNPACKER2_REG0_ENABLE_ARG_FIFO,
    ),
    (
        "THCON_UNPACKER2_REG0_INSTRN_COUNT",
        thcon::THCON_UNPACKER2_REG0_INSTRN_COUNT,
    ),
    (
        "THCON_UNPACKER2_REG0_INSTRN_LOOP_COUNT",
        thcon::THCON_UNPACKER2_REG0_INSTRN_LOOP_COUNT,
    ),
    (
        "THCON_UNPACKER2_REG0_OUT_DATA_FORMAT",
        thcon::THCON_UNPACKER2_REG0_OUT_DATA_FORMAT,
    ),
    (
        "THCON_UNPACKER2_REG0_UNPACK_STRIDE_COL_MASK",
        thcon::THCON_UNPACKER2_REG0_UNPACK_STRIDE_COL_MASK,
    ),
    (
        "THCON_UNPACKER2_REG0_UNUSED0",
        thcon::THCON_UNPACKER2_REG0_UNUSED0,
    ),
    (
        "THCON_UNPACKER2_REG0_UNUSED1",
        thcon::THCON_UNPACKER2_REG0_UNUSED1,
    ),
    (
        "THCON_UNPACKER2_REG0_UNUSED2",
        thcon::THCON_UNPACKER2_REG0_UNUSED2,
    ),
    (
        "THCON_UNPACKER2_REG0_WRAP_TILE_DIM",
        thcon::THCON_UNPACKER2_REG0_WRAP_TILE_DIM,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_MASK_VAL",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_MASK_VAL,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_NO_WRITE",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_NO_WRITE,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK0",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK0,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK1",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK1,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK2",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK2,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK3",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK3,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK4",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK4,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK5",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK5,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK6",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK6,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK7",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_ROW_MASK7,
    ),
    (
        "THCON_UNPACKER2_REG1_UNPACK_STRIDE_VAL_SOURCE",
        thcon::THCON_UNPACKER2_REG1_UNPACK_STRIDE_VAL_SOURCE,
    ),
    (
        "THCON_UNPACKER2_REG1_UNUSED0",
        thcon::THCON_UNPACKER2_REG1_UNUSED0,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_0",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_0,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_1",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_1,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_2",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_2,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_3",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_3,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_4",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_4,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_5",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_5,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_6",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_6,
    ),
    (
        "THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_7",
        thcon::THCON_UNPACKER2_REG2_UNPACK_STRIDE_OFFSET_7,
    ),
];
