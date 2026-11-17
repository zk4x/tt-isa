// Copyright (C) 2025 zk4x
// SPDX-License-Identifier: Apache-2.0

//! Pinned upstream revisions.
//!
//! A pinned input that drifted would silently change what the tables say, so each
//! one is fetched by revision and then verified by digest. Bumping either half of
//! a pin without the other is a loud failure, not a new table.

/// The ISA specification commit (`tenstorrent/tt-isa-documentation`).
pub const SPEC_REV: &str = "f848eb668c2aeae742a88a49a86157e24a0a20c6";

/// SHA-256 over the specification files the generator reads — not over the tarball,
/// whose bytes GitHub does not promise to keep stable. See [`crate::spec`].
pub const SPEC_CONTENT_SHA256: &str =
    "218cc3281f0216070c2586db23cc747b3ba3ae0b23de81114cbd15820abbfc30";

/// The tt-metal commit whose Blackhole `cfg_defines.h` the configuration table is
/// generated from.
pub const TT_METAL_REV: &str = "81989dcdb8f9b340c932ae7a71a346f4f08703eb";

/// SHA-256 of that header.
pub const CFG_DEFINES_SHA256: &str =
    "bc2636abc3ea04e6ca322923e6f2713b242857d021d8ef39b9180a14bcb1a5b8";

/// The tt-metal commit whose Quasar LLK `assembly.yaml` the Quasar instruction
/// table is generated from.
///
/// Quasar is not in `tt-isa-documentation`; tt-metal's Quasar LLK carries the
/// table as its assembly source.
pub const QUASAR_REV: &str = "b1d492357ba0f7a6f996f58af318b39592d5afe4";

/// SHA-256 of that `assembly.yaml`.
pub const QUASAR_ASSEMBLY_SHA256: &str =
    "85db27258f94e6ac081ba177bbeb3d4e49635d161dfb85f85f95b0e6a2d2a15e";
