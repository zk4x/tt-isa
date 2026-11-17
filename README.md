# tt-isa

## Provenance and credit

This crate is the `tt-isa` crate of **metalium-rs**, written by its authors:

- https://github.com/Knight-Ops/metalium-rs
- original crate: `crates/tt-isa`

It is published here as a standalone crate so that projects can depend on it
from crates.io. All credit for the design, the generated encodings, the
documentation and the tests belongs to the metalium-rs authors. This repository
is a fork maintained by **zk4x** (https://github.com/zk4x), the author of zyx
(https://github.com/zk4x/zyx); it exists because the upstream crate is not
published and a path dependency blocks publication of downstream crates.

The upstream authors were asked on 2026-11-04 (issue
https://github.com/Knight-Ops/metalium-rs/issues/31) whether they want to
continue the project, publish the crate themselves, or accept a fork. If they
ask for this crate, ownership of the crates.io package will be transferred to
them; the name is not claimed against them.

License: Apache-2.0 (see `LICENSE`), the same as upstream. Every source file
keeps its `SPDX-License-Identifier: Apache-2.0` header. Changes made in this
fork are recorded in `CHANGES.md`.

## What it is

Blackhole hardware definitions shared by the host and the firmware: instruction
encoders, register and memory maps, coordinate spaces, and the host/firmware
protocols. It also carries the Quasar instruction encodings, generated from
tt-metal's Quasar LLK table; the rest of the Quasar port lives in its consumers.
`no_std`, `#![forbid(unsafe_code)]`, zero dependencies, because it is compiled
for both the host and `riscv32im-unknown-none-elf`. Anything that cannot build
for both does not belong here.

## Use

```bash
cargo add tt-isa
```

```rust
use tt_isa::isa::blackhole::encode;

// `MOVA2D` with `Move8Rows`: the word a kernel pushes to the instruction FIFO.
let word = encode::Mova2D::ZERO.move8_rows(1).encode().unwrap().word();
assert_eq!(word >> 24, 0x12);

// The fixed L1 regions, which a kernel is placed inside rather than at.
println!("data arena {:#x}..{:#x}", tt_isa::l1::DATA.base, tt_isa::l1::DATA.end);
```

Quasar instructions have their own table and the same bit-level encoder:

```rust
use tt_isa::isa::quasar::encode as quasar;

let word = quasar::setrwc(0, 0, 0, 1).unwrap().word();
assert_eq!(word, 0x3700_0001);
```

## Modules

| Module | What it holds |
|---|---|
| `isa` | The Tensix instruction set. `isa::blackhole` is produced by `cargo xtask gen-isa`; each encoding carries a `Provenance` (Blackhole, shared, Wormhole-only `UNVERIFIED`, measured, or Quasar). Wormhole forms are in `isa::blackhole::defs::wormhole`. `isa::quasar` is produced separately by `cargo xtask gen-quasar` from tt-metal's Quasar LLK `assembly.yaml`. |
| `cfg` | Backend-configuration fields, one table per architecture: `cfg::blackhole` and `cfg::quasar`, both produced by `cargo xtask gen-cfg` from their pinned `cfg_defines.h`. |
| `backend`, `sfpu`, `sync` | Hand-written layers over the encodings: checked config reads/mutations/writes and unit waits, SFPU encodings, Tensix semaphores. |
| `noc` | NoC coordinate spaces (typed per NoC), NIU registers, `noc::grid::Tensix` (the surviving columns), `noc::probe` (the probe image's protocol) and `noc::read_loop` (the mover's read loop). |
| `tensix` | Tile memory map, baby RISC-V cores (`Core`), reset PCs, soft reset. |
| `arc` | The ARC telemetry tags: how to ask a chip what it is (harvesting, translation). |
| `dram` | GDDR6 channels (`Dram`, `DramRange`); only a chip's grid can produce a range. |
| `eth` | Ethernet tile register map, firmware-owned L1, and the `eth::mover` protocol. |
| `mailbox` | The L1 contract between host and firmware: status words, role mailboxes, program region. `mailbox::guard` is the role-side deadline, blocked status and host release that make a blocking instruction safe to run. |
| `tag_search` | The L1 tag-search accelerator's registers and helpers (host and ttsim only: armed trigger loads hang the core on silicon). |
| `dm` | The RISCV B data mover protocol: descriptors, list entries (`op::READ`, `WRITE`, `LIST`, `COMPUTE`, `KERNEL`, `WAIT`, ...), element-wise `kind`s. |
| `dm::record` | Op records (`GATHER`, `SCATTER`, `ELTWISE`, `SUM`): a whole op in a few list entries, `expand`ed on the tile and, first, on the host. |
| `l1` | The fixed L1 region map (`REGIONS`: images, mover list, data arena, mailboxes, program slots, trace, program cache), checked at compile time. Kernels never address `l1::DATA` directly; the compiler places buffers there. |

## Test

```bash
cargo test                   # in the crate
cargo test -p tt-isa         # in this repository's workspace
```

`tests/llk_crosscheck.rs` checks the generated Blackhole encodings against
tt-metal LLK's Blackhole `TT_OP_*` macros. The Quasar table is checked against a
few Quasar LLK macros in `src/isa/mod.rs`, and `cargo xtask gen-quasar --check`
keeps the committed table in step with the pinned `assembly.yaml`.

Measured BFP8/BFP4/BFP2 format codes are 6/7/15. `TileImage` describes headers,
shared exponents and sub-byte datums; native encoding lives in `tt-kernels`
rather than a shipping host arithmetic fallback. `backend::write_prng_seed`
is a checked diagnostic WRCFG sequence: writing the register does not restart
silicon's Vector Unit stream. The separately gated RISC-V store path with a
settling interval does restart it.

## Gotchas

- Never edit `src/isa/blackhole.rs`, `src/isa/quasar.rs`,
  `src/cfg/blackhole.rs` or `src/cfg/quasar.rs`. Regenerate with
  `cargo xtask gen-isa` / `gen-quasar` /
  `gen-cfg` in this repository (`xtask` is not in the published package, which
  ships the generated Rust and needs nothing else); the generators fetch their
  pinned input into `vendor/` themselves, and `--check` fails instead of
  rewriting when the committed file is out of date.
- The Quasar table records the LLK source, not silicon evidence. The LLK table
  does not mark signed fields, so they are all `Field::signed == false`; the bit
  pattern is still exact.
- A fact from a Wormhole page is a hypothesis on Blackhole. Several Matrix Unit
  fields sit one bit lower on Blackhole than Wormhole draws them; those are measured
  layouts in `xtask/src/genisa/Bits32_BH.lua`.
- Constants cite their specification page. Keep that when adding one.
