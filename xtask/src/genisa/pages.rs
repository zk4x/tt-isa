// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Which diagrams each specification page embeds.
//!
//! Every page that documents an encoding embeds its diagram —
//! `![](../../../Diagrams/Out/Bits32_<KEY>.svg)` — so the page a diagram belongs to
//! is a fact, not a naming convention. That mapping is what says which chip a layout
//! is evidence for, and the macro name in the page's `## Syntax` block is the
//! instruction's mnemonic.
//!
//! The `TT_*(…)` templates are no longer cross-checked against the diagrams here:
//! that comparison is worth having, but it lives in `tt-isa`'s own gates rather than
//! in a table generator that must stay short.

use std::path::Path;

use crate::spec;

/// One page of the specification that embeds at least one diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    /// Path relative to the specification root.
    pub path: String,
    /// Diagram keys, in the order the page embeds them.
    pub keys: Vec<String>,
    /// Macro names from the `## Syntax` block, in order, without the `TT_` prefix.
    pub macros: Vec<String>,
}

/// A Blackhole page that embeds no diagram.
///
/// Either it delegates to the Wormhole tree — "This document is shared with the
/// Wormhole tree; see …" — or it is not about an encoding at all.
pub struct Bare {
    pub stem: String,
    pub delegates: bool,
}

pub fn scan(root: &Path) -> Result<(Vec<Page>, Vec<Bare>), String> {
    let mut pages = Vec::new();
    let mut bare = Vec::new();
    for rel in spec::consumed_files(root)? {
        if !rel.ends_with(".md") {
            continue;
        }
        let text =
            std::fs::read_to_string(root.join(&rel)).map_err(|e| format!("reading {rel}: {e}"))?;
        let keys = embedded(&text);
        if keys.is_empty() {
            if rel.starts_with("BlackholeA0/") {
                bare.push(Bare {
                    stem: stem(&rel).to_string(),
                    delegates: text.contains("This document is shared with the Wormhole tree"),
                });
            }
            continue;
        }
        let macros = syntax_block(&text).map(macro_names).unwrap_or_default();
        pages.push(Page {
            path: rel,
            keys,
            macros,
        });
    }
    Ok((pages, bare))
}

fn stem(path: &str) -> &str {
    path.rsplit('/')
        .next()
        .and_then(|f| f.strip_suffix(".md"))
        .unwrap_or(path)
}

/// Diagram keys the page embeds, in order.
fn embedded(source: &str) -> Vec<String> {
    const OPEN: &str = "Diagrams/Out/Bits32_";
    let mut keys = Vec::new();
    let mut rest = source;
    while let Some(i) = rest.find(OPEN) {
        rest = &rest[i + OPEN.len()..];
        if let Some(j) = rest.find(".svg") {
            keys.push(rest[..j].to_string());
            rest = &rest[j..];
        }
    }
    keys
}

/// The fenced block directly under `## Syntax`.
///
/// Scoped to that heading: `TT_*(…)` also appears in prose — `SETC16.md` discusses
/// `TT_SETC16(CFG_STATE_REG_x, x)` in its notes — and a whole-file search would take
/// those for encodings.
fn syntax_block(source: &str) -> Option<&str> {
    let heading = source
        .match_indices("\n## Syntax")
        .map(|(i, _)| i + 1)
        .next()?;
    let after = &source[heading..];
    let fence = after.find("```")?;
    let body = &after[fence..];
    let start = body.find('\n')? + 1;
    let end = body[start..].find("```")?;
    Some(&body[start..start + end])
}

/// The macro name each call in the block invokes.
fn macro_names(block: &str) -> Vec<String> {
    block
        .lines()
        .filter(|line| {
            line.starts_with("TT_") || line.starts_with("TTI_") || line.starts_with("TT_OP_")
        })
        .filter_map(|line| {
            let head = line.split('(').next().unwrap_or(line).trim();
            let name = ["TT_OP_", "TTI_", "TT_"]
                .iter()
                .find_map(|prefix| head.strip_prefix(prefix))?;
            let name = name.trim();
            (!name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
                .then(|| name.to_string())
        })
        .collect()
}
