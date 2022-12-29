#[doc(hidden)]
pub mod magic;

use crate::prelude::*;

use std::ops::{
    BitAnd, BitAndAssign,
    BitOr,  BitOrAssign,
    BitXor, BitXorAssign,
    Not,
    Shl,
    Add,
};

#[derive(Copy, Eq)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[derive_const(Clone, PartialEq)]
#[repr(transparent)]
#[must_use]
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
        self == Self::EMPTY
    }

    //// Returns [`true`] if the [`Bitboard`] contains any spaces.
    #[inline]
    #[must_use]
    pub const fn is_any(self) -> bool {
        !self.is_empty()
    }

    //// Returns [`true`] if the [`Bitboard`] contains only one space.
    #[inline]
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0.count_ones() == 1
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
        self.0 & (self.0.wrapping_sub(1)) != 0
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

    #[inline]
    #[must_use]
    pub fn into_some_square(self) -> Option<Square> {
        Square::VARIANTS.get(self.0.trailing_zeros() as usize).copied()
    }

    // Returns the number of squares in the bitboard.
    #[inline]
    #[must_use]
    pub const fn count(self) -> usize {
        self.0.count_ones() as _
    }
}

impl std::fmt::Debug for Bitboard {
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ranks = self.0.to_be_bytes();

        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;

        for (rank, bits) in ranks.iter().enumerate() {
            write!(f, "{} |", 8 - rank)?;

            for file in 0..8 {
                match bits & (1 << file) {
                    0 => write!(f, "   |")?,
                    _ => write!(f, " X |")?,
                }
            }

            writeln!(f)?;

            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }

        writeln!(f, "    A   B   C   D   E   F   G   H")
    }
}

impl const From<u64> for Bitboard {
    #[inline]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl const From<File> for Bitboard {
    #[inline]
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
    fn from(s: Square) -> Self {
        Self(1 << s.as_u8())
    }
}

impl const BitAnd<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.0.bitand(rhs.0).into()
    }
}

impl const BitAndAssign<Self> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = (*self).bitand(rhs);
    }
}

impl const BitAnd<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Square) -> Self::Output {
        self.bitand(Self::from(rhs))
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
    fn bitor(self, rhs: Self) -> Self::Output {
        self.0.bitor(rhs.0).into()
    }
}

impl const BitOr<Square> for Bitboard {
    type Output = Self;

    #[inline]
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

impl const BitXor<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl const BitXor<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Square) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const BitXorAssign<Self> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = (*self).bitxor(rhs);
    }
}

impl const Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}

impl const Shl<u8> for Bitboard {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        (self.0 << rhs).into()
    }
}

impl const Add<Direction> for Bitboard {
    type Output = Self;

    #[inline]
    fn add(self, direction: Direction) -> Self {
        let shift: i8 = direction.into();
        let mask      = !direction.discarded_files();

        match shift {
            0.. => (self & mask).0 << shift,
            _   => (self & mask).0 >> shift.abs(),
        }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        assert_eq!(Bitboard::ALL, Bitboard::ALL.clone());
    }

    #[test]
    fn is_empty() {
        assert!(Bitboard::EMPTY           .is_empty());
        refute!(Bitboard::ALL             .is_empty());
        refute!(Bitboard::FILE_B          .is_empty());
        refute!(Bitboard::FILE_C          .is_empty());
        refute!(Bitboard::RANK_1          .is_empty());
        refute!(Bitboard::RANK_7          .is_empty());
        refute!(Bitboard::from(Square::G2).is_empty());
    }

    #[test]
    fn is_any() {
        refute!(Bitboard::EMPTY           .is_any());
        assert!(Bitboard::ALL             .is_any());
        assert!(Bitboard::FILE_B          .is_any());
        assert!(Bitboard::RANK_1          .is_any());
        assert!(Bitboard::FILE_C          .is_any());
        assert!(Bitboard::RANK_7          .is_any());
        assert!(Bitboard::from(Square::G2).is_any());
    }

    #[test]
    fn is_one() {
        refute!(Bitboard::EMPTY           .is_one());
        refute!(Bitboard::ALL             .is_one());
        refute!(Bitboard::FILE_B          .is_one());
        refute!(Bitboard::FILE_C          .is_one());
        refute!(Bitboard::RANK_1          .is_one());
        refute!(Bitboard::RANK_7          .is_one());
        assert!(Bitboard::from(Square::G2).is_one());
    }

    #[test]
    fn is_many() {
        refute!(Bitboard::EMPTY           .is_many());
        assert!(Bitboard::ALL             .is_many());
        assert!(Bitboard::FILE_B          .is_many());
        assert!(Bitboard::FILE_C          .is_many());
        assert!(Bitboard::RANK_1          .is_many());
        assert!(Bitboard::RANK_7          .is_many());
        refute!(Bitboard::from(Square::G2).is_many());
    }

    #[test]
    fn is_all() {
        refute!(Bitboard::EMPTY           .is_all());
        assert!(Bitboard::ALL             .is_all());
        refute!(Bitboard::FILE_B          .is_all());
        refute!(Bitboard::RANK_1          .is_all());
        refute!(Bitboard::FILE_C          .is_all());
        refute!(Bitboard::RANK_7          .is_all());
        refute!(Bitboard::from(Square::G2).is_all());
    }

    #[test]
    fn contains() {
        refute!(Bitboard::LIGHT_SQUARES.contains(Square::A1));
        assert!(Bitboard::LIGHT_SQUARES.contains(Square::A2));
        refute!(Bitboard::LIGHT_SQUARES.contains(Square::A3));
        assert!(Bitboard::LIGHT_SQUARES.contains(Square::A4));
        refute!(Bitboard::LIGHT_SQUARES.contains(Square::A5));
        assert!(Bitboard::LIGHT_SQUARES.contains(Square::A6));
        refute!(Bitboard::LIGHT_SQUARES.contains(Square::A7));
        assert!(Bitboard::LIGHT_SQUARES.contains(Square::A8));
    }

    #[test]
    fn overlaps() {
        assert!(Bitboard::FILE_H.overlaps(Bitboard::RANK_2));
    }

    #[test]
    fn into_some_square() {
        for s in Square::into_iter() {
            assert_eq!(Some(s), Bitboard::from(s).into_some_square());
        }

        assert_eq!(None, Bitboard::EMPTY.into_some_square());
        assert!(Bitboard::DARK_SQUARES.into_some_square().is_some());
    }

    #[test]
    fn count() {
        assert_eq!(0,  Bitboard::EMPTY.count());
        assert_eq!(32, Bitboard::DARK_SQUARES.count());
        assert_eq!(32, Bitboard::LIGHT_SQUARES.count());
        assert_eq!(64, Bitboard::ALL.count());

        assert_eq!(8, Bitboard::FILE_A.count());
        assert_eq!(8, Bitboard::FILE_H.count());
        assert_eq!(8, Bitboard::RANK_3.count());
        assert_eq!(8, Bitboard::RANK_6.count());
    }

    #[test]
    fn fmt_a1() {
        assert_eq!(
            concat!(
                "  +---+---+---+---+---+---+---+---+\n",
                "8 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "7 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "6 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "5 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "4 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "3 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "2 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "1 | X |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "    A   B   C   D   E   F   G   H\n",
            ),

            format!("{:?}", Bitboard::from(Square::A1))
        );
    }

    #[test]
    fn fmt_h8() {
        assert_eq!(
            concat!(
                "  +---+---+---+---+---+---+---+---+\n",
                "8 |   |   |   |   |   |   |   | X |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "7 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "6 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "5 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "4 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "3 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "2 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "1 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "    A   B   C   D   E   F   G   H\n",
            ),

            format!("{:?}", Bitboard::from(Square::H8))
        );
    }

    #[test]
    fn fmt_dark_squares() {
        assert_eq!(
            concat!(
                "  +---+---+---+---+---+---+---+---+\n",
                "8 |   | X |   | X |   | X |   | X |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "7 | X |   | X |   | X |   | X |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "6 |   | X |   | X |   | X |   | X |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "5 | X |   | X |   | X |   | X |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "4 |   | X |   | X |   | X |   | X |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "3 | X |   | X |   | X |   | X |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "2 |   | X |   | X |   | X |   | X |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "1 | X |   | X |   | X |   | X |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "    A   B   C   D   E   F   G   H\n",
            ),

            format!("{:?}", Bitboard::DARK_SQUARES)
        );
    }

    #[test]
    fn from_file() {
        assert_eq!(Bitboard::FILE_A, Bitboard::from(File::_A));
        assert_eq!(Bitboard::FILE_B, Bitboard::from(File::_B));
        assert_eq!(Bitboard::FILE_C, Bitboard::from(File::_C));
        assert_eq!(Bitboard::FILE_D, Bitboard::from(File::_D));
        assert_eq!(Bitboard::FILE_E, Bitboard::from(File::_E));
        assert_eq!(Bitboard::FILE_F, Bitboard::from(File::_F));
        assert_eq!(Bitboard::FILE_G, Bitboard::from(File::_G));
        assert_eq!(Bitboard::FILE_H, Bitboard::from(File::_H));
    }

    #[test]
    fn from_rank() {
        assert_eq!(Bitboard::RANK_1, Bitboard::from(Rank::_1));
        assert_eq!(Bitboard::RANK_2, Bitboard::from(Rank::_2));
        assert_eq!(Bitboard::RANK_3, Bitboard::from(Rank::_3));
        assert_eq!(Bitboard::RANK_4, Bitboard::from(Rank::_4));
        assert_eq!(Bitboard::RANK_5, Bitboard::from(Rank::_5));
        assert_eq!(Bitboard::RANK_6, Bitboard::from(Rank::_6));
        assert_eq!(Bitboard::RANK_7, Bitboard::from(Rank::_7));
        assert_eq!(Bitboard::RANK_8, Bitboard::from(Rank::_8));
    }

    #[test]
    fn bitand() {
        assert_eq!(Bitboard::EMPTY,  Bitboard::EMPTY         & Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_A, Bitboard::FILE_A        & Bitboard::FILE_A);
        assert_eq!(Bitboard::EMPTY,  Bitboard::LIGHT_SQUARES & Bitboard::DARK_SQUARES);
    }

    #[test]
    fn bitand_square() {
        assert_eq!(Bitboard::EMPTY,            Bitboard::LIGHT_SQUARES & Square::C1);
        assert_eq!(Bitboard::from(Square::C2), Bitboard::LIGHT_SQUARES & Square::C2);
    }

    #[test]
    fn bitor() {
        assert_eq!(Bitboard::EMPTY,  Bitboard::EMPTY         | Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_A, Bitboard::FILE_A        & Bitboard::FILE_A);
        assert_eq!(Bitboard::ALL,    Bitboard::LIGHT_SQUARES | Bitboard::DARK_SQUARES);
    }

    #[test]
    fn bitxor() {
        assert_eq!(Bitboard::EMPTY,  Bitboard::EMPTY         ^ Bitboard::EMPTY);
        assert_eq!(Bitboard::EMPTY,  Bitboard::FILE_A        ^ Bitboard::FILE_A);
        assert_eq!(Bitboard::ALL,    Bitboard::LIGHT_SQUARES ^ Bitboard::DARK_SQUARES);
    }

    #[test]
    fn not() {
        assert_eq!(Bitboard::EMPTY,         !Bitboard::ALL);
        assert_eq!(Bitboard::LIGHT_SQUARES, !Bitboard::DARK_SQUARES);
        assert_eq!(Bitboard::FILE_C,        !!Bitboard::FILE_C);
    }

    #[test]
    fn shl() {
        assert_eq!(Bitboard::from(Square::E7), Bitboard::from(Square::D7) << 1);
        assert_eq!(Bitboard::from(Square::C3), Bitboard::from(Square::C2) << 8);
    }
}
