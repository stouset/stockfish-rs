use crate::prelude::*;

use core::ops::{BitOr, Not};

enumeration! {
    /// A rank, 1 through 8, on a chess board. The variants for this enum are
    /// prefixed with an underscore since identifiers may not begin with a
    /// number.
    #[derive_const(Ord, PartialOrd)]
    pub Rank, [
        _1, _2, _3, _4, _5, _6, _7, _8,
    ]
}

impl Rank {
    /// Returns a [`File`] parsed from a FEN name. `None` if it isn't a valid
    /// file name.
    #[inline]
    #[must_use]
    pub const fn from_fen(byte: u8) -> Option<Self> {
        match byte {
            b'1'..=b'8' => Self::from_u8(byte - b'1'),
            _           => None,
        }
    }

    /// The number of steps it would take a king to move from one rank to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.as_u8())
    }
}

impl IntoIterator for Rank {
    type Item     = Square;
    type IntoIter = core::array::IntoIter<Square, 8>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        use Square as S;

        match self {
            Self::_1 => [S::A1, S::B1, S::C1, S::D1, S::E1, S::F1, S::G1, S::H1],
            Self::_2 => [S::A2, S::B2, S::C2, S::D2, S::E2, S::F2, S::G2, S::H2],
            Self::_3 => [S::A3, S::B3, S::C3, S::D3, S::E3, S::F3, S::G3, S::H3],
            Self::_4 => [S::A4, S::B4, S::C4, S::D4, S::E4, S::F4, S::G4, S::H4],
            Self::_5 => [S::A5, S::B5, S::C5, S::D5, S::E5, S::F5, S::G5, S::H5],
            Self::_6 => [S::A6, S::B6, S::C6, S::D6, S::E6, S::F6, S::G6, S::H6],
            Self::_7 => [S::A7, S::B7, S::C7, S::D7, S::E7, S::F7, S::G7, S::H7],
            Self::_8 => [S::A8, S::B8, S::C8, S::D8, S::E8, S::F8, S::G8, S::H8],
        }.into_iter()
    }
}

impl const From<Rank> for char {
    #[inline]
    fn from(value: Rank) -> Self {
        (value.as_u8() + b'1') as _
    }
}

impl const From<Square> for Rank {
    #[inline]
    fn from(s: Square) -> Self {
        unsafe_optimization!(
            Self::from_u8(s.rank_index()).unwrap(),
            Self::from_u8_unchecked(s.rank_index()),
        )
    }
}

impl const BitOr<Self> for Rank {
    type Output = Bitboard;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard::from(self) | rhs
    }
}

impl const BitOr<File> for Rank {
    type Output = Square;

    #[inline]
    fn bitor(self, rhs: File) -> Self::Output {
        Square::new(rhs, self)
    }
}

impl const Not for Rank {
    type Output = Bitboard;

    #[inline]
    fn not(self) -> Self::Output {
        ! Bitboard::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_distance() {
        assert_eq!(Rank::_1.distance(Rank::_1), 0);
        assert_eq!(Rank::_1.distance(Rank::_2), 1);
        assert_eq!(Rank::_1.distance(Rank::_3), 2);
        assert_eq!(Rank::_1.distance(Rank::_4), 3);
        assert_eq!(Rank::_1.distance(Rank::_5), 4);
        assert_eq!(Rank::_1.distance(Rank::_6), 5);
        assert_eq!(Rank::_1.distance(Rank::_7), 6);
        assert_eq!(Rank::_1.distance(Rank::_8), 7);
        assert_eq!(Rank::_2.distance(Rank::_1), 1);
        assert_eq!(Rank::_2.distance(Rank::_7), 5);
        assert_eq!(Rank::_2.distance(Rank::_8), 6);
        assert_eq!(Rank::_3.distance(Rank::_3), 0);
        assert_eq!(Rank::_7.distance(Rank::_1), 6);
        assert_eq!(Rank::_7.distance(Rank::_8), 1);
        assert_eq!(Rank::_8.distance(Rank::_8), 0);
    }

    #[test]
    fn rank_bitor_file() {
        for square in Square::iter() {
            assert_eq!(square.rank() | square.file(), square);
        }
    }

    #[test]
    fn rank_into_iter() {
        for rank in Rank::iter() {
            assert_eq!(8, rank.into_iter().count());
            assert!(rank.into_iter().is_sorted_by_key(Square::rank));
        }
    }
}
