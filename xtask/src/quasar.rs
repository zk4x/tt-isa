// Copyright (C) 2026 zk4x
// SPDX-License-Identifier: Apache-2.0

//! tt-metal's Quasar LLK `assembly.yaml` -> `tt_isa::isa::quasar`.
//!
//! Quasar is not covered by `tt-isa-documentation`; tt-metal carries the
//! instruction table as the input to its Quasar LLK assembly generator. The
//! table is regular: one top-level name per instruction, an `op_binary`, and a
//! list of bit-positioned arguments. That is enough to generate the raw
//! encoding layer, so the same pin-and-verify discipline applies as for the
//! Blackhole tables.
//!
//! Deliberately no YAML dependency: the handful of shapes this file needs are
//! parsed directly, and a malformed input is an error rather than a guess.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use crate::fetch;
use crate::pin;
use crate::util;

/// Up to this many fields an encoder takes its operands positionally; above it
/// the builder form is generated so call sites cannot transpose two same-typed
/// operands by accident.
const BUILDER_AFTER: usize = 4;

#[derive(Clone, Debug)]
struct Arg {
    name: String,
    start_bit: u8,
    size: u8,
}

#[derive(Debug)]
struct Instruction {
    name: String,
    opcode: u8,
    /// High bits first, the order [`super::Instruction::with_operands`] expects.
    args: Vec<Arg>,
}

#[derive(Debug)]
enum ArgsSpec {
    Inline {
        args: Vec<Arg>,
        anchor: Option<String>,
    },
    Alias(String),
}

#[derive(Debug)]
struct RawInstruction {
    name: String,
    opcode: Option<u8>,
    spec: ArgsSpec,
}

enum ArgsKind {
    Inline { anchor: Option<String> },
    Alias(String),
}

struct Parser {
    instructions: Vec<RawInstruction>,
    current: Option<RawInstruction>,
    args: Vec<Arg>,
    current_arg: Option<Arg>,
    args_kind: ArgsKind,
    args_indent: usize,
    in_args: bool,
}

impl Parser {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            current: None,
            args: Vec::new(),
            current_arg: None,
            args_kind: ArgsKind::Inline { anchor: None },
            args_indent: 0,
            in_args: false,
        }
    }

    fn parse(mut self, text: &str) -> Result<Vec<RawInstruction>, String> {
        for (index, raw) in text.lines().enumerate() {
            self.line(index + 1, raw)?;
        }
        self.flush_instruction()?;
        Ok(self.instructions)
    }

    fn line(&mut self, lineno: usize, raw: &str) -> Result<(), String> {
        let line = strip_comment(raw);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        let indent = line.len() - line.trim_start().len();

        if indent == 0 {
            self.flush_instruction()?;
            let Some(name) = trimmed.strip_suffix(':') else {
                return Err(format!(
                    "line {lineno}: expected an instruction name ending in `:`, got `{trimmed}`"
                ));
            };
            let name = name.trim();
            if !is_ident(name) {
                return Err(format!("line {lineno}: `{name}` is not a Rust identifier"));
            }
            self.current = Some(RawInstruction {
                name: name.to_string(),
                opcode: None,
                spec: ArgsSpec::Inline {
                    args: Vec::new(),
                    anchor: None,
                },
            });
            self.args_kind = ArgsKind::Inline { anchor: None };
            self.args.clear();
            self.current_arg = None;
            self.in_args = false;
            return Ok(());
        }

        if let Some(rest) = trimmed.strip_prefix("op_binary:") {
            let opcode = parse_u8(rest.trim()).map_err(|e| format!("line {lineno}: {e}"))?;
            let Some(current) = self.current.as_mut() else {
                return Err(format!(
                    "line {lineno}: `op_binary` before the first instruction"
                ));
            };
            current.opcode = Some(opcode);
            return Ok(());
        }

        if let Some(rest) = trimmed.strip_prefix("arguments:") {
            let rest = rest.trim();
            self.args_indent = indent;
            self.args.clear();
            self.current_arg = None;
            self.in_args = true;
            self.args_kind = if rest.is_empty() || rest == "0" {
                ArgsKind::Inline { anchor: None }
            } else if let Some(alias) = rest.strip_prefix('*') {
                self.in_args = false;
                ArgsKind::Alias(alias.trim().to_string())
            } else if let Some(anchor) = rest.strip_prefix('&') {
                ArgsKind::Inline {
                    anchor: Some(anchor.trim().to_string()),
                }
            } else {
                return Err(format!(
                    "line {lineno}: unsupported `arguments:` value `{rest}`"
                ));
            };
            return Ok(());
        }

        if self.in_args {
            if indent <= self.args_indent && !trimmed.starts_with('-') {
                self.in_args = false;
            } else if let Some(name) = arg_name(trimmed) {
                self.flush_arg()
                    .map_err(|e| format!("line {lineno}: {e}"))?;
                if !is_ident(name) {
                    return Err(format!(
                        "line {lineno}: argument `{name}` is not a Rust identifier"
                    ));
                }
                self.current_arg = Some(Arg {
                    name: name.to_string(),
                    start_bit: 0,
                    size: 0,
                });
                return Ok(());
            } else if let Some(arg) = self.current_arg.as_mut() {
                if let Some(rest) = trimmed.strip_prefix("start_bit:") {
                    arg.start_bit =
                        parse_u8(rest.trim()).map_err(|e| format!("line {lineno}: {e}"))?;
                    return Ok(());
                }
                if let Some(rest) = trimmed.strip_prefix("size:") {
                    arg.size = parse_u8(rest.trim()).map_err(|e| format!("line {lineno}: {e}"))?;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn flush_arg(&mut self) -> Result<(), String> {
        if let Some(arg) = self.current_arg.take() {
            if arg.size == 0 {
                return Err(format!("argument `{}` has no size", arg.name));
            }
            self.args.push(arg);
        }
        Ok(())
    }

    fn flush_instruction(&mut self) -> Result<(), String> {
        self.flush_arg()?;
        let Some(current) = self.current.take() else {
            return Ok(());
        };
        let spec = match std::mem::replace(&mut self.args_kind, ArgsKind::Inline { anchor: None }) {
            ArgsKind::Inline { anchor } => ArgsSpec::Inline {
                args: std::mem::take(&mut self.args),
                anchor,
            },
            ArgsKind::Alias(name) => ArgsSpec::Alias(name),
        };
        self.instructions.push(RawInstruction {
            name: current.name,
            opcode: current.opcode,
            spec,
        });
        self.args.clear();
        self.current_arg = None;
        self.in_args = false;
        Ok(())
    }
}

/// Generate `src/isa/quasar.rs`, or check the committed copy with `--check`.
pub fn generate(check_only: bool) -> Result<(), String> {
    fetch::quasar_assembly_if_needed()?;
    let root = util::workspace_root();
    let source = fetch::quasar_assembly_path();
    let text = std::fs::read_to_string(&source)
        .map_err(|e| format!("reading {}: {e}", source.display()))?;
    let raw = Parser::new().parse(&text)?;
    let instructions = resolve(raw)?;
    check_names(&instructions)?;
    let rendered = render(&instructions)?;
    let formatted = util::rustfmt(&rendered, &root)?;
    util::commit(
        &root.join("src/isa/quasar.rs"),
        &formatted,
        check_only,
        "tt-metal Quasar LLK assembly.yaml",
    )
}

fn resolve(raw: Vec<RawInstruction>) -> Result<Vec<Instruction>, String> {
    let mut anchors: BTreeMap<String, Vec<Arg>> = BTreeMap::new();
    for inst in &raw {
        if let ArgsSpec::Inline {
            args,
            anchor: Some(anchor),
        } = &inst.spec
        {
            if anchors.insert(anchor.clone(), args.clone()).is_some() {
                return Err(format!("anchor `{anchor}` is defined twice"));
            }
        }
    }

    let mut instructions = Vec::with_capacity(raw.len());
    for inst in raw {
        let opcode = inst
            .opcode
            .ok_or_else(|| format!("`{}` has no `op_binary`", inst.name))?;
        let mut args = match inst.spec {
            ArgsSpec::Inline { args, .. } => args,
            ArgsSpec::Alias(alias) => anchors
                .get(&alias)
                .cloned()
                .ok_or_else(|| format!("`{}` uses unknown argument anchor `{alias}`", inst.name))?,
        };
        args.sort_by(|a, b| b.start_bit.cmp(&a.start_bit));
        validate(&inst.name, &args)?;
        instructions.push(Instruction {
            name: inst.name,
            opcode,
            args,
        });
    }
    Ok(instructions)
}

fn validate(name: &str, args: &[Arg]) -> Result<(), String> {
    let mut claimed = 0u32;
    let mut names = BTreeSet::new();
    for arg in args {
        if arg.size == 0 {
            return Err(format!("`{name}`: argument `{}` has no size", arg.name));
        }
        if arg.size > 24 {
            return Err(format!(
                "`{name}`: argument `{}` is {} bits, wider than the 24-bit operand field",
                arg.name, arg.size
            ));
        }
        let end = u32::from(arg.start_bit) + u32::from(arg.size);
        if end > 24 {
            return Err(format!(
                "`{name}`: argument `{}` ends at bit {end}, past the 24-bit operand field",
                arg.name
            ));
        }
        let mask = ((1u32 << arg.size) - 1) << arg.start_bit;
        if claimed & mask != 0 {
            return Err(format!(
                "`{name}`: argument `{}` overlaps another argument",
                arg.name
            ));
        }
        claimed |= mask;
        if !names.insert(arg.name.clone()) {
            return Err(format!("`{name}`: duplicate argument `{}`", arg.name));
        }
    }
    Ok(())
}

fn check_names(instructions: &[Instruction]) -> Result<(), String> {
    let mut opcodes: BTreeMap<u8, &str> = BTreeMap::new();
    let mut encoders = BTreeSet::new();
    let mut builders = BTreeSet::new();
    for inst in instructions {
        if let Some(previous) = opcodes.insert(inst.opcode, &inst.name) {
            return Err(format!(
                "`{}` and `{}` share opcode {:#04x}",
                previous, inst.name, inst.opcode
            ));
        }

        let encoder = snake(&inst.name);
        if encoder.is_empty() || !encoders.insert(encoder.clone()) {
            return Err(format!("encoder name collision at `{encoder}`"));
        }

        let mut arg_names = BTreeSet::new();
        for arg in &inst.args {
            if !arg_names.insert(snake(&arg.name)) {
                return Err(format!(
                    "`{}`: argument names collide after snake_case",
                    inst.name
                ));
            }
        }

        if inst.args.len() > BUILDER_AFTER {
            let builder = camel(&inst.name);
            if builder.is_empty() || !builders.insert(builder.clone()) {
                return Err(format!("builder name collision at `{builder}`"));
            }
        }
    }
    Ok(())
}

fn render(instructions: &[Instruction]) -> Result<String, String> {
    let mut out = String::new();

    writeln!(
        out,
        "//! The Quasar Tensix instruction set, generated from tt-metal's Quasar LLK\n\
         //! `assembly.yaml`.\n\
         //!\n\
         //! **Do not edit.** Regenerate with `cargo xtask gen-quasar`.\n\
         //!\n\
         //! Source: tt-metal `{}`, {} instruction encodings.\n\
         //!\n\
         //! This is the raw layer: a field is refused only when the value does not\n\
         //! fit. What the hardware honours is a separate question.\n\
         //!\n\
         //! The LLK table does not mark signedness, so every field is marked\n\
         //! unsigned here. A two's-complement immediate is still encoded by passing\n\
         //! its bit pattern; only [`Field::signed`] reports false.\n\
         #![allow(non_upper_case_globals)]\n\
         #![allow(clippy::unreadable_literal)]\n\n\
         use super::{{EncodeError, Field, Instruction, InstructionDef, Provenance}};\n",
        pin::QUASAR_REV,
        instructions.len()
    )
    .map_err(|e| e.to_string())?;

    writeln!(
        out,
        "/// One `InstructionDef` per Quasar instruction, from the LLK assembly table.\n\
         pub mod defs {{\n    use super::*;\n"
    )
    .map_err(|e| e.to_string())?;

    for inst in instructions {
        writeln!(
            out,
            "    /// `{name}`, from the Quasar LLK assembly table.",
            name = inst.name
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            out,
            "    pub static {name}: InstructionDef = InstructionDef::new(",
            name = inst.name
        )
        .map_err(|e| e.to_string())?;
        writeln!(out, "        {:?},", inst.name).map_err(|e| e.to_string())?;
        writeln!(out, "        {:?},", inst.name).map_err(|e| e.to_string())?;
        writeln!(out, "        {:#04x},", inst.opcode).map_err(|e| e.to_string())?;
        let fields: Vec<String> = inst
            .args
            .iter()
            .map(|arg| {
                format!(
                    "Field::new({:?}, {}, {}, false, None)",
                    arg.name, arg.start_bit, arg.size
                )
            })
            .collect();
        writeln!(out, "        &[{}],", fields.join(", ")).map_err(|e| e.to_string())?;
        writeln!(out, "        &[],").map_err(|e| e.to_string())?;
        let claimed = inst.args.iter().fold(0u32, |claimed, arg| {
            claimed | (((1u32 << arg.size) - 1) << arg.start_bit)
        });
        let unspecified = 0x00FF_FFFF & !claimed;
        writeln!(out, "        {unspecified:#010x},").map_err(|e| e.to_string())?;
        writeln!(out, "        Provenance::Quasar,").map_err(|e| e.to_string())?;
        writeln!(
            out,
            "        \"tt_metal/tt-llk/tt_llk_quasar/instructions/assembly.yaml#{}\",",
            inst.name
        )
        .map_err(|e| e.to_string())?;
        writeln!(out, "    );\n").map_err(|e| e.to_string())?;
    }

    writeln!(
        out,
        "}}\n\n/// Every Quasar instruction encoding, for table-wide checks.\n\
         pub static ALL: &[&InstructionDef] = &["
    )
    .map_err(|e| e.to_string())?;
    for inst in instructions {
        writeln!(out, "    &defs::{},", inst.name).map_err(|e| e.to_string())?;
    }
    writeln!(
        out,
        "];\n\n/// One encoder per instruction, taking its operands from the high bits of\n\
         /// the word down.\npub mod encode {{\n    use super::*;\n{}\n",
        BUILDER
    )
    .map_err(|e| e.to_string())?;

    for inst in instructions {
        let args: Vec<String> = inst.args.iter().map(|arg| snake(&arg.name)).collect();
        let encoder = snake(&inst.name);
        if args.len() <= BUILDER_AFTER {
            let params = args
                .iter()
                .map(|arg| format!("{arg}: u32"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                out,
                "    /// `{name}`.\n    pub const fn {encoder}({params}) -> Result<Instruction, EncodeError> {{\n        Instruction::with_operands(&defs::{name}, &[{values}])\n    }}\n",
                name = inst.name,
                values = args.join(", ")
            )
            .map_err(|e| e.to_string())?;
        } else {
            let builder = camel(&inst.name);
            writeln!(
                out,
                "    builder!(\n        #[doc = \"`{name}`.\"]\n        {builder},\n        defs::{name},\n        [{fields}]\n    );\n",
                name = inst.name,
                fields = args.join(", ")
            )
            .map_err(|e| e.to_string())?;
        }
    }

    writeln!(out, "}}").map_err(|e| e.to_string())?;
    Ok(out)
}

const BUILDER: &str = r#"
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
"#;

fn strip_comment(line: &str) -> &str {
    match line.find('#') {
        Some(index) => &line[..index],
        None => line,
    }
}

/// Parse the flexible spellings the LLK table uses for a list item:
/// `- name: dst`, `- name : dst` and `-  name: dst`.
fn arg_name(trimmed: &str) -> Option<&str> {
    let rest = trimmed.strip_prefix('-')?.trim_start();
    let rest = rest.strip_prefix("name")?.trim_start();
    rest.strip_prefix(':').map(str::trim)
}

fn parse_u8(text: &str) -> Result<u8, String> {
    let text = text.trim();
    let value = if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16)
    } else {
        text.parse::<u32>()
    }
    .map_err(|e| format!("`{text}` is not an integer: {e}"))?;
    u8::try_from(value).map_err(|_| format!("`{text}` does not fit in a byte"))
}

fn is_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn snake(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (index, c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev = index.checked_sub(1).and_then(|i| chars.get(i)).copied();
            let next = chars.get(index + 1).copied();
            let prev_lower = prev.is_some_and(|p| p.is_ascii_lowercase());
            let prev_digit = prev.is_some_and(|p| p.is_ascii_digit());
            let prev_upper = prev.is_some_and(|p| p.is_ascii_uppercase());
            let next_lower = next.is_some_and(|n| n.is_ascii_lowercase());
            if (prev_lower || prev_digit || (prev_upper && next_lower)) && !out.ends_with('_') {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(*c);
        }
    }
    let out = out.to_ascii_lowercase();
    if is_keyword(&out) {
        format!("{out}_")
    } else {
        out
    }
}

/// Rust keywords cannot name the generated builder fields or encoder functions.
fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "gen"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}

fn camel(s: &str) -> String {
    snake(s)
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
