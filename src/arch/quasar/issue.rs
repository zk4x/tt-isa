//! How a Quasar hart hands an instruction to the Tensix it drives, and how a
//! mover issues a NoC transaction.
//!
//! There are two mechanisms, and neither is a store to a per-thread queue:
//!
//! * A **TRISC issues a Tensix instruction in its own stream**. The word is the
//!   Tensix encoding rotated left by two, so its low two bits carry the top two
//!   bits of the encoding and the word can never be mistaken for a RISC-V
//!   instruction (every RISC-V word ends in `0b11`):
//!   `(enc >> 30) | (enc << 2)`. tt-metal writes it through
//!   `__builtin_rvtt_ttinsn` (`tt-llk/tt_llk_quasar/common/inc/ckernel_ops.h`)
//!   and, where the compiler cannot reach, as
//!   `.4byte ((%0 >> 30) & 3) | ((%0 & 0x3fffffff) << 2)`. The same rotation is
//!   what the `.ttinsn` assembler directive encodes, and `tensix_isa.h`
//!   documents it: "`.ttinsn` stores TT_OP rotated left by 2".
//!
//! * A **mover issues a NoC transaction through a RoCC command buffer**. The
//!   command-buffer registers are written with `tt.rocc.cmdbuf_wr_reg`, read with
//!   `tt.rocc.cmdbuf_rd_reg`, and fired with `tt.rocc.cmdbuf_issue_trans`
//!   (`internal/tt-2xx/quasar/noc_cmd_buf_common.h`,
//!   `noc_nonblocking_api_v2.h`). The atomic/inline buffer is the *simple*
//!   command buffer, a separate instruction group (`scmdbuf_*`) that takes no
//!   buffer index.
//!
//! Every word here was measured, not inferred: they are what the SFPI compiler
//! in the tt-metal checkout (`runtime/sfpi/compiler/bin/riscv-tt-elf-g++`,
//! `tenstorrent/sfpi:7.52.0`, GCC 15.1.0) emits for those builtins at
//! `-mcpu=tt-qsr64-rocc`, read back with `riscv-tt-elf-objdump -d`. The
//! instruction-family layout they spell out is:
//!
//! ```text
//! RoCC word (opcode 0x0b for a command buffer, 0x2b for the simple one):
//!   funct7 = buf << 6 | function,   funct3 = operation,   rs1 = operand
//! ```
//!
//! where a command-buffer *register* access puts the register index in the low
//! six bits of `funct7` and discriminates write (2) from read (4) in `funct3`,
//! while the fixed operations use function ids 0x3b (reset) and 0x3f (issue),
//! and the simple buffer's status reads use 0x3c (`tr_ack`) and 0x3d
//! (`wr_sent`).

/// Rotate a Tensix instruction encoding into the word a hart executes to issue
/// it: `(enc >> 30) | (enc << 2)`.
pub const fn issue(enc: u32) -> u32 {
    (enc >> 30) | (enc << 2)
}

/// The inverse of [`issue`], for reading back a word a compiler emitted.
pub const fn issued(enc: u32) -> u32 {
    (enc << 30) | (enc >> 2)
}

/// Opcode of a command-buffer RoCC instruction (`custom-0`).
pub const OPCODE_CMD_BUF: u32 = 0x0b;
/// Opcode of a simple-command-buffer RoCC instruction (`custom-1`).
pub const OPCODE_SIMPLE_CMD_BUF: u32 = 0x2b;

/// `funct3` of `cmdbuf_wr_reg` / `scmdbuf_wr_reg`.
pub const FN_WRITE_REG: u32 = 2;
/// `funct3` of `cmdbuf_rd_reg` / `scmdbuf_rd_reg`.
pub const FN_READ_REG: u32 = 4;
/// `funct3` of the transaction-id status reads (`*_trid`), which take their
/// transaction id in a GPR rather than as a fixed function id.
pub const FN_STATUS_GPR: u32 = 6;

/// `funct7` of `cmdbuf_reset` / `scmdbuf_reset`.
pub const FN_RESET: u32 = 0x3b;
/// `funct7` of `cmdbuf_issue_trans` / `scmdbuf_issue_trans`.
pub const FN_ISSUE: u32 = 0x3f;
/// `funct7` of `scmdbuf_tr_ack`: transactions not yet acknowledged.
pub const FN_TR_ACK: u32 = 0x3c;
/// `funct7` of `scmdbuf_wr_sent`: writes not yet read out of L1.
pub const FN_WR_SENT: u32 = 0x3d;

/// One RoCC word.
const fn rocc(funct7: u32, funct3: u32, rd: u8, rs1: u8, opcode: u32) -> u32 {
    (funct7 << 25) | ((rs1 as u32) << 15) | (funct3 << 12) | ((rd as u32) << 7) | opcode
}

/// `tt.rocc.cmdbuf_wr_reg`: write `rs1` into command-buffer register `reg` of
/// buffer `buf`.
pub const fn cmdbuf_wr_reg(buf: u8, reg: u8, rs1: u8) -> u32 {
    rocc(
        (reg as u32 & 0x3f) | ((buf as u32 & 1) << 6),
        FN_WRITE_REG,
        0,
        rs1,
        OPCODE_CMD_BUF,
    )
}

/// `tt.rocc.cmdbuf_rd_reg`: read command-buffer register `reg` of buffer `buf`
/// into `rd`.
pub const fn cmdbuf_rd_reg(buf: u8, reg: u8, rd: u8) -> u32 {
    rocc(
        (reg as u32 & 0x3f) | ((buf as u32 & 1) << 6),
        FN_READ_REG,
        rd,
        0,
        OPCODE_CMD_BUF,
    )
}

/// `tt.rocc.cmdbuf_issue_trans`: fire the transaction staged in buffer `buf`.
pub const fn cmdbuf_issue(buf: u8) -> u32 {
    rocc(
        (FN_ISSUE) | ((buf as u32 & 1) << 6),
        0,
        0,
        0,
        OPCODE_CMD_BUF,
    )
}

/// `tt.rocc.cmdbuf_reset`: discard the transaction staged in buffer `buf`.
pub const fn cmdbuf_reset(buf: u8) -> u32 {
    rocc(
        (FN_RESET) | ((buf as u32 & 1) << 6),
        0,
        0,
        0,
        OPCODE_CMD_BUF,
    )
}

/// One simple-command-buffer word: bit 6 of `funct7` is set, as the compiler
/// emits it for every `scmdbuf_*` instruction.
const fn simple(funct7: u32, funct3: u32, rd: u8, rs1: u8) -> u32 {
    rocc(0x40 | funct7, funct3, rd, rs1, OPCODE_SIMPLE_CMD_BUF)
}

/// `tt.rocc.scmdbuf_wr_reg`: write `rs1` into simple-buffer register `reg`.
pub const fn scmdbuf_wr_reg(reg: u8, rs1: u8) -> u32 {
    simple(reg as u32 & 0x3f, FN_WRITE_REG, 0, rs1)
}

/// `tt.rocc.scmdbuf_rd_reg`: read simple-buffer register `reg` into `rd`.
pub const fn scmdbuf_rd_reg(reg: u8, rd: u8) -> u32 {
    simple(reg as u32 & 0x3f, FN_READ_REG, rd, 0)
}

/// `tt.rocc.scmdbuf_issue_trans`: fire the simple buffer's transaction.
pub const fn scmdbuf_issue() -> u32 {
    simple(FN_ISSUE, 0, 0, 0)
}

/// `tt.rocc.scmdbuf_reset`.
pub const fn scmdbuf_reset() -> u32 {
    simple(FN_RESET, 0, 0, 0)
}

/// `tt.rocc.scmdbuf_tr_ack`: the number of transactions issued on this tile and
/// not yet acknowledged. Zero means every transfer has landed
/// (`ncrisc_noc_reads_flushed`, `noc_cmd_buf_common.h`).
pub const fn scmdbuf_tr_ack(rd: u8) -> u32 {
    simple(FN_TR_ACK, FN_READ_REG, rd, 0)
}

/// `tt.rocc.scmdbuf_wr_sent`: the number of writes not yet read out of L1.
pub const fn scmdbuf_wr_sent(rd: u8) -> u32 {
    simple(FN_WR_SENT, FN_READ_REG, rd, 0)
}

/// `tt.rocc.scmdbuf_tr_ack_trid`: the same count for one transaction id, which
/// `rs1` holds (`funct3` 6 instead of 4).
pub const fn scmdbuf_tr_ack_trid(trid_rs1: u8, rd: u8) -> u32 {
    simple(FN_TR_ACK, FN_STATUS_GPR, rd, trid_rs1)
}

/// `tt.rocc.scmdbuf_wr_sent_trid`.
pub const fn scmdbuf_wr_sent_trid(trid_rs1: u8, rd: u8) -> u32 {
    simple(FN_WR_SENT, FN_STATUS_GPR, rd, trid_rs1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every expectation below is a word the SFPI compiler emitted, or a
    /// `.ttinsn` directive the assembler encoded, read back with
    /// `riscv-tt-elf-objdump -d`.
    #[test]
    fn issue_rotates_the_encoding_left_by_two() {
        // `.ttinsn 0x08000000` -> `20000000` (TTI_NOP).
        assert_eq!(issue(0x0800_0000), 0x2000_0000);
        // `.ttinsn 0x0C000005` -> `30000014` (UNPACR_NOP on unpacker A).
        assert_eq!(issue(0x0C00_0005), 0x3000_0014);
        // `.ttinsn 0x74000000` -> `d0000001` (PACR_STRIDE writes nothing).
        assert_eq!(issue(0x7400_0000), 0xD000_0001);
        // `.ttinsn 0x00000002` -> `00000008` (TT_OP_NOP).
        assert_eq!(issue(0x0000_0002), 0x0000_0008);
        // A word whose encoding has its top bits set keeps them in bits 1..0.
        assert_eq!(issue(0xF000_0000), 0xC000_0003);
        assert_eq!(issue(0x8000_0000), 0x0000_0002);
        for enc in [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, 0x8000_0000] {
            assert_eq!(issued(issue(enc)), enc);
        }
        // The encodings a hart issues have top bits below `0b11`, so the word
        // never ends in `0b11` and is never read as a RISC-V instruction.
        assert_ne!(issue(0x0800_0000) & 3, 3);
    }

    #[test]
    fn command_buffer_register_writes_match_the_compiler() {
        // tt.rocc.cmdbuf_wr_reg zero,0,0,a1,zero
        assert_eq!(cmdbuf_wr_reg(0, 0, 11), 0x0005_a00b);
        // tt.rocc.cmdbuf_wr_reg zero,1,4,a1,zero
        assert_eq!(cmdbuf_wr_reg(1, 4, 11), 0x8805_a00b);
        // tt.rocc.cmdbuf_wr_reg zero,0,4,a0,zero
        assert_eq!(cmdbuf_wr_reg(0, 4, 10), 0x0805_200b);
    }

    #[test]
    fn command_buffer_register_reads_match_the_compiler() {
        // tt.rocc.cmdbuf_rd_reg a1,0,0,zero,zero
        assert_eq!(cmdbuf_rd_reg(0, 0, 11), 0x0000_458b);
        // tt.rocc.cmdbuf_rd_reg a0,0,4,zero,zero
        assert_eq!(cmdbuf_rd_reg(0, 4, 10), 0x0800_450b);
        // tt.rocc.cmdbuf_rd_reg a1,1,4,zero,zero
        assert_eq!(cmdbuf_rd_reg(1, 4, 11), 0x8800_458b);
    }

    #[test]
    fn issue_and_reset_match_the_compiler() {
        // tt.rocc.cmdbuf_issue_trans 0 / 1
        assert_eq!(cmdbuf_issue(0), 0x7e00_000b);
        assert_eq!(cmdbuf_issue(1), 0xfe00_000b);
        // tt.rocc.cmdbuf_reset 0
        assert_eq!(cmdbuf_reset(0), 0x7600_000b);
    }

    #[test]
    fn simple_command_buffer_words_match_the_compiler() {
        // tt.rocc.scmdbuf_issue_trans
        assert_eq!(scmdbuf_issue(), 0xfe00_002b);
        // tt.rocc.scmdbuf_tr_ack a0
        assert_eq!(scmdbuf_tr_ack(10), 0xf800_452b);
        // tt.rocc.scmdbuf_wr_sent a0
        assert_eq!(scmdbuf_wr_sent(10), 0xfa00_452b);
        // tt.rocc.scmdbuf_tr_ack_trid a0,a0 (trid in a0, result in a0)
        assert_eq!(scmdbuf_tr_ack_trid(10, 10), 0xf805_652b);
        // tt.rocc.scmdbuf_wr_sent_trid a0,a0
        assert_eq!(scmdbuf_wr_sent_trid(10, 10), 0xfa05_652b);
        // tt.rocc.scmdbuf_wr_reg zero,4,a5,zero
        assert_eq!(scmdbuf_wr_reg(4, 15), 0x8807_a02b);
        // tt.rocc.scmdbuf_rd_reg a0,4,zero,zero
        assert_eq!(scmdbuf_rd_reg(4, 10), 0x8800_452b);
    }

    /// The simple command buffer's fixed-operation ids, read off the same words.
    #[test]
    fn simple_command_buffer_functions() {
        assert_eq!(FN_TR_ACK, 0x3c);
        assert_eq!(FN_WR_SENT, 0x3d);
        assert_eq!(FN_ISSUE, 0x3f);
        assert_eq!(FN_RESET, 0x3b);
    }
}
