# Changes from upstream metalium-rs

The base is `Knight-Ops/metalium-rs` at `b22a245` (`crates/tt-isa`).

## 0.1.0 — 2026-11-08

- Forked from upstream to publish the crate standalone (the upstream path
  dependency blocked downstream publication).
- `Cargo.toml` no longer inherits from a workspace: version `0.1.0`, edition
  `2024`, `rust-version = 1.85`, `license = "Apache-2.0"`.
- `LICENSE` is the Apache-2.0 text from the repository's initial commit.
- `src/dm/encoded.rs` (fuzz tests): the `#[ignore]`d million-case test no longer
  calls `std::env::set_var` (unsafe in edition 2024, and this crate forbids
  unsafe code); the iteration count override is a process-wide atomic instead.
  No library code changed.
