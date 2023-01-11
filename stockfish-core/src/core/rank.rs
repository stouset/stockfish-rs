use crate::prelude::*;

use std::ops::{BitOr, Not};

enumeration! {
    /// A rank, 1 through 8, on a chess board. The variants for this enum are
    /// prefixed with an underscore since identifiers may not begin with a
    /// number.
    pub Rank, [
        _1, _2, _3, _4, _5, _6, _7, _8,
    ]
}

impl Rank {
    /// The number of steps it would take a king to move from one rank to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.as_u8())
    }
}

impl const IntoIterator for Rank {
    type Item     = Square;
    type IntoIter = std::ops::RangeInclusive<Square>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::_1 => Square::A1 ..= Square::H1,
            Self::_2 => Square::A2 ..= Square::H2,
            Self::_3 => Square::A3 ..= Square::H3,
            Self::_4 => Square::A4 ..= Square::H4,
            Self::_5 => Square::A5 ..= Square::H5,
            Self::_6 => Square::A6 ..= Square::H6,
            Self::_7 => Square::A7 ..= Square::H7,
            Self::_8 => Square::A8 ..= Square::H8,
        }
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

impl const BitOr<File> for Rank {
    type Output = Square;

    fn bitor(self, rhs: File) -> Self::Output {
        Square::new(rhs, self)
    }
}

impl const Not for Rank {
    type Output = Bitboard;

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
}
