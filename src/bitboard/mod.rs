//! Bitboards for fast lookups.

use crate::types::{Direction, File, Rank, Square};

use std::ops::{
    BitAnd, BitAndAssign,
    BitOr,  BitOrAssign,
    BitXor, BitXorAssign,
    Not,
    Shl,
    Add,
};

mod magic;

#[cfg(use_computed_bitboards)]
mod fast;

#[cfg(any(not(use_computed_bitboards), test))]
mod slow;

#[cfg(use_computed_bitboards)]
pub use fast::*;

#[cfg(not(use_computed_bitboards))]
pub(crate) use slow::*;

pub use magic::*;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Self = 0.into();
    pub const ALL:   Self = u64::MAX.into();

    pub const DARK_SQUARES:  Self = 0xAA_55_AA_55_AA_55_AA_55.into();
    pub const LIGHT_SQUARES: Self = 0x55_AA_55_AA_55_AA_55_AA.into();

    pub const FILE_A: Bitboard = 0x01_01_01_01_01_01_01_01.into();
    pub const FILE_B: Bitboard = Self::FILE_A << 1;
    pub const FILE_C: Bitboard = Self::FILE_A << 2;
    pub const FILE_D: Bitboard = Self::FILE_A << 3;
    pub const FILE_E: Bitboard = Self::FILE_A << 4;
    pub const FILE_F: Bitboard = Self::FILE_A << 5;
    pub const FILE_G: Bitboard = Self::FILE_A << 6;
    pub const FILE_H: Bitboard = Self::FILE_A << 7;

    pub const RANK_1: Bitboard = 0xFF.into();
    pub const RANK_2: Bitboard = Self::RANK_1 << (8);
    pub const RANK_3: Bitboard = Self::RANK_1 << (8 * 2);
    pub const RANK_4: Bitboard = Self::RANK_1 << (8 * 3);
    pub const RANK_5: Bitboard = Self::RANK_1 << (8 * 4);
    pub const RANK_6: Bitboard = Self::RANK_1 << (8 * 5);
    pub const RANK_7: Bitboard = Self::RANK_1 << (8 * 6);
    pub const RANK_8: Bitboard = Self::RANK_1 << (8 * 7);

    // const BB_QUEEN_SIDE:   Bitboard = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
    // const BB_CENTER_FILES: Bitboard = BB_FILE_C | BB_FILE_D | BB_FILE_E | BB_FILE_F;
    // const BB_KING_SIDE:    Bitboard = BB_FILE_E | BB_FILE_F | BB_FILE_G | BB_FILE_H;
    // const BB_CENTER:       Bitboard = (BB_FILE_D | BB_FILE_E) & (BB_RANK_4 | BB_RANK_5);

    // const BB_KING_FLANK: [Bitboard; File::COUNT] = [
    //     BB_QUEEN_SIDE ^ BB_FILE_D, BB_QUEEN_SIDE,
    //     BB_QUEEN_SIDE,             BB_CENTER_FILES,
    //     BB_CENTER_FILES,           BB_KING_SIDE,
    //     BB_KING_SIDE,              BB_KING_SIDE ^ BB_FILE_E,
    // ];

    /// Returns [`true`] if the [`Bitboard`] does not contain any spaces.
    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == Self::EMPTY.0
    }

    //// Returns [`true`] if the [`Bitboard`] contains any spaces.
    #[inline]
    #[must_use]
    pub const fn is_any(self) -> bool {
        !self.is_empty()
    }

    /// Returns [`true`] if the [`Bitboard`] contains more than one space.
    #[inline]
    #[must_use]
    pub const fn is_many(self) -> bool {
        // If more than one bit is set, subtracting one will flip the
        // lowest set bit and any bits lower than it. But any *higher*
        // set bits will be unchanged.
        //
        // In the case of zero, all bits will be flipped.
        self.0 & (self.0 - 1) != 0
    }

    // Returns [`true`] if the [`Bitboard`] contains all squares.
    #[inline]
    #[must_use]
    pub const fn is_all(self) -> bool {
        self.0 == Self::ALL.0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, s: Square) -> bool {
        self.overlaps(s.into())
    }

    #[inline]
    #[must_use]
    pub const fn overlaps(self, rhs: Self) -> bool {
        (self & rhs).is_any()
    }

    // Returns the number of squares in the bitboard.
    #[inline]
    #[must_use]
    pub const fn count(self) -> usize {
        popcnt64(self.0) as _
    }

    // Returns the underlying integer representation of the bitboard.
    #[must_use]
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl const From<u64> for Bitboard {
    #[inline]
    #[must_use]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl const From<File> for Bitboard {
    #[inline]
    #[must_use]
    fn from(f: File) -> Self {
        match f {
            File::_A => Self::FILE_A,
            File::_B => Self::FILE_B,
            File::_C => Self::FILE_C,
            File::_D => Self::FILE_D,
            File::_E => Self::FILE_E,
            File::_F => Self::FILE_F,
            File::_G => Self::FILE_G,
            File::_H => Self::FILE_H,
        }
    }
}

impl const From<Rank> for Bitboard {
    #[inline]
    #[must_use]
    fn from(r: Rank) -> Self {
        match r {
            Rank::_1 => Self::RANK_1,
            Rank::_2 => Self::RANK_2,
            Rank::_3 => Self::RANK_3,
            Rank::_4 => Self::RANK_4,
            Rank::_5 => Self::RANK_5,
            Rank::_6 => Self::RANK_6,
            Rank::_7 => Self::RANK_7,
            Rank::_8 => Self::RANK_8,
        }
    }
}

impl const From<Square> for Bitboard {
    #[inline]
    #[must_use]
    fn from(s: Square) -> Self {
        square(s)
    }
}

impl const From<Option<Square>> for Bitboard {
    #[inline]
    #[must_use]
    fn from(o: Option<Square>) -> Self {
        match o {
            Some(s) => s.into(),
            None    => Self::EMPTY,
        }
    }
}

impl const From<Bitboard> for u64 {
    #[inline]
    #[must_use]
    fn from(bb: Bitboard) -> Self {
        bb.0
    }
}

impl const Not for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}

impl const BitAnd<Self> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.0.bitand(rhs.0).into()
    }
}

impl const BitAnd<Square> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitand(self, rhs: Square) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl const BitAnd<Option<Square>> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitand(self, rhs: Option<Square>) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl const BitAndAssign<Self> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = (*self).bitand(rhs);
    }
}

impl const BitAndAssign<Square> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Square) {
        *self = (*self).bitand(rhs);
    }
}

impl const BitOr<Self> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.0.bitor(rhs.0).into()
    }
}

impl const BitOr<Square> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitor(self, rhs: Square) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl const BitOrAssign<Self> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = (*self).bitor(rhs);
    }
}

impl const BitOrAssign<Square> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Square) {
        *self = (*self).bitor(rhs);
    }
}

impl const BitOr<Option<Square>> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitor(self, rhs: Option<Square>) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl const BitXor<Self> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl const BitXor<Square> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitxor(self, rhs: Square) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const BitXor<Option<Square>> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn bitxor(self, rhs: Option<Square>) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const BitXorAssign<Self> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = (*self).bitxor(rhs);
    }
}

impl const BitXorAssign<Square> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Square) {
        *self = (*self).bitxor(rhs);
    }
}

impl const Shl<u8> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn shl(self, rhs: u8) -> Self::Output {
        (self.0 << rhs).into()
    }
}

impl const Add<Direction> for Bitboard {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, direction: Direction) -> Self {
        let shift = direction.as_i8();
        let mask  = !direction.discarded_files();

        #[allow(clippy::match_bool)]
        Bitboard(match shift.is_positive() {
            true  => (self & mask).0 << shift,
            false => (self & mask).0 >> shift.abs(),
        })
    }
}

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;

        for rank in Rank::iter().rev() {
            write!(f, "{} ", u8::from(rank) + 1)?;

            for file in File::iter() {
                let s  = Square::new(file, rank);
                let bb = *self & s;
                let c  = if bb.is_any() { "X" } else { " " };
                write!(f, "| {} ", c)?;
            }

            writeln!(f, "|")?;
            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }

        writeln!(f, "    A   B   C   D   E   F   G   H")
    }
}

#[cfg(test)]
mod tests {
    // We want to explicitly disambiguate between fast/slow
    // implementations.
    #![allow(unused_qualifications)]

    use super::*;
    use crate::misc::Prng;
    use crate::types::{Color, PieceType, Square};

    impl Bitboard {
        /// Returns a pseudorandom bitboard with ~1/8th of its spaces filled
        /// for testing.
        #[inline]
        #[must_use]
        pub(crate) fn pseudorandom_sparse(prng: &mut Prng) -> Self {
            Self(prng.next_sparse_u64())
        }
    }

    #[test]
    fn using_computed_bitboards() {
        // check that the config flag is enabled
        assert!(cfg!(use_computed_bitboards));

        // pick one of the fast functions, and make sure the version exported on
        // the module itself is the same one
        assert_eq!(popcnt64 as usize, fast::popcnt64 as usize);
        assert_ne!(popcnt64 as usize, slow::popcnt64 as usize);
    }

    #[test]
    fn popcnt64_is_correct() {
        let mut prng = Prng::from_seed(0xcba2_a28b_df33_e328);

        for _ in 0..4_096 {
            let i = prng.next_u64();

            assert_eq!(
                fast::popcnt64(i),
                slow::popcnt64(i),
            );
        }
    }

    #[test]
    fn square_distance_is_correct() {
        for s1 in Square::iter() {
            for s2 in Square::iter() {
                assert_eq!(
                    fast::square_distance(s1, s2),
                    slow::square_distance(s1, s2),
                );
            }
        }
    }

    #[test]
    fn square_is_correct() {
        for s in Square::iter() {
            assert_eq!(
                fast::square(s),
                slow::square(s),
            );
        }
    }

    #[test]
    fn line_is_correct() {
        for s1 in Square::iter() {
            for s2 in Square::iter() {
                assert_eq!(
                    fast::line(s1, s2),
                    slow::line(s1, s2),
                );
            }
        }
    }

    #[test]
    fn moves_are_correct() {
        for c in Color::iter() {
            for pt in PieceType::iter() {
                for s in Square::iter() {
                    assert_eq!(
                        fast::moves(c, pt, s),
                        slow::moves(c, pt, s),
                    );
                }
            }
        }
    }

    #[test]
    fn attacks_are_correct() {
        let mut prng = Prng::from_seed(0x054b_1917_6345_7ee6);

        for c in Color::iter() {
            for pt in PieceType::iter() {
                for s in Square::iter() {
                    for _ in 0..2_048 {
                        let square = Bitboard::from(s);
                        let board  = Bitboard::pseudorandom_sparse(&mut prng) & !square;

                        assert_eq!(
                            fast::attacks(c, pt, s, board),
                            slow::attacks(c, pt, s, board),
                        );
                    }
                }
            }
        }
    }
}
