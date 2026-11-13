// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Which chip a bit layout is evidence for.
//!
//! The Blackhole documentation is a delta over Wormhole's, so a fact drawn only from
//! a Wormhole page is a hypothesis until silicon says otherwise. The specification
//! already answers which tree a diagram lives in, so the classification is a
//! measurement rather than a `_BH` naming convention.

use std::collections::{BTreeMap, BTreeSet};

use super::pages::{Bare, Page};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provenance {
    /// A Blackhole page embeds this diagram. Authoritative.
    Blackhole,
    /// Only a Wormhole page embeds it, but the Blackhole tree has the same page
    /// stating the document is shared and the behaviour identical.
    SharedWithWormhole,
    /// Only a Wormhole page embeds it, and the Blackhole page of the same name
    /// embeds a *different* diagram. This layout is Wormhole's.
    SupersededOnBlackhole { by: String },
    /// Only a Wormhole page embeds it, and Blackhole has no such page at all.
    /// `UNVERIFIED` until silicon or the simulator says otherwise.
    WormholeOnly,
    /// Not from the specification: a Blackhole layout measured on ttsim or silicon,
    /// replacing a `WormholeOnly` diagram whose `moved` fields sit elsewhere.
    Measured {
        evidence: String,
        moved: Vec<String>,
        dropped: Vec<String>,
        widened: Vec<String>,
    },
    /// Not from the specification: a `WormholeOnly` layout a gate ran on Blackhole and
    /// found to be the Wormhole layout unchanged.
    Confirmed { evidence: String },
}

impl Provenance {
    pub fn variant_name(&self) -> &'static str {
        match self {
            Provenance::Blackhole => "Blackhole",
            Provenance::SharedWithWormhole => "SharedWithWormhole",
            Provenance::SupersededOnBlackhole { .. } => "SupersededOnBlackhole",
            Provenance::WormholeOnly => "WormholeOnly",
            Provenance::Measured { .. } => "Measured",
            Provenance::Confirmed { .. } => "Confirmed",
        }
    }
}

const BLACKHOLE: &str = "BlackholeA0/";

/// Classify every diagram key, and name the page it is best documented on.
pub fn classify(
    pages: &[Page],
    bare: &[Bare],
) -> Result<BTreeMap<String, (Provenance, String)>, String> {
    let mut blackhole_page: BTreeMap<&str, &Page> = BTreeMap::new();
    let mut wormhole_page: BTreeMap<&str, &Page> = BTreeMap::new();
    // Blackhole pages that embed a diagram, by stem: these are what supersede a
    // Wormhole encoding.
    let mut by_stem: BTreeMap<&str, &Page> = BTreeMap::new();

    for page in pages {
        let is_blackhole = page.path.starts_with(BLACKHOLE);
        if is_blackhole {
            by_stem.insert(stem(&page.path), page);
        }
        for key in &page.keys {
            let table = if is_blackhole {
                &mut blackhole_page
            } else {
                &mut wormhole_page
            };
            table.entry(key.as_str()).or_insert(page);
        }
    }

    let delegating: BTreeSet<&str> = bare
        .iter()
        .filter(|p| p.delegates)
        .map(|p| p.stem.as_str())
        .collect();

    let keys: BTreeSet<&str> = blackhole_page
        .keys()
        .chain(wormhole_page.keys())
        .copied()
        .collect();
    let mut out = BTreeMap::new();
    for key in keys {
        if let Some(page) = blackhole_page.get(key) {
            out.insert(key.to_string(), (Provenance::Blackhole, page.path.clone()));
            continue;
        }
        let page = wormhole_page[key];
        let s = stem(&page.path);
        // The Blackhole page with the same name as the Wormhole one decides between
        // the three remaining cases.
        let provenance = if let Some(blackhole) = by_stem.get(s) {
            Provenance::SupersededOnBlackhole {
                by: blackhole.keys[0].clone(),
            }
        } else if delegating.contains(s) {
            Provenance::SharedWithWormhole
        } else {
            Provenance::WormholeOnly
        };
        out.insert(key.to_string(), (provenance, page.path.clone()));
    }
    Ok(out)
}

fn stem(path: &str) -> &str {
    path.rsplit('/')
        .next()
        .and_then(|f| f.strip_suffix(".md"))
        .unwrap_or(path)
}
