//! Blackhole hardware and Quasar ISA definitions: coordinate spaces, register maps,
//! instruction encoders.
//!
//! `no_std` with zero dependencies, because this crate is compiled twice: once for
//! the host, to *generate* instruction streams, and once for `riscv32im`, to *push*
//! them. Anything that cannot build for both belongs somewhere else.
//!
//! Every constant here cites the specification page it came from. The Blackhole
//! documentation tree is a delta over Wormhole's, so a fact sourced from a Wormhole
//! page is a hypothesis until verified — such facts are marked `UNVERIFIED`.
//!
//! The Quasar encodings are a separate table in [`isa::quasar`]. Their source is
//! tt-metal's Quasar LLK `assembly.yaml`, because the Blackhole documentation tree
//! has no Quasar pages.
//!
//! The root holds only what both architectures share: [`arch`] (the traits and one
//! module per architecture), [`isa`] and [`mod@cfg`] (a generated table per
//! architecture), and [`tensix`] (the hart vocabulary). Everything with one
//! architecture's protocol in it — its NoC, its DRAM, its L1 layout, its mailboxes,
//! its movers, its ARC and Ethernet — lives under that architecture's module.

#![no_std]
#![forbid(unsafe_code)]

pub mod arch;
pub mod cfg;
pub mod isa;
pub mod tensix;
