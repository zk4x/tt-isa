// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Fetching the two pinned inputs.
//!
//! One is a whole repository (the specification tree), the other a single header.
//! Both are verified on arrival: the generator must never read a tree it cannot
//! name the revision of.

use std::path::Path;

use crate::pin;
use crate::spec;
use crate::util::{download, sha256, workspace_root};

/// Ensure the specification tree is present and matches the pin.
pub fn spec_if_needed() -> Result<(), String> {
    if spec::root().is_ok() {
        return Ok(());
    }
    spec(true)
}

/// Download the pinned specification tree into `vendor/` and verify it.
///
/// Extracts into a staging directory and swaps it in, so a failed extraction
/// cannot leave a half-populated tree the digest check would then have to diagnose.
pub fn spec(force: bool) -> Result<(), String> {
    let dest = spec::vendored_root();
    if dest.is_dir() && !force {
        let got = spec::digest(&dest)?;
        if got == pin::SPEC_CONTENT_SHA256 {
            println!("{} is already present and matches the pin", dest.display());
            return Ok(());
        }
        return Err(format!(
            "{} hashes {got}, not the pinned {}.\nRe-run with --force to replace it.",
            dest.display(),
            pin::SPEC_CONTENT_SHA256
        ));
    }

    let vendor = workspace_root().join("vendor");
    let tarball = vendor.join("tt-isa-documentation.tar.gz");
    let url = format!(
        "https://github.com/tenstorrent/tt-isa-documentation/archive/{}.tar.gz",
        pin::SPEC_REV
    );
    download(&url, &tarball)?;

    let staging = vendor.join("tt-isa-documentation.staging");
    let _ = std::fs::remove_dir_all(&staging);
    std::fs::create_dir_all(&staging)
        .map_err(|e| format!("creating {}: {e}", staging.display()))?;
    let status = std::process::Command::new("tar")
        .arg("-xzf")
        .arg(&tarball)
        .arg("-C")
        .arg(&staging)
        // The archive has a single `tt-isa-documentation-<rev>/` top level.
        .args(["--strip-components", "1"])
        .status()
        .map_err(|e| format!("could not run tar: {e}"))?;
    if !status.success() {
        return Err(format!("tar failed extracting {}", tarball.display()));
    }

    let got = spec::digest(&staging)?;
    if got != pin::SPEC_CONTENT_SHA256 {
        return Err(format!(
            "content digest mismatch for the specification tree:\n  expected {}\n  got      \
             {got}\nThe pinned revision is immutable, so this means a corrupted download.",
            pin::SPEC_CONTENT_SHA256
        ));
    }

    let _ = std::fs::remove_dir_all(&dest);
    std::fs::rename(&staging, &dest).map_err(|e| format!("renaming into place: {e}"))?;
    let _ = std::fs::remove_file(&tarball);
    println!("wrote {} ({})", dest.display(), pin::SPEC_REV);
    Ok(())
}

/// Ensure `cfg_defines.h` is present and matches the pin.
pub fn cfg_defines_if_needed() -> Result<(), String> {
    let path = workspace_root().join("vendor").join("cfg_defines.h");
    if path.exists() {
        let bytes = std::fs::read(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
        if sha256(&bytes)? == pin::CFG_DEFINES_SHA256 {
            return Ok(());
        }
    }
    cfg_defines(true)
}

/// Download the pinned Blackhole `cfg_defines.h` into `vendor/`.
pub fn cfg_defines(force: bool) -> Result<(), String> {
    let dest = workspace_root().join("vendor").join("cfg_defines.h");
    if dest.exists() && !force {
        let bytes = std::fs::read(&dest).map_err(|e| format!("reading {}: {e}", dest.display()))?;
        let got = sha256(&bytes)?;
        if got == pin::CFG_DEFINES_SHA256 {
            println!("{} is already present and matches the pin", dest.display());
            return Ok(());
        }
        return Err(format!(
            "{} hashes {got}, not the pinned {}.\nRe-run with --force to replace it.",
            dest.display(),
            pin::CFG_DEFINES_SHA256
        ));
    }
    let url = format!(
        "https://raw.githubusercontent.com/tenstorrent/tt-metal/{}/tt_metal/hw/inc/blackhole/cfg_defines.h",
        pin::TT_METAL_REV
    );
    download(&url, &dest)?;
    verify(&dest)?;
    Ok(())
}

fn verify(path: &Path) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let got = sha256(&bytes)?;
    if got != pin::CFG_DEFINES_SHA256 {
        let _ = std::fs::remove_file(path);
        return Err(format!(
            "{} hashes {got}, not the pinned {}: the download is corrupted",
            path.display(),
            pin::CFG_DEFINES_SHA256
        ));
    }
    Ok(())
}
