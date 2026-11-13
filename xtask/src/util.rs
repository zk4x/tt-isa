// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Paths, hashing, formatting and downloading: everything the two generators share.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The repository root. `xtask` sits directly below it.
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives directly below the workspace root")
        .to_path_buf()
}

pub fn sha256(bytes: &[u8]) -> Result<String, String> {
    use std::io::Write;

    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("could not run sha256sum: {e}"))?;
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(bytes)
        .map_err(|e| format!("writing to sha256sum: {e}"))?;
    let out = child
        .wait_with_output()
        .map_err(|e| format!("waiting for sha256sum: {e}"))?;
    if !out.status.success() {
        return Err("sha256sum failed".into());
    }
    String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next()
        .map(str::to_string)
        .ok_or_else(|| "sha256sum produced no output".to_string())
}

/// Run generated source through rustfmt.
///
/// The output is formatted rather than emitted rustfmt-clean by hand: guessing
/// where rustfmt would wrap a 140-character field list is a losing game, and doing
/// it this way means `--check` compares two things produced identically.
pub fn rustfmt(source: &str, root: &Path) -> Result<String, String> {
    use std::io::Write;

    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout", "--quiet"])
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("could not run rustfmt: {e}"))?;
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(source.as_bytes())
        .map_err(|e| format!("writing to rustfmt: {e}"))?;
    let out = child
        .wait_with_output()
        .map_err(|e| format!("waiting for rustfmt: {e}"))?;
    if !out.status.success() {
        return Err("rustfmt rejected the generated source".into());
    }
    String::from_utf8(out.stdout).map_err(|e| format!("rustfmt produced invalid UTF-8: {e}"))
}

/// Download `url` to `path` via `path.partial`, so an interrupted fetch cannot
/// leave a truncated file that later hashes as "the wrong version".
pub fn download(url: &str, path: &Path) -> Result<(), String> {
    println!("fetching {url}");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("creating {}: {e}", parent.display()))?;
    }
    let partial = path.with_extension("partial");
    let status = Command::new("curl")
        .args([
            "--fail",
            "--location",
            "--silent",
            "--show-error",
            "--max-time",
            "300",
        ])
        .arg("-o")
        .arg(&partial)
        .arg(url)
        .status()
        .map_err(|e| format!("could not run curl: {e}"))?;
    if !status.success() {
        let _ = std::fs::remove_file(&partial);
        return Err(format!("curl failed downloading {url}"));
    }
    std::fs::rename(&partial, path).map_err(|e| format!("renaming into place: {e}"))
}

/// Write `contents` to `path`, or report that the committed copy differs.
///
/// The one place both generators end, so `--check` cannot drift between them.
pub fn commit(path: &Path, contents: &str, check_only: bool, source: &str) -> Result<(), String> {
    if check_only {
        let current = std::fs::read_to_string(path)
            .map_err(|e| format!("reading {}: {e}", path.display()))?;
        if current == contents {
            println!(
                "ok: {} is up to date with the pinned {source}",
                path.display()
            );
            return Ok(());
        }
        return Err(format!(
            "{} is out of date with the pinned {source}.\nRun the same task without \
             --check and commit the result.",
            path.display()
        ));
    }
    std::fs::write(path, contents).map_err(|e| format!("writing {}: {e}", path.display()))?;
    println!("wrote {} ({} bytes)", path.display(), contents.len());
    Ok(())
}
