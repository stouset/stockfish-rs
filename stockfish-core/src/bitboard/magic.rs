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
        let mut m = bytemuck::zeroed_box::<Magic<N>>();

        let size = Square::into_iter().fold(0, |offset, square| {
            let mask  = MagicSquare::mask(piece, square);
            let shift = MagicSquare::shift(mask);

            // Set the size for the attacks table of the square. We have
            // individual table sizes for each square with "Fancy Magic
            // Bitboards".
            let size    = mask.powerset().size_hint().1.unwrap();
            let magic   = &mut m.magics[square];
            let attacks = &mut m.attacks[offset..(offset + size)];

            magic.mask   = mask;
            magic.shift  = shift;
            magic.offset = offset;

            let mut occupancy = [Bitboard::EMPTY; 2_usize.pow(12)];
            let mut reference = [Bitboard::EMPTY; 2_usize.pow(12)];

            // calculate the attacks for every combination of pieces on the
            // bitboard
            for (i, bitboard) in mask.powerset().enumerate() {
                occupancy[i] = bitboard;
                reference[i] = computed::sliding_attacks(piece, square, bitboard);

                #[cfg(use_pext)] {
                    attacks[std::arch::x86_64::_pext_u64(b.0, mask.0)] = reference[i];
                }
            }

            #[cfg(not(use_pext))] {
                let     seed  = Self::SEEDS[square.rank()];
                let mut prng  = Prng::from(seed);

                let mut i     = 0;
                let mut count = 0;
                let mut epoch = [0; 2_usize.pow(12)];

                // Find a magic for square 's' picking up an (almost) random
                // number until we find the one that passes the verification
                // test.
                //
                // TODO: decide whether or not to implement multiplication as an
                // operator on bitboards and u64
                while i < size {
                    magic.magic = 0;

                    // heuristically find a magic that could plausibly work by
                    // checking that it potentially pushes the bits in `mask`
                    // the upper bits of the result; we will verify that it is
                    // actually a good magic number in the next step
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
                        let index = magic.relative_index(occupancy[i]);

                        if epoch[index] < count {
                            epoch[index]   = count;
                            attacks[index] = reference[i];
                        } else if attacks[index] != reference[i] {
                            break;
                        }

                        i += 1;
                    }
                }
            }

            offset + size
        });

        // the hardcoded size of this magic bitboard should be exactly thes size
        // necessary to contain it and no larger
        debug_assert_eq!(N, size);

        m
    }

    #[inline]
    pub(crate) const fn attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let magic  = self.magics[square];
        let index  = magic.index(occupied);

        self.attacks[index]
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
    /// Calculates the `mask` to be used for a particular [`Piece`] on a given
    /// [`Square`].
    ///
    /// The `mask` is the [`Bitboard`] of squares that would block
    /// the piece from attacking if another piece were on it.
    const fn mask(piece: Piece, square: Square) -> Bitboard {
        // Board edges are not considered to be
        let edges =
            ((Bitboard::FILE_A | Bitboard::FILE_H) & !square.file()) |
            ((Bitboard::RANK_1 | Bitboard::RANK_8) & !square.rank());

        // Given a square `s`, the mask is the bitboard of sliding attacks
        // from `s` computed on an empty board. The index must be big enough
        // to contain all the attacks for each possible subset of the mask
        // and so is 2 power the number of 1s of the mask. Hence we deduce
        // the size of the shift to apply to get the index.
        computed::pseudo_attacks(piece, square) & !edges
    }

    /// Calculates the `shift` sized to be used for a magic's `mask`.
    ///
    /// Magic bitboards work by finding "magic numbers" with a few particularly
    /// useful properties. One property is that, when multiplied by a
    /// [`Bitboard`] that is a subset of the magic bitboard's `mask`, all of the
    /// set bits will be in the most significant bits of the result.
    ///
    /// The `shift` is the number places these bits need to be shifted in order
    /// to be the *least* significant bits in the result. This will let us use
    /// the result as an index into an array.
    ///
    /// In this way modular multiplication followed by a rightward shift becomes
    /// a hash function that produces an index.
    const fn shift(mask: Bitboard) -> usize {
        // the result of multiplying by the magic number will put the set bits
        // in `mask` into the MSBs of the result, so they need to be shifted by
        // the size of the
        (std::mem::size_of::<usize>() * 8) - mask.count()
    }

    #[inline]
    #[must_use]
    const fn index(&self, occupied: Bitboard) -> usize {
        self.offset + self.relative_index(occupied)
    }

    #[cfg(use_pext)]
    #[inline]
    #[must_use]
    const fn relative_index(&self, occupied: Bitboard) -> usize {
        std::arch::x86_64::_pext_u64(occupied.0, self.mask.0)
    }

    #[cfg(all(target_pointer_width = "64", not(use_pext)))]
    #[inline]
    #[must_use]
    const fn relative_index(&self, occupied: Bitboard) -> usize {
        let masked = (occupied & self.mask).0;

        // we have explicitly opted into 64-bit platforms, where a
        // u64 should be the same size as a usize
        #[allow(clippy::cast_possible_truncation)] {
            (masked.wrapping_mul(self.magic) >> self.shift) as _
        }
    }

    #[cfg(all(target_pointer_width = "32", not(use_pext)))]
    #[inline]
    #[must_use]
    const fn relative_index(&self, occupied: Bitboard) -> usize {
        let masked           = (occupied & self.mask).0;
        let masked_lo: usize = masked             as _;
        let masked_hi: usize = (masked >> 32)     as _;
        let magic_lo:  usize = self.magic         as _;
        let magic_hi:  usize = (self.magic >> 32) as _;

        (lo.wrapping_mul(magic_lo) ^ hi.wrapping_mul(magic_hi)) >> self.shift
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_passes_spot_check() {
        for piece in [Piece::Bishop, Piece::Rook] {
            for square in Square::into_iter() {
                let mask = MagicSquare::mask(piece, square);

                let edges = if piece == Piece::Rook {
                    // the mask shouldn't contain the edges, except when the
                    // piece is a Rook and *on* an edge, in which case we should
                    // allow everything except the very first and last square on
                    // that edge
                    Bitboard::EDGES
                        & (!Bitboard::from(square.rank()) | Bitboard::EDGE_FILES)
                        & (!Bitboard::from(square.file()) | Bitboard::EDGE_RANKS)
                } else {
                    Bitboard::EDGES
                };

                refute!(mask.contains(square));
                assert!(mask.disjoint(edges));
            }
        }
    }
}
