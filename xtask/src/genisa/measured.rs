// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Blackhole layouts measured on ttsim or silicon, where the specification draws
//! only Wormhole's.
//!
//! Hand-writing these encoders would put a second source of truth beside the
//! generated table, so each measurement enters *through* the generator, as data with
//! its evidence attached, and is held to three rules:
//!
//! * **It supersedes a `WormholeOnly` diagram and nothing else.** A measurement
//!   never overrides documentation; the day upstream documents the instruction for
//!   Blackhole is a build failure that says to reconcile the two.
//! * **Only the fields it lists may differ.** Every other field is carried from the
//!   Wormhole diagram bit for bit, and stays exactly as unverified as it was.
//! * **It names the gate that measured it.** The evidence is emitted into the
//!   table's doc comment.
//!
//! The superseded Wormhole diagram stays under `defs::wormhole`, so it can still be
//! reached deliberately.

use std::collections::BTreeMap;

use super::lua::{self, Diagram, Label};
use super::provenance::Provenance;

/// The measured layouts, in `Bits32.lua`'s dialect: same grammar, same structural
/// checks, not part of the specification and not covered by its digest.
pub const SOURCE: &str = include_str!("Bits32_BH.lua");

pub struct Measured {
    /// Key in [`SOURCE`], always `<supersedes>_BH`.
    pub key: &'static str,
    /// The `WormholeOnly` diagram this replaces on Blackhole.
    pub supersedes: &'static str,
    /// Fields whose position was measured to differ.
    pub moved: &'static [&'static str],
    /// Exact documented fixed fields the measurement replaces.
    pub replaced_fixed: &'static [((u8, u8, u32), (u8, u8, u32))],
    /// Fixed bits measured in positions the Wormhole diagram leaves undrawn.
    pub added_fixed: &'static [(u8, u8, u32)],
    /// Wormhole fields the Blackhole layout does not carry, each with the reason: a
    /// field whose bits hold something else and whose own position is unknown is
    /// dropped rather than guessed.
    pub dropped: &'static [(&'static str, &'static str)],
    /// Fields whose width differs from the Wormhole diagram's.
    pub widened: &'static [&'static str],
    /// `(file, test function)` pairs that measured the moved fields.
    pub evidence: &'static [(&'static str, &'static str)],
}

pub const MEASURED: &[Measured] = &[
    Measured {
        key: "UNPACR_NOP_SETDVALID_BH",
        supersedes: "UNPACR_NOP_SETDVALID",
        moved: &[],
        // BH uses UNP_CLR_SRC, clear-to-one selection and repurposed format 3 to
        // preserve data. The WH fixed mode 7 is a stream-pop on BH.
        replaced_fixed: &[((0, 3, 7), (0, 9, 0x1e9))],
        added_fixed: &[],
        dropped: &[],
        widened: &[],
        evidence: &[
            (
                "crates/tt-tests/tests/step104_unpacker_handover.rs",
                "llk_nonclearing_dvalid_a",
            ),
            (
                "crates/tt-tests/tests/step104_unpacker_handover.rs",
                "llk_nonclearing_dvalid_b",
            ),
        ],
    },
    Measured {
        key: "UNPACR_NOP_ZEROSRC_BH",
        supersedes: "UNPACR_NOP_ZEROSRC",
        moved: &["BothBanks", "WaitLikeUnpacr"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        // The inherited name is historical: BH uses a two-bit clear-value code, not
        // a negative-infinity flag. Checked APIs expose only zero.
        widened: &["NegativeInfSrcA"],
        evidence: &[
            (
                "crates/tt-tests/tests/step103_source_banks.rs",
                "unpacr_zero_blackhole_bank_and_clear_value_fields",
            ),
            (
                "crates/tt-tests/tests/step103_source_banks.rs",
                "unpacr_zero_waits_on_current_unpacker_bank_with_matrix_bank_held",
            ),
        ],
    },
    Measured {
        key: "GMPOOL_BH",
        supersedes: "GMPOOL",
        moved: &[],
        replaced_fixed: &[],
        added_fixed: &[(19, 1, 1)],
        dropped: &[],
        widened: &[],
        evidence: &[(
            "crates/tt-tests/tests/step75_fpu_pooling.rs",
            "matrix_pooling_uses_explicit_weights_and_releases_banks",
        )],
    },
    Measured {
        key: "GAPOOL_BH",
        supersedes: "GAPOOL",
        moved: &[],
        replaced_fixed: &[],
        added_fixed: &[(19, 1, 1)],
        dropped: &[],
        widened: &[],
        evidence: &[(
            "crates/tt-tests/tests/step75_fpu_pooling.rs",
            "matrix_pooling_uses_explicit_weights_and_releases_banks",
        )],
    },
    Measured {
        key: "MVMUL_BH",
        supersedes: "MVMUL",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "mvmul_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    Measured {
        key: "MOVA2D_BH",
        supersedes: "MOVA2D",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/probe_src.rs",
            "mov_to_dst_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    // `BroadcastCol0` and `Broadcast1RowTo8` move with `Move4Rows`: the three are
    // LLK's three-bit `instr_mod` at bit 11. Only `Move4Rows` is measured; the
    // broadcasts rest on LLK until a gate exercises them.
    Measured {
        key: "MOVB2D_BH",
        supersedes: "MOVB2D",
        moved: &["BroadcastCol0", "Broadcast1RowTo8", "Move4Rows", "AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[
            (
                "crates/tt-tests/tests/probe_src.rs",
                "movb2d_move4_rows_is_bit_13_on_blackhole",
            ),
            (
                "crates/tt-tests/tests/probe_src.rs",
                "mov_to_dst_addr_mod_sits_one_bit_lower_on_blackhole",
            ),
        ],
    },
    Measured {
        key: "MOVD2A_BH",
        supersedes: "MOVD2A",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    Measured {
        key: "MOVD2B_BH",
        supersedes: "MOVD2B",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    Measured {
        key: "MOVB2A_BH",
        supersedes: "MOVB2A",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "mov_to_src_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    Measured {
        key: "ELWADD_BH",
        supersedes: "ELWADD",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[
            (
                "crates/tt-tests/tests/step9_matmul.rs",
                "matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
            ),
            (
                "crates/tt-tests/tests/step9_matmul.rs",
                "elw_broadcast_assignment_and_destination_fields",
            ),
            (
                "crates/tt-tests/tests/step90_matrix_eltwise.rs",
                "resident_matrix_arithmetic_and_rhs_broadcasts",
            ),
        ],
    },
    Measured {
        key: "ELWSUB_BH",
        supersedes: "ELWSUB",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[
            (
                "crates/tt-tests/tests/step9_matmul.rs",
                "matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
            ),
            (
                "crates/tt-tests/tests/step9_matmul.rs",
                "elw_broadcast_assignment_and_destination_fields",
            ),
            (
                "crates/tt-tests/tests/step90_matrix_eltwise.rs",
                "resident_matrix_arithmetic_and_rhs_broadcasts",
            ),
        ],
    },
    Measured {
        key: "ELWMUL_BH",
        supersedes: "ELWMUL",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[
            (
                "crates/tt-tests/tests/step9_matmul.rs",
                "matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
            ),
            (
                "crates/tt-tests/tests/step9_matmul.rs",
                "elw_broadcast_assignment_and_destination_fields",
            ),
            (
                "crates/tt-tests/tests/step90_matrix_eltwise.rs",
                "resident_matrix_arithmetic_and_rhs_broadcasts",
            ),
        ],
    },
    Measured {
        key: "DOTPV_BH",
        supersedes: "DOTPV",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    Measured {
        key: "MOVDBGA2D_BH",
        supersedes: "MOVDBGA2D",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "matrix_unit_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    Measured {
        key: "SHIFTXB_BH",
        supersedes: "SHIFTXB",
        moved: &["AddrMod"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "shiftxb_addr_mod_sits_one_bit_lower_on_blackhole",
        )],
    },
    // LLK's Blackhole `ZEROACC` is `clear_mode << 19`, `use_32_bit_mode << 18`,
    // `clear_zero_flags << 17`, `addr_mode << 14`. `clear_zero_flags` is not drawn:
    // nothing has measured it, so the encoder cannot set it.
    Measured {
        key: "ZEROACC_BH",
        supersedes: "ZEROACC",
        moved: &["AddrMod", "UseDst32b"],
        replaced_fixed: &[],
        added_fixed: &[],
        dropped: &[(
            "Revert",
            "its Wormhole bit 18 is `UseDst32b` on Blackhole (measured, and LLK's \
             `use_32_bit_mode << 18`); LLK has no such field, and Wormhole's `ZEROACC.md` \
             makes any use of it `UndefinedBehavior`",
        )],
        widened: &["AddrMod"],
        evidence: &[(
            "crates/tt-tests/tests/step9_matmul.rs",
            "zeroacc_addr_mod_and_use_dst32b_on_blackhole",
        )],
    },
];

/// A `WormholeOnly` layout a gate ran on Blackhole and found unchanged.
pub struct Confirmed {
    /// The diagram key, as the specification draws it.
    pub key: &'static str,
    /// `(file, test function)` pairs: ttsim and silicon, both.
    pub evidence: &'static [(&'static str, &'static str)],
}

pub const CONFIRMED: &[Confirmed] = &[
    // Immediate multiplication and the other new scalar families refuse ttsim; they
    // retain WormholeOnly provenance despite silicon coverage.
    Confirmed {
        key: "MULDMAREG",
        evidence: &[(
            "crates/tt-tests/tests/step100_scalar_config.rs",
            "scalar_register_and_immediate_semantics",
        )],
    },
    // The register form; the six-bit immediate form (`ADDDMAREGi`) runs on silicon
    // but not on ttsim, so it is not confirmed here.
    Confirmed {
        key: "ADDDMAREG",
        evidence: &[(
            "crates/tt-tests/tests/step38_gpr_add.rs",
            "adddmareg_adds_two_registers",
        )],
    },
    Confirmed {
        key: "MOP",
        evidence: &[(
            "crates/tt-tests/tests/step36_mop.rs",
            "a_mop_expands_as_the_page_models_it",
        )],
    },
    Confirmed {
        key: "MOP_CFG",
        evidence: &[(
            "crates/tt-tests/tests/step36_mop.rs",
            "a_mop_expands_as_the_page_models_it",
        )],
    },
];

/// Fold the measured layouts in: each joins `diagrams`, the diagram it supersedes
/// becomes `SupersededOnBlackhole`, and the measured key takes its page and mnemonic.
pub fn apply(
    source: &str,
    table: &[Measured],
    diagrams: &mut Vec<Diagram>,
    provenance: &mut BTreeMap<String, (Provenance, String)>,
    mnemonics: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    let measured = lua::dedupe(lua::parse(source)?)?;
    for d in &measured {
        if !table.iter().any(|m| m.key == d.key) {
            return Err(format!(
                "measured layout `{}` has no row in `MEASURED`: every measured layout must say \
                 what it supersedes and which gate measured it",
                d.key
            ));
        }
    }

    for m in table {
        let Some(d) = measured.iter().find(|d| d.key == m.key) else {
            return Err(format!(
                "`MEASURED` lists `{}`, which `Bits32_BH.lua` does not define",
                m.key
            ));
        };
        if m.key != format!("{}_BH", m.supersedes) {
            return Err(format!(
                "`{}` must be named `{}_BH`, after the diagram it supersedes",
                m.key, m.supersedes
            ));
        }
        if diagrams.iter().any(|s| s.key == m.key) {
            return Err(format!(
                "`{}` is defined by the specification itself; compare the document with the \
                 measurement and remove the override",
                m.key
            ));
        }
        let Some(wormhole) = diagrams.iter().find(|s| s.key == m.supersedes) else {
            return Err(format!(
                "`{}` supersedes `{}`, which the specification does not define",
                m.key, m.supersedes
            ));
        };
        match provenance.get(m.supersedes) {
            Some((Provenance::WormholeOnly, _)) => {}
            Some((other, page)) => {
                return Err(format!(
                    "`{}` is no longer Wormhole-only (it is `{}`, per `{page}`): the \
                     specification now says something about it for Blackhole, so reconcile that \
                     with the measurement and remove the override",
                    m.supersedes,
                    other.variant_name()
                ));
            }
            None => return Err(format!("`{}` has no provenance", m.supersedes)),
        }
        compare(d, wormhole, m)?;
        if m.evidence.is_empty() {
            return Err(format!("`{}` names no gate that measured it", m.key));
        }
    }

    for m in table {
        let d = measured
            .iter()
            .find(|d| d.key == m.key)
            .expect("checked above")
            .clone();
        let page = provenance[m.supersedes].1.clone();
        provenance.insert(
            m.supersedes.to_string(),
            (
                Provenance::SupersededOnBlackhole {
                    by: m.key.to_string(),
                },
                page.clone(),
            ),
        );
        provenance.insert(
            m.key.to_string(),
            (
                Provenance::Measured {
                    evidence: evidence(m.evidence),
                    moved: m.moved.iter().map(|s| (*s).to_string()).collect(),
                    dropped: m
                        .dropped
                        .iter()
                        .map(|(name, _)| (*name).to_string())
                        .collect(),
                    widened: m.widened.iter().map(|s| (*s).to_string()).collect(),
                },
                page,
            ),
        );
        if let Some(mnemonic) = mnemonics.get(m.supersedes).cloned() {
            mnemonics.insert(m.key.to_string(), mnemonic);
        }
        diagrams.push(d);
    }
    Ok(())
}

/// Mark each [`CONFIRMED`] layout so. Refused unless it is `WormholeOnly`: a
/// confirmation only stands in for missing documentation.
pub fn confirm(
    table: &[Confirmed],
    provenance: &mut BTreeMap<String, (Provenance, String)>,
) -> Result<(), String> {
    for c in table {
        let Some((p, page)) = provenance.get(c.key).cloned() else {
            return Err(format!("confirmed layout `{}` is not in the table", c.key));
        };
        if p != Provenance::WormholeOnly {
            return Err(format!(
                "`{}` is {}, not WormholeOnly: a confirmation only stands in for missing \
                 documentation; drop its row from `CONFIRMED`",
                c.key,
                p.variant_name()
            ));
        }
        if c.evidence.is_empty() {
            return Err(format!("confirmed layout `{}` names no gate", c.key));
        }
        provenance.insert(
            c.key.to_string(),
            (
                Provenance::Confirmed {
                    evidence: evidence(c.evidence),
                },
                page,
            ),
        );
    }
    Ok(())
}

pub fn evidence(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(file, name)| format!("{file}::{name}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The measured layout must be the Wormhole one with exactly `m.moved` relocated:
/// same width, same opcode, same fixed fields modulo the listed replacements, and no
/// relocation or width change that is not listed.
fn compare(measured: &Diagram, wormhole: &Diagram, m: &Measured) -> Result<(), String> {
    if measured.nbits != wormhole.nbits {
        return Err(format!(
            "`{}` and `{}` differ in width",
            m.key, m.supersedes
        ));
    }
    if measured.opcode().map(|f| &f.label) != wormhole.opcode().map(|f| &f.label) {
        return Err(format!(
            "`{}` has a different opcode from `{}`; a measured layout relocates fields of one \
             instruction, it does not define a new one",
            m.key, m.supersedes
        ));
    }

    let fixed = |d: &Diagram| -> Vec<(u8, u8, u32)> {
        d.fields
            .iter()
            .filter_map(|f| match &f.label {
                Label::Fixed { value } => Some((f.first_bit, f.width, *value)),
                _ => None,
            })
            .collect()
    };
    let mut expected = fixed(wormhole);
    for &(before, after) in m.replaced_fixed {
        let Some(index) = expected.iter().position(|f| *f == before) else {
            return Err(format!("`{}` replaces an absent fixed field", m.key));
        };
        let (first, width, value) = after;
        if width == 0 || width >= 32 || first as u32 + width as u32 > 32 || value >= (1u32 << width)
        {
            return Err(format!(
                "`{}` has an invalid replacement fixed field",
                m.key
            ));
        }
        let old_mask = ((1u32 << before.1) - 1) << before.0;
        let new_mask = ((1u32 << width) - 1) << first;
        // Only the replaced bits or undrawn bits: named fields and the opcode stay.
        if before.0 >= 24 || new_mask & !(wormhole.undrawn() | old_mask) != 0 {
            return Err(format!(
                "`{}` replaces a fixed field over protected bits",
                m.key
            ));
        }
        expected[index] = after;
    }
    for &(first, width, value) in m.added_fixed {
        if width == 0 || width >= 32 || first as u32 + width as u32 > 32 || value >= (1u32 << width)
        {
            return Err(format!("`{}` has an invalid added fixed field", m.key));
        }
        let mask = ((1u32 << width) - 1) << first;
        if wormhole.undrawn() & mask != mask {
            return Err(format!(
                "`{}` adds a fixed field over documented bits",
                m.key
            ));
        }
        expected.push((first, width, value));
    }
    expected.sort_unstable();
    let mut actual = fixed(measured);
    actual.sort_unstable();
    if actual != expected {
        return Err(format!(
            "`{}` changes a fixed field of `{}` without an exact measurement credential",
            m.key, m.supersedes
        ));
    }

    let positions = |d: &Diagram| -> BTreeMap<String, (u8, u8)> {
        d.fields
            .iter()
            .filter_map(|f| match &f.label {
                Label::Named { name, .. } => Some((name.clone(), (f.first_bit, f.width))),
                _ => None,
            })
            .collect()
    };
    let (measured_fields, mut wormhole_fields) = (positions(measured), positions(wormhole));
    for (name, why) in m.dropped {
        if why.trim().is_empty() {
            return Err(format!("`{}` drops `{name}` without saying why", m.key));
        }
        if measured_fields.contains_key(*name) {
            return Err(format!(
                "`{}` lists `{name}` as dropped, but still draws it",
                m.key
            ));
        }
        if wormhole_fields.remove(*name).is_none() {
            return Err(format!(
                "`{}` drops `{name}`, which `{}` does not have",
                m.key, m.supersedes
            ));
        }
    }
    if measured_fields.keys().ne(wormhole_fields.keys()) {
        return Err(format!(
            "`{}` and `{}` name different fields: {:?} against {:?}",
            m.key,
            m.supersedes,
            measured_fields.keys().collect::<Vec<_>>(),
            wormhole_fields.keys().collect::<Vec<_>>()
        ));
    }
    for name in m.moved.iter().chain(m.widened) {
        if !measured_fields.contains_key(*name) {
            return Err(format!("`{}` lists `{name}`, but has no such field", m.key));
        }
    }
    for (name, (bit, width)) in &measured_fields {
        let (wormhole_bit, wormhole_width) = wormhole_fields[name];
        let listed = |list: &[&str]| list.contains(&name.as_str());
        match (listed(m.widened), *width == wormhole_width) {
            (false, false) => {
                return Err(format!(
                    "`{}` changes the width of `{name}` without listing it as widened",
                    m.key
                ));
            }
            (true, true) => {
                return Err(format!(
                    "`{}` lists `{name}` as widened, but it is as wide as in `{}`",
                    m.key, m.supersedes
                ));
            }
            _ => {}
        }
        match (listed(m.moved), *bit == wormhole_bit) {
            (true, true) => {
                return Err(format!(
                    "`{}` lists `{name}` as moved, but it sits where `{}` has it. A stale listing \
                     claims a measurement that no longer applies.",
                    m.key, m.supersedes
                ));
            }
            (false, false) => {
                return Err(format!(
                    "`{}` moves `{name}` from bit {wormhole_bit} to bit {bit} without listing it \
                     as moved; every relocation must be one the evidence measured",
                    m.key
                ));
            }
            _ => {}
        }
    }
    Ok(())
}
