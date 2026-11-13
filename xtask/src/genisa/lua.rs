// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! `Diagrams/Src/Bits32.lua` -> bit-layout diagrams.
//!
//! `Bits32.lua` is a drawing script: each entry places labelled rectangles over a
//! 16-, 19- or 32-bit word. Running it through Lua would be shorter and wrong: an
//! interpreter evaluates a changed grammar happily, so an upstream edit would
//! silently produce a different table. This parser fails on anything it does not
//! recognise, which is the whole point of having one.
//!
//! ```lua
//! local diagrams = {
//!   KEY = function()
//!     return Bits32{                        -- optionally `{nbits = N,` or `{extra_w = N,`
//!       {first_bit, width, "label"},        -- optionally `, y = N` and/or `, edge = "..."`
//!     }
//!   end,
//! }
//! ```
//!
//! `y` and `edge` place the label text in the rendered SVG and `extra_w` widens the
//! drawing: cosmetic, discarded. Any *other* key is an error, so a semantically
//! meaningful addition upstream cannot be dropped along with the decoration.

/// One entry of the `diagrams` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagram {
    /// The table key, e.g. `SFPSTORE_BH`. Not the mnemonic: mode variants such as
    /// `UNPACR_Regular` and `UNPACR_FlushCache` are separate keys of one mnemonic.
    pub key: String,
    /// Width of the word drawn: 32 for instructions, 16 or 19 for `Src`/`Dst`.
    pub nbits: u8,
    /// In the order the diagram lists them, which is descending `first_bit`.
    pub fields: Vec<Field>,
    /// Line the entry starts on, for error messages.
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub first_bit: u8,
    pub width: u8,
    pub label: Label,
    pub line: usize,
}

/// What a diagram label means.
///
/// A bare number is not a field at all: it is a bit pattern the instruction must
/// carry, and treating the two alike would generate encoders that let software
/// write a sub-opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    /// An operand: `VD`, `Imm10`, `AddrMod`. `note` is a parenthesised qualifier:
    /// `Imm12 (signed)`, `Magnitude (low)`.
    Named { name: String, note: Option<String> },
    /// A constant the instruction carries: the opcode, a sub-opcode selector, or
    /// must-be-zero bits.
    Fixed { value: u32 },
    /// An opcode that carries an operand: `0xB3 + Index1` in `RMWCIB`.
    Computed { base: u32, addend: String },
}

impl Field {
    pub fn mask(&self) -> u32 {
        if self.width >= 32 {
            u32::MAX
        } else {
            ((1u32 << self.width) - 1) << self.first_bit
        }
    }

    pub fn last_bit(&self) -> u32 {
        self.first_bit as u32 + self.width as u32 - 1
    }
}

impl Diagram {
    /// Bits no field claims. These are the norm rather than the exception, so they
    /// are recorded rather than assumed to be zero.
    pub fn undrawn(&self) -> u32 {
        let covered = self.fields.iter().fold(0u32, |acc, f| acc | f.mask());
        let all = if self.nbits >= 32 {
            u32::MAX
        } else {
            (1u32 << self.nbits) - 1
        };
        all & !covered
    }

    /// The opcode field, if this diagram has one at bits 24..31. Its presence is
    /// what separates an instruction from a datum layout.
    pub fn opcode(&self) -> Option<&Field> {
        self.fields
            .iter()
            .find(|f| f.first_bit == 24 && f.width == 8)
    }

    /// Operands, high bits first — the order the specification lists them in.
    pub fn operands(&self) -> Vec<(&str, &Field)> {
        let mut out: Vec<(&str, &Field)> = self
            .fields
            .iter()
            .filter_map(|f| match &f.label {
                Label::Named { name, .. } => Some((name.as_str(), f)),
                _ => None,
            })
            .collect();
        out.sort_by_key(|(_, f)| std::cmp::Reverse(f.first_bit));
        out
    }
}

const TABLE_OPEN: &str = "local diagrams = {";

/// Labels that are none of the general shapes, with what each means.
///
/// Deliberately short and hard to grow: every entry is a judgement about what a
/// piece of the instruction set encodes. [`unused`] refuses an entry nothing uses,
/// so a specification bump cannot leave one behind.
const SPECIAL_LABELS: &[(&str, &str, &str)] = &[(
    "16 or 0", "Exponent",
    // `Dst.md`: integer "8" is "overlaid onto FP16, using a fixed raw exponent of
    // 16 (or sometimes a raw exponent of 0 when the magnitude is zero)". The other
    // 16-bit layouts label the same bits `Exponent`.
    "16 or 0",
)];

/// Which [`SPECIAL_LABELS`] entries no longer appear in the source.
pub fn unused(source: &str) -> Vec<&'static str> {
    SPECIAL_LABELS
        .iter()
        .filter(|(text, ..)| !source.contains(&format!("\"{text}\"")))
        .map(|(text, ..)| *text)
        .collect()
}

pub fn parse(source: &str) -> Result<Vec<Diagram>, String> {
    let lines: Vec<&str> = source.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim_end() == TABLE_OPEN)
        .ok_or_else(|| format!("`{TABLE_OPEN}` not found — is this Bits32.lua?"))?;

    let mut diagrams = Vec::new();
    let mut i = start + 1;
    while let Some(line) = lines.get(i) {
        // A `}` at the left margin closes the table; everything inside is indented.
        if line.starts_with('}') {
            break;
        }
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        let (diagram, next) = entry(&lines, i)?;
        diagrams.push(diagram);
        i = next;
    }
    if diagrams.is_empty() {
        return Err("the diagrams table is empty".into());
    }
    Ok(diagrams)
}

/// Collapse keys defined more than once, insisting the definitions agree.
///
/// `SEMINIT` is defined twice in `Bits32.lua`, identically; Lua keeps the second
/// silently. Doing the same would hide the day the two copies diverge.
pub fn dedupe(diagrams: Vec<Diagram>) -> Result<Vec<Diagram>, String> {
    let mut out: Vec<Diagram> = Vec::new();
    for diagram in diagrams {
        match out.iter().find(|d| d.key == diagram.key) {
            None => out.push(diagram),
            Some(first) => {
                let same = first.nbits == diagram.nbits
                    && first.fields.len() == diagram.fields.len()
                    && first.fields.iter().zip(&diagram.fields).all(|(a, b)| {
                        a.first_bit == b.first_bit && a.width == b.width && a.label == b.label
                    });
                if !same {
                    return Err(format!(
                        "`{}` is defined twice, at lines {} and {}, and the two definitions \
                         differ. Lua keeps the second silently; decide which is right rather \
                         than inheriting that.",
                        diagram.key, first.line, diagram.line
                    ));
                }
            }
        }
    }
    Ok(out)
}

/// Per-diagram structural invariants: fields inside the word, no overlap, fixed
/// values that fit, and exactly the one computed opcode the instruction set has.
pub fn structure(diagrams: &[Diagram]) -> Result<(), String> {
    let mut computed: Vec<&str> = Vec::new();
    for d in diagrams {
        for (i, f) in d.fields.iter().enumerate() {
            if f.last_bit() >= d.nbits as u32 {
                return Err(format!(
                    "line {}: `{}` field at bits {}..={} runs past the {}-bit word",
                    f.line,
                    d.key,
                    f.first_bit,
                    f.last_bit(),
                    d.nbits
                ));
            }
            for g in &d.fields[i + 1..] {
                if f.mask() & g.mask() != 0 {
                    return Err(format!(
                        "line {}: `{}` fields at bits {}..={} and {}..={} overlap",
                        f.line,
                        d.key,
                        f.first_bit,
                        f.last_bit(),
                        g.first_bit,
                        g.last_bit()
                    ));
                }
            }
            if let Label::Fixed { value } = &f.label {
                let max = if f.width >= 32 {
                    u32::MAX
                } else {
                    (1u32 << f.width) - 1
                };
                if *value > max {
                    return Err(format!(
                        "line {}: `{}` fixed value {value:#x} does not fit the {}-bit field at \
                         bit {}",
                        f.line, d.key, f.width, f.first_bit
                    ));
                }
            }
            if matches!(f.label, Label::Computed { .. }) {
                computed.push(d.key.as_str());
            }
        }
    }
    // `RMWCIB`'s `0xB3 + Index1` is the only opcode in the instruction set that
    // carries an operand. A second one needs its own decision about how to split
    // the opcode from the field, so it stops the build rather than being absorbed.
    match computed.as_slice() {
        ["RMWCIB"] => Ok(()),
        [] => Err("`RMWCIB`'s computed opcode has disappeared from Bits32.lua".into()),
        other => Err(format!(
            "expected exactly one computed opcode (`RMWCIB`), found {}: {}",
            other.len(),
            other.join(", ")
        )),
    }
}

/// Parse one `KEY = function() … end,`, returning it and the line after it.
fn entry(lines: &[&str], start: usize) -> Result<(Diagram, usize), String> {
    let at = |n: usize| n + 1; // human line numbers
    let head = lines[start].trim();
    let key = head
        .strip_suffix(" = function()")
        .filter(|k| !k.is_empty() && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        .ok_or_else(|| {
            format!(
                "line {}: expected `KEY = function()`, found `{head}`",
                at(start)
            )
        })?
        .to_string();

    let open = lines
        .get(start + 1)
        .map(|l| l.trim())
        .ok_or_else(|| format!("line {}: `{key}` is never closed", at(start)))?;
    let rest = open.strip_prefix("return Bits32{").ok_or_else(|| {
        format!(
            "line {}: expected `return Bits32{{`, found `{open}`",
            at(start + 1)
        )
    })?;

    let mut nbits = 32u8;
    for assignment in rest.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (name, value) = assignment
            .split_once('=')
            .map(|(a, b)| (a.trim(), b.trim()))
            .ok_or_else(|| {
                format!(
                    "line {}: expected `name = value`, found `{assignment}`",
                    at(start + 1)
                )
            })?;
        match name {
            "nbits" => {
                nbits =
                    u32::try_from(parse_u32(value).ok_or_else(|| format!("bad nbits `{value}`"))?)
                        .ok()
                        .and_then(|v| u8::try_from(v).ok())
                        .ok_or_else(|| format!("line {}: bad nbits `{value}`", at(start + 1)))?;
            }
            // Widens the drawing to fit long labels. Cosmetic.
            "extra_w" => {}
            other => {
                return Err(format!(
                    "line {}: unknown Bits32 table key `{other}` — it may be meaningful rather \
                     than cosmetic, so this refuses to discard it",
                    at(start + 1)
                ));
            }
        }
    }
    if !matches!(nbits, 16 | 19 | 32) {
        return Err(format!(
            "line {}: `{key}` has nbits = {nbits}; only 16, 19 and 32 are known",
            at(start + 1)
        ));
    }

    let mut fields = Vec::new();
    let mut i = start + 2;
    loop {
        let line = lines
            .get(i)
            .ok_or_else(|| format!("line {}: `{key}` is never closed", at(start)))?;
        let trimmed = line.trim();
        if trimmed == "}" {
            i += 1;
            break;
        }
        if !trimmed.is_empty() {
            fields.push(field(trimmed, at(i))?);
        }
        i += 1;
    }

    let end = lines.get(i).map(|l| l.trim()).unwrap_or_default();
    if end != "end," {
        return Err(format!(
            "line {}: expected `end,` closing `{key}`, found `{end}`",
            at(i)
        ));
    }
    Ok((
        Diagram {
            key,
            nbits,
            fields,
            line: at(start),
        },
        i + 1,
    ))
}

/// Parse `{first_bit, width, "label"}` plus optional cosmetic keys.
fn field(text: &str, line: usize) -> Result<Field, String> {
    let body = text
        .strip_prefix('{')
        .and_then(|t| t.strip_suffix(','))
        .and_then(|t| t.strip_suffix('}'))
        .ok_or_else(|| format!("line {line}: expected `{{a, b, \"label\"}},`, found `{text}`"))?;

    // The label is the only quoted part, so splitting on the quotes separates the
    // two numbers from it without worrying about commas inside the label.
    let (before, rest) = body
        .split_once('"')
        .ok_or_else(|| format!("line {line}: field has no quoted label: `{text}`"))?;
    let (label, after) = rest
        .split_once('"')
        .ok_or_else(|| format!("line {line}: unterminated label: `{text}`"))?;

    let numbers: Vec<&str> = before
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if numbers.len() != 2 {
        return Err(format!(
            "line {line}: expected two numbers before the label, found {}: `{text}`",
            numbers.len()
        ));
    }
    let bit = |s: &str, what: &str| -> Result<u8, String> {
        parse_u32(s)
            .and_then(|v| u8::try_from(v).ok())
            .ok_or_else(|| format!("line {line}: bad {what} `{s}`"))
    };
    let first_bit = bit(numbers[0], "first_bit")?;
    let width = bit(numbers[1], "width")?;
    if width == 0 {
        return Err(format!("line {line}: zero-width field: `{text}`"));
    }

    for assignment in after.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (name, _) = assignment
            .split_once('=')
            .ok_or_else(|| format!("line {line}: expected `name = value`, found `{assignment}`"))?;
        match name.trim() {
            // A leader line dropping the label below the diagram, and the text
            // anchor. Both purely how the SVG looks.
            "y" | "edge" => {}
            other => {
                return Err(format!(
                    "line {line}: unknown field key `{other}` — it may be meaningful rather than \
                     cosmetic, so this refuses to discard it"
                ));
            }
        }
    }

    Ok(Field {
        first_bit,
        width,
        label: parse_label(label, line)?,
        line,
    })
}

fn parse_label(text: &str, line: usize) -> Result<Label, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err(format!(
            "line {line}: empty label. No diagram uses one today; if upstream introduces one, \
             decide what it means rather than guessing here."
        ));
    }
    if let Some(value) = parse_u32(text) {
        return Ok(Label::Fixed { value });
    }
    // `0xB3 + Index1` — an opcode that carries an operand.
    if let Some((base, addend)) = text.split_once('+') {
        if let (Some(base), Some(addend)) = (parse_u32(base.trim()), identifier(addend.trim())) {
            return Ok(Label::Computed {
                base,
                addend: addend.to_string(),
            });
        }
    }
    // `Imm12 (signed)`, `Magnitude (low)`.
    let (name, note) = match text.split_once(" (") {
        Some((name, note)) => match note.strip_suffix(')') {
            Some(note) => (name, Some(note.to_string())),
            None => (text, None),
        },
        None => (text, None),
    };
    if let Some(name) = identifier(name) {
        return Ok(Label::Named {
            name: name.to_string(),
            note,
        });
    }
    if let Some((_, name, note)) = SPECIAL_LABELS.iter().find(|(t, ..)| *t == text) {
        return Ok(Label::Named {
            name: (*name).to_string(),
            note: Some((*note).to_string()),
        });
    }
    Err(format!(
        "line {line}: unrecognised label `{text}`. It is neither an identifier, a number, nor \
         `0xNN + Name`, and SPECIAL_LABELS has no entry for it, so what it encodes has to be \
         decided rather than guessed."
    ))
}

fn identifier(s: &str) -> Option<&str> {
    let ok = !s.is_empty()
        && s.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    ok.then_some(s)
}

fn parse_u32(s: &str) -> Option<u32> {
    let s = s.trim();
    match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(hex) if !hex.is_empty() => u32::from_str_radix(hex, 16).ok(),
        Some(_) => None,
        None if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) => s.parse().ok(),
        None => None,
    }
}
