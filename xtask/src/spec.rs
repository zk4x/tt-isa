// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Locating and verifying the ISA specification tree.
//!
//! The specification is a pinned *input*: bumping it invalidates everything the
//! tables say until they are regenerated. The pin is enforced over the content the
//! generator reads, not over the tarball: GitHub does not promise archive bytes are
//! stable, and a content digest can say *which file* moved.

use std::path::{Path, PathBuf};

use crate::pin;
use crate::util::sha256;

/// The machine-readable bit-layout source every encoding diagram is rendered from.
pub const BITS32: &str = "Diagrams/Src/Bits32.lua";

/// A local checkout of the specification, for working against edits. It is not an
/// escape hatch: the tree still has to match the pinned digest.
pub const ROOT_ENV: &str = "TT_ISA_DOCS";

/// Directories whose `.md` files the generator reads. Every instruction page lives
/// in a `TensixCoprocessor` directory.
const MARKDOWN_ROOTS: &[&str] = &[
    "BlackholeA0/TensixTile/TensixCoprocessor",
    "WormholeB0/TensixTile/TensixCoprocessor",
];

/// Files outside [`MARKDOWN_ROOTS`] that are hashed with everything else: the two
/// `Atomics.md` pages carry the `NOC_AT_LEN_BE_*` encodings, and `fma.c` is the
/// numerics oracle of a hand port.
const EXTRA_FILES: &[&str] = &[
    "BlackholeA0/NoC/Atomics.md",
    "WormholeB0/NoC/Atomics.md",
    "Miscellaneous/FMA/fma.c",
];

pub fn vendored_root() -> PathBuf {
    crate::util::workspace_root()
        .join("vendor")
        .join("tt-isa-documentation")
}

/// The verified specification tree, whichever of the two it is.
pub fn root() -> Result<PathBuf, String> {
    let (root, source) = match std::env::var_os(ROOT_ENV) {
        Some(path) => (PathBuf::from(path), ROOT_ENV),
        None => (vendored_root(), "vendor/tt-isa-documentation"),
    };
    if !root.is_dir() {
        return Err(format!(
            "{} does not exist. Run `cargo xtask fetch-spec`.",
            root.display()
        ));
    }
    let got = digest(&root)?;
    if got != pin::SPEC_CONTENT_SHA256 {
        return Err(format!(
            "{} ({source}) hashes {got}, not the pinned {}.\nIt is not the revision this \
             table was generated from; run `cargo xtask fetch-spec`.",
            root.display(),
            pin::SPEC_CONTENT_SHA256
        ));
    }
    Ok(root)
}

/// Every file the generator reads, sorted, relative to the tree root.
///
/// Sorted because the digest is order-sensitive and directory iteration order is
/// not. A missing directory is an error: silently hashing nothing would make the
/// pin vacuous.
pub fn consumed_files(root: &Path) -> Result<Vec<String>, String> {
    let mut files = vec![BITS32.to_string()];
    files.extend(EXTRA_FILES.iter().map(|f| (*f).to_string()));
    for dir in MARKDOWN_ROOTS {
        let base = root.join(dir);
        if !base.is_dir() {
            return Err(format!(
                "{} is not a directory — is {} really a tt-isa-documentation checkout?",
                base.display(),
                root.display()
            ));
        }
        walk(&base, dir, &mut files)?;
    }
    for rel in &files {
        if !root.join(rel).is_file() {
            return Err(format!(
                "{} is missing from the specification tree",
                root.join(rel).display()
            ));
        }
    }
    files.sort();
    Ok(files)
}

fn walk(dir: &Path, rel: &str, out: &mut Vec<String>) -> Result<(), String> {
    for entry in std::fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("reading {}: {e}", dir.display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let child = format!("{rel}/{name}");
        if entry.path().is_dir() {
            walk(&entry.path(), &child, out)?;
        } else if name.ends_with(".md") {
            out.push(child);
        }
    }
    Ok(())
}

/// SHA-256 over every consumed file: its relative path, its length and its bytes,
/// in sorted order. Lengths are included so no rearrangement between two files can
/// produce the same digest.
pub fn digest(root: &Path) -> Result<String, String> {
    let mut buffer = Vec::new();
    for rel in consumed_files(root)? {
        let contents = std::fs::read(root.join(&rel))
            .map_err(|e| format!("reading {}: {e}", root.join(&rel).display()))?;
        buffer.extend_from_slice(rel.as_bytes());
        buffer.push(0);
        buffer.extend_from_slice(contents.len().to_string().as_bytes());
        buffer.push(0);
        buffer.extend_from_slice(&contents);
        buffer.push(0);
    }
    sha256(&buffer)
}
