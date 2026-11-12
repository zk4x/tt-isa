# Changes from upstream metalium-rs

The base is `Knight-Ops/metalium-rs` at `b22a245` (`crates/tt-isa`).

## Unreleased — 2026-11-12

- Deleted `tile` (1,362 lines) after the L1 tile library and every FP16/BF16
  conversion in it turned out to have no caller: `TileImage`, the `bfp*`/`fp*`
  conversion family and the `TileDescriptor` builder chain. The four-word
  unpacker descriptor it was built around lives in the consumer now, laid out as
  tt-metal's `unpack_tile_descriptor_t`, and the `InDataFormat` codes
  (`MEASURED`, ttsim-divergence rows G/H) travel with them.
- Deleted the modules nothing in this repository or in skamandros imports:
  `matrix` (1,025 lines), `matrix::debug` (1,278), `sfpu::load_macro` (1,520),
  `scalar` (244), `scalar::atomic` (589), `scalar::mmio` (306), `numerics`
  (523), `numerics::sfpu` (354), `numerics::stochastic` (81), `hazard` (563),
  `packer` (395), `unpacker` (176), `adc` (338), `sync::mutex` (204),
  `noc::atomic` (389) and `noc::multicast` (485). 8,478 lines, 44,719 → 36,241.
- `noc::multicast::clear_outstanding` was the one thing used from those 16 files
  (by `tt-firmware`'s `noc_probe_b`); it moves to `noc::clear_outstanding`.

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
