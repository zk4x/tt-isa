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

#![no_std]
#![forbid(unsafe_code)]

pub mod arc;
pub mod arch;
pub mod cfg;
pub mod dataflow;
pub mod dm;
pub mod dram;
pub mod eth;
pub mod isa;
pub mod l1;
pub mod mailbox;
pub mod noc;
pub mod tensix;
