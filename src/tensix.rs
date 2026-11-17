//! The shared vocabulary of a Tensix tile: which cores there are, and which Tensix
//! thread each of them feeds.
//!
//! Everything else a tile has — addresses, sizes, reset bits, the push table — is a
//! fact about one *architecture*, and lives in [`crate::arch`]. This module holds
//! only what every architecture shares: the core identities and the thread
//! identities.
//!
//! Core identity cannot be discovered at run time — `mhartid` reads zero on every
//! baby RISC-V and `misa` lies (it claims RV32IMABFV when A, B, F and V are each
//! only partial) — so a core is named by the code that starts it, never probed.

/// Which baby RISC-V hart in a Tensix tile.
///
/// A core is named by the code that starts it, and the name is enough to find
/// every register and reset bit it owns — except where the hart shares an address
/// block with its neighbours, which is what the two Quasar variants carry.
///
/// # What a name has to answer
///
/// On Blackhole a name is the whole identity: five harts, one `SOFT_RESET_0`, one
/// instruction-buffer pair, one register block. On Quasar the tile has 24 harts and
/// the address blocks are per *compute engine* ("neo"): four `SOFT_RESET_0`
/// registers, four `DEST` banks, four configuration shadows, four register windows
/// at `0x0180_0000 + 0x10000·n` (`tt-2xx/quasar/tensix_neo_reg.h`,
/// `tt_t6_trisc_map.h`). So a Quasar TRISC is named by `(engine, trisc)` and a
/// data mover by its hart id; there is no bare `T0` on that chip.
///
/// Which of these a tile has is [`Arch::cores`](crate::arch::Arch::cores), and
/// asking a chip about a core it does not have is
/// [`EncodeError::NoSuchCore`](crate::isa::EncodeError::NoSuchCore) rather than a
/// made-up answer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Core {
    /// RISCV B. Pushes Tensix instructions to all three threads.
    B,
    /// RISCV T0, feeding Tensix thread 0.
    T0,
    /// RISCV T1, feeding Tensix thread 1.
    T1,
    /// RISCV T2, feeding Tensix thread 2.
    T2,
    /// RISCV NC. Cannot push Tensix instructions at all, and cannot invalidate its
    /// own instruction cache.
    NC,
    /// Quasar's data movers, by hart id (`0..8`): `TensixProcessorTypes::DM0..DM7`.
    ///
    /// Eight per tile over four compute engines, so a data mover is not owned by
    /// one engine — which engine's register block holds a given mover's reset is
    /// not stated in `tt-2xx/quasar`, and this variant therefore carries no engine.
    Dm(u8),
    /// Quasar's compute harts: `TensixProcessorTypes::E<engine>_MATH<trisc>`.
    ///
    /// A TRISC shared a thread's identity on Blackhole; here each of the four
    /// TRISCs of an engine drives its own unit — TRISC0 unpack, TRISC1 math,
    /// TRISC2 pack, TRISC3 SFPU (`tt_llk_quasar/common/inc/cunpack_common.h:43`,
    /// `cmath_common.h:22-26`, `cpack_common.h:37`).
    Math {
        /// The compute engine, `0..4`, named `E0..E3` in the specification.
        engine: u8,
        /// The TRISC inside that engine, `0..4`.
        trisc: u8,
    },
}

impl Core {
    /// The five cores of a Blackhole tile, in the order `SoftReset.md` lists them.
    pub const BLACKHOLE: [Core; 5] = [Core::B, Core::T0, Core::T1, Core::T2, Core::NC];

    /// The 24 harts of a Quasar tile, in `TensixProcessorTypes` order: eight data
    /// movers, then four engines of four TRISCs.
    pub const QUASAR: [Core; 24] = [
        Core::Dm(0),
        Core::Dm(1),
        Core::Dm(2),
        Core::Dm(3),
        Core::Dm(4),
        Core::Dm(5),
        Core::Dm(6),
        Core::Dm(7),
        Core::Math {
            engine: 0,
            trisc: 0,
        },
        Core::Math {
            engine: 0,
            trisc: 1,
        },
        Core::Math {
            engine: 0,
            trisc: 2,
        },
        Core::Math {
            engine: 0,
            trisc: 3,
        },
        Core::Math {
            engine: 1,
            trisc: 0,
        },
        Core::Math {
            engine: 1,
            trisc: 1,
        },
        Core::Math {
            engine: 1,
            trisc: 2,
        },
        Core::Math {
            engine: 1,
            trisc: 3,
        },
        Core::Math {
            engine: 2,
            trisc: 0,
        },
        Core::Math {
            engine: 2,
            trisc: 1,
        },
        Core::Math {
            engine: 2,
            trisc: 2,
        },
        Core::Math {
            engine: 2,
            trisc: 3,
        },
        Core::Math {
            engine: 3,
            trisc: 0,
        },
        Core::Math {
            engine: 3,
            trisc: 1,
        },
        Core::Math {
            engine: 3,
            trisc: 2,
        },
        Core::Math {
            engine: 3,
            trisc: 3,
        },
    ];

    /// Every core any architecture in this crate has.
    pub const ALL: [Core; 29] = [
        Core::B,
        Core::T0,
        Core::T1,
        Core::T2,
        Core::NC,
        Core::Dm(0),
        Core::Dm(1),
        Core::Dm(2),
        Core::Dm(3),
        Core::Dm(4),
        Core::Dm(5),
        Core::Dm(6),
        Core::Dm(7),
        Core::Math {
            engine: 0,
            trisc: 0,
        },
        Core::Math {
            engine: 0,
            trisc: 1,
        },
        Core::Math {
            engine: 0,
            trisc: 2,
        },
        Core::Math {
            engine: 0,
            trisc: 3,
        },
        Core::Math {
            engine: 1,
            trisc: 0,
        },
        Core::Math {
            engine: 1,
            trisc: 1,
        },
        Core::Math {
            engine: 1,
            trisc: 2,
        },
        Core::Math {
            engine: 1,
            trisc: 3,
        },
        Core::Math {
            engine: 2,
            trisc: 0,
        },
        Core::Math {
            engine: 2,
            trisc: 1,
        },
        Core::Math {
            engine: 2,
            trisc: 2,
        },
        Core::Math {
            engine: 2,
            trisc: 3,
        },
        Core::Math {
            engine: 3,
            trisc: 0,
        },
        Core::Math {
            engine: 3,
            trisc: 1,
        },
        Core::Math {
            engine: 3,
            trisc: 2,
        },
        Core::Math {
            engine: 3,
            trisc: 3,
        },
    ];

    /// The core's name, as the specification and the tooling write it.
    ///
    /// A hart index outside the tile's range panics here rather than answering:
    /// no architecture has a `DM9` or an `E7_MATH0`, and a name is used in errors.
    pub const fn name(self) -> &'static str {
        /// `TensixProcessorTypes::DM0..DM7`.
        const DM: [&str; 8] = ["DM0", "DM1", "DM2", "DM3", "DM4", "DM5", "DM6", "DM7"];
        /// `TensixProcessorTypes::E0_MATH0..E3_MATH3`.
        const MATH: [[&str; 4]; 4] = [
            ["E0_MATH0", "E0_MATH1", "E0_MATH2", "E0_MATH3"],
            ["E1_MATH0", "E1_MATH1", "E1_MATH2", "E1_MATH3"],
            ["E2_MATH0", "E2_MATH1", "E2_MATH2", "E2_MATH3"],
            ["E3_MATH0", "E3_MATH1", "E3_MATH2", "E3_MATH3"],
        ];
        match self {
            Core::B => "B",
            Core::T0 => "T0",
            Core::T1 => "T1",
            Core::T2 => "T2",
            Core::NC => "NC",
            Core::Dm(hart) => DM[hart as usize],
            Core::Math { engine, trisc } => MATH[engine as usize][trisc as usize],
        }
    }
}

/// Bytes one 128-bit timestamper event occupies in L1.
///
/// The event format is the same on every architecture this crate describes, which
/// is why it is here rather than in [`crate::arch`]: the addresses differ, the
/// record does not.
pub const TIMESTAMP_EVENT_BYTES: u64 = 16;

/// Which Tensix thread an instruction is being pushed to.
///
/// A marker type rather than a number because the legal `(core, thread)` pairs are
/// a sparse table rather than a range — see
/// [`BlackholeSpecials::pushed_to`](crate::arch::BlackholeSpecials::pushed_to).
pub trait TensixThread {
    /// 0, 1 or 2. Needed where a thread index is data rather than a choice, such as
    /// the mailbox word the host and the image cross-check against each other.
    const INDEX: u32;
}

/// Tensix thread 0.
pub struct Thread0;
/// Tensix thread 1.
pub struct Thread1;
/// Tensix thread 2.
pub struct Thread2;

impl TensixThread for Thread0 {
    const INDEX: u32 = 0;
}
impl TensixThread for Thread1 {
    const INDEX: u32 = 1;
}
impl TensixThread for Thread2 {
    const INDEX: u32 = 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both architectures' core lists are the sizes the headers give: five harts on
    /// Blackhole (`SoftReset.md`), eight movers plus four engines of four on
    /// Quasar (`core_config.h:17-42`).
    #[test]
    fn the_core_sets_are_the_architectures_sizes() {
        assert_eq!(Core::BLACKHOLE.len(), 5);
        assert_eq!(Core::QUASAR.len(), 24);
        // Eight movers, then four engines of four TRISCs, in the order
        // `TensixProcessorTypes` declares them.
        assert_eq!(Core::QUASAR[0], Core::Dm(0));
        assert_eq!(Core::QUASAR[7], Core::Dm(7));
        assert_eq!(
            Core::QUASAR[8],
            Core::Math {
                engine: 0,
                trisc: 0
            }
        );
        assert_eq!(
            Core::QUASAR[23],
            Core::Math {
                engine: 3,
                trisc: 3
            }
        );
    }

    /// `ALL` is the union, with nothing named twice: a duplicate would make
    /// `cores()` ambiguous and a global-id table silently wrong.
    #[test]
    fn all_is_the_union_without_duplicates() {
        assert_eq!(Core::ALL.len(), Core::BLACKHOLE.len() + Core::QUASAR.len());
        for (i, a) in Core::ALL.iter().enumerate() {
            for b in &Core::ALL[i + 1..] {
                assert_ne!(a, b, "{} is listed twice", a.name());
            }
        }
    }

    /// The names are the ones the specification and the tooling print, so an error
    /// naming a core can be searched for in the headers.    #[test]
    fn names_are_the_tooling_names() {
        assert_eq!(Core::NC.name(), "NC");
        assert_eq!(Core::Dm(0).name(), "DM0");
        assert_eq!(Core::Dm(7).name(), "DM7");
        assert_eq!(
            Core::Math {
                engine: 2,
                trisc: 3
            }
            .name(),
            "E2_MATH3"
        );
        assert_eq!(
            Core::Math {
                engine: 0,
                trisc: 0
            }
            .name(),
            "E0_MATH0"
        );
        for (i, a) in Core::ALL.iter().enumerate() {
            for b in &Core::ALL[i + 1..] {
                assert_ne!(a.name(), b.name(), "two cores print the same name");
            }
        }
    }
}
