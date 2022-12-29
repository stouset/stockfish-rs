use crate::prelude::*;
use crate::accelerate::computed;

use crate::misc::Prng;

#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(bytemuck::Zeroable)]
pub struct Magic<const N: usize> {
    pub magics:  [MagicSquare; Square::COUNT],
    pub attacks: [Bitboard; N],
}

// There's no reasonable better name for this struct.
#[allow(clippy::module_name_repetitions)]
#[must_use]
#[derive(Copy, Debug, Eq)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[derive_const(Clone, PartialEq)]
#[repr(C)]
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

    /// [Magic bitboards](https://www.chessprogramming.org/Magic_Bitboards/) are
    /// used to quickly look up attacks of sliding pieces. In particular, here
    /// we use the so- called "fancy" approach.
    #[must_use]
    pub(crate) fn new(piece: Piece) -> Box<Self> {
        debug_assert!(piece == Piece::Bishop || piece == Piece::Rook,
            "magic bitboards are not defined for {piece:?}");

        let mut m = bytemuck::zeroed_box::<Magic<N>>();

        let mut occupancy = [Bitboard::EMPTY; 4096];
        let mut reference = [Bitboard::EMPTY; 4096];
        let mut epoch     = [0; 4096];
        let mut count     = 0;
        let mut offset    = 0;
        let mut size      = 0;

        for (square, mut magic) in Square::into_iter().zip(m.magics.iter_mut()) {
            let mut b = Bitboard::EMPTY;

            let file: Bitboard = square.file().into();
            let rank: Bitboard = square.rank().into();

            // Board edges are not considered in the relevant occupancies.
            let edges =
                ((Bitboard::FILE_A | Bitboard::FILE_H) & !file) |
                ((Bitboard::RANK_1 | Bitboard::RANK_8) & !rank);

            // Given a square `s`, the mask is the bitboard of sliding attacks
            // from `s` computed on an empty board. The index must be big enough
            // to contain all the attacks for each possible subset of the mask
            // and so is 2 power the number of 1s of the mask. Hence we deduce
            // the size of the shift to apply to get the index.
            magic.mask  = computed::attacks(Color::White, piece, square, Bitboard::EMPTY) & !edges;
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
                reference[size] = computed::attacks(Color::White, piece, square, b);

                #[cfg(use_pext)] {
                    attacks[std::arch::x86_64::_pext_u64(b.0, magic.mask.0)] = reference[size];
                }

                size += 1;

                // Iterate over every possible bit pattern in the mask.
                b = Bitboard::from(b.0.wrapping_sub(magic.mask.0)) & magic.mask;

                if b.is_empty() {
                    break;
                }
            }

            if cfg!(use_pext) {
                continue;
            }

            let     seed  = Self::SEEDS[square.rank()];
            let mut prng  = Prng::from_seed(seed);
            let mut i     = 0;

            // Find a magic for square 's' picking up an (almost) random number
            // until we find the one that passes the verification test.
            //
            // TODO: decide whether or not to implement multiplication as an
            // operator on bitboards and u64
            while i < size {
                magic.magic = 0;

                while ((magic.magic.wrapping_mul(magic.mask.0)) >> 56).count_ones() < 6 {
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
                        m.attacks[index]   = reference[i];
                    } else if m.attacks[index] != reference[i] {
                        break;
                    }

                    i += 1;
                }
            }
        }

        m
    }

    #[inline]
    pub(crate) const fn attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let magic  = self.magics[square];
        let offset = magic.offset;
        let index  = magic.index(occupied);

        self.attacks[offset + index]
    }
}

impl Magic<0x1480> {
    #[must_use]
    pub fn new_bishop() -> Box<Self> {
        Self::new(Piece::Bishop)
    }
}

impl Magic<0x19000> {
    #[must_use]
    pub fn new_rook() -> Box<Self> {
        Self::new(Piece::Rook)
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
