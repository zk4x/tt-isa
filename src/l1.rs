//! The fixed regions of a Tensix tile's L1.
//!
//! Everything that is one address for every kernel -- the image area, the movers'
//! mailboxes and lists, the data arena, the program cache -- is a constant here,
//! and the assertions at the bottom keep them in order and inside L1. Kernels
//! never name an address in the data arena: the caller declares what it needs and
//! the compiler places it there.

use crate::{dm, mailbox, tensix};

/// A byte range of L1, `[base, end)`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Region {
    pub name: &'static str,
    pub base: u64,
    pub end: u64,
}

impl Region {
    pub const fn contains(&self, at: u64, len: u64) -> bool {
        at >= self.base && at + len <= self.end
    }
}

/// Where the three role images (T0 unpack, T1 math, T2 pack) are linked and
/// loaded: packed against the end of the image area, each in a [`roles::SLOT`]
/// reached through its core's reset-PC override. The slots are what T0..T2's
/// default reset PCs would leave -- 16 KiB each, of which the images used about
/// half -- and the space they free goes to B's mover ([`dm::IMAGE_MAX`]), which
/// cannot be moved: its entry point is L1 offset 0.
pub mod roles {
    use crate::tensix::Core;

    /// One role image's slot.
    pub const SLOT: u64 = 0x2800;
    /// The end of the last slot: NC's default reset PC, where its stub lives.
    pub const END: u64 = Core::NC.default_reset_pc() as u64;
    /// The load address (and link address) of the T0, T1 and T2 role images.
    pub const BASE: [u64; 3] = [END - 3 * SLOT, END - 2 * SLOT, END - SLOT];
    const _: () = assert!(BASE[0] % 32 == 0);
}

/// The data arena: every kernel's operands, results, staging and circular
/// buffers, placed by the planner.
pub const DATA: Region = Region {
    name: "data arena",
    base: 0x2_0000,
    end: mailbox::MAILBOX_BASE,
};

/// RISCV NC's mover image (`dm::nc`), in what was the free L1 between B's
/// mover area and the data arena.
pub const NC_IMAGE: Region = Region {
    name: "NC mover image",
    base: dm::nc::IMAGE_BASE,
    end: dm::nc::IMAGE_BASE + dm::nc::IMAGE_MAX,
};

/// The rest of L1, kept for resident kernel programs.
pub const PROGRAM_CACHE: Region = Region {
    name: "program cache",
    base: mailbox::TRACE_BUFFER + mailbox::TRACE_BUFFER_BYTES,
    end: tensix::L1_SIZE,
};

const _: () = assert!(DATA.base % crate::dram::ALIGN == 0);
const _: () = assert!(roles::BASE[0] == dm::IMAGE_BASE + dm::IMAGE_MAX);
const _: () = assert!(NC_IMAGE.end <= DATA.base);
const _: () = assert!(DATA.end <= PROGRAM_CACHE.base);
const _: () = assert!(PROGRAM_CACHE.end == tensix::L1_SIZE);
