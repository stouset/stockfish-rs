use super::Bitboard;
use crate::misc::Prng;
use crate::types::{Color, PieceType, Square};

#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Magic<const N: usize> {
    // TODO: magics and attacks should be private, but they're
    // exposed right now so they can be serialized as bytes during
    // build
    pub(crate) magics:  [MagicSquare; Square::COUNT],
    pub(crate) attacks: [Bitboard; N],
}

#[allow(clippy::module_name_repetitions)]
#[must_use]
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MagicSquare {
    mask:   Bitboard,
    magic:  u64,
    offset: usize,
    shift:  usize,
}

impl<const N: usize> Magic<N> {
    // TODO: is `target_pointer_width` guaranteed to be equivalent to the size
    // of `usize`?
    #[cfg(target_pointer_width = "64")]
    const SEEDS: [u64; 8] = [ 728, 10316, 55013, 32803, 12281, 15100, 16645, 255 ];

    #[cfg(target_pointer_width = "32")]
    const SEEDS: [u64; 8] = [ 8977, 44560, 54343, 38998, 5731, 95205, 104912, 17020 ];

    /// Computes all attacks for the given sliding piece at startup. [Magic
    /// bitboards](https://www.chessprogramming.org/Magic_Bitboards/) are used
    /// to look up attacks of sliding pieces. In particular, here we use the so-
    /// called "fancy" approach.
    #[must_use]
    pub fn new(pt: PieceType) -> Box<Self> {
        debug_assert!(pt == PieceType::BISHOP || pt == PieceType::ROOK,
            "unable to generate a magic bitboard for {:?}", pt);

        let mut attacks = bytemuck::allocation::zeroed_box::<[Bitboard; N]>();
        let mut magics  = [MagicSquare::new(); 64];

        let mut occupancy = [Bitboard::EMPTY; 4096];
        let mut reference = [Bitboard::EMPTY; 4096];
        let mut epoch     = [0; 4096];
        let mut count     = 0;
        let mut offset    = 0;
        let mut size      = 0;

        for s in Square::iter() {
            let mut magic  = &mut magics[s];
            let mut b      = Bitboard::EMPTY;

            let file: Bitboard = s.file().into();
            let rank: Bitboard = s.rank().into();

            // Board edges are not considered in the relevant
            // occupancies.
            let edges =
                ((Bitboard::FILE_A | Bitboard::FILE_H) & !file) |
                ((Bitboard::RANK_1 | Bitboard::RANK_8) & !rank);

            // Given a square `s`, the mask is the bitboard of sliding attacks
            // from `s` computed on an empty board. The index must be big enough
            // to contain all the attacks for each possible subset of the mask
            // and so is 2 power the number of 1s of the mask. Hence we deduce
            // the size of the shift to apply to get the index.
            magic.mask  = super::attacks(Color::White, pt, s, Bitboard::EMPTY) & !edges;
            magic.shift = (std::mem::size_of::<usize>() * 8) - magic.mask.count();

            // Set the offset for the attacks table of the square. We have
            // individual table sizes for each square with "Fancy Magic
            // Bitboards".
            offset      += size;
            magic.offset = offset;
            size         = 0;

            // Use Carry-Rippler trick to enumerate all subsets of masks[s] and
            // store the corresponding sliding attack bitboard in reference[].
            loop {
                occupancy[size] = b;
                reference[size] = super::attacks(Color::White, pt, s, b);

                #[cfg(use_pext)] {
                    attacks[std::arch::x86_64::_pext_u64(b.0, magic.mask.0)] = reference[size];
                }

                size += 1;

                // Iterate over every possible bit pattern in the mask.
                b = Bitboard(b.0.wrapping_sub(magic.mask.0)) & magic.mask;

                if b.is_empty() {
                    break;
                }
            }

            if cfg!(use_pext) {
                continue;
            }

            let     seed  = Self::SEEDS[s.rank()];
            let mut prng  = Prng::from_seed(seed);
            let mut i     = 0;

            // Find a magic for square 's' picking up an (almost) random number
            // until we find the one that passes the verification test.
            //
            // TODO: decide whether or not to implement multiplication as an
            // operator on bitboards and u64
            while i < size {
                magic.magic = 0;

                while super::popcnt64((magic.magic.wrapping_mul(magic.mask.0)) >> 56) < 6 {
                    magic.magic = prng.next_sparse_u64();
                }

                count += 1;
                i      = 0;

                // A good magic must map every possible occupancy to an index
                // that looks up the correct sliding attack in the attacks[s]
                // database. Note that we build up the database for square 's'
                // as a side effect of verifying the magic. Keep track of the
                // attempt count and save it in epoch[], little speed-up trick
                // to avoid resetting m.attacks[] after every failed attempt.
                while i < size {
                    let index_rel = magic.index(occupancy[i]);
                    let index     = magic.offset + index_rel;

                    if epoch[index_rel] < count {
                        epoch[index_rel] = count;
                        attacks[index]   = reference[i];
                    } else if attacks[index] != reference[i] {
                        break;
                    }

                    i += 1;
                }
            }
        }

        // TODO: enforce `Magic` is bytemuck::zeroable at compile-time
        // so we can `Box::new_zeroable().assume_init()` the entire
        // thing instead of fucking around with pointers
        #[allow(unsafe_code)]
        unsafe {
            use std::mem::MaybeUninit;
            use std::ptr;

            let mut magic: Box<MaybeUninit<Magic<N>>> = Box::new_uninit();
            let     ptr                               = magic.as_mut_ptr();

            let magics_ptr:  *mut MagicSquare = ptr::addr_of_mut!((*ptr).magics) .cast();
            let attacks_ptr: *mut Bitboard    = ptr::addr_of_mut!((*ptr).attacks).cast();

            magics_ptr .copy_from_nonoverlapping(magics .as_ptr(), 64);
            attacks_ptr.copy_from_nonoverlapping(attacks.as_ptr(), N);

            magic.assume_init()
        }
    }

    #[inline]
    #[must_use]
    pub const fn attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let magic  = self.magics[square];
        let offset = magic.offset;
        let index  = magic.index(occupied);

        self.attacks[offset + index]
    }
}

impl Magic<0x1480> {
    #[must_use]
    pub fn new_bishop() -> Box<Self> {
        Self::new(PieceType::BISHOP)
    }
}

impl Magic<0x19000> {
    #[must_use]
    pub fn new_rook() -> Box<Self> {
        Self::new(PieceType::ROOK)
    }
}

impl MagicSquare {
    #[inline]
    pub const fn new() -> Self {
        MagicSquare {
            mask:   Bitboard::EMPTY,
            magic:  0,
            offset: 0,
            shift:  0,
        }
    }

    #[cfg(use_pext)]
    #[inline]
    #[must_use]
    const fn index(&self, occupied: Bitboard) -> usize {
        std::arch::x86_64::_pext_u64(occupied.0, self.mask.0)
    }

    #[cfg(all(not(use_pext), target_pointer_width = "64"))]
    #[inline]
    #[must_use]
    const fn index(&self, occupied: Bitboard) -> usize {
        // we have explicitly opted into 64-bit platforms, where a
        // u64 should be the same size as a usize
        #[allow(clippy::cast_possible_truncation)] {
            ((occupied & self.mask).0.wrapping_mul(self.magic) >> self.shift) as _
        }
    }

    #[cfg(all(not(use_pext), target_pointer_width = "32"))]
    #[inline]
    #[must_use]
    const fn index(&self, occupied: Bitboard) -> usize {
        let index           = (occupied & self.mask).0;
        let magic_lo: usize = (self.magic & 0xffff_ffff) as _;
        let magic_hi: usize = (self.magic >> 32)         as _;
        let index_lo: usize = (index  & 0xffff_ffff)     as _;
        let index_hi: usize = (index >> 32         )     as _;

        ((index_lo * magic_lo) ^ index_hi).wrapping_mul(magic_hi) >> self.shift
    }
}
