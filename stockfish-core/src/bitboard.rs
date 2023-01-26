//! Fast bitboards.

#[doc(hidden)]
pub mod magic;

use crate::prelude::*;

use core::iter::FusedIterator;
use core::ops::{
    BitAnd, BitAndAssign,
    BitOr,  BitOrAssign,
    BitXor, BitXorAssign,
    Not,
    Shl,
    Add,
};

/// A fast bitboard for representing chess positions. Bitboards compactly
/// represent a subset of squares on a chess board in a way that allows fast
/// bitwise operations to be performed.
///
/// They only indicate whether or not a particular square is included in a set.
#[derive(Copy, Eq)]
#[derive_const(Clone, Default, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
#[must_use]
pub struct Bitboard(u64);

impl Bitboard {
    /// An empty board with no squares occupied.
    pub const EMPTY: Self = 0.into();

    /// A board with all squares occupied.
    pub const ALL:   Self = u64::MAX.into();

    /// A board with all the dark squares occupied.
    pub const DARK_SQUARES:  Self = 0xAA_55_AA_55_AA_55_AA_55.into();

    /// A board with all the light squares occupied.
    pub const LIGHT_SQUARES: Self = 0x55_AA_55_AA_55_AA_55_AA.into();

    /// A board with all of the squares on the A file occupied.
    pub const FILE_A: Bitboard = 0x01_01_01_01_01_01_01_01.into();

    /// A board with all of the squares on the B file occupied.
    pub const FILE_B: Bitboard = Self::FILE_A << 1;

    /// A board with all of the squares on the C file occupied.
    pub const FILE_C: Bitboard = Self::FILE_A << 2;

    /// A board with all of the squares on the D file occupied.
    pub const FILE_D: Bitboard = Self::FILE_A << 3;

    /// A board with all of the squares on the E file occupied.
    pub const FILE_E: Bitboard = Self::FILE_A << 4;

    /// A board with all of the squares on the F file occupied.
    pub const FILE_F: Bitboard = Self::FILE_A << 5;

    /// A board with all of the squares on the G file occupied.
    pub const FILE_G: Bitboard = Self::FILE_A << 6;

    /// A board with all of the squares on the H file occupied.
    pub const FILE_H: Bitboard = Self::FILE_A << 7;

    /// A board with all of the squares on the 1st rank occupied.
    pub const RANK_1: Bitboard = 0xFF.into();

    /// A board with all of the squares on the 2nd rank occupied.
    pub const RANK_2: Bitboard = Self::RANK_1 << (8);

    /// A board with all of the squares on the 3rd rank occupied.
    pub const RANK_3: Bitboard = Self::RANK_1 << (8 * 2);

    /// A board with all of the squares on the 4th rank occupied.
    pub const RANK_4: Bitboard = Self::RANK_1 << (8 * 3);

    /// A board with all of the squares on the 5th rank occupied.
    pub const RANK_5: Bitboard = Self::RANK_1 << (8 * 4);

    /// A board with all of the squares on the 6th rank occupied.
    pub const RANK_6: Bitboard = Self::RANK_1 << (8 * 5);

    /// A board with all of the squares on the 7th rank occupied.
    pub const RANK_7: Bitboard = Self::RANK_1 << (8 * 6);

    /// A board with all of the squares on the 8th rank occupied.
    pub const RANK_8: Bitboard = Self::RANK_1 << (8 * 7);

    /// A board with all of the squares on the queenside occupied.
    pub const QUEEN_SIDE: Bitboard =
        Self::FILE_A | Self::FILE_B | Self::FILE_C | Self::FILE_D;

    /// A board with all of the center files occupied.
    pub const CENTER_FILES: Bitboard =
        Self::FILE_C | Self::FILE_D | Self::FILE_E | Self::FILE_F;

    /// A board with all of the squares on the kingside occupied.
    pub const KING_SIDE: Bitboard =
        Self::FILE_E | Self::FILE_F | Self::FILE_G | Self::FILE_H;

    /// A board with all of the squares in the center (D4, D5, E4, and E5)
    /// occupied.
    pub const CENTER: Bitboard =
        (Self::FILE_D | Self::FILE_E) &
        (Self::RANK_4 | Self::RANK_5);

    /// A board with the edge files (A and H) occupied.
    pub const EDGE_FILES: Bitboard = Self::FILE_A | Self::FILE_H;

    /// A board with the edge ranks (1 and 8) occupied;
    pub const EDGE_RANKS: Bitboard = Self::RANK_1 | Self::RANK_8;

    /// A board with all of the edges occupied.
    pub const EDGES: Bitboard = Self::EDGE_FILES | Self::EDGE_RANKS;

    /// A board with all the corner squares occupied.
    pub const CORNERS: Bitboard =
        Square::A1 | Square::A8 |
        Square::H1 | Square::H8;

    // pub const KING_FLANK: [Bitboard; File::COUNT] = [
    //     Self::QUEEN_SIDE ^ Self::FILE_D, Self::QUEEN_SIDE,
    //     Self::QUEEN_SIDE,                Self::CENTER_FILES,
    //     Self::CENTER_FILES,              Self::KING_SIDE,
    //     Self::KING_SIDE,                 Self::KING_SIDE ^ Self::FILE_E,
    // ];

    /// Returns [`true`] if the [`Bitboard`] does not contain any spaces.
    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    /// Returns [`true`] if the [`Bitboard`] contains any spaces.
    #[inline]
    #[must_use]
    pub const fn is_any(self) -> bool {
        !self.is_empty()
    }

    /// Returns [`true`] if the [`Bitboard`] contains only one space.
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

    /// Returns [`true`] if the [`Bitboard`] contains all squares.
    #[inline]
    #[must_use]
    pub const fn is_all(self) -> bool {
        self.0 == Self::ALL.0
    }

    /// Returns [`true`] if the [`Bitboard`] contains the given square.
    #[inline]
    #[must_use]
    pub const fn contains(self, s: Square) -> bool {
        self.overlaps(s.into())
    }

    /// Returns [`true`] if the [`Bitboard`] does not contain the given square.
    #[inline]
    #[must_use]
    pub const fn omits(self, s: Square) -> bool {
        (!self).overlaps(s.into())
    }

    /// Returns [`true`] if the [`Bitboard`] has any squares in common with the
    /// given [`Bitboard`].
    #[inline]
    #[must_use]
    pub const fn overlaps(self, rhs: Self) -> bool {
        (self & rhs).is_any()
    }

    /// Returns [`true`] if the [`Bitboard`] has no overlapping squares in
    /// common with the given [`Bitboard`]
    #[inline]
    #[must_use]
    pub const fn disjoint(self, rhs: Self) -> bool {
        ! self.overlaps(rhs)
    }

    /// Returns the number of [`Square`]s set in this [`Bitboard`].
    #[inline]
    #[must_use]
    pub const fn count(self) -> usize {
        self.0.count_ones() as _
    }

    /// Returns an iterator over every individual square in the bitboard.
    #[inline]
    pub const fn iter(self) -> Iter {
        Iter::new(self)
    }

    /// Returns an iterator over every possible subset of squares on the
    /// bitboard.
    ///
    /// Use caution with this function. For boards with larger numbers of bits
    /// this function may require longer than the age of the universe to
    /// complete.
    #[inline]
    const fn powerset(self) -> Powerset {
        debug_assert!(self.0.count_ones() < 24);

        Powerset::new(self)
    }
}

impl core::fmt::Debug for Bitboard {
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ranks = self.0.to_be_bytes();

        writeln!(f)?;
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

impl const IntoIterator for Bitboard {
    type Item     = Square;
    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl const From<Bitboard> for Option<Square> {
    #[inline]
    fn from(value: Bitboard) -> Self {
        Square::VARIANTS.get(value.0.trailing_zeros() as usize).copied()
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
        self.0 &= rhs.0;
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
        self.bitand_assign(Self::from(rhs));
    }
}

impl const BitAnd<File> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: File) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl const BitAndAssign<File> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: File) {
        self.bitand_assign(Self::from(rhs));
    }
}

impl const BitAnd<Rank> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Rank) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl const BitAndAssign<Rank> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Rank) {
        self.bitand_assign(Self::from(rhs));
    }
}

impl const BitOr<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.0.bitor(rhs.0).into()
    }
}

impl const BitOrAssign<Self> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl const BitOr<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Square) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl const BitOrAssign<Square> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Square) {
        self.bitor_assign(Self::from(rhs));
    }
}

impl const BitOr<File> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: File) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl const BitOrAssign<File> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: File) {
        self.bitor_assign(Self::from(rhs));
    }
}

impl const BitOr<Rank> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Rank) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl const BitOrAssign<Rank> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Rank) {
        self.bitor_assign(Self::from(rhs));
    }
}

impl const BitXor<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.0.bitxor(rhs.0).into()
    }
}

impl const BitXorAssign<Self> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl const BitXor<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Square) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const BitXorAssign<Square> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Square) {
        self.bitxor_assign(Self::from(rhs));
    }
}

impl const BitXor<File> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: File) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const BitXorAssign<File> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: File) {
        self.bitxor_assign(Self::from(rhs));
    }
}

impl const BitXor<Rank> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Rank) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const BitXorAssign<Rank> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Rank) {
        self.bitxor_assign(Self::from(rhs));
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

/// An [`Iterator`] that enumerates over every [`Square`] contained in a
/// [`Bitboard`].
#[derive(Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[must_use]
pub struct Iter {
    bb: Bitboard,
}

impl Iter {
    const fn new(bitboard: Bitboard) -> Self {
        Self {
            bb: bitboard,
        }
    }
}

impl Iterator for Iter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let lsb = self.bb.0.trailing_zeros() as usize;
        let s   = Square::VARIANTS.get(lsb).copied();

        if s.is_some() {
            self.bb &= Bitboard(self.bb.0 - 1);
        }

        s
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bb.count(), Some(self.bb.count()))
    }
}

impl FusedIterator for Iter {}

/// An [`Iterator`] that enumerates over every combination of [`Square`]s
/// contained in a [`Bitboard`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct Powerset {
    source: Bitboard,
    next:   Option<Bitboard>,
}

impl Powerset {
    const fn new(bitboard: Bitboard) -> Self {
        Self {
            source: bitboard,
            next:   Some(Bitboard::EMPTY),
        }
    }
}

impl Iterator for Powerset {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        // use Carry-Ripler trick to enumerate all subsets of the source
        // bitboard
        let next  = self.next;
        self.next = self.next
            .map(|bb| bb.0.wrapping_sub(self.source.0) & self.source.0)
            .map(Bitboard::from)
            .filter(|bb| bb.is_any());

        next
    }

    // TODO: more accurately estimate the bounds
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(2_usize.pow(self.source.0.count_ones())))
    }
}

impl FusedIterator for Powerset {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives() {
        assert_eq!(
            core::cmp::Ord::cmp(&Bitboard::ALL, &Bitboard::ALL),
            core::cmp::Ordering::Equal
        );
    }

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
    fn omits() {
        assert!(Bitboard::LIGHT_SQUARES.omits(Square::A1));
        refute!(Bitboard::LIGHT_SQUARES.omits(Square::A2));
        assert!(Bitboard::LIGHT_SQUARES.omits(Square::A3));
        refute!(Bitboard::LIGHT_SQUARES.omits(Square::A4));
        assert!(Bitboard::LIGHT_SQUARES.omits(Square::A5));
        refute!(Bitboard::LIGHT_SQUARES.omits(Square::A6));
        assert!(Bitboard::LIGHT_SQUARES.omits(Square::A7));
        refute!(Bitboard::LIGHT_SQUARES.omits(Square::A8));
    }

    #[test]
    fn overlaps() {
        assert!(Bitboard::FILE_H.overlaps(Bitboard::RANK_2));
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
    fn into_option_square() {
        for s in Square::iter() {
            assert_eq!(Some(s), Bitboard::from(s).into());
        }

        assert_eq!(None, Option::<Square>::from(Bitboard::EMPTY));

        assert!(Option::<Square>::from(Bitboard::DARK_SQUARES).is_some());
        assert!(Option::<Square>::from(Bitboard::KING_SIDE)   .is_some());
    }

    #[test]
    fn powerset_derives() {
        let set1 = Bitboard::EMPTY.powerset();
        let set2 = set1.clone();

        assert_eq!(set1, set2);

        assert!(format!("{set1:?}").is_ascii());
    }

    #[test]
    fn powerset() {
        use Square::{D4, D5, E4, E5};

        let mut powerset = Bitboard::CENTER.powerset().collect::<Vec<Bitboard>>();
        let mut expected = [
            Bitboard::EMPTY,

            Bitboard::from(D4), Bitboard::from(E4),
            Bitboard::from(D5), Bitboard::from(E5),

            D4 | E4,
            D4 | D5,
            D4 | E5,
            E4 | D5,
            E4 | E5,
            D5 | E5,

            D4 | E4 | D5,
            D4 | E4 | E5,
            D4 | D5 | E5,
            D5 | E4 | E5,

            Bitboard::CENTER,
        ];

        powerset.sort();
        expected.sort();

        assert_eq!(expected, &powerset[..]);
    }

    #[test]
    fn fmt_a1() {
        assert_eq!(
            concat!(
                "\n",
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
                "\n",
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
                "\n",
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
    fn bitand_assign() {
        let mut bb1 = Bitboard::EMPTY;
        let mut bb2 = Bitboard::from(Square::C2);

        bb1 &= Square::C3;
        bb2 &= Bitboard::LIGHT_SQUARES;

        assert_eq!(Bitboard::EMPTY,            bb1);
        assert_eq!(Bitboard::from(Square::C2), bb2);
    }

    #[test]
    fn bitor() {
        assert_eq!(Bitboard::EMPTY,  Bitboard::EMPTY         | Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_A, Bitboard::FILE_A        & Bitboard::FILE_A);
        assert_eq!(Bitboard::ALL,    Bitboard::LIGHT_SQUARES | Bitboard::DARK_SQUARES);
    }

    #[test]
    fn bitor_square() {
        assert_eq!(Bitboard::from(Square::D4),  Bitboard::EMPTY  | Square::D4);
        assert_eq!(Bitboard::FILE_A,            Bitboard::FILE_A | Square::A1);
        assert_eq!(Square::A1 | Square::H8,     Bitboard::EMPTY  | Square::A1 | Square::H8);
    }

    #[test]
    fn bitor_assign() {
        let mut bb1 = Bitboard::CENTER_FILES;
        let mut bb2 = Bitboard::RANK_2;

        bb1 |= Square::C3;
        bb2 |= Bitboard::FILE_A;

        assert_eq!(bb1, Bitboard::CENTER_FILES | Square::C3);
        assert_eq!(bb2, Bitboard::FILE_A | Bitboard::RANK_2);
    }

    #[test]
    fn bitxor() {
        assert_eq!(Bitboard::EMPTY,  Bitboard::EMPTY         ^ Bitboard::EMPTY);
        assert_eq!(Bitboard::EMPTY,  Bitboard::FILE_A        ^ Bitboard::FILE_A);
        assert_eq!(Bitboard::ALL,    Bitboard::LIGHT_SQUARES ^ Bitboard::DARK_SQUARES);
    }

    #[test]
    fn bitxor_square() {
        assert_eq!(Bitboard::from(Square::D4),     Bitboard::EMPTY  ^ Square::D4);
        assert_eq!(Bitboard::FILE_A & !Square::A1, Bitboard::FILE_A ^ Square::A1);
        assert_eq!(Square::A1 | Square::H8,        Bitboard::EMPTY  ^ Square::A1 ^ Square::H8);
    }

    #[test]
    fn bitxor_assign() {
        let mut bb = Bitboard::CENTER_FILES;

        bb ^= Bitboard::FILE_D;

        assert_eq!(bb, Bitboard::CENTER_FILES ^ Bitboard::FILE_D);
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
