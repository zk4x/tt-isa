//! Quasar's SFPU immediates, and the two-instruction FP32 constant load.
//!
//! The register-file format codes are the same numbers as Blackhole's for the
//! formats both name (`Float32` 3, `Int32`/`SMAG32` 4, `BF16` 2), but the two
//! half-write modes are not: Quasar's `SFPLOADI` takes `LO16_ONLY` `0b1010` and
//! `HI16_ONLY` `0b1000` (`tt_llk_quasar/instructions/assembly.yaml#SFPLOADI`,
//! and tt-metal's `_sfpu_load_config32_` writes `10` then `8`,
//! `common/inc/cmath_common.h:194-201`), where Blackhole names `9` and `7`.
//! `MEASURED`: the qsr simulator refuses `instr_mod0=9` on `SFPLOADI`
//! (`tensix_sfploadi: instr_mod0=9`) and accepts `10`/`8`, which is the same
//! answer tt-metal's constant load gives.

use crate::isa::quasar::encode;
use crate::isa::{EncodeError, Instruction};

/// `SFPLOAD`/`SFPSTORE` register-file formats (`inc` `instr_mod0`).
pub mod mod0_fmt {
    pub const FP16A: u32 = 1;
    pub const FP16B: u32 = 2;
    pub const FP32: u32 = 3;
    pub const INT32: u32 = 4;
    pub const INT8: u32 = 5;
    pub const UINT16: u32 = 6;
    pub const HI16: u32 = 7;
    pub const INT16: u32 = 8;
    pub const LO16: u32 = 9;
    pub const STACK: u32 = 10;
    pub const UINT8: u32 = 11;
    pub const LO16_ONLY: u32 = 14;
    pub const HI16_ONLY: u32 = 15;
}

/// `SFPLOADI`'s half-write immediate formats.
pub mod loadi_mode {
    pub const FP16B: u32 = 0;
    pub const FP16A: u32 = 1;
    pub const UINT16: u32 = 2;
    pub const INT16: u32 = 4;
    pub const HI16_ONLY: u32 = 8;
    pub const LO16_ONLY: u32 = 10;
}

/// `SFPMAD`'s modifier bits (`instr_mod1`).
pub mod mad_mod1 {
    /// Negate the addend: `VD = VA * VB - VC`.
    pub const NEGATE_VC: u32 = 2;
}

/// The `LReg` holding a constant `+0` (`p_sfpu::LCONST_0`,
/// `ckernel_instr_params.h:388`).
pub const LREG_ZERO: u32 = 9;

/// The `LReg` holding a constant `1.0` in every lane (`p_sfpu::LCONST_1`, and
/// `LReg.md:13` for the value).
pub const LREG_ONE: u32 = 10;

/// The `LReg` holding a constant `-1` (all ones) in every lane
/// (`p_sfpu::LCONST_neg1`, `ckernel_instr_params.h:388`).
pub const LREG_NEG_ONE: u32 = 11;

/// `SFPMAD`: lanewise `LReg[vd] = ±(LReg[va] * LReg[vb]) ± LReg[vc]`.
pub const fn mad(
    va: u32,
    vb: u32,
    vc: u32,
    vd: u32,
    mod1: u32,
) -> Result<Instruction, EncodeError> {
    encode::Sfpmad::ZERO
        .lreg_a(va)
        .lreg_b(vb)
        .lreg_c(vc)
        .lreg_dest(vd)
        .instr_mod1(mod1)
        .encode()
}

/// `LReg[vd] = LReg[vb] + LReg[vc]`, an `SFPMAD` against the constant one —
/// the sequence tt-metal's add uses, and Blackhole's encoder builds it the same
/// way.
pub const fn add(vb: u32, vc: u32, vd: u32) -> Result<Instruction, EncodeError> {
    mad(LREG_ONE, vb, vc, vd, 0)
}

/// `LReg[vd] = LReg[vb] - LReg[vc]`, with the addend negated.
pub const fn sub(vb: u32, vc: u32, vd: u32) -> Result<Instruction, EncodeError> {
    mad(LREG_ONE, vb, vc, vd, mad_mod1::NEGATE_VC)
}

/// `LReg[vd] = LReg[va] * LReg[vb]`: an `SFPMAD` whose addend is the constant
/// zero, negated so the result is `+0` and not `-0`.
pub const fn mul(va: u32, vb: u32, vd: u32) -> Result<Instruction, EncodeError> {
    mad(va, vb, LREG_ZERO, vd, mad_mod1::NEGATE_VC)
}

/// Load a full FP32 constant into `LReg[vd]`; the low half first, as tt-metal's
/// `_sfpu_load_config32_` writes it.
///
/// Each `SFPLOADI` carries 16 bits and writes only its own half, so the order is
/// immaterial; tt-metal's is kept so the sequence reads the same as the LLK's.
pub const fn load_f32(vd: u32, bits: u32) -> Result<[Instruction; 2], EncodeError> {
    let lo = match encode::sfploadi(vd, loadi_mode::LO16_ONLY, bits & 0xFFFF) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    let hi = match encode::sfploadi(vd, loadi_mode::HI16_ONLY, bits >> 16) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    Ok([lo, hi])
}
