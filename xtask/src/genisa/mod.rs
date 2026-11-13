// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! `Diagrams/Src/Bits32.lua` -> `tt_isa::isa::generated`.
//!
//! The specification describes the instruction set twice: as a drawing script, and
//! as a `TT_*(…)` macro template on each instruction's page. Only the drawing is
//! read here. The templates are no longer parsed at all — the comparison they
//! offered is worth having, but it is a check, not a source, and keeping it would
//! have meant a C-expression parser inside a table generator.

use std::collections::{BTreeMap, BTreeSet};

use crate::fetch;
use crate::pin;
use crate::spec;
use crate::util;

pub mod lua;
pub mod measured;
pub mod pages;
pub mod provenance;
pub mod render;

use provenance::Provenance;

/// Everything the generator needs, parsed and checked.
pub struct Sources {
    pub diagrams: Vec<lua::Diagram>,
    pub provenance: BTreeMap<String, (Provenance, String)>,
    /// Diagram key -> the `TT_*` macro name its page documents.
    pub mnemonics: BTreeMap<String, String>,
    /// Diagram key -> every macro name on its page. Almost always one; `RMWCIB` has
    /// four, because the digit is part of its opcode.
    pub variants: BTreeMap<String, Vec<String>>,
}

/// Fetch the pinned specification if it is not in place, then read it.
pub fn load() -> Result<Sources, String> {
    fetch::spec_if_needed()?;
    let root = spec::root()?;
    let bits32 = std::fs::read_to_string(root.join(spec::BITS32))
        .map_err(|e| format!("reading {}: {e}", root.join(spec::BITS32).display()))?;

    let unused = lua::unused(&bits32);
    if !unused.is_empty() {
        return Err(format!(
            "SPECIAL_LABELS has entries nothing uses: {}.\nAn exception that is no longer \
             needed means the specification changed shape somewhere nobody looked.",
            unused.join(", ")
        ));
    }
    let mut diagrams = lua::dedupe(lua::parse(&bits32)?)?;
    lua::structure(&diagrams)?;

    let (pages, bare) = pages::scan(&root)?;
    let embedded: BTreeSet<&str> = pages
        .iter()
        .flat_map(|p| p.keys.iter().map(String::as_str))
        .collect();
    let unembedded: Vec<&str> = diagrams
        .iter()
        .map(|d| d.key.as_str())
        .filter(|key| !embedded.contains(key))
        .collect();
    if !unembedded.is_empty() {
        return Err(format!(
            "Bits32.lua defines diagrams no page embeds: {}.\nThe page is what says which chip the \
             encoding applies to, so a diagram without one has no provenance.",
            unembedded.join(", ")
        ));
    }
    if let Some(undefined) = embedded
        .iter()
        .find(|key| !diagrams.iter().any(|d| d.key == **key))
    {
        return Err(format!(
            "a page embeds Bits32_{undefined}.svg, which Bits32.lua does not define"
        ));
    }

    let mut provenance = provenance::classify(&pages, &bare)?;
    let mut mnemonics = BTreeMap::new();
    let mut variants: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for page in &pages {
        for (key, mnemonic) in page.keys.iter().zip(&page.macros) {
            mnemonics
                .entry(key.clone())
                .or_insert_with(|| mnemonic.clone());
        }
        // A page with one diagram and several macros documents several encodings of
        // it. Only `RMWCIB` does this, and `render` insists on that.
        if page.keys.len() == 1 && page.macros.len() > 1 {
            variants
                .entry(page.keys[0].clone())
                .or_insert_with(|| page.macros.clone());
        }
    }

    // Measured Blackhole layouts join last, after the specification has been checked
    // on its own terms: they are not part of it and must not mask a disagreement
    // inside it.
    measured::apply(
        measured::SOURCE,
        measured::MEASURED,
        &mut diagrams,
        &mut provenance,
        &mut mnemonics,
    )?;
    measured::confirm(measured::CONFIRMED, &mut provenance)?;
    lua::structure(&diagrams)?;

    Ok(Sources {
        diagrams,
        provenance,
        mnemonics,
        variants,
    })
}

/// Regenerate `src/isa/generated.rs`.
pub fn generate(check_only: bool) -> Result<(), String> {
    let sources = load()?;
    let rendered = render::render(&render::Input {
        diagrams: &sources.diagrams,
        provenance: &sources.provenance,
        mnemonics: &sources.mnemonics,
        variants: &sources.variants,
        spec_rev: pin::SPEC_REV,
    })?;
    let root = util::workspace_root();
    let generated = util::rustfmt(&rendered, &root)?;
    util::commit(
        &root.join("src/isa/generated.rs"),
        &generated,
        check_only,
        "Bits32.lua",
    )
}
