//! The Quasar Tensix instruction set, generated from tt-metal's Quasar LLK
//! `assembly.yaml`.
//!
//! **Do not edit.** Regenerate with `cargo xtask gen-quasar`.
//!
//! Source: tt-metal `b1d492357ba0f7a6f996f58af318b39592d5afe4`, 164 instruction encodings.
//!
//! This is the raw layer: a field is refused only when the value does not
//! fit. What the hardware honours is a separate question.
//!
//! The LLK table does not mark signedness, so every field is marked
//! unsigned here. A two's-complement immediate is still encoded by passing
//! its bit pattern; only [`Field::signed`] reports false.
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]

use super::{EncodeError, Field, Instruction, InstructionDef, Provenance};

/// One `InstructionDef` per Quasar instruction, from the LLK assembly table.
pub mod defs {
    use super::*;

    /// `ATGETM`, from the Quasar LLK assembly table.
    pub static ATGETM: InstructionDef = InstructionDef::new(
        "ATGETM",
        "ATGETM",
        0xa0,
        &[Field::new("mutex_index", 0, 16, false, None)],
        &[],
        0x00ff0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ATGETM",
    );

    /// `ATRELM`, from the Quasar LLK assembly table.
    pub static ATRELM: InstructionDef = InstructionDef::new(
        "ATRELM",
        "ATRELM",
        0xa1,
        &[Field::new("mutex_index", 0, 16, false, None)],
        &[],
        0x00ff0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ATRELM",
    );

    /// `STALLWAIT`, from the Quasar LLK assembly table.
    pub static STALLWAIT: InstructionDef = InstructionDef::new(
        "STALLWAIT",
        "STALLWAIT",
        0xa2,
        &[
            Field::new("stall_res", 15, 9, false, None),
            Field::new("wait_res_idx_2", 10, 5, false, None),
            Field::new("wait_res_idx_1", 5, 5, false, None),
            Field::new("wait_res_idx_0", 0, 5, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#STALLWAIT",
    );

    /// `SEMINIT`, from the Quasar LLK assembly table.
    pub static SEMINIT: InstructionDef = InstructionDef::new(
        "SEMINIT",
        "SEMINIT",
        0xa3,
        &[
            Field::new("max_value", 20, 4, false, None),
            Field::new("init_value", 16, 4, false, None),
            Field::new("sem_bank_sel", 8, 5, false, None),
            Field::new("sem_sel", 0, 8, false, None),
        ],
        &[],
        0x0000e000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SEMINIT",
    );

    /// `SEMPOST`, from the Quasar LLK assembly table.
    pub static SEMPOST: InstructionDef = InstructionDef::new(
        "SEMPOST",
        "SEMPOST",
        0xa4,
        &[
            Field::new("sem_bank_sel", 8, 5, false, None),
            Field::new("sem_sel", 0, 8, false, None),
        ],
        &[],
        0x00ffe000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SEMPOST",
    );

    /// `SEMGET`, from the Quasar LLK assembly table.
    pub static SEMGET: InstructionDef = InstructionDef::new(
        "SEMGET",
        "SEMGET",
        0xa5,
        &[
            Field::new("sem_bank_sel", 8, 5, false, None),
            Field::new("sem_sel", 0, 8, false, None),
        ],
        &[],
        0x00ffe000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SEMGET",
    );

    /// `SEMWAIT`, from the Quasar LLK assembly table.
    pub static SEMWAIT: InstructionDef = InstructionDef::new(
        "SEMWAIT",
        "SEMWAIT",
        0xa6,
        &[
            Field::new("stall_res", 15, 9, false, None),
            Field::new("wait_sem_cond", 13, 2, false, None),
            Field::new("sem_bank_sel", 8, 5, false, None),
            Field::new("sem_sel", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SEMWAIT",
    );

    /// `WAIT_TILES`, from the Quasar LLK assembly table.
    pub static WAIT_TILES: InstructionDef = InstructionDef::new(
        "WAIT_TILES",
        "WAIT_TILES",
        0xa9,
        &[
            Field::new("stall_res", 15, 9, false, None),
            Field::new("num_tiles", 5, 10, false, None),
            Field::new("buffer_sel", 0, 5, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#WAIT_TILES",
    );

    /// `WAIT_FREE`, from the Quasar LLK assembly table.
    pub static WAIT_FREE: InstructionDef = InstructionDef::new(
        "WAIT_FREE",
        "WAIT_FREE",
        0xab,
        &[
            Field::new("stall_res", 15, 9, false, None),
            Field::new("num_tiles", 5, 10, false, None),
            Field::new("buffer_sel", 0, 5, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#WAIT_FREE",
    );

    /// `PUSH_TILES`, from the Quasar LLK assembly table.
    pub static PUSH_TILES: InstructionDef = InstructionDef::new(
        "PUSH_TILES",
        "PUSH_TILES",
        0x3d,
        &[
            Field::new("packer_wr_done_wait_mask", 15, 2, false, None),
            Field::new("num_tiles", 5, 10, false, None),
            Field::new("buffer_sel", 0, 5, false, None),
        ],
        &[],
        0x00fe0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PUSH_TILES",
    );

    /// `POP_TILES`, from the Quasar LLK assembly table.
    pub static POP_TILES: InstructionDef = InstructionDef::new(
        "POP_TILES",
        "POP_TILES",
        0x3e,
        &[
            Field::new("unpacker_rd_done_wait_mask", 15, 3, false, None),
            Field::new("num_tiles", 5, 10, false, None),
            Field::new("buffer_sel", 0, 5, false, None),
        ],
        &[],
        0x00fc0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#POP_TILES",
    );

    /// `NOP`, from the Quasar LLK assembly table.
    pub static NOP: InstructionDef = InstructionDef::new(
        "NOP",
        "NOP",
        0x02,
        &[],
        &[],
        0x00ffffff,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#NOP",
    );

    /// `MOVD2A`, from the Quasar LLK assembly table.
    pub static MOVD2A: InstructionDef = InstructionDef::new(
        "MOVD2A",
        "MOVD2A",
        0x08,
        &[
            Field::new("dest_32b_lo", 23, 1, false, None),
            Field::new("src", 17, 6, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("instr_mod", 12, 2, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00000800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVD2A",
    );

    /// `MOVD2B`, from the Quasar LLK assembly table.
    pub static MOVD2B: InstructionDef = InstructionDef::new(
        "MOVD2B",
        "MOVD2B",
        0x0a,
        &[
            Field::new("dest_32b_lo", 23, 1, false, None),
            Field::new("src", 17, 6, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("instr_mod", 12, 2, false, None),
            Field::new("transpose", 11, 1, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVD2B",
    );

    /// `MOVB2A`, from the Quasar LLK assembly table.
    pub static MOVB2A: InstructionDef = InstructionDef::new(
        "MOVB2A",
        "MOVB2A",
        0x0b,
        &[
            Field::new("srca", 17, 7, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("instr_mod", 12, 2, false, None),
            Field::new("srcb", 0, 12, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVB2A",
    );

    /// `ZEROACC`, from the Quasar LLK assembly table.
    pub static ZEROACC: InstructionDef = InstructionDef::new(
        "ZEROACC",
        "ZEROACC",
        0x10,
        &[
            Field::new("clear_mode", 19, 5, false, None),
            Field::new("use_32_bit_mode", 18, 1, false, None),
            Field::new("clear_zero_flags", 17, 1, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("where", 0, 14, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ZEROACC",
    );

    /// `ZEROSRC`, from the Quasar LLK assembly table.
    pub static ZEROSRC: InstructionDef = InstructionDef::new(
        "ZEROSRC",
        "ZEROSRC",
        0x11,
        &[
            Field::new("packed_fmt", 7, 1, false, None),
            Field::new("int_fmt", 6, 1, false, None),
            Field::new("exp_bias", 5, 1, false, None),
            Field::new("zero_val", 4, 1, false, None),
            Field::new("write_mode", 3, 1, false, None),
            Field::new("bank_mask", 2, 1, false, None),
            Field::new("src_mask", 0, 2, false, None),
        ],
        &[],
        0x00ffff00,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ZEROSRC",
    );

    /// `MOVA2D`, from the Quasar LLK assembly table.
    pub static MOVA2D: InstructionDef = InstructionDef::new(
        "MOVA2D",
        "MOVA2D",
        0x12,
        &[
            Field::new("dest_32b_lo", 23, 1, false, None),
            Field::new("src", 17, 6, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("instr_mod", 12, 2, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00000800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVA2D",
    );

    /// `MOVB2D`, from the Quasar LLK assembly table.
    pub static MOVB2D: InstructionDef = InstructionDef::new(
        "MOVB2D",
        "MOVB2D",
        0x13,
        &[
            Field::new("dest_32b_lo", 23, 1, false, None),
            Field::new("src", 17, 6, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("transfer_sz", 12, 2, false, None),
            Field::new("bcast_datum0", 11, 1, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVB2D",
    );

    /// `MOVDBGA2D`, from the Quasar LLK assembly table.
    pub static MOVDBGA2D: InstructionDef = InstructionDef::new(
        "MOVDBGA2D",
        "MOVDBGA2D",
        0x09,
        &[
            Field::new("dest_32b_lo", 23, 1, false, None),
            Field::new("src", 17, 6, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("instr_mod", 12, 2, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00000800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVDBGA2D",
    );

    /// `MOVDBGB2D`, from the Quasar LLK assembly table.
    pub static MOVDBGB2D: InstructionDef = InstructionDef::new(
        "MOVDBGB2D",
        "MOVDBGB2D",
        0x0c,
        &[
            Field::new("dest_32b_lo", 23, 1, false, None),
            Field::new("src", 17, 6, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("transfer_sz", 12, 2, false, None),
            Field::new("bcast_datum0", 11, 1, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOVDBGB2D",
    );

    /// `SHIFTXB`, from the Quasar LLK assembly table.
    pub static SHIFTXB: InstructionDef = InstructionDef::new(
        "SHIFTXB",
        "SHIFTXB",
        0x18,
        &[
            Field::new("addr_mode", 14, 10, false, None),
            Field::new("rot_shift", 10, 4, false, None),
            Field::new("shift_row", 0, 10, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SHIFTXB",
    );

    /// `HALT`, from the Quasar LLK assembly table.
    pub static HALT: InstructionDef = InstructionDef::new(
        "HALT",
        "HALT",
        0x23,
        &[],
        &[],
        0x00ffffff,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#HALT",
    );

    /// `MVMULDI`, from the Quasar LLK assembly table.
    pub static MVMULDI: InstructionDef = InstructionDef::new(
        "MVMULDI",
        "MVMULDI",
        0x25,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("ins_mod", 19, 3, false, None),
            Field::new("srcb_addr", 15, 4, false, None),
            Field::new("srca_addr", 11, 4, false, None),
            Field::new("addr_mode", 8, 3, false, None),
            Field::new("dst", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MVMULDI",
    );

    /// `MVMUL`, from the Quasar LLK assembly table.
    pub static MVMUL: InstructionDef = InstructionDef::new(
        "MVMUL",
        "MVMUL",
        0x26,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("instr_mod19", 19, 3, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00063800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MVMUL",
    );

    /// `ELWMUL`, from the Quasar LLK assembly table.
    pub static ELWMUL: InstructionDef = InstructionDef::new(
        "ELWMUL",
        "ELWMUL",
        0x27,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("dest_accum_en", 21, 1, false, None),
            Field::new("instr_mod19", 19, 2, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00063800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ELWMUL",
    );

    /// `ELWADD`, from the Quasar LLK assembly table.
    pub static ELWADD: InstructionDef = InstructionDef::new(
        "ELWADD",
        "ELWADD",
        0x28,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("dest_accum_en", 21, 1, false, None),
            Field::new("instr_mod19", 19, 2, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00063800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ELWADD",
    );

    /// `ELWSUB`, from the Quasar LLK assembly table.
    pub static ELWSUB: InstructionDef = InstructionDef::new(
        "ELWSUB",
        "ELWSUB",
        0x30,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("dest_accum_en", 21, 1, false, None),
            Field::new("instr_mod19", 19, 2, false, None),
            Field::new("addr_mode", 14, 3, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00063800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ELWSUB",
    );

    /// `ELWADDDI`, from the Quasar LLK assembly table.
    pub static ELWADDDI: InstructionDef = InstructionDef::new(
        "ELWADDDI",
        "ELWADDDI",
        0x31,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("ins_mod", 19, 3, false, None),
            Field::new("srcb_addr", 15, 4, false, None),
            Field::new("srca_addr", 11, 4, false, None),
            Field::new("addr_mode", 8, 3, false, None),
            Field::new("dst", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ELWADDDI",
    );

    /// `ELWSUBDI`, from the Quasar LLK assembly table.
    pub static ELWSUBDI: InstructionDef = InstructionDef::new(
        "ELWSUBDI",
        "ELWSUBDI",
        0x32,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("ins_mod", 19, 3, false, None),
            Field::new("srcb_addr", 15, 4, false, None),
            Field::new("srca_addr", 11, 4, false, None),
            Field::new("addr_mode", 8, 3, false, None),
            Field::new("dst", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ELWSUBDI",
    );

    /// `GMPOOL`, from the Quasar LLK assembly table.
    pub static GMPOOL: InstructionDef = InstructionDef::new(
        "GMPOOL",
        "GMPOOL",
        0x33,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("instr_mod19", 19, 3, false, None),
            Field::new("pool_addr_mode", 15, 3, false, None),
            Field::new("rsvd", 14, 1, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00043800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#GMPOOL",
    );

    /// `GAPOOL`, from the Quasar LLK assembly table.
    pub static GAPOOL: InstructionDef = InstructionDef::new(
        "GAPOOL",
        "GAPOOL",
        0x34,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("instr_mod19", 19, 3, false, None),
            Field::new("pool_addr_mode", 15, 3, false, None),
            Field::new("rsvd", 14, 1, false, None),
            Field::new("dst", 0, 11, false, None),
        ],
        &[],
        0x00043800,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#GAPOOL",
    );

    /// `SETDVALID`, from the Quasar LLK assembly table.
    pub static SETDVALID: InstructionDef = InstructionDef::new(
        "SETDVALID",
        "SETDVALID",
        0x57,
        &[Field::new("setvalid", 0, 5, false, None)],
        &[],
        0x00ffffe0,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SETDVALID",
    );

    /// `RV_PACR`, from the Quasar LLK assembly table.
    pub static RV_PACR: InstructionDef = InstructionDef::new(
        "RV_PACR",
        "RV_PACR",
        0x35,
        &[
            Field::new("reg_idx2", 10, 5, false, None),
            Field::new("reg_idx1", 5, 5, false, None),
            Field::new("reg_idx0", 0, 5, false, None),
        ],
        &[],
        0x00ff8000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RV_PACR",
    );

    /// `PACR_STRIDE`, from the Quasar LLK assembly table.
    pub static PACR_STRIDE: InstructionDef = InstructionDef::new(
        "PACR_STRIDE",
        "PACR_STRIDE",
        0x1d,
        &[
            Field::new("Src_Row_Idx_or_Inc_Mul4", 18, 6, false, None),
            Field::new("Src_Row_Idx_Inc", 17, 1, false, None),
            Field::new("L1_Tile_Idx_or_Tile_Idx_Inc", 14, 3, false, None),
            Field::new("Tile_Idx_Inc", 13, 1, false, None),
            Field::new("L1_16datums_Row_Index", 7, 6, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("PackerSel", 1, 1, false, None),
            Field::new("ClrDatValid", 0, 1, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR_STRIDE",
    );

    /// `CLEARDVALID`, from the Quasar LLK assembly table.
    pub static CLEARDVALID: InstructionDef = InstructionDef::new(
        "CLEARDVALID",
        "CLEARDVALID",
        0x36,
        &[
            Field::new("cleardvalid", 22, 2, false, None),
            Field::new("cleardvalid_S", 20, 2, false, None),
            Field::new("dest_dvalid_reset", 10, 4, false, None),
            Field::new("dest_dvalid_client_bank_reset", 6, 4, false, None),
            Field::new("dest_pulse_last", 2, 4, false, None),
            Field::new("reset", 0, 2, false, None),
        ],
        &[],
        0x000fc000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#CLEARDVALID",
    );

    /// `SETRWC`, from the Quasar LLK assembly table.
    pub static SETRWC: InstructionDef = InstructionDef::new(
        "SETRWC",
        "SETRWC",
        0x37,
        &[
            Field::new("clear_ab_vld", 22, 2, false, None),
            Field::new("rwc_cr", 18, 4, false, None),
            Field::new("rwc_val", 6, 12, false, None),
            Field::new("BitMask", 0, 6, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SETRWC",
    );

    /// `INCRWC`, from the Quasar LLK assembly table.
    pub static INCRWC: InstructionDef = InstructionDef::new(
        "INCRWC",
        "INCRWC",
        0x38,
        &[
            Field::new("rwc_cr", 18, 6, false, None),
            Field::new("rwc_a", 13, 5, false, None),
            Field::new("rwc_b", 8, 5, false, None),
            Field::new("rwc_d", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#INCRWC",
    );

    /// `RV_UNPACR`, from the Quasar LLK assembly table.
    pub static RV_UNPACR: InstructionDef = InstructionDef::new(
        "RV_UNPACR",
        "RV_UNPACR",
        0x39,
        &[
            Field::new("reg_idx2", 10, 5, false, None),
            Field::new("reg_idx1", 5, 5, false, None),
            Field::new("reg_idx0", 0, 5, false, None),
        ],
        &[],
        0x00ff8000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RV_UNPACR",
    );

    /// `ELWMULDI`, from the Quasar LLK assembly table.
    pub static ELWMULDI: InstructionDef = InstructionDef::new(
        "ELWMULDI",
        "ELWMULDI",
        0x3a,
        &[
            Field::new("clear_dvalid", 22, 2, false, None),
            Field::new("ins_mod", 19, 3, false, None),
            Field::new("srcb_addr", 15, 4, false, None),
            Field::new("srca_addr", 11, 4, false, None),
            Field::new("addr_mode", 8, 3, false, None),
            Field::new("dst", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ELWMULDI",
    );

    /// `PACR0_TILE`, from the Quasar LLK assembly table.
    pub static PACR0_TILE: InstructionDef = InstructionDef::new(
        "PACR0_TILE",
        "PACR0_TILE",
        0x0f,
        &[
            Field::new("Dst_Tile_Idx", 16, 8, false, None),
            Field::new("Src_Tile_Idx", 7, 9, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR0_TILE",
    );

    /// `PACR0_TILE_INC`, from the Quasar LLK assembly table.
    pub static PACR0_TILE_INC: InstructionDef = InstructionDef::new(
        "PACR0_TILE_INC",
        "PACR0_TILE_INC",
        0x19,
        &[
            Field::new("Dst_Tile_Idx_Inc", 16, 8, false, None),
            Field::new("Src_Tile_Idx_Inc", 7, 9, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR0_TILE_INC",
    );

    /// `PACR0_FACE`, from the Quasar LLK assembly table.
    pub static PACR0_FACE: InstructionDef = InstructionDef::new(
        "PACR0_FACE",
        "PACR0_FACE",
        0x1f,
        &[
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR0_FACE",
    );

    /// `PACR0_FACE_INC`, from the Quasar LLK assembly table.
    pub static PACR0_FACE_INC: InstructionDef = InstructionDef::new(
        "PACR0_FACE_INC",
        "PACR0_FACE_INC",
        0x20,
        &[
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR0_FACE_INC",
    );

    /// `PACR0_ROW`, from the Quasar LLK assembly table.
    pub static PACR0_ROW: InstructionDef = InstructionDef::new(
        "PACR0_ROW",
        "PACR0_ROW",
        0x2a,
        &[
            Field::new("Dst_Row_Idx", 20, 4, false, None),
            Field::new("Src_Row_Idx", 16, 4, false, None),
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR0_ROW",
    );

    /// `PACR0_ROW_INC`, from the Quasar LLK assembly table.
    pub static PACR0_ROW_INC: InstructionDef = InstructionDef::new(
        "PACR0_ROW_INC",
        "PACR0_ROW_INC",
        0x2b,
        &[
            Field::new("Dst_Row_Idx_Inc", 20, 4, false, None),
            Field::new("Src_Row_Idx_Inc", 16, 4, false, None),
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR0_ROW_INC",
    );

    /// `PACR1_TILE`, from the Quasar LLK assembly table.
    pub static PACR1_TILE: InstructionDef = InstructionDef::new(
        "PACR1_TILE",
        "PACR1_TILE",
        0x2c,
        &[
            Field::new("Dst_Tile_Idx", 16, 8, false, None),
            Field::new("Src_Tile_Idx", 7, 9, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR1_TILE",
    );

    /// `PACR1_TILE_INC`, from the Quasar LLK assembly table.
    pub static PACR1_TILE_INC: InstructionDef = InstructionDef::new(
        "PACR1_TILE_INC",
        "PACR1_TILE_INC",
        0x2d,
        &[
            Field::new("Dst_Tile_Idx_Inc", 16, 8, false, None),
            Field::new("Src_Tile_Idx_Inc", 7, 9, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR1_TILE_INC",
    );

    /// `PACR1_FACE`, from the Quasar LLK assembly table.
    pub static PACR1_FACE: InstructionDef = InstructionDef::new(
        "PACR1_FACE",
        "PACR1_FACE",
        0x2e,
        &[
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR1_FACE",
    );

    /// `PACR1_FACE_INC`, from the Quasar LLK assembly table.
    pub static PACR1_FACE_INC: InstructionDef = InstructionDef::new(
        "PACR1_FACE_INC",
        "PACR1_FACE_INC",
        0x2f,
        &[
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR1_FACE_INC",
    );

    /// `PACR1_ROW`, from the Quasar LLK assembly table.
    pub static PACR1_ROW: InstructionDef = InstructionDef::new(
        "PACR1_ROW",
        "PACR1_ROW",
        0x3b,
        &[
            Field::new("Dst_Row_Idx", 20, 4, false, None),
            Field::new("Src_Row_Idx", 16, 4, false, None),
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR1_ROW",
    );

    /// `PACR1_ROW_INC`, from the Quasar LLK assembly table.
    pub static PACR1_ROW_INC: InstructionDef = InstructionDef::new(
        "PACR1_ROW_INC",
        "PACR1_ROW_INC",
        0x6e,
        &[
            Field::new("Dst_Row_Idx_Inc", 20, 4, false, None),
            Field::new("Src_Row_Idx_Inc", 16, 4, false, None),
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 9, 3, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR1_ROW_INC",
    );

    /// `PACR_UNTILIZE`, from the Quasar LLK assembly table.
    pub static PACR_UNTILIZE: InstructionDef = InstructionDef::new(
        "PACR_UNTILIZE",
        "PACR_UNTILIZE",
        0x42,
        &[
            Field::new("Reserved", 14, 10, false, None),
            Field::new("Cntr_Reset_mask", 12, 2, false, None),
            Field::new("Dst_Z_Cntr_inc", 10, 2, false, None),
            Field::new("Src_Z_Cntr_inc", 8, 2, false, None),
            Field::new("Packer_Sel", 7, 1, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("ClrDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#PACR_UNTILIZE",
    );

    /// `UNPACR0_TILE`, from the Quasar LLK assembly table.
    pub static UNPACR0_TILE: InstructionDef = InstructionDef::new(
        "UNPACR0_TILE",
        "UNPACR0_TILE",
        0x3c,
        &[
            Field::new("Dst_Tile_Idx", 15, 9, false, None),
            Field::new("Src_Tile_Idx", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_TILE",
    );

    /// `UNPACR0_TILE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR0_TILE_INC: InstructionDef = InstructionDef::new(
        "UNPACR0_TILE_INC",
        "UNPACR0_TILE_INC",
        0x44,
        &[
            Field::new("Dst_Tile_Idx_Inc", 15, 9, false, None),
            Field::new("Src_Tile_Idx_Inc", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_TILE_INC",
    );

    /// `UNPACR0_FACE`, from the Quasar LLK assembly table.
    pub static UNPACR0_FACE: InstructionDef = InstructionDef::new(
        "UNPACR0_FACE",
        "UNPACR0_FACE",
        0x47,
        &[
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_FACE",
    );

    /// `UNPACR0_FACE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR0_FACE_INC: InstructionDef = InstructionDef::new(
        "UNPACR0_FACE_INC",
        "UNPACR0_FACE_INC",
        0x3f,
        &[
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_FACE_INC",
    );

    /// `UNPACR0_ROW`, from the Quasar LLK assembly table.
    pub static UNPACR0_ROW: InstructionDef = InstructionDef::new(
        "UNPACR0_ROW",
        "UNPACR0_ROW",
        0x4b,
        &[
            Field::new("Dst_Row_Idx", 20, 4, false, None),
            Field::new("Src_Row_Idx", 16, 4, false, None),
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_ROW",
    );

    /// `UNPACR0_ROW_INC`, from the Quasar LLK assembly table.
    pub static UNPACR0_ROW_INC: InstructionDef = InstructionDef::new(
        "UNPACR0_ROW_INC",
        "UNPACR0_ROW_INC",
        0x4c,
        &[
            Field::new("Dst_Row_Idx_Inc", 20, 4, false, None),
            Field::new("Src_Row_Idx_Inc", 16, 4, false, None),
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_ROW_INC",
    );

    /// `RV_WRCFG`, from the Quasar LLK assembly table.
    pub static RV_WRCFG: InstructionDef = InstructionDef::new(
        "RV_WRCFG",
        "RV_WRCFG",
        0x54,
        &[
            Field::new("byte_mask", 16, 8, false, None),
            Field::new("write_64b", 15, 1, false, None),
            Field::new("index_of_reg_containing_cfg_index", 10, 5, false, None),
            Field::new("index_of_reg_containing_wrdata_msbs", 5, 5, false, None),
            Field::new("index_of_reg_containing_wrdata_lsbs", 0, 5, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RV_WRCFG",
    );

    /// `UNPACR1_TILE`, from the Quasar LLK assembly table.
    pub static UNPACR1_TILE: InstructionDef = InstructionDef::new(
        "UNPACR1_TILE",
        "UNPACR1_TILE",
        0x5f,
        &[
            Field::new("Dst_Tile_Idx", 15, 9, false, None),
            Field::new("Src_Tile_Idx", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_TILE",
    );

    /// `UNPACR1_TILE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR1_TILE_INC: InstructionDef = InstructionDef::new(
        "UNPACR1_TILE_INC",
        "UNPACR1_TILE_INC",
        0x69,
        &[
            Field::new("Dst_Tile_Idx_Inc", 15, 9, false, None),
            Field::new("Src_Tile_Idx_Inc", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_TILE_INC",
    );

    /// `UNPACR1_FACE`, from the Quasar LLK assembly table.
    pub static UNPACR1_FACE: InstructionDef = InstructionDef::new(
        "UNPACR1_FACE",
        "UNPACR1_FACE",
        0x6a,
        &[
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_FACE",
    );

    /// `UNPACR1_FACE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR1_FACE_INC: InstructionDef = InstructionDef::new(
        "UNPACR1_FACE_INC",
        "UNPACR1_FACE_INC",
        0x6b,
        &[
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_FACE_INC",
    );

    /// `UNPACR1_ROW`, from the Quasar LLK assembly table.
    pub static UNPACR1_ROW: InstructionDef = InstructionDef::new(
        "UNPACR1_ROW",
        "UNPACR1_ROW",
        0x6c,
        &[
            Field::new("Dst_Row_Idx", 20, 4, false, None),
            Field::new("Src_Row_Idx", 16, 4, false, None),
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_ROW",
    );

    /// `UNPACR1_ROW_INC`, from the Quasar LLK assembly table.
    pub static UNPACR1_ROW_INC: InstructionDef = InstructionDef::new(
        "UNPACR1_ROW_INC",
        "UNPACR1_ROW_INC",
        0x6d,
        &[
            Field::new("Dst_Row_Idx_Inc", 20, 4, false, None),
            Field::new("Src_Row_Idx_Inc", 16, 4, false, None),
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_ROW_INC",
    );

    /// `UNPACR2_TILE`, from the Quasar LLK assembly table.
    pub static UNPACR2_TILE: InstructionDef = InstructionDef::new(
        "UNPACR2_TILE",
        "UNPACR2_TILE",
        0x9b,
        &[
            Field::new("Dst_Tile_Idx", 15, 9, false, None),
            Field::new("Src_Tile_Idx", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_TILE",
    );

    /// `UNPACR2_TILE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR2_TILE_INC: InstructionDef = InstructionDef::new(
        "UNPACR2_TILE_INC",
        "UNPACR2_TILE_INC",
        0x9c,
        &[
            Field::new("Dst_Tile_Idx_Inc", 15, 9, false, None),
            Field::new("Src_Tile_Idx_Inc", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_TILE_INC",
    );

    /// `UNPACR2_FACE`, from the Quasar LLK assembly table.
    pub static UNPACR2_FACE: InstructionDef = InstructionDef::new(
        "UNPACR2_FACE",
        "UNPACR2_FACE",
        0x9d,
        &[
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_FACE",
    );

    /// `UNPACR2_FACE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR2_FACE_INC: InstructionDef = InstructionDef::new(
        "UNPACR2_FACE_INC",
        "UNPACR2_FACE_INC",
        0x9e,
        &[
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_FACE_INC",
    );

    /// `UNPACR2_ROW`, from the Quasar LLK assembly table.
    pub static UNPACR2_ROW: InstructionDef = InstructionDef::new(
        "UNPACR2_ROW",
        "UNPACR2_ROW",
        0x9f,
        &[
            Field::new("Dst_Row_Idx", 20, 4, false, None),
            Field::new("Src_Row_Idx", 16, 4, false, None),
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_ROW",
    );

    /// `UNPACR2_ROW_INC`, from the Quasar LLK assembly table.
    pub static UNPACR2_ROW_INC: InstructionDef = InstructionDef::new(
        "UNPACR2_ROW_INC",
        "UNPACR2_ROW_INC",
        0xa8,
        &[
            Field::new("Dst_Row_Idx_Inc", 20, 4, false, None),
            Field::new("Src_Row_Idx_Inc", 16, 4, false, None),
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_ROW_INC",
    );

    /// `UNPACR_DEST_TILE`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_TILE: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_TILE",
        "UNPACR_DEST_TILE",
        0xac,
        &[
            Field::new("Dst_Tile_Idx", 15, 9, false, None),
            Field::new("Src_Tile_Idx", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_TILE",
    );

    /// `UNPACR_DEST_TILE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_TILE_INC: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_TILE_INC",
        "UNPACR_DEST_TILE_INC",
        0xad,
        &[
            Field::new("Dst_Tile_Idx_Inc", 15, 9, false, None),
            Field::new("Src_Tile_Idx_Inc", 7, 8, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_TILE_INC",
    );

    /// `UNPACR_DEST_FACE`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_FACE: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_FACE",
        "UNPACR_DEST_FACE",
        0xae,
        &[
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_FACE",
    );

    /// `UNPACR_DEST_FACE_INC`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_FACE_INC: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_FACE_INC",
        "UNPACR_DEST_FACE_INC",
        0xaf,
        &[
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00ff0001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_FACE_INC",
    );

    /// `UNPACR_DEST_ROW`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_ROW: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_ROW",
        "UNPACR_DEST_ROW",
        0xa7,
        &[
            Field::new("Dst_Row_Idx", 20, 4, false, None),
            Field::new("Src_Row_Idx", 16, 4, false, None),
            Field::new("Dst_Face_Idx", 14, 2, false, None),
            Field::new("Src_Face_Idx", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_ROW",
    );

    /// `UNPACR_DEST_ROW_INC`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_ROW_INC: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_ROW_INC",
        "UNPACR_DEST_ROW_INC",
        0x65,
        &[
            Field::new("Dst_Row_Idx_Inc", 20, 4, false, None),
            Field::new("Src_Row_Idx_Inc", 16, 4, false, None),
            Field::new("Dst_Face_Idx_Inc", 14, 2, false, None),
            Field::new("Src_Face_Idx_Inc", 12, 2, false, None),
            Field::new("Dst_Tile_Offset_Idx_Inc", 10, 2, false, None),
            Field::new("Src_Tile_Offset_Idx_Inc", 7, 3, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_ROW_INC",
    );

    /// `UNPACR_TILE_MISC`, from the Quasar LLK assembly table.
    pub static UNPACR_TILE_MISC: InstructionDef = InstructionDef::new(
        "UNPACR_TILE_MISC",
        "UNPACR_TILE_MISC",
        0xbf,
        &[
            Field::new("Unpack_Type", 21, 3, false, None),
            Field::new("Row_Bcast_Row_Idx", 15, 6, false, None),
            Field::new("Tile_Idx_Inc", 14, 1, false, None),
            Field::new("Dst_Tile_Idx", 12, 2, false, None),
            Field::new("Src_Tile_Idx", 7, 5, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_TILE_MISC",
    );

    /// `UNPACR0_STRIDE`, from the Quasar LLK assembly table.
    pub static UNPACR0_STRIDE: InstructionDef = InstructionDef::new(
        "UNPACR0_STRIDE",
        "UNPACR0_STRIDE",
        0x6f,
        &[
            Field::new("Src_Reg_Y_Cntr_Incr", 20, 4, false, None),
            Field::new("L1_Tile_Idx_or_Tile_Idx_Inc", 17, 3, false, None),
            Field::new("Tile_Idx_Inc", 16, 1, false, None),
            Field::new("Row_Mask_Reg_Sel", 13, 3, false, None),
            Field::new("L1_16datums_Row_Index", 7, 6, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR0_STRIDE",
    );

    /// `UNPACR1_STRIDE`, from the Quasar LLK assembly table.
    pub static UNPACR1_STRIDE: InstructionDef = InstructionDef::new(
        "UNPACR1_STRIDE",
        "UNPACR1_STRIDE",
        0xaa,
        &[
            Field::new("Src_Reg_Y_Cntr_Incr", 20, 4, false, None),
            Field::new("L1_Tile_Idx_or_Tile_Idx_Inc", 17, 3, false, None),
            Field::new("Tile_Idx_Inc", 16, 1, false, None),
            Field::new("Row_Mask_Reg_Sel", 13, 3, false, None),
            Field::new("L1_16datums_Row_Index", 7, 6, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR1_STRIDE",
    );

    /// `UNPACR2_STRIDE`, from the Quasar LLK assembly table.
    pub static UNPACR2_STRIDE: InstructionDef = InstructionDef::new(
        "UNPACR2_STRIDE",
        "UNPACR2_STRIDE",
        0x4e,
        &[
            Field::new("Src_Reg_Y_Cntr_Incr", 20, 4, false, None),
            Field::new("L1_Tile_Idx_or_Tile_Idx_Inc", 17, 3, false, None),
            Field::new("Tile_Idx_Inc", 16, 1, false, None),
            Field::new("Row_Mask_Reg_Sel", 13, 3, false, None),
            Field::new("L1_16datums_Row_Index", 7, 6, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR2_STRIDE",
    );

    /// `UNPACR_DEST_STRIDE`, from the Quasar LLK assembly table.
    pub static UNPACR_DEST_STRIDE: InstructionDef = InstructionDef::new(
        "UNPACR_DEST_STRIDE",
        "UNPACR_DEST_STRIDE",
        0xbd,
        &[
            Field::new("Src_Reg_Y_Cntr_Incr", 20, 4, false, None),
            Field::new("L1_Tile_Idx_or_Tile_Idx_Inc", 17, 3, false, None),
            Field::new("Tile_Idx_Inc", 16, 1, false, None),
            Field::new("Row_Mask_Reg_Sel", 13, 3, false, None),
            Field::new("L1_16datums_Row_Index", 7, 6, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_DEST_STRIDE",
    );

    /// `UNPACR_TILIZE`, from the Quasar LLK assembly table.
    pub static UNPACR_TILIZE: InstructionDef = InstructionDef::new(
        "UNPACR_TILIZE",
        "UNPACR_TILIZE",
        0xbe,
        &[
            Field::new("Reserved", 15, 9, false, None),
            Field::new("Cntr_Reset_mask", 13, 2, false, None),
            Field::new("Dst_Z_Cntr_inc", 11, 2, false, None),
            Field::new("Src_Z_Cntr_inc", 9, 2, false, None),
            Field::new("Unpack_Sel", 7, 2, false, None),
            Field::new("Buffer_Descriptor_Table_Sel", 2, 5, false, None),
            Field::new("SetDatValid", 1, 1, false, None),
        ],
        &[],
        0x00000001,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_TILIZE",
    );

    /// `UNPACR_NOP`, from the Quasar LLK assembly table.
    pub static UNPACR_NOP: InstructionDef = InstructionDef::new(
        "UNPACR_NOP",
        "UNPACR_NOP",
        0x43,
        &[
            Field::new("Unpacker_Select", 8, 2, false, None),
            Field::new("Set_Dvalid", 7, 1, false, None),
            Field::new("Stall_Cntrl", 5, 1, false, None),
            Field::new("Bank_Clr_Ctrl", 4, 1, false, None),
            Field::new("Src_ClrVal_Ctrl", 2, 2, false, None),
            Field::new("Nop_type", 0, 2, false, None),
        ],
        &[],
        0x00fffc40,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#UNPACR_NOP",
    );

    /// `SETGPR`, from the Quasar LLK assembly table.
    pub static SETGPR: InstructionDef = InstructionDef::new(
        "SETGPR",
        "SETGPR",
        0x45,
        &[
            Field::new("Payload_SigSelSize", 22, 2, false, None),
            Field::new("Payload_SigSel", 8, 14, false, None),
            Field::new("SetSignalsMode", 7, 1, false, None),
            Field::new("GPR_Index16b", 0, 7, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SETGPR",
    );

    /// `LOADIND`, from the Quasar LLK assembly table.
    pub static LOADIND: InstructionDef = InstructionDef::new(
        "LOADIND",
        "LOADIND",
        0x49,
        &[
            Field::new("SizeSel", 22, 2, false, None),
            Field::new("OffsetIndex", 14, 8, false, None),
            Field::new("AutoIncSpec", 12, 2, false, None),
            Field::new("Data_GPR_Index", 6, 6, false, None),
            Field::new("Addr_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#LOADIND",
    );

    /// `SET_SRC_TILE_FACE_ROW_IDX`, from the Quasar LLK assembly table.
    pub static SET_SRC_TILE_FACE_ROW_IDX: InstructionDef = InstructionDef::new(
        "SET_SRC_TILE_FACE_ROW_IDX",
        "SET_SRC_TILE_FACE_ROW_IDX",
        0x06,
        &[
            Field::new("Tile_Face_Row_Sel", 21, 2, false, None),
            Field::new("EngineSel", 18, 3, false, None),
            Field::new("Value", 0, 18, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SET_SRC_TILE_FACE_ROW_IDX",
    );

    /// `INC_SRC_TILE_FACE_ROW_IDX`, from the Quasar LLK assembly table.
    pub static INC_SRC_TILE_FACE_ROW_IDX: InstructionDef = InstructionDef::new(
        "INC_SRC_TILE_FACE_ROW_IDX",
        "INC_SRC_TILE_FACE_ROW_IDX",
        0x07,
        &[
            Field::new("Tile_Face_Row_Sel", 21, 2, false, None),
            Field::new("EngineSel", 18, 3, false, None),
            Field::new("Value", 0, 18, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#INC_SRC_TILE_FACE_ROW_IDX",
    );

    /// `SET_DST_TILE_FACE_ROW_IDX`, from the Quasar LLK assembly table.
    pub static SET_DST_TILE_FACE_ROW_IDX: InstructionDef = InstructionDef::new(
        "SET_DST_TILE_FACE_ROW_IDX",
        "SET_DST_TILE_FACE_ROW_IDX",
        0x0d,
        &[
            Field::new("Tile_Face_Row_Sel", 21, 2, false, None),
            Field::new("EngineSel", 18, 3, false, None),
            Field::new("Value", 0, 18, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SET_DST_TILE_FACE_ROW_IDX",
    );

    /// `INC_DST_TILE_FACE_ROW_IDX`, from the Quasar LLK assembly table.
    pub static INC_DST_TILE_FACE_ROW_IDX: InstructionDef = InstructionDef::new(
        "INC_DST_TILE_FACE_ROW_IDX",
        "INC_DST_TILE_FACE_ROW_IDX",
        0x0e,
        &[
            Field::new("Tile_Face_Row_Sel", 21, 2, false, None),
            Field::new("EngineSel", 18, 3, false, None),
            Field::new("Value", 0, 18, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#INC_DST_TILE_FACE_ROW_IDX",
    );

    /// `ADDGPR`, from the Quasar LLK assembly table.
    pub static ADDGPR: InstructionDef = InstructionDef::new(
        "ADDGPR",
        "ADDGPR",
        0x58,
        &[
            Field::new("OpB_is_Const", 23, 1, false, None),
            Field::new("Result_GPR_Index", 12, 6, false, None),
            Field::new("OpB_GPR_Index", 6, 6, false, None),
            Field::new("OpA_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x007c0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ADDGPR",
    );

    /// `SUBGPR`, from the Quasar LLK assembly table.
    pub static SUBGPR: InstructionDef = InstructionDef::new(
        "SUBGPR",
        "SUBGPR",
        0x59,
        &[
            Field::new("OpB_is_Const", 23, 1, false, None),
            Field::new("Result_GPR_Index", 12, 6, false, None),
            Field::new("OpB_GPR_Index", 6, 6, false, None),
            Field::new("OpA_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x007c0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SUBGPR",
    );

    /// `MULGPR`, from the Quasar LLK assembly table.
    pub static MULGPR: InstructionDef = InstructionDef::new(
        "MULGPR",
        "MULGPR",
        0x5a,
        &[
            Field::new("OpB_is_Const", 23, 1, false, None),
            Field::new("Result_GPR_Index", 12, 6, false, None),
            Field::new("OpB_GPR_Index", 6, 6, false, None),
            Field::new("OpA_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x007c0000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MULGPR",
    );

    /// `BITWOPGPR`, from the Quasar LLK assembly table.
    pub static BITWOPGPR: InstructionDef = InstructionDef::new(
        "BITWOPGPR",
        "BITWOPGPR",
        0x5b,
        &[
            Field::new("OpB_is_Const", 23, 1, false, None),
            Field::new("OpSel", 18, 5, false, None),
            Field::new("Result_GPR_Index", 12, 6, false, None),
            Field::new("OpB_GPR_Index", 6, 6, false, None),
            Field::new("OpA_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#BITWOPGPR",
    );

    /// `SHIFTGPR`, from the Quasar LLK assembly table.
    pub static SHIFTGPR: InstructionDef = InstructionDef::new(
        "SHIFTGPR",
        "SHIFTGPR",
        0x5c,
        &[
            Field::new("OpB_is_Const", 23, 1, false, None),
            Field::new("OpSel", 18, 5, false, None),
            Field::new("Result_GPR_Index", 12, 6, false, None),
            Field::new("OpB_GPR_Index", 6, 6, false, None),
            Field::new("OpA_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SHIFTGPR",
    );

    /// `CMPGPR`, from the Quasar LLK assembly table.
    pub static CMPGPR: InstructionDef = InstructionDef::new(
        "CMPGPR",
        "CMPGPR",
        0x5d,
        &[
            Field::new("OpB_is_Const", 23, 1, false, None),
            Field::new("OpSel", 18, 5, false, None),
            Field::new("Result_GPR_Index", 12, 6, false, None),
            Field::new("OpB_GPR_Index", 6, 6, false, None),
            Field::new("OpA_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#CMPGPR",
    );

    /// `DMANOP`, from the Quasar LLK assembly table.
    pub static DMANOP: InstructionDef = InstructionDef::new(
        "DMANOP",
        "DMANOP",
        0x60,
        &[],
        &[],
        0x00ffffff,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#DMANOP",
    );

    /// `ATINCGET`, from the Quasar LLK assembly table.
    pub static ATINCGET: InstructionDef = InstructionDef::new(
        "ATINCGET",
        "ATINCGET",
        0x61,
        &[
            Field::new("WrapVal", 14, 9, false, None),
            Field::new("Sel32b", 12, 2, false, None),
            Field::new("Data_GPR_Index", 6, 6, false, None),
            Field::new("Addr_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ATINCGET",
    );

    /// `ATINCGETPTR`, from the Quasar LLK assembly table.
    pub static ATINCGETPTR: InstructionDef = InstructionDef::new(
        "ATINCGETPTR",
        "ATINCGETPTR",
        0x62,
        &[
            Field::new("NoIncr", 22, 1, false, None),
            Field::new("IncrVal", 18, 4, false, None),
            Field::new("WrapVal", 14, 4, false, None),
            Field::new("Sel32b", 12, 2, false, None),
            Field::new("Data_GPR_Index", 6, 6, false, None),
            Field::new("Addr_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ATINCGETPTR",
    );

    /// `ATSWAP`, from the Quasar LLK assembly table.
    pub static ATSWAP: InstructionDef = InstructionDef::new(
        "ATSWAP",
        "ATSWAP",
        0x63,
        &[
            Field::new("SwapMask", 14, 9, false, None),
            Field::new("Data_GPR_Index", 6, 8, false, None),
            Field::new("Addr_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00800000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ATSWAP",
    );

    /// `ATCAS`, from the Quasar LLK assembly table.
    pub static ATCAS: InstructionDef = InstructionDef::new(
        "ATCAS",
        "ATCAS",
        0x64,
        &[
            Field::new("SwapVal", 18, 4, false, None),
            Field::new("CmpVal", 14, 4, false, None),
            Field::new("Sel32b", 12, 2, false, None),
            Field::new("Addr_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00c00fc0,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#ATCAS",
    );

    /// `STOREIND`, from the Quasar LLK assembly table.
    pub static STOREIND: InstructionDef = InstructionDef::new(
        "STOREIND",
        "STOREIND",
        0x66,
        &[
            Field::new("SizeSel", 22, 2, false, None),
            Field::new("MemSel", 21, 1, false, None),
            Field::new("OffsetIndex", 14, 7, false, None),
            Field::new("AutoIncSpec", 12, 2, false, None),
            Field::new("Data_GPR_Index", 6, 6, false, None),
            Field::new("Addr_GPR_Index", 0, 6, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#STOREIND",
    );

    /// `STOREREG`, from the Quasar LLK assembly table.
    pub static STOREREG: InstructionDef = InstructionDef::new(
        "STOREREG",
        "STOREREG",
        0x67,
        &[
            Field::new("Data_GPR_Index", 18, 6, false, None),
            Field::new("RegAddr", 0, 18, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#STOREREG",
    );

    /// `LOADREG`, from the Quasar LLK assembly table.
    pub static LOADREG: InstructionDef = InstructionDef::new(
        "LOADREG",
        "LOADREG",
        0x68,
        &[
            Field::new("Data_GPR_Index", 18, 6, false, None),
            Field::new("RegAddr", 0, 18, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#LOADREG",
    );

    /// `SFPLOAD`, from the Quasar LLK assembly table.
    pub static SFPLOAD: InstructionDef = InstructionDef::new(
        "SFPLOAD",
        "SFPLOAD",
        0x70,
        &[
            Field::new("lreg_ind", 20, 4, false, None),
            Field::new("instr_mod0", 16, 4, false, None),
            Field::new("sfpu_addr_mode", 13, 3, false, None),
            Field::new("done", 11, 1, false, None),
            Field::new("dest_reg_addr", 0, 11, false, None),
        ],
        &[],
        0x00001000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLOAD",
    );

    /// `SFPLOADI`, from the Quasar LLK assembly table.
    pub static SFPLOADI: InstructionDef = InstructionDef::new(
        "SFPLOADI",
        "SFPLOADI",
        0x71,
        &[
            Field::new("lreg_ind", 20, 4, false, None),
            Field::new("instr_mod0", 16, 4, false, None),
            Field::new("imm16", 0, 16, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLOADI",
    );

    /// `SFPSTORE`, from the Quasar LLK assembly table.
    pub static SFPSTORE: InstructionDef = InstructionDef::new(
        "SFPSTORE",
        "SFPSTORE",
        0x72,
        &[
            Field::new("lreg_ind", 20, 4, false, None),
            Field::new("instr_mod0", 16, 4, false, None),
            Field::new("sfpu_addr_mode", 13, 3, false, None),
            Field::new("done", 11, 1, false, None),
            Field::new("dest_reg_addr", 0, 11, false, None),
        ],
        &[],
        0x00001000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSTORE",
    );

    /// `SFPLOADMACRO`, from the Quasar LLK assembly table.
    pub static SFPLOADMACRO: InstructionDef = InstructionDef::new(
        "SFPLOADMACRO",
        "SFPLOADMACRO",
        0x93,
        &[
            Field::new("seq_id", 22, 2, false, None),
            Field::new("lreg_ind_lo", 20, 2, false, None),
            Field::new("instr_mod0", 16, 4, false, None),
            Field::new("sfpu_addr_mode", 13, 3, false, None),
            Field::new("done", 11, 1, false, None),
            Field::new("dest_reg_addr", 1, 10, false, None),
            Field::new("lreg_ind_hi", 0, 1, false, None),
        ],
        &[],
        0x00001000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLOADMACRO",
    );

    /// `SFPCOMPC`, from the Quasar LLK assembly table.
    pub static SFPCOMPC: InstructionDef = InstructionDef::new(
        "SFPCOMPC",
        "SFPCOMPC",
        0x8b,
        &[],
        &[],
        0x00ffffff,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPCOMPC",
    );

    /// `SFPENCC`, from the Quasar LLK assembly table.
    pub static SFPENCC: InstructionDef = InstructionDef::new(
        "SFPENCC",
        "SFPENCC",
        0x8a,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000ff0,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPENCC",
    );

    /// `SFPPOPC`, from the Quasar LLK assembly table.
    pub static SFPPOPC: InstructionDef = InstructionDef::new(
        "SFPPOPC",
        "SFPPOPC",
        0x88,
        &[Field::new("instr_mod1", 0, 4, false, None)],
        &[],
        0x00fffff0,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPPOPC",
    );

    /// `SFPPUSHC`, from the Quasar LLK assembly table.
    pub static SFPPUSHC: InstructionDef = InstructionDef::new(
        "SFPPUSHC",
        "SFPPUSHC",
        0x87,
        &[Field::new("instr_mod1", 0, 4, false, None)],
        &[],
        0x00fffff0,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPPUSHC",
    );

    /// `SFPSETCC`, from the Quasar LLK assembly table.
    pub static SFPSETCC: InstructionDef = InstructionDef::new(
        "SFPSETCC",
        "SFPSETCC",
        0x7b,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x000000f0,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSETCC",
    );

    /// `SFPCAST`, from the Quasar LLK assembly table.
    pub static SFPCAST: InstructionDef = InstructionDef::new(
        "SFPCAST",
        "SFPCAST",
        0x90,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPCAST",
    );

    /// `SFP_STOCH_RND`, from the Quasar LLK assembly table.
    pub static SFP_STOCH_RND: InstructionDef = InstructionDef::new(
        "SFP_STOCH_RND",
        "SFP_STOCH_RND",
        0x8e,
        &[
            Field::new("rnd_mode", 21, 3, false, None),
            Field::new("imm8_math", 16, 5, false, None),
            Field::new("lreg_b", 12, 4, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFP_STOCH_RND",
    );

    /// `SFPABS`, from the Quasar LLK assembly table.
    pub static SFPABS: InstructionDef = InstructionDef::new(
        "SFPABS",
        "SFPABS",
        0x7d,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPABS",
    );

    /// `SFPGT`, from the Quasar LLK assembly table.
    pub static SFPGT: InstructionDef = InstructionDef::new(
        "SFPGT",
        "SFPGT",
        0x97,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPGT",
    );

    /// `SFPIADD`, from the Quasar LLK assembly table.
    pub static SFPIADD: InstructionDef = InstructionDef::new(
        "SFPIADD",
        "SFPIADD",
        0x79,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPIADD",
    );

    /// `SFPLE`, from the Quasar LLK assembly table.
    pub static SFPLE: InstructionDef = InstructionDef::new(
        "SFPLE",
        "SFPLE",
        0x96,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLE",
    );

    /// `SFPNONLINEAR`, from the Quasar LLK assembly table.
    pub static SFPNONLINEAR: InstructionDef = InstructionDef::new(
        "SFPNONLINEAR",
        "SFPNONLINEAR",
        0x99,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPNONLINEAR",
    );

    /// `SFPSHFT`, from the Quasar LLK assembly table.
    pub static SFPSHFT: InstructionDef = InstructionDef::new(
        "SFPSHFT",
        "SFPSHFT",
        0x7a,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSHFT",
    );

    /// `SFPSWAP`, from the Quasar LLK assembly table.
    pub static SFPSWAP: InstructionDef = InstructionDef::new(
        "SFPSWAP",
        "SFPSWAP",
        0x92,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSWAP",
    );

    /// `SFPAND`, from the Quasar LLK assembly table.
    pub static SFPAND: InstructionDef = InstructionDef::new(
        "SFPAND",
        "SFPAND",
        0x7e,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
        ],
        &[],
        0x00fff00f,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPAND",
    );

    /// `SFPDIVP2`, from the Quasar LLK assembly table.
    pub static SFPDIVP2: InstructionDef = InstructionDef::new(
        "SFPDIVP2",
        "SFPDIVP2",
        0x76,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPDIVP2",
    );

    /// `SFPEXEXP`, from the Quasar LLK assembly table.
    pub static SFPEXEXP: InstructionDef = InstructionDef::new(
        "SFPEXEXP",
        "SFPEXEXP",
        0x77,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPEXEXP",
    );

    /// `SFPEXMAN`, from the Quasar LLK assembly table.
    pub static SFPEXMAN: InstructionDef = InstructionDef::new(
        "SFPEXMAN",
        "SFPEXMAN",
        0x78,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPEXMAN",
    );

    /// `SFPLZ`, from the Quasar LLK assembly table.
    pub static SFPLZ: InstructionDef = InstructionDef::new(
        "SFPLZ",
        "SFPLZ",
        0x81,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLZ",
    );

    /// `SFPNOT`, from the Quasar LLK assembly table.
    pub static SFPNOT: InstructionDef = InstructionDef::new(
        "SFPNOT",
        "SFPNOT",
        0x80,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
        ],
        &[],
        0x00fff00f,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPNOT",
    );

    /// `SFPOR`, from the Quasar LLK assembly table.
    pub static SFPOR: InstructionDef = InstructionDef::new(
        "SFPOR",
        "SFPOR",
        0x7f,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
        ],
        &[],
        0x00fff00f,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPOR",
    );

    /// `SFPSETEXP`, from the Quasar LLK assembly table.
    pub static SFPSETEXP: InstructionDef = InstructionDef::new(
        "SFPSETEXP",
        "SFPSETEXP",
        0x82,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSETEXP",
    );

    /// `SFPSETMAN`, from the Quasar LLK assembly table.
    pub static SFPSETMAN: InstructionDef = InstructionDef::new(
        "SFPSETMAN",
        "SFPSETMAN",
        0x83,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSETMAN",
    );

    /// `SFPSETSGN`, from the Quasar LLK assembly table.
    pub static SFPSETSGN: InstructionDef = InstructionDef::new(
        "SFPSETSGN",
        "SFPSETSGN",
        0x89,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSETSGN",
    );

    /// `SFPXOR`, from the Quasar LLK assembly table.
    pub static SFPXOR: InstructionDef = InstructionDef::new(
        "SFPXOR",
        "SFPXOR",
        0x8d,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
        ],
        &[],
        0x00fff00f,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPXOR",
    );

    /// `SFPADD`, from the Quasar LLK assembly table.
    pub static SFPADD: InstructionDef = InstructionDef::new(
        "SFPADD",
        "SFPADD",
        0x85,
        &[
            Field::new("lreg_a", 16, 8, false, None),
            Field::new("lreg_b", 12, 4, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPADD",
    );

    /// `SFPADDI`, from the Quasar LLK assembly table.
    pub static SFPADDI: InstructionDef = InstructionDef::new(
        "SFPADDI",
        "SFPADDI",
        0x75,
        &[
            Field::new("imm16_math", 8, 16, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPADDI",
    );

    /// `SFPLUT`, from the Quasar LLK assembly table.
    pub static SFPLUT: InstructionDef = InstructionDef::new(
        "SFPLUT",
        "SFPLUT",
        0x73,
        &[
            Field::new("lreg_ind", 20, 4, false, None),
            Field::new("instr_mod0", 16, 4, false, None),
        ],
        &[],
        0x0000ffff,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLUT",
    );

    /// `SFPLUTFP32`, from the Quasar LLK assembly table.
    pub static SFPLUTFP32: InstructionDef = InstructionDef::new(
        "SFPLUTFP32",
        "SFPLUTFP32",
        0x95,
        &[
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00ffff00,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPLUTFP32",
    );

    /// `SFPMAD`, from the Quasar LLK assembly table.
    pub static SFPMAD: InstructionDef = InstructionDef::new(
        "SFPMAD",
        "SFPMAD",
        0x84,
        &[
            Field::new("lreg_a", 16, 8, false, None),
            Field::new("lreg_b", 12, 4, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPMAD",
    );

    /// `SFPMUL`, from the Quasar LLK assembly table.
    pub static SFPMUL: InstructionDef = InstructionDef::new(
        "SFPMUL",
        "SFPMUL",
        0x86,
        &[
            Field::new("lreg_a", 16, 8, false, None),
            Field::new("lreg_b", 12, 4, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPMUL",
    );

    /// `SFPMULI`, from the Quasar LLK assembly table.
    pub static SFPMULI: InstructionDef = InstructionDef::new(
        "SFPMULI",
        "SFPMULI",
        0x74,
        &[
            Field::new("imm16_math", 8, 16, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPMULI",
    );

    /// `SFPMUL24`, from the Quasar LLK assembly table.
    pub static SFPMUL24: InstructionDef = InstructionDef::new(
        "SFPMUL24",
        "SFPMUL24",
        0x98,
        &[
            Field::new("lreg_a", 16, 8, false, None),
            Field::new("lreg_b", 12, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000f00,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPMUL24",
    );

    /// `SFPMOV`, from the Quasar LLK assembly table.
    pub static SFPMOV: InstructionDef = InstructionDef::new(
        "SFPMOV",
        "SFPMOV",
        0x7c,
        &[
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00fff000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPMOV",
    );

    /// `SFPSHFT2`, from the Quasar LLK assembly table.
    pub static SFPSHFT2: InstructionDef = InstructionDef::new(
        "SFPSHFT2",
        "SFPSHFT2",
        0x94,
        &[
            Field::new("imm12_math", 12, 12, false, None),
            Field::new("lreg_c", 8, 4, false, None),
            Field::new("lreg_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPSHFT2",
    );

    /// `SFPTRANSP`, from the Quasar LLK assembly table.
    pub static SFPTRANSP: InstructionDef = InstructionDef::new(
        "SFPTRANSP",
        "SFPTRANSP",
        0x8c,
        &[],
        &[],
        0x00ffffff,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPTRANSP",
    );

    /// `SFPCONFIG`, from the Quasar LLK assembly table.
    pub static SFPCONFIG: InstructionDef = InstructionDef::new(
        "SFPCONFIG",
        "SFPCONFIG",
        0x91,
        &[
            Field::new("imm16_math", 8, 16, false, None),
            Field::new("config_dest", 4, 4, false, None),
            Field::new("instr_mod1", 0, 4, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPCONFIG",
    );

    /// `SFPNOP`, from the Quasar LLK assembly table.
    pub static SFPNOP: InstructionDef = InstructionDef::new(
        "SFPNOP",
        "SFPNOP",
        0x8f,
        &[
            Field::new("srcs_wr_done", 2, 1, false, None),
            Field::new("srcs_rd_done", 1, 1, false, None),
            Field::new("dest_done", 0, 1, false, None),
        ],
        &[],
        0x00fffff8,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#SFPNOP",
    );

    /// `WRCFG`, from the Quasar LLK assembly table.
    pub static WRCFG: InstructionDef = InstructionDef::new(
        "WRCFG",
        "WRCFG",
        0xb0,
        &[
            Field::new("GprAddress", 16, 8, false, None),
            Field::new("wr128b", 15, 1, false, None),
            Field::new("CfgReg", 0, 15, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#WRCFG",
    );

    /// `RDCFG`, from the Quasar LLK assembly table.
    pub static RDCFG: InstructionDef = InstructionDef::new(
        "RDCFG",
        "RDCFG",
        0xb1,
        &[
            Field::new("GprAddress", 16, 8, false, None),
            Field::new("CfgReg", 0, 16, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RDCFG",
    );

    /// `RMWCIB0`, from the Quasar LLK assembly table.
    pub static RMWCIB0: InstructionDef = InstructionDef::new(
        "RMWCIB0",
        "RMWCIB0",
        0xb2,
        &[
            Field::new("CfgRegAddr", 16, 8, false, None),
            Field::new("Mask", 8, 8, false, None),
            Field::new("Data", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB0",
    );

    /// `RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`, from the Quasar LLK assembly table.
    pub static RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE: InstructionDef =
        InstructionDef::new(
            "RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            "RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            0xb3,
            &[
                Field::new("CfgRegAddr", 16, 8, false, None),
                Field::new("Mask", 8, 8, false, None),
                Field::new("Data", 0, 8, false, None),
            ],
            &[],
            0x00000000,
            Provenance::Quasar,
            "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
        );

    /// `RMWCIB1`, from the Quasar LLK assembly table.
    pub static RMWCIB1: InstructionDef = InstructionDef::new(
        "RMWCIB1",
        "RMWCIB1",
        0xb4,
        &[
            Field::new("CfgRegAddr", 16, 8, false, None),
            Field::new("Mask", 8, 8, false, None),
            Field::new("Data", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB1",
    );

    /// `RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`, from the Quasar LLK assembly table.
    pub static RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE: InstructionDef =
        InstructionDef::new(
            "RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            "RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            0xb5,
            &[
                Field::new("CfgRegAddr", 16, 8, false, None),
                Field::new("Mask", 8, 8, false, None),
                Field::new("Data", 0, 8, false, None),
            ],
            &[],
            0x00000000,
            Provenance::Quasar,
            "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
        );

    /// `RMWCIB2`, from the Quasar LLK assembly table.
    pub static RMWCIB2: InstructionDef = InstructionDef::new(
        "RMWCIB2",
        "RMWCIB2",
        0xb6,
        &[
            Field::new("CfgRegAddr", 16, 8, false, None),
            Field::new("Mask", 8, 8, false, None),
            Field::new("Data", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB2",
    );

    /// `RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`, from the Quasar LLK assembly table.
    pub static RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE: InstructionDef =
        InstructionDef::new(
            "RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            "RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            0xb7,
            &[
                Field::new("CfgRegAddr", 16, 8, false, None),
                Field::new("Mask", 8, 8, false, None),
                Field::new("Data", 0, 8, false, None),
            ],
            &[],
            0x00000000,
            Provenance::Quasar,
            "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
        );

    /// `RMWCIB3`, from the Quasar LLK assembly table.
    pub static RMWCIB3: InstructionDef = InstructionDef::new(
        "RMWCIB3",
        "RMWCIB3",
        0xb8,
        &[
            Field::new("CfgRegAddr", 16, 8, false, None),
            Field::new("Mask", 8, 8, false, None),
            Field::new("Data", 0, 8, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB3",
    );

    /// `RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`, from the Quasar LLK assembly table.
    pub static RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE: InstructionDef =
        InstructionDef::new(
            "RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            "RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            0xb9,
            &[
                Field::new("CfgRegAddr", 16, 8, false, None),
                Field::new("Mask", 8, 8, false, None),
                Field::new("Data", 0, 8, false, None),
            ],
            &[],
            0x00000000,
            Provenance::Quasar,
            "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
        );

    /// `CFGSHIFTMASK`, from the Quasar LLK assembly table.
    pub static CFGSHIFTMASK: InstructionDef = InstructionDef::new(
        "CFGSHIFTMASK",
        "CFGSHIFTMASK",
        0xba,
        &[
            Field::new("CfgRegAddr", 16, 8, false, None),
            Field::new("disable_mask_on_old_val", 15, 1, false, None),
            Field::new("operation", 12, 3, false, None),
            Field::new("mask_width", 7, 5, false, None),
            Field::new("right_cshift_amt", 2, 5, false, None),
            Field::new("scratch_sel", 0, 2, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#CFGSHIFTMASK",
    );

    /// `CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`, from the Quasar LLK assembly table.
    pub static CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE: InstructionDef =
        InstructionDef::new(
            "CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            "CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
            0xbb,
            &[
                Field::new("CfgRegAddr", 16, 8, false, None),
                Field::new("disable_mask_on_old_val", 15, 1, false, None),
                Field::new("operation", 12, 3, false, None),
                Field::new("mask_width", 7, 5, false, None),
                Field::new("right_cshift_amt", 2, 5, false, None),
                Field::new("scratch_sel", 0, 2, false, None),
            ],
            &[],
            0x00000000,
            Provenance::Quasar,
            "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE",
        );

    /// `COMMIT_SHADOW`, from the Quasar LLK assembly table.
    pub static COMMIT_SHADOW: InstructionDef = InstructionDef::new(
        "COMMIT_SHADOW",
        "COMMIT_SHADOW",
        0x41,
        &[Field::new("force_commit", 0, 20, false, None)],
        &[],
        0x00f00000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#COMMIT_SHADOW",
    );

    /// `MOP`, from the Quasar LLK assembly table.
    pub static MOP: InstructionDef = InstructionDef::new(
        "MOP",
        "MOP",
        0x01,
        &[
            Field::new("mop_type", 23, 1, false, None),
            Field::new("done", 22, 1, false, None),
            Field::new("loop_count", 15, 7, false, None),
            Field::new("zmask_lo8_or_loop_count", 0, 15, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOP",
    );

    /// `MOP_CFG`, from the Quasar LLK assembly table.
    pub static MOP_CFG: InstructionDef = InstructionDef::new(
        "MOP_CFG",
        "MOP_CFG",
        0x03,
        &[Field::new("zmask_hi24", 0, 24, false, None)],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#MOP_CFG",
    );

    /// `REPLAY`, from the Quasar LLK assembly table.
    pub static REPLAY: InstructionDef = InstructionDef::new(
        "REPLAY",
        "REPLAY",
        0x04,
        &[
            Field::new("start_idx", 14, 10, false, None),
            Field::new("len", 4, 10, false, None),
            Field::new("last", 3, 1, false, None),
            Field::new("set_mutex", 2, 1, false, None),
            Field::new("execute_while_loading", 1, 1, false, None),
            Field::new("load_mode", 0, 1, false, None),
        ],
        &[],
        0x00000000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#REPLAY",
    );

    /// `RESOURCEDECL`, from the Quasar LLK assembly table.
    pub static RESOURCEDECL: InstructionDef = InstructionDef::new(
        "RESOURCEDECL",
        "RESOURCEDECL",
        0x05,
        &[
            Field::new("linger_time", 15, 4, false, None),
            Field::new("resources", 5, 10, false, None),
            Field::new("op_class", 0, 5, false, None),
        ],
        &[],
        0x00f80000,
        Provenance::Quasar,
        "tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#RESOURCEDECL",
    );
}

/// Every Quasar instruction encoding, for table-wide checks.
pub static ALL: &[&InstructionDef] = &[
    &defs::ATGETM,
    &defs::ATRELM,
    &defs::STALLWAIT,
    &defs::SEMINIT,
    &defs::SEMPOST,
    &defs::SEMGET,
    &defs::SEMWAIT,
    &defs::WAIT_TILES,
    &defs::WAIT_FREE,
    &defs::PUSH_TILES,
    &defs::POP_TILES,
    &defs::NOP,
    &defs::MOVD2A,
    &defs::MOVD2B,
    &defs::MOVB2A,
    &defs::ZEROACC,
    &defs::ZEROSRC,
    &defs::MOVA2D,
    &defs::MOVB2D,
    &defs::MOVDBGA2D,
    &defs::MOVDBGB2D,
    &defs::SHIFTXB,
    &defs::HALT,
    &defs::MVMULDI,
    &defs::MVMUL,
    &defs::ELWMUL,
    &defs::ELWADD,
    &defs::ELWSUB,
    &defs::ELWADDDI,
    &defs::ELWSUBDI,
    &defs::GMPOOL,
    &defs::GAPOOL,
    &defs::SETDVALID,
    &defs::RV_PACR,
    &defs::PACR_STRIDE,
    &defs::CLEARDVALID,
    &defs::SETRWC,
    &defs::INCRWC,
    &defs::RV_UNPACR,
    &defs::ELWMULDI,
    &defs::PACR0_TILE,
    &defs::PACR0_TILE_INC,
    &defs::PACR0_FACE,
    &defs::PACR0_FACE_INC,
    &defs::PACR0_ROW,
    &defs::PACR0_ROW_INC,
    &defs::PACR1_TILE,
    &defs::PACR1_TILE_INC,
    &defs::PACR1_FACE,
    &defs::PACR1_FACE_INC,
    &defs::PACR1_ROW,
    &defs::PACR1_ROW_INC,
    &defs::PACR_UNTILIZE,
    &defs::UNPACR0_TILE,
    &defs::UNPACR0_TILE_INC,
    &defs::UNPACR0_FACE,
    &defs::UNPACR0_FACE_INC,
    &defs::UNPACR0_ROW,
    &defs::UNPACR0_ROW_INC,
    &defs::RV_WRCFG,
    &defs::UNPACR1_TILE,
    &defs::UNPACR1_TILE_INC,
    &defs::UNPACR1_FACE,
    &defs::UNPACR1_FACE_INC,
    &defs::UNPACR1_ROW,
    &defs::UNPACR1_ROW_INC,
    &defs::UNPACR2_TILE,
    &defs::UNPACR2_TILE_INC,
    &defs::UNPACR2_FACE,
    &defs::UNPACR2_FACE_INC,
    &defs::UNPACR2_ROW,
    &defs::UNPACR2_ROW_INC,
    &defs::UNPACR_DEST_TILE,
    &defs::UNPACR_DEST_TILE_INC,
    &defs::UNPACR_DEST_FACE,
    &defs::UNPACR_DEST_FACE_INC,
    &defs::UNPACR_DEST_ROW,
    &defs::UNPACR_DEST_ROW_INC,
    &defs::UNPACR_TILE_MISC,
    &defs::UNPACR0_STRIDE,
    &defs::UNPACR1_STRIDE,
    &defs::UNPACR2_STRIDE,
    &defs::UNPACR_DEST_STRIDE,
    &defs::UNPACR_TILIZE,
    &defs::UNPACR_NOP,
    &defs::SETGPR,
    &defs::LOADIND,
    &defs::SET_SRC_TILE_FACE_ROW_IDX,
    &defs::INC_SRC_TILE_FACE_ROW_IDX,
    &defs::SET_DST_TILE_FACE_ROW_IDX,
    &defs::INC_DST_TILE_FACE_ROW_IDX,
    &defs::ADDGPR,
    &defs::SUBGPR,
    &defs::MULGPR,
    &defs::BITWOPGPR,
    &defs::SHIFTGPR,
    &defs::CMPGPR,
    &defs::DMANOP,
    &defs::ATINCGET,
    &defs::ATINCGETPTR,
    &defs::ATSWAP,
    &defs::ATCAS,
    &defs::STOREIND,
    &defs::STOREREG,
    &defs::LOADREG,
    &defs::SFPLOAD,
    &defs::SFPLOADI,
    &defs::SFPSTORE,
    &defs::SFPLOADMACRO,
    &defs::SFPCOMPC,
    &defs::SFPENCC,
    &defs::SFPPOPC,
    &defs::SFPPUSHC,
    &defs::SFPSETCC,
    &defs::SFPCAST,
    &defs::SFP_STOCH_RND,
    &defs::SFPABS,
    &defs::SFPGT,
    &defs::SFPIADD,
    &defs::SFPLE,
    &defs::SFPNONLINEAR,
    &defs::SFPSHFT,
    &defs::SFPSWAP,
    &defs::SFPAND,
    &defs::SFPDIVP2,
    &defs::SFPEXEXP,
    &defs::SFPEXMAN,
    &defs::SFPLZ,
    &defs::SFPNOT,
    &defs::SFPOR,
    &defs::SFPSETEXP,
    &defs::SFPSETMAN,
    &defs::SFPSETSGN,
    &defs::SFPXOR,
    &defs::SFPADD,
    &defs::SFPADDI,
    &defs::SFPLUT,
    &defs::SFPLUTFP32,
    &defs::SFPMAD,
    &defs::SFPMUL,
    &defs::SFPMULI,
    &defs::SFPMUL24,
    &defs::SFPMOV,
    &defs::SFPSHFT2,
    &defs::SFPTRANSP,
    &defs::SFPCONFIG,
    &defs::SFPNOP,
    &defs::WRCFG,
    &defs::RDCFG,
    &defs::RMWCIB0,
    &defs::RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
    &defs::RMWCIB1,
    &defs::RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
    &defs::RMWCIB2,
    &defs::RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
    &defs::RMWCIB3,
    &defs::RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
    &defs::CFGSHIFTMASK,
    &defs::CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
    &defs::COMMIT_SHADOW,
    &defs::MOP,
    &defs::MOP_CFG,
    &defs::REPLAY,
    &defs::RESOURCEDECL,
];

/// One encoder per instruction, taking its operands from the high bits of
/// the word down.
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
                /// Every operand zero.
                pub const ZERO: Self = $ty { $($field: 0,)* };
                $(pub const fn $field(mut self, value: u32) -> Self { self.$field = value; self })*
                pub const fn encode(self) -> Result<Instruction, EncodeError> {
                    Instruction::with_operands(&$def, &[$(self.$field),*])
                }
            }
        };
    }

    /// `ATGETM`.
    pub const fn atgetm(mutex_index: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATGETM, &[mutex_index])
    }

    /// `ATRELM`.
    pub const fn atrelm(mutex_index: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATRELM, &[mutex_index])
    }

    /// `STALLWAIT`.
    pub const fn stallwait(
        stall_res: u32,
        wait_res_idx_2: u32,
        wait_res_idx_1: u32,
        wait_res_idx_0: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::STALLWAIT,
            &[stall_res, wait_res_idx_2, wait_res_idx_1, wait_res_idx_0],
        )
    }

    /// `SEMINIT`.
    pub const fn seminit(
        max_value: u32,
        init_value: u32,
        sem_bank_sel: u32,
        sem_sel: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SEMINIT,
            &[max_value, init_value, sem_bank_sel, sem_sel],
        )
    }

    /// `SEMPOST`.
    pub const fn sempost(sem_bank_sel: u32, sem_sel: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SEMPOST, &[sem_bank_sel, sem_sel])
    }

    /// `SEMGET`.
    pub const fn semget(sem_bank_sel: u32, sem_sel: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SEMGET, &[sem_bank_sel, sem_sel])
    }

    /// `SEMWAIT`.
    pub const fn semwait(
        stall_res: u32,
        wait_sem_cond: u32,
        sem_bank_sel: u32,
        sem_sel: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SEMWAIT,
            &[stall_res, wait_sem_cond, sem_bank_sel, sem_sel],
        )
    }

    /// `WAIT_TILES`.
    pub const fn wait_tiles(
        stall_res: u32,
        num_tiles: u32,
        buffer_sel: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::WAIT_TILES, &[stall_res, num_tiles, buffer_sel])
    }

    /// `WAIT_FREE`.
    pub const fn wait_free(
        stall_res: u32,
        num_tiles: u32,
        buffer_sel: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::WAIT_FREE, &[stall_res, num_tiles, buffer_sel])
    }

    /// `PUSH_TILES`.
    pub const fn push_tiles(
        packer_wr_done_wait_mask: u32,
        num_tiles: u32,
        buffer_sel: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::PUSH_TILES,
            &[packer_wr_done_wait_mask, num_tiles, buffer_sel],
        )
    }

    /// `POP_TILES`.
    pub const fn pop_tiles(
        unpacker_rd_done_wait_mask: u32,
        num_tiles: u32,
        buffer_sel: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::POP_TILES,
            &[unpacker_rd_done_wait_mask, num_tiles, buffer_sel],
        )
    }

    /// `NOP`.
    pub const fn nop() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::NOP, &[])
    }

    builder!(
        #[doc = "`MOVD2A`."]
        Movd2A,
        defs::MOVD2A,
        [dest_32b_lo, src, addr_mode, instr_mod, dst]
    );

    builder!(
        #[doc = "`MOVD2B`."]
        Movd2B,
        defs::MOVD2B,
        [dest_32b_lo, src, addr_mode, instr_mod, transpose, dst]
    );

    /// `MOVB2A`.
    pub const fn movb2_a(
        srca: u32,
        addr_mode: u32,
        instr_mod: u32,
        srcb: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MOVB2A, &[srca, addr_mode, instr_mod, srcb])
    }

    builder!(
        #[doc = "`ZEROACC`."]
        Zeroacc,
        defs::ZEROACC,
        [
            clear_mode,
            use_32_bit_mode,
            clear_zero_flags,
            addr_mode,
            where_
        ]
    );

    builder!(
        #[doc = "`ZEROSRC`."]
        Zerosrc,
        defs::ZEROSRC,
        [
            packed_fmt, int_fmt, exp_bias, zero_val, write_mode, bank_mask, src_mask
        ]
    );

    builder!(
        #[doc = "`MOVA2D`."]
        Mova2D,
        defs::MOVA2D,
        [dest_32b_lo, src, addr_mode, instr_mod, dst]
    );

    builder!(
        #[doc = "`MOVB2D`."]
        Movb2D,
        defs::MOVB2D,
        [dest_32b_lo, src, addr_mode, transfer_sz, bcast_datum0, dst]
    );

    builder!(
        #[doc = "`MOVDBGA2D`."]
        Movdbga2D,
        defs::MOVDBGA2D,
        [dest_32b_lo, src, addr_mode, instr_mod, dst]
    );

    builder!(
        #[doc = "`MOVDBGB2D`."]
        Movdbgb2D,
        defs::MOVDBGB2D,
        [dest_32b_lo, src, addr_mode, transfer_sz, bcast_datum0, dst]
    );

    /// `SHIFTXB`.
    pub const fn shiftxb(
        addr_mode: u32,
        rot_shift: u32,
        shift_row: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SHIFTXB, &[addr_mode, rot_shift, shift_row])
    }

    /// `HALT`.
    pub const fn halt() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::HALT, &[])
    }

    builder!(
        #[doc = "`MVMULDI`."]
        Mvmuldi,
        defs::MVMULDI,
        [clear_dvalid, ins_mod, srcb_addr, srca_addr, addr_mode, dst]
    );

    /// `MVMUL`.
    pub const fn mvmul(
        clear_dvalid: u32,
        instr_mod19: u32,
        addr_mode: u32,
        dst: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MVMUL, &[clear_dvalid, instr_mod19, addr_mode, dst])
    }

    builder!(
        #[doc = "`ELWMUL`."]
        Elwmul,
        defs::ELWMUL,
        [clear_dvalid, dest_accum_en, instr_mod19, addr_mode, dst]
    );

    builder!(
        #[doc = "`ELWADD`."]
        Elwadd,
        defs::ELWADD,
        [clear_dvalid, dest_accum_en, instr_mod19, addr_mode, dst]
    );

    builder!(
        #[doc = "`ELWSUB`."]
        Elwsub,
        defs::ELWSUB,
        [clear_dvalid, dest_accum_en, instr_mod19, addr_mode, dst]
    );

    builder!(
        #[doc = "`ELWADDDI`."]
        Elwadddi,
        defs::ELWADDDI,
        [clear_dvalid, ins_mod, srcb_addr, srca_addr, addr_mode, dst]
    );

    builder!(
        #[doc = "`ELWSUBDI`."]
        Elwsubdi,
        defs::ELWSUBDI,
        [clear_dvalid, ins_mod, srcb_addr, srca_addr, addr_mode, dst]
    );

    builder!(
        #[doc = "`GMPOOL`."]
        Gmpool,
        defs::GMPOOL,
        [clear_dvalid, instr_mod19, pool_addr_mode, rsvd, dst]
    );

    builder!(
        #[doc = "`GAPOOL`."]
        Gapool,
        defs::GAPOOL,
        [clear_dvalid, instr_mod19, pool_addr_mode, rsvd, dst]
    );

    /// `SETDVALID`.
    pub const fn setdvalid(setvalid: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SETDVALID, &[setvalid])
    }

    /// `RV_PACR`.
    pub const fn rv_pacr(
        reg_idx2: u32,
        reg_idx1: u32,
        reg_idx0: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RV_PACR, &[reg_idx2, reg_idx1, reg_idx0])
    }

    builder!(
        #[doc = "`PACR_STRIDE`."]
        PacrStride,
        defs::PACR_STRIDE,
        [
            src_row_idx_or_inc_mul4,
            src_row_idx_inc,
            l1_tile_idx_or_tile_idx_inc,
            tile_idx_inc,
            l1_16datums_row_index,
            buffer_descriptor_table_sel,
            packer_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`CLEARDVALID`."]
        Cleardvalid,
        defs::CLEARDVALID,
        [
            cleardvalid,
            cleardvalid_s,
            dest_dvalid_reset,
            dest_dvalid_client_bank_reset,
            dest_pulse_last,
            reset
        ]
    );

    /// `SETRWC`.
    pub const fn setrwc(
        clear_ab_vld: u32,
        rwc_cr: u32,
        rwc_val: u32,
        bit_mask: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SETRWC, &[clear_ab_vld, rwc_cr, rwc_val, bit_mask])
    }

    /// `INCRWC`.
    pub const fn incrwc(
        rwc_cr: u32,
        rwc_a: u32,
        rwc_b: u32,
        rwc_d: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::INCRWC, &[rwc_cr, rwc_a, rwc_b, rwc_d])
    }

    /// `RV_UNPACR`.
    pub const fn rv_unpacr(
        reg_idx2: u32,
        reg_idx1: u32,
        reg_idx0: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RV_UNPACR, &[reg_idx2, reg_idx1, reg_idx0])
    }

    builder!(
        #[doc = "`ELWMULDI`."]
        Elwmuldi,
        defs::ELWMULDI,
        [clear_dvalid, ins_mod, srcb_addr, srca_addr, addr_mode, dst]
    );

    /// `PACR0_TILE`.
    pub const fn pacr0_tile(
        dst_tile_idx: u32,
        src_tile_idx: u32,
        buffer_descriptor_table_sel: u32,
        clr_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::PACR0_TILE,
            &[
                dst_tile_idx,
                src_tile_idx,
                buffer_descriptor_table_sel,
                clr_dat_valid,
            ],
        )
    }

    /// `PACR0_TILE_INC`.
    pub const fn pacr0_tile_inc(
        dst_tile_idx_inc: u32,
        src_tile_idx_inc: u32,
        buffer_descriptor_table_sel: u32,
        clr_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::PACR0_TILE_INC,
            &[
                dst_tile_idx_inc,
                src_tile_idx_inc,
                buffer_descriptor_table_sel,
                clr_dat_valid,
            ],
        )
    }

    builder!(
        #[doc = "`PACR0_FACE`."]
        Pacr0Face,
        defs::PACR0_FACE,
        [
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR0_FACE_INC`."]
        Pacr0FaceInc,
        defs::PACR0_FACE_INC,
        [
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR0_ROW`."]
        Pacr0Row,
        defs::PACR0_ROW,
        [
            dst_row_idx,
            src_row_idx,
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR0_ROW_INC`."]
        Pacr0RowInc,
        defs::PACR0_ROW_INC,
        [
            dst_row_idx_inc,
            src_row_idx_inc,
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    /// `PACR1_TILE`.
    pub const fn pacr1_tile(
        dst_tile_idx: u32,
        src_tile_idx: u32,
        buffer_descriptor_table_sel: u32,
        clr_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::PACR1_TILE,
            &[
                dst_tile_idx,
                src_tile_idx,
                buffer_descriptor_table_sel,
                clr_dat_valid,
            ],
        )
    }

    /// `PACR1_TILE_INC`.
    pub const fn pacr1_tile_inc(
        dst_tile_idx_inc: u32,
        src_tile_idx_inc: u32,
        buffer_descriptor_table_sel: u32,
        clr_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::PACR1_TILE_INC,
            &[
                dst_tile_idx_inc,
                src_tile_idx_inc,
                buffer_descriptor_table_sel,
                clr_dat_valid,
            ],
        )
    }

    builder!(
        #[doc = "`PACR1_FACE`."]
        Pacr1Face,
        defs::PACR1_FACE,
        [
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR1_FACE_INC`."]
        Pacr1FaceInc,
        defs::PACR1_FACE_INC,
        [
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR1_ROW`."]
        Pacr1Row,
        defs::PACR1_ROW,
        [
            dst_row_idx,
            src_row_idx,
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR1_ROW_INC`."]
        Pacr1RowInc,
        defs::PACR1_ROW_INC,
        [
            dst_row_idx_inc,
            src_row_idx_inc,
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    builder!(
        #[doc = "`PACR_UNTILIZE`."]
        PacrUntilize,
        defs::PACR_UNTILIZE,
        [
            reserved,
            cntr_reset_mask,
            dst_z_cntr_inc,
            src_z_cntr_inc,
            packer_sel,
            buffer_descriptor_table_sel,
            clr_dat_valid
        ]
    );

    /// `UNPACR0_TILE`.
    pub const fn unpacr0_tile(
        dst_tile_idx: u32,
        src_tile_idx: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR0_TILE,
            &[
                dst_tile_idx,
                src_tile_idx,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    /// `UNPACR0_TILE_INC`.
    pub const fn unpacr0_tile_inc(
        dst_tile_idx_inc: u32,
        src_tile_idx_inc: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR0_TILE_INC,
            &[
                dst_tile_idx_inc,
                src_tile_idx_inc,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    builder!(
        #[doc = "`UNPACR0_FACE`."]
        Unpacr0Face,
        defs::UNPACR0_FACE,
        [
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR0_FACE_INC`."]
        Unpacr0FaceInc,
        defs::UNPACR0_FACE_INC,
        [
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR0_ROW`."]
        Unpacr0Row,
        defs::UNPACR0_ROW,
        [
            dst_row_idx,
            src_row_idx,
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR0_ROW_INC`."]
        Unpacr0RowInc,
        defs::UNPACR0_ROW_INC,
        [
            dst_row_idx_inc,
            src_row_idx_inc,
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`RV_WRCFG`."]
        RvWrcfg,
        defs::RV_WRCFG,
        [
            byte_mask,
            write_64b,
            index_of_reg_containing_cfg_index,
            index_of_reg_containing_wrdata_msbs,
            index_of_reg_containing_wrdata_lsbs
        ]
    );

    /// `UNPACR1_TILE`.
    pub const fn unpacr1_tile(
        dst_tile_idx: u32,
        src_tile_idx: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR1_TILE,
            &[
                dst_tile_idx,
                src_tile_idx,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    /// `UNPACR1_TILE_INC`.
    pub const fn unpacr1_tile_inc(
        dst_tile_idx_inc: u32,
        src_tile_idx_inc: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR1_TILE_INC,
            &[
                dst_tile_idx_inc,
                src_tile_idx_inc,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    builder!(
        #[doc = "`UNPACR1_FACE`."]
        Unpacr1Face,
        defs::UNPACR1_FACE,
        [
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR1_FACE_INC`."]
        Unpacr1FaceInc,
        defs::UNPACR1_FACE_INC,
        [
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR1_ROW`."]
        Unpacr1Row,
        defs::UNPACR1_ROW,
        [
            dst_row_idx,
            src_row_idx,
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR1_ROW_INC`."]
        Unpacr1RowInc,
        defs::UNPACR1_ROW_INC,
        [
            dst_row_idx_inc,
            src_row_idx_inc,
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    /// `UNPACR2_TILE`.
    pub const fn unpacr2_tile(
        dst_tile_idx: u32,
        src_tile_idx: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR2_TILE,
            &[
                dst_tile_idx,
                src_tile_idx,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    /// `UNPACR2_TILE_INC`.
    pub const fn unpacr2_tile_inc(
        dst_tile_idx_inc: u32,
        src_tile_idx_inc: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR2_TILE_INC,
            &[
                dst_tile_idx_inc,
                src_tile_idx_inc,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    builder!(
        #[doc = "`UNPACR2_FACE`."]
        Unpacr2Face,
        defs::UNPACR2_FACE,
        [
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR2_FACE_INC`."]
        Unpacr2FaceInc,
        defs::UNPACR2_FACE_INC,
        [
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR2_ROW`."]
        Unpacr2Row,
        defs::UNPACR2_ROW,
        [
            dst_row_idx,
            src_row_idx,
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR2_ROW_INC`."]
        Unpacr2RowInc,
        defs::UNPACR2_ROW_INC,
        [
            dst_row_idx_inc,
            src_row_idx_inc,
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    /// `UNPACR_DEST_TILE`.
    pub const fn unpacr_dest_tile(
        dst_tile_idx: u32,
        src_tile_idx: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR_DEST_TILE,
            &[
                dst_tile_idx,
                src_tile_idx,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    /// `UNPACR_DEST_TILE_INC`.
    pub const fn unpacr_dest_tile_inc(
        dst_tile_idx_inc: u32,
        src_tile_idx_inc: u32,
        buffer_descriptor_table_sel: u32,
        set_dat_valid: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::UNPACR_DEST_TILE_INC,
            &[
                dst_tile_idx_inc,
                src_tile_idx_inc,
                buffer_descriptor_table_sel,
                set_dat_valid,
            ],
        )
    }

    builder!(
        #[doc = "`UNPACR_DEST_FACE`."]
        UnpacrDestFace,
        defs::UNPACR_DEST_FACE,
        [
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_DEST_FACE_INC`."]
        UnpacrDestFaceInc,
        defs::UNPACR_DEST_FACE_INC,
        [
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_DEST_ROW`."]
        UnpacrDestRow,
        defs::UNPACR_DEST_ROW,
        [
            dst_row_idx,
            src_row_idx,
            dst_face_idx,
            src_face_idx,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_DEST_ROW_INC`."]
        UnpacrDestRowInc,
        defs::UNPACR_DEST_ROW_INC,
        [
            dst_row_idx_inc,
            src_row_idx_inc,
            dst_face_idx_inc,
            src_face_idx_inc,
            dst_tile_offset_idx_inc,
            src_tile_offset_idx_inc,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_TILE_MISC`."]
        UnpacrTileMisc,
        defs::UNPACR_TILE_MISC,
        [
            unpack_type,
            row_bcast_row_idx,
            tile_idx_inc,
            dst_tile_idx,
            src_tile_idx,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR0_STRIDE`."]
        Unpacr0Stride,
        defs::UNPACR0_STRIDE,
        [
            src_reg_y_cntr_incr,
            l1_tile_idx_or_tile_idx_inc,
            tile_idx_inc,
            row_mask_reg_sel,
            l1_16datums_row_index,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR1_STRIDE`."]
        Unpacr1Stride,
        defs::UNPACR1_STRIDE,
        [
            src_reg_y_cntr_incr,
            l1_tile_idx_or_tile_idx_inc,
            tile_idx_inc,
            row_mask_reg_sel,
            l1_16datums_row_index,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR2_STRIDE`."]
        Unpacr2Stride,
        defs::UNPACR2_STRIDE,
        [
            src_reg_y_cntr_incr,
            l1_tile_idx_or_tile_idx_inc,
            tile_idx_inc,
            row_mask_reg_sel,
            l1_16datums_row_index,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_DEST_STRIDE`."]
        UnpacrDestStride,
        defs::UNPACR_DEST_STRIDE,
        [
            src_reg_y_cntr_incr,
            l1_tile_idx_or_tile_idx_inc,
            tile_idx_inc,
            row_mask_reg_sel,
            l1_16datums_row_index,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_TILIZE`."]
        UnpacrTilize,
        defs::UNPACR_TILIZE,
        [
            reserved,
            cntr_reset_mask,
            dst_z_cntr_inc,
            src_z_cntr_inc,
            unpack_sel,
            buffer_descriptor_table_sel,
            set_dat_valid
        ]
    );

    builder!(
        #[doc = "`UNPACR_NOP`."]
        UnpacrNop,
        defs::UNPACR_NOP,
        [
            unpacker_select,
            set_dvalid,
            stall_cntrl,
            bank_clr_ctrl,
            src_clr_val_ctrl,
            nop_type
        ]
    );

    /// `SETGPR`.
    pub const fn setgpr(
        payload_sig_sel_size: u32,
        payload_sig_sel: u32,
        set_signals_mode: u32,
        gpr_index16b: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SETGPR,
            &[
                payload_sig_sel_size,
                payload_sig_sel,
                set_signals_mode,
                gpr_index16b,
            ],
        )
    }

    builder!(
        #[doc = "`LOADIND`."]
        Loadind,
        defs::LOADIND,
        [
            size_sel,
            offset_index,
            auto_inc_spec,
            data_gpr_index,
            addr_gpr_index
        ]
    );

    /// `SET_SRC_TILE_FACE_ROW_IDX`.
    pub const fn set_src_tile_face_row_idx(
        tile_face_row_sel: u32,
        engine_sel: u32,
        value: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SET_SRC_TILE_FACE_ROW_IDX,
            &[tile_face_row_sel, engine_sel, value],
        )
    }

    /// `INC_SRC_TILE_FACE_ROW_IDX`.
    pub const fn inc_src_tile_face_row_idx(
        tile_face_row_sel: u32,
        engine_sel: u32,
        value: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::INC_SRC_TILE_FACE_ROW_IDX,
            &[tile_face_row_sel, engine_sel, value],
        )
    }

    /// `SET_DST_TILE_FACE_ROW_IDX`.
    pub const fn set_dst_tile_face_row_idx(
        tile_face_row_sel: u32,
        engine_sel: u32,
        value: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SET_DST_TILE_FACE_ROW_IDX,
            &[tile_face_row_sel, engine_sel, value],
        )
    }

    /// `INC_DST_TILE_FACE_ROW_IDX`.
    pub const fn inc_dst_tile_face_row_idx(
        tile_face_row_sel: u32,
        engine_sel: u32,
        value: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::INC_DST_TILE_FACE_ROW_IDX,
            &[tile_face_row_sel, engine_sel, value],
        )
    }

    /// `ADDGPR`.
    pub const fn addgpr(
        op_b_is_const: u32,
        result_gpr_index: u32,
        op_b_gpr_index: u32,
        op_a_gpr_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::ADDGPR,
            &[
                op_b_is_const,
                result_gpr_index,
                op_b_gpr_index,
                op_a_gpr_index,
            ],
        )
    }

    /// `SUBGPR`.
    pub const fn subgpr(
        op_b_is_const: u32,
        result_gpr_index: u32,
        op_b_gpr_index: u32,
        op_a_gpr_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SUBGPR,
            &[
                op_b_is_const,
                result_gpr_index,
                op_b_gpr_index,
                op_a_gpr_index,
            ],
        )
    }

    /// `MULGPR`.
    pub const fn mulgpr(
        op_b_is_const: u32,
        result_gpr_index: u32,
        op_b_gpr_index: u32,
        op_a_gpr_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::MULGPR,
            &[
                op_b_is_const,
                result_gpr_index,
                op_b_gpr_index,
                op_a_gpr_index,
            ],
        )
    }

    builder!(
        #[doc = "`BITWOPGPR`."]
        Bitwopgpr,
        defs::BITWOPGPR,
        [
            op_b_is_const,
            op_sel,
            result_gpr_index,
            op_b_gpr_index,
            op_a_gpr_index
        ]
    );

    builder!(
        #[doc = "`SHIFTGPR`."]
        Shiftgpr,
        defs::SHIFTGPR,
        [
            op_b_is_const,
            op_sel,
            result_gpr_index,
            op_b_gpr_index,
            op_a_gpr_index
        ]
    );

    builder!(
        #[doc = "`CMPGPR`."]
        Cmpgpr,
        defs::CMPGPR,
        [
            op_b_is_const,
            op_sel,
            result_gpr_index,
            op_b_gpr_index,
            op_a_gpr_index
        ]
    );

    /// `DMANOP`.
    pub const fn dmanop() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::DMANOP, &[])
    }

    /// `ATINCGET`.
    pub const fn atincget(
        wrap_val: u32,
        sel32b: u32,
        data_gpr_index: u32,
        addr_gpr_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::ATINCGET,
            &[wrap_val, sel32b, data_gpr_index, addr_gpr_index],
        )
    }

    builder!(
        #[doc = "`ATINCGETPTR`."]
        Atincgetptr,
        defs::ATINCGETPTR,
        [
            no_incr,
            incr_val,
            wrap_val,
            sel32b,
            data_gpr_index,
            addr_gpr_index
        ]
    );

    /// `ATSWAP`.
    pub const fn atswap(
        swap_mask: u32,
        data_gpr_index: u32,
        addr_gpr_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATSWAP, &[swap_mask, data_gpr_index, addr_gpr_index])
    }

    /// `ATCAS`.
    pub const fn atcas(
        swap_val: u32,
        cmp_val: u32,
        sel32b: u32,
        addr_gpr_index: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::ATCAS, &[swap_val, cmp_val, sel32b, addr_gpr_index])
    }

    builder!(
        #[doc = "`STOREIND`."]
        Storeind,
        defs::STOREIND,
        [
            size_sel,
            mem_sel,
            offset_index,
            auto_inc_spec,
            data_gpr_index,
            addr_gpr_index
        ]
    );

    /// `STOREREG`.
    pub const fn storereg(data_gpr_index: u32, reg_addr: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::STOREREG, &[data_gpr_index, reg_addr])
    }

    /// `LOADREG`.
    pub const fn loadreg(data_gpr_index: u32, reg_addr: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::LOADREG, &[data_gpr_index, reg_addr])
    }

    builder!(
        #[doc = "`SFPLOAD`."]
        Sfpload,
        defs::SFPLOAD,
        [lreg_ind, instr_mod0, sfpu_addr_mode, done, dest_reg_addr]
    );

    /// `SFPLOADI`.
    pub const fn sfploadi(
        lreg_ind: u32,
        instr_mod0: u32,
        imm16: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLOADI, &[lreg_ind, instr_mod0, imm16])
    }

    builder!(
        #[doc = "`SFPSTORE`."]
        Sfpstore,
        defs::SFPSTORE,
        [lreg_ind, instr_mod0, sfpu_addr_mode, done, dest_reg_addr]
    );

    builder!(
        #[doc = "`SFPLOADMACRO`."]
        Sfploadmacro,
        defs::SFPLOADMACRO,
        [
            seq_id,
            lreg_ind_lo,
            instr_mod0,
            sfpu_addr_mode,
            done,
            dest_reg_addr,
            lreg_ind_hi
        ]
    );

    /// `SFPCOMPC`.
    pub const fn sfpcompc() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPCOMPC, &[])
    }

    /// `SFPENCC`.
    pub const fn sfpencc(imm12_math: u32, instr_mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPENCC, &[imm12_math, instr_mod1])
    }

    /// `SFPPOPC`.
    pub const fn sfppopc(instr_mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPPOPC, &[instr_mod1])
    }

    /// `SFPPUSHC`.
    pub const fn sfppushc(instr_mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPPUSHC, &[instr_mod1])
    }

    /// `SFPSETCC`.
    pub const fn sfpsetcc(
        imm12_math: u32,
        lreg_c: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSETCC, &[imm12_math, lreg_c, instr_mod1])
    }

    /// `SFPCAST`.
    pub const fn sfpcast(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPCAST, &[lreg_c, lreg_dest, instr_mod1])
    }

    builder!(
        #[doc = "`SFP_STOCH_RND`."]
        SfpStochRnd,
        defs::SFP_STOCH_RND,
        [rnd_mode, imm8_math, lreg_b, lreg_c, lreg_dest, instr_mod1]
    );

    /// `SFPABS`.
    pub const fn sfpabs(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPABS, &[lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPGT`.
    pub const fn sfpgt(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPGT, &[imm12_math, lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPIADD`.
    pub const fn sfpiadd(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPIADD, &[imm12_math, lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPLE`.
    pub const fn sfple(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLE, &[imm12_math, lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPNONLINEAR`.
    pub const fn sfpnonlinear(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPNONLINEAR, &[lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPSHFT`.
    pub const fn sfpshft(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSHFT, &[imm12_math, lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPSWAP`.
    pub const fn sfpswap(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPSWAP, &[imm12_math, lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPAND`.
    pub const fn sfpand(lreg_c: u32, lreg_dest: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPAND, &[lreg_c, lreg_dest])
    }

    /// `SFPDIVP2`.
    pub const fn sfpdivp2(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SFPDIVP2,
            &[imm12_math, lreg_c, lreg_dest, instr_mod1],
        )
    }

    /// `SFPEXEXP`.
    pub const fn sfpexexp(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPEXEXP, &[lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPEXMAN`.
    pub const fn sfpexman(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPEXMAN, &[lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPLZ`.
    pub const fn sfplz(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLZ, &[lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPNOT`.
    pub const fn sfpnot(lreg_c: u32, lreg_dest: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPNOT, &[lreg_c, lreg_dest])
    }

    /// `SFPOR`.
    pub const fn sfpor(lreg_c: u32, lreg_dest: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPOR, &[lreg_c, lreg_dest])
    }

    /// `SFPSETEXP`.
    pub const fn sfpsetexp(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SFPSETEXP,
            &[imm12_math, lreg_c, lreg_dest, instr_mod1],
        )
    }

    /// `SFPSETMAN`.
    pub const fn sfpsetman(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SFPSETMAN,
            &[imm12_math, lreg_c, lreg_dest, instr_mod1],
        )
    }

    /// `SFPSETSGN`.
    pub const fn sfpsetsgn(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SFPSETSGN,
            &[imm12_math, lreg_c, lreg_dest, instr_mod1],
        )
    }

    /// `SFPXOR`.
    pub const fn sfpxor(lreg_c: u32, lreg_dest: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPXOR, &[lreg_c, lreg_dest])
    }

    builder!(
        #[doc = "`SFPADD`."]
        Sfpadd,
        defs::SFPADD,
        [lreg_a, lreg_b, lreg_c, lreg_dest, instr_mod1]
    );

    /// `SFPADDI`.
    pub const fn sfpaddi(
        imm16_math: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPADDI, &[imm16_math, lreg_dest, instr_mod1])
    }

    /// `SFPLUT`.
    pub const fn sfplut(lreg_ind: u32, instr_mod0: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLUT, &[lreg_ind, instr_mod0])
    }

    /// `SFPLUTFP32`.
    pub const fn sfplutfp32(lreg_dest: u32, instr_mod1: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPLUTFP32, &[lreg_dest, instr_mod1])
    }

    builder!(
        #[doc = "`SFPMAD`."]
        Sfpmad,
        defs::SFPMAD,
        [lreg_a, lreg_b, lreg_c, lreg_dest, instr_mod1]
    );

    builder!(
        #[doc = "`SFPMUL`."]
        Sfpmul,
        defs::SFPMUL,
        [lreg_a, lreg_b, lreg_c, lreg_dest, instr_mod1]
    );

    /// `SFPMULI`.
    pub const fn sfpmuli(
        imm16_math: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPMULI, &[imm16_math, lreg_dest, instr_mod1])
    }

    /// `SFPMUL24`.
    pub const fn sfpmul24(
        lreg_a: u32,
        lreg_b: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPMUL24, &[lreg_a, lreg_b, lreg_dest, instr_mod1])
    }

    /// `SFPMOV`.
    pub const fn sfpmov(
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPMOV, &[lreg_c, lreg_dest, instr_mod1])
    }

    /// `SFPSHFT2`.
    pub const fn sfpshft2(
        imm12_math: u32,
        lreg_c: u32,
        lreg_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::SFPSHFT2,
            &[imm12_math, lreg_c, lreg_dest, instr_mod1],
        )
    }

    /// `SFPTRANSP`.
    pub const fn sfptransp() -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPTRANSP, &[])
    }

    /// `SFPCONFIG`.
    pub const fn sfpconfig(
        imm16_math: u32,
        config_dest: u32,
        instr_mod1: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPCONFIG, &[imm16_math, config_dest, instr_mod1])
    }

    /// `SFPNOP`.
    pub const fn sfpnop(
        srcs_wr_done: u32,
        srcs_rd_done: u32,
        dest_done: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::SFPNOP, &[srcs_wr_done, srcs_rd_done, dest_done])
    }

    /// `WRCFG`.
    pub const fn wrcfg(
        gpr_address: u32,
        wr128b: u32,
        cfg_reg: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::WRCFG, &[gpr_address, wr128b, cfg_reg])
    }

    /// `RDCFG`.
    pub const fn rdcfg(gpr_address: u32, cfg_reg: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RDCFG, &[gpr_address, cfg_reg])
    }

    /// `RMWCIB0`.
    pub const fn rmwcib0(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB0, &[cfg_reg_addr, mask, data])
    }

    /// `RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`.
    pub const fn rmwcib0_but_alias_bit_8_of_cfg_reg_addr_with_lsb_of_opcode(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::RMWCIB0_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
            &[cfg_reg_addr, mask, data],
        )
    }

    /// `RMWCIB1`.
    pub const fn rmwcib1(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB1, &[cfg_reg_addr, mask, data])
    }

    /// `RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`.
    pub const fn rmwcib1_but_alias_bit_8_of_cfg_reg_addr_with_lsb_of_opcode(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::RMWCIB1_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
            &[cfg_reg_addr, mask, data],
        )
    }

    /// `RMWCIB2`.
    pub const fn rmwcib2(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB2, &[cfg_reg_addr, mask, data])
    }

    /// `RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`.
    pub const fn rmwcib2_but_alias_bit_8_of_cfg_reg_addr_with_lsb_of_opcode(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::RMWCIB2_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
            &[cfg_reg_addr, mask, data],
        )
    }

    /// `RMWCIB3`.
    pub const fn rmwcib3(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RMWCIB3, &[cfg_reg_addr, mask, data])
    }

    /// `RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`.
    pub const fn rmwcib3_but_alias_bit_8_of_cfg_reg_addr_with_lsb_of_opcode(
        cfg_reg_addr: u32,
        mask: u32,
        data: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::RMWCIB3_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
            &[cfg_reg_addr, mask, data],
        )
    }

    builder!(
        #[doc = "`CFGSHIFTMASK`."]
        Cfgshiftmask,
        defs::CFGSHIFTMASK,
        [
            cfg_reg_addr,
            disable_mask_on_old_val,
            operation,
            mask_width,
            right_cshift_amt,
            scratch_sel
        ]
    );

    builder!(
        #[doc = "`CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE`."]
        CfgshiftmaskButAliasBit8OfCfgRegAddrWithLsbOfOpcode,
        defs::CFGSHIFTMASK_BUT_ALIAS_BIT_8_OF_CFG_REG_ADDR_WITH_LSB_OF_OPCODE,
        [
            cfg_reg_addr,
            disable_mask_on_old_val,
            operation,
            mask_width,
            right_cshift_amt,
            scratch_sel
        ]
    );

    /// `COMMIT_SHADOW`.
    pub const fn commit_shadow(force_commit: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::COMMIT_SHADOW, &[force_commit])
    }

    /// `MOP`.
    pub const fn mop(
        mop_type: u32,
        done: u32,
        loop_count: u32,
        zmask_lo8_or_loop_count: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(
            &defs::MOP,
            &[mop_type, done, loop_count, zmask_lo8_or_loop_count],
        )
    }

    /// `MOP_CFG`.
    pub const fn mop_cfg(zmask_hi24: u32) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::MOP_CFG, &[zmask_hi24])
    }

    builder!(
        #[doc = "`REPLAY`."]
        Replay,
        defs::REPLAY,
        [
            start_idx,
            len,
            last,
            set_mutex,
            execute_while_loading,
            load_mode
        ]
    );

    /// `RESOURCEDECL`.
    pub const fn resourcedecl(
        linger_time: u32,
        resources: u32,
        op_class: u32,
    ) -> Result<Instruction, EncodeError> {
        Instruction::with_operands(&defs::RESOURCEDECL, &[linger_time, resources, op_class])
    }
}
