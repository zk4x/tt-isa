//! The fixed regions of a Tensix tile's L1.
//!
//! Everything that is one address for every kernel -- the image area, the movers'
//! mailboxes and lists, the data arena, the program cache -- is a constant here,
//! and the assertions at the bottom keep them in order and inside L1. Kernels
//! never name an address in the data arena: the caller declares what it needs and
//! the compiler places it there.

use crate::arch::blackhole::{dm, mailbox};
use crate::arch::{Arch, blackhole::Blackhole};

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
/// half -- and the space they free goes to B's mover ([`dm::IMAGE_MAX`]), whose
/// image is above the low L1 words and reached by the jump at its hardwired
/// reset PC.
pub mod roles {

    /// One role image's slot.
    pub const SLOT: u64 = 0x2800;
    /// The end of the last slot: NC's default reset PC, where its stub lives.
    ///
    /// The value is `Arch::default_reset_pc(Core::NC)` on Blackhole, spelled as a
    /// literal because this is a `const` position and the per-core lookup is not
    /// allowed to be one; `the_role_slots_end_at_ncs_reset_pc` proves they agree.
    pub const END: u64 = 0x1_2000;
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
    end: Blackhole::L1_SIZE,
};

const _: () = assert!(DATA.base % crate::arch::blackhole::dram::ALIGN == 0);
const _: () = assert!(roles::BASE[0] == dm::IMAGE_BASE + dm::IMAGE_MAX);
const _: () = assert!(NC_IMAGE.end <= DATA.base);
const _: () = assert!(DATA.end <= PROGRAM_CACHE.base);
const _: () = assert!(PROGRAM_CACHE.end == Blackhole::L1_SIZE);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensix::Core;

    /// The role slots end where NC starts, which is a per-core lookup and so
    /// cannot be a `const`: the literal in [`roles::END`] is checked against the
    /// architecture here instead.
    #[test]
    fn the_role_slots_end_at_ncs_reset_pc() {
        assert_eq!(
            Blackhole::default_reset_pc(Core::NC),
            Ok(roles::END as u32),
            "roles::END must be NC's default reset PC"
        );
    }
}
