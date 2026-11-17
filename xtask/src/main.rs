// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! The three generated tables: `src/isa/blackhole.rs`, `src/cfg/blackhole.rs`
//! and `src/isa/quasar.rs`.
//!
//! All are committed rather than produced by a build script, so that `tt-isa`
//! stays dependency-free and buildable for `riscv32im` with nothing but rustc.
//! That makes regeneration a command someone runs, and `--check` the gate that
//! keeps the committed file honest.
//!
//! Deliberately dependency-free: it shells out to `curl`, `tar`, `sha256sum` and
//! `rustfmt` for a handful of calls.

use std::process::ExitCode;

mod fetch;
mod gencfg;
mod genisa;
mod pin;
mod quasar;
mod spec;
mod util;

const USAGE: &str = "\
usage: cargo xtask <task>

tasks:
  gen-isa [--check]   regenerate the instruction table from the pinned Bits32.lua;
                      --check fails instead of rewriting when the file is out of date
  gen-cfg [--check]   regenerate both configuration-field tables (Blackhole's and
                      Quasar's) from their pinned cfg_defines.h; --check as above
  gen-quasar [--check]
                      regenerate the Quasar instruction table from the pinned
                      tt-metal Quasar LLK assembly.yaml; --check as above
  fetch-spec          download the pinned specification tree into vendor/
  fetch-cfg           download the pinned cfg_defines.h into vendor/
  fetch-quasar        download the pinned Quasar LLK assembly.yaml into vendor/

All generators fetch their pinned input when it is missing or does not match
its digest, so a fresh clone needs no setup step.";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let task = args.next();
    let check = args.any(|a| a == "--check");
    let result = match task.as_deref() {
        Some("gen-isa") => genisa::generate(check),
        Some("gen-cfg") => gencfg::generate(check).and_then(|()| gencfg::generate_quasar(check)),
        Some("gen-quasar") => quasar::generate(check),
        Some("fetch-spec") => fetch::spec(true),
        Some("fetch-cfg") => fetch::cfg_defines(true),
        Some("fetch-quasar") => fetch::quasar_assembly(true),
        Some(other) => Err(format!("unknown task `{other}`\n\n{USAGE}")),
        None => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("xtask: {message}");
            ExitCode::FAILURE
        }
    }
}
