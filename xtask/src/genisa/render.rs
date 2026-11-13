// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Emit `src/isa/generated.rs`.
//!
//! Two layers. The **table** is what makes table-wide invariants assertable — no
//! two mode variants ambiguous, every field inside its word — and what any future
//! tooling would read. The per-instruction **encoders** are the call-site API, and
//! they must be `const fn` because the firmware encodes instructions at run time on
//! the device, where there is no allocator and no lookup by name.
//!
//! Every encoder is one call to [`Instruction::with_operands`], which holds the one
//! copy of the field-fits check. Inlining that check per operand is what made an
//! earlier version of this file fourteen thousand lines of the same seven lines.

use std::collections::BTreeMap;
use std::fmt::Write;

use super::lua::{Diagram, Field as DrawnField, Label};
use super::provenance::Provenance;

/// Above this many operands a positional call is a bug farm and the encoder gets a
/// builder instead. `UNPACR_Regular` has thirteen.
const POSITIONAL_LIMIT: usize = 4;

/// One emitted encoding: usually one per diagram, `RMWCIB` is four.
struct Entry<'a> {
    diagram: &'a Diagram,
    /// The `Bits32.lua` key, or the variant name where one diagram is several.
    key: String,
    /// The `TT_*` macro name for this encoding specifically.
    mnemonic: String,
    opcode: u8,
}

pub struct Input<'a> {
    pub diagrams: &'a [Diagram],
    pub provenance: &'a BTreeMap<String, (Provenance, String)>,
    pub mnemonics: &'a BTreeMap<String, String>,
    pub variants: &'a BTreeMap<String, Vec<String>>,
    pub spec_rev: &'a str,
}

pub fn render(input: &Input<'_>) -> Result<String, String> {
    let mut instructions: Vec<Entry<'_>> = Vec::new();
    let mut layouts: Vec<&Diagram> = Vec::new();
    for d in input.diagrams {
        let Some(opcode) = d.opcode() else {
            layouts.push(d);
            continue;
        };
        match &opcode.label {
            Label::Fixed { value } => instructions.push(Entry {
                diagram: d,
                key: d.key.clone(),
                mnemonic: input
                    .mnemonics
                    .get(&d.key)
                    .cloned()
                    .unwrap_or_else(|| d.key.clone()),
                opcode: *value as u8,
            }),
            // `0xB3 + Index1` is an *addition*, not a bitfield: the opcodes are
            // 0xB3..0xB6, and 0xB0..0xB2 belong to other instructions. The
            // specification documents it as `TT_RMWCIB0`..`TT_RMWCIB3`, so it is
            // emitted as four encodings.
            Label::Computed { base, addend } => {
                let names = input.variants.get(&d.key).ok_or_else(|| {
                    format!(
                        "`{}` has the computed opcode `{base:#x} + {addend}` but its page \
                         documents no macro variants",
                        d.key
                    )
                })?;
                for (i, name) in names.iter().enumerate() {
                    let opcode = u8::try_from(*base as usize + i).map_err(|_| {
                        format!("`{}` variant {i} overflows the opcode byte", d.key)
                    })?;
                    instructions.push(Entry {
                        diagram: d,
                        key: name.clone(),
                        mnemonic: name.clone(),
                        opcode,
                    });
                }
            }
            Label::Named { .. } => return Err(format!("`{}`'s opcode field is an operand", d.key)),
        }
    }

    // Blackhole's form of an instruction takes the plain name; the Wormhole form it
    // supersedes is reachable, but only through `wormhole::`.
    let mut names: BTreeMap<&str, (String, bool)> = BTreeMap::new();
    let mut taken: BTreeMap<String, &str> = BTreeMap::new();
    for e in &instructions {
        let (provenance, _) = &input.provenance[&e.diagram.key];
        let superseded = matches!(provenance, Provenance::SupersededOnBlackhole { .. });
        let base = if superseded {
            e.key.as_str()
        } else {
            e.key.strip_suffix("_BH").unwrap_or(&e.key)
        };
        let scoped = if superseded {
            format!("wormhole::{base}")
        } else {
            base.to_string()
        };
        if let Some(other) = taken.insert(scoped.clone(), &e.key) {
            return Err(format!(
                "`{other}` and `{}` both want the name `{scoped}`",
                e.key
            ));
        }
        names.insert(&e.key, (base.to_string(), superseded));
    }

    let mut out = String::new();
    header(&mut out, input, instructions.len(), layouts.len());
    out.push_str(
        "use super::{DatumLayout, EncodeError, Field, Instruction, InstructionDef, Provenance};\n\n",
    );

    writeln!(
        out,
        "/// Every instruction encoding, by name. Blackhole's form takes the plain\n\
         /// name; the Wormhole form it supersedes is under [`defs::wormhole`].\n\
         pub mod defs {{\n    use super::*;\n"
    )
    .unwrap();
    for e in instructions.iter().filter(|e| !names[e.key.as_str()].1) {
        emit_def(&mut out, input, e, &names[e.key.as_str()].0, "    ");
    }
    writeln!(
        out,
        "    /// Encodings Blackhole replaces. Present so that the difference is\n\
         \x20   /// visible and testable, not so that they can be used here.\n\
         \x20   pub mod wormhole {{\n        use super::super::*;\n"
    )
    .unwrap();
    for e in instructions.iter().filter(|e| names[e.key.as_str()].1) {
        emit_def(&mut out, input, e, &names[e.key.as_str()].0, "        ");
    }
    writeln!(out, "    }}\n}}\n").unwrap();

    writeln!(
        out,
        "/// Every instruction encoding, for table-wide checks.\npub static ALL: &[&InstructionDef] = &["
    )
    .unwrap();
    for e in &instructions {
        let (name, superseded) = &names[e.key.as_str()];
        let path = if *superseded {
            format!("defs::wormhole::{name}")
        } else {
            format!("defs::{name}")
        };
        writeln!(out, "    &{path},").unwrap();
    }
    writeln!(out, "];\n").unwrap();

    writeln!(
        out,
        "/// The documented bit layout of each datum type in `Src` and `Dst`.\n\
         ///\n\
         /// The coprocessor does not entirely follow IEEE 754, so conversions are\n\
         /// built against these rather than against assumptions.\n\
         pub mod datum {{\n    use super::*;\n"
    )
    .unwrap();
    for d in &layouts {
        emit_layout(&mut out, input, d);
    }
    writeln!(out, "}}\n").unwrap();
    writeln!(
        out,
        "/// Every datum layout, for table-wide checks.\npub static ALL_LAYOUTS: &[&DatumLayout] = &["
    )
    .unwrap();
    for d in &layouts {
        writeln!(out, "    &datum::{},", d.key.to_uppercase()).unwrap();
    }
    writeln!(out, "];\n").unwrap();

    writeln!(
        out,
        "/// One encoder per instruction, taking its operands from the high bits of\n\
         /// the word down — the order the specification lists them in.\n\
         pub mod encode {{\n    use super::*;\n\n    \
         /// An encoder whose operands are too many to pass positionally without inviting a\n    \
         /// transposition: `Ty::ZERO.a(1).b(2).encode()`.\n    \
         macro_rules! builder {{\n        \
         ($(#[$meta:meta])* $ty:ident, $def:path, [$($field:ident),* $(,)?]) => {{\n            \
         $(#[$meta])*\n            \
         #[derive(Copy, Clone, Debug, Eq, PartialEq)]\n            \
         pub struct $ty {{ $($field: u32,)* }}\n            \
         impl $ty {{\n                \
         /// Every operand zero. Fixed bits are added by [`Self::encode`].\n                \
         pub const ZERO: Self = $ty {{ $($field: 0,)* }};\n                \
         $(pub const fn $field(mut self, value: u32) -> Self {{ self.$field = value; self }})\n\
         *\n                \
         pub const fn encode(self) -> Result<Instruction, EncodeError> {{\n                    \
         Instruction::with_operands(&$def, &[$(self.$field),*])\n                \
         }}\n            \
         }}\n        \
         }};\n    \
         }}\n"
    )
    .unwrap();
    for e in instructions.iter().filter(|e| !names[e.key.as_str()].1) {
        emit_encoder(&mut out, e, &names[e.key.as_str()].0, "defs", "    ");
    }
    writeln!(
        out,
        "    /// Encoders for the forms Blackhole replaces. Reaching one has to be\n\
         \x20   /// deliberate.\n\
         \x20   pub mod wormhole {{\n        use super::super::*;\n"
    )
    .unwrap();
    for e in instructions.iter().filter(|e| names[e.key.as_str()].1) {
        emit_encoder(
            &mut out,
            e,
            &names[e.key.as_str()].0,
            "defs::wormhole",
            "        ",
        );
    }
    writeln!(out, "    }}\n}}").unwrap();

    Ok(out)
}

fn header(out: &mut String, input: &Input<'_>, instructions: usize, layouts: usize) {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for (provenance, _) in input.provenance.values() {
        *counts.entry(provenance.variant_name()).or_default() += 1;
    }
    let count = |name: &str| counts.get(name).copied().unwrap_or(0);
    writeln!(
        out,
        "//! The Tensix instruction set, generated from `Diagrams/Src/Bits32.lua`.\n\
         //!\n\
         //! **Do not edit.** Regenerate with `cargo xtask gen-isa`, which fetches the\n\
         //! pinned specification itself; `cargo xtask gen-isa --check` fails when this\n\
         //! file is out of date.\n\
         //!\n\
         //! Source: tt-isa-documentation `{}`,\n\
         //! {instructions} instruction encodings and {layouts} datum layouts.\n\
         //!\n\
         //! Provenance: {} documented for Blackhole, {} shared with Wormhole and stated\n\
         //! to be identical, {} superseded on Blackhole, {} Wormhole-only and therefore\n\
         //! **`UNVERIFIED`**, {} **`MEASURED`** on ttsim or silicon where the specification\n\
         //! draws only Wormhole's layout (`xtask/src/genisa/Bits32_BH.lua`), and {}\n\
         //! Wormhole-only layouts **`CONFIRMED`** unchanged on Blackhole by a gate\n\
         //! (`xtask/src/genisa/measured.rs`).\n\
         //!\n\
         //! Names are the `Bits32.lua` diagram keys, so a name in the specification can\n\
         //! be found here without translation — except that a Blackhole-specific form\n\
         //! drops its `_BH` suffix and the Wormhole form it replaces moves into the\n\
         //! `wormhole` submodule, so the plain name is always the one to use here.\n\
         //! That is why this module allows globals that are not upper case.\n\
         #![allow(non_upper_case_globals)]\n\
         #![allow(clippy::unreadable_literal)]\n",
        input.spec_rev,
        count("Blackhole"),
        count("SharedWithWormhole"),
        count("SupersededOnBlackhole"),
        count("WormholeOnly"),
        count("Measured"),
        count("Confirmed"),
    )
    .unwrap();
}

/// `Field::new(name, first_bit, width, signed, part)`.
///
/// `signed` stays its own argument rather than being derived from `part` at
/// runtime: `Field::signed` is `const fn` on the encode path.
fn field_expr(name: &str, f: &DrawnField) -> String {
    let (signed, part) = match &f.label {
        Label::Named { note: Some(n), .. } if n == "signed" => (true, "None".to_string()),
        Label::Named { note: Some(n), .. } => (false, format!("Some({n:?})")),
        _ => (false, "None".to_string()),
    };
    format!(
        "Field::new(\"{name}\", {}, {}, {signed}, {part})",
        f.first_bit, f.width
    )
}

fn fixed_bits(f: &DrawnField) -> Option<String> {
    match &f.label {
        Label::Fixed { value } => Some(format!(
            "(Field::new(\"\", {}, {}, false, None), {value})",
            f.first_bit, f.width
        )),
        _ => None,
    }
}

fn emit_def(out: &mut String, input: &Input<'_>, e: &Entry<'_>, name: &str, pad: &str) {
    let d = e.diagram;
    let (provenance, page) = &input.provenance[&d.key];
    let fields: Vec<String> = d.operands().iter().map(|(n, f)| field_expr(n, f)).collect();
    let fixed: Vec<String> = d
        .fields
        .iter()
        .filter_map(|f| {
            // The opcode is carried separately.
            if f.first_bit == 24 && f.width == 8 {
                return None;
            }
            fixed_bits(f)
        })
        .collect();
    let prov_expr = match provenance {
        Provenance::SupersededOnBlackhole { by } => {
            format!("Provenance::SupersededOnBlackhole {{ by: \"{by}\" }}")
        }
        Provenance::Measured {
            evidence,
            moved,
            dropped,
            widened,
        } => {
            let quoted = |v: &[String]| {
                v.iter()
                    .map(|m| format!("\"{m}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            format!(
                "Provenance::Measured {{ evidence: \"{evidence}\", moved: &[{}], dropped: &[{}], \
                 widened: &[{}] }}",
                quoted(moved),
                quoted(dropped),
                quoted(widened)
            )
        }
        Provenance::Confirmed { evidence } => {
            format!("Provenance::Confirmed {{ evidence: \"{evidence}\" }}")
        }
        other => format!("Provenance::{}", other.variant_name()),
    };
    let doc = doc(e, provenance, page);
    writeln!(out, "{pad}/// `{}`. {doc}", e.key).unwrap();
    writeln!(
        out,
        "{pad}pub static {name}: InstructionDef = InstructionDef::new(\n\
         {pad}    \"{}\",\n{pad}    \"{}\",\n{pad}    {:#04x},\n\
         {pad}    &[{}],\n{pad}    &[{}],\n{pad}    {:#010x},\n{pad}    {prov_expr},\n\
         {pad}    \"{page}\",\n{pad});\n",
        e.key,
        e.mnemonic,
        e.opcode,
        fields.join(", "),
        fixed.join(", "),
        // Bits no field claims, excluding the opcode byte.
        d.undrawn() & 0x00FF_FFFF,
    )
    .unwrap();
}

fn doc(e: &Entry<'_>, provenance: &Provenance, page: &str) -> String {
    if let (Provenance::Measured { evidence, .. }, "UNPACR_NOP_SETDVALID_BH") =
        (provenance, e.key.as_str())
    {
        return format!(
            "**`MEASURED`** on Blackhole silicon by `{evidence}`. Replaces Wormhole fixed mode 7 \
             with the bounded non-clearing publication profile 0x1e9; only WhichUnpacker varies. \
             ttsim refuses this format-selector flavor."
        );
    }
    match provenance {
        Provenance::Blackhole => format!("Documented for Blackhole in `{page}`."),
        Provenance::SharedWithWormhole => {
            format!("`{page}`, which the Blackhole tree states is shared and identical.")
        }
        Provenance::SupersededOnBlackhole { by } => format!(
            "**Wormhole's encoding**, from `{page}`. Blackhole replaces it with `{by}`; use that \
             instead."
        ),
        Provenance::WormholeOnly => format!(
            "**`UNVERIFIED`.** `{page}` is a Wormhole page and Blackhole has none, so this layout \
             is a hypothesis until silicon or the simulator confirms it."
        ),
        Provenance::Confirmed { evidence } => format!(
            "**`CONFIRMED`** on Blackhole, ttsim and silicon, by `{evidence}`: the layout of \
             `{page}` (a Wormhole page; Blackhole has none), every field exercised."
        ),
        Provenance::Measured {
            evidence,
            moved,
            dropped,
            widened,
        } => {
            let ticked = |v: &[String]| {
                v.iter()
                    .map(|m| format!("`{m}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let dropped = if dropped.is_empty() {
                String::new()
            } else {
                format!(
                    " {} {} not carried: {} bits hold something else on Blackhole and {} own \
                     position is unknown.",
                    ticked(dropped),
                    if dropped.len() == 1 { "is" } else { "are" },
                    if dropped.len() == 1 { "its" } else { "their" },
                    if dropped.len() == 1 { "its" } else { "their" },
                )
            };
            let widened = if widened.is_empty() {
                String::new()
            } else {
                format!(
                    " {} {} a different width on Blackhole.",
                    ticked(widened),
                    if widened.len() == 1 { "has" } else { "have" },
                )
            };
            let remaining = if matches!(e.key.as_str(), "ELWADD_BH" | "ELWSUB_BH" | "ELWMUL_BH") {
                "Other fields are carried from that diagram. Step9 and step90 validate broadcast, \
                 assignment, Dst addressing and repeated bank release on ttsim and both Blackhole \
                 cards; floating arithmetic is not IEEE754."
            } else {
                "Every other field is carried from that diagram and is as unverified as it was. \
                 Re-derive on silicon."
            };
            format!(
                "**`MEASURED`** on ttsim or silicon by `{evidence}`, not documented: the only \
                 diagram is Wormhole's (`{page}`), and on Blackhole {} sit{} \
                 elsewhere.{widened}{dropped} {remaining}",
                ticked(moved),
                if moved.len() == 1 { "s" } else { "" }
            )
        }
    }
}

fn emit_layout(out: &mut String, input: &Input<'_>, d: &Diagram) {
    let (_, page) = &input.provenance[&d.key];
    let fields: Vec<String> = d.operands().iter().map(|(n, f)| field_expr(n, f)).collect();
    // The must-be-zero padding `Src` formats carry between mantissa and exponent.
    // Without it the table describes 16 of a 19-bit datum, and code building a datum
    // has no way to learn which bits to leave alone.
    let fixed: Vec<String> = d.fields.iter().filter_map(fixed_bits).collect();
    writeln!(out, "    /// `{}`, from `{page}`.", d.key).unwrap();
    writeln!(
        out,
        "    pub static {}: DatumLayout = DatumLayout::new(\"{}\", {}, &[{}], &[{}], \"{page}\");\n",
        d.key.to_uppercase(),
        d.key,
        d.nbits,
        fields.join(", "),
        fixed.join(", "),
    )
    .unwrap();
}

fn emit_encoder(out: &mut String, e: &Entry<'_>, name: &str, defs: &str, pad: &str) {
    let args: Vec<(String, &DrawnField)> = e
        .diagram
        .operands()
        .iter()
        .map(|(n, f)| (snake(n), *f))
        .collect();
    let fn_name = snake(name);

    if args.len() <= POSITIONAL_LIMIT {
        let params: Vec<String> = args.iter().map(|(n, _)| format!("{n}: u32")).collect();
        writeln!(
            out,
            "{pad}/// `{}`.\n\
             {pad}pub const fn {fn_name}({}) -> Result<Instruction, EncodeError> {{\n\
             {pad}    Instruction::with_operands(&{defs}::{name}, &[{}])\n\
             {pad}}}\n",
            e.key,
            params.join(", "),
            args.iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        )
        .unwrap();
        return;
    }

    // Too many operands to pass positionally without inviting a transposition.
    let ty = camel(name);
    writeln!(
        out,
        "{pad}builder!(\n\
         {pad}    #[doc = \"`{}`.\"]\n\
         {pad}    {ty},\n\
         {pad}    {defs}::{name},\n\
         {pad}    [{}]\n\
         {pad});\n",
        e.key,
        args.iter()
            .map(|(n, _)| n.as_str())
            .collect::<Vec<_>>()
            .join(", "),
    )
    .unwrap();
}

/// `SFPSTORE_BH` -> `sfpstore_bh`, `WhichPackers` -> `which_packers`.
fn snake(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && chars[i - 1].is_ascii_lowercase();
            let prev_digit = i > 0 && chars[i - 1].is_ascii_digit();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_ascii_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_ascii_uppercase();
            if (prev_lower || prev_digit || (prev_upper && next_lower)) && !out.ends_with('_') {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(*c);
        }
    }
    let out = out.to_ascii_lowercase();
    // `type`, `mod` and friends cannot be identifiers.
    if matches!(out.as_str(), "type" | "mod" | "ref" | "move" | "box" | "in") {
        format!("{out}_")
    } else {
        out
    }
}

fn camel(s: &str) -> String {
    snake(s)
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
