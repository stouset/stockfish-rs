use crate::prelude::*;

use std::ops::{BitOr, Not};

enumeration! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed with an underscore to mimic those of [`Rank`].
    pub File, [ _A, _B, _C, _D, _E, _F, _G, _H ]
}

impl File {
    /// The number of steps it would take a king to move from one file to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.into())
    }
}

impl const From<Square> for File {
    #[inline]
    fn from(s: Square) -> Self {
        unsafe_optimization!(
            Self::from_u8(s.file_index()).unwrap(),
            Self::from_u8_unchecked(s.file_index())
        )
    }
}

impl const BitOr<Rank> for File {
    type Output = Square;

    fn bitor(self, rhs: Rank) -> Self::Output {
        Square::new(self, rhs)
    }
}

impl const Not for File {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        ! Bitboard::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_distance() {
        assert_eq!(File::_A.distance(File::_A), 0);
        assert_eq!(File::_A.distance(File::_B), 1);
        assert_eq!(File::_A.distance(File::_C), 2);
        assert_eq!(File::_A.distance(File::_D), 3);
        assert_eq!(File::_A.distance(File::_E), 4);
        assert_eq!(File::_A.distance(File::_F), 5);
        assert_eq!(File::_A.distance(File::_G), 6);
        assert_eq!(File::_A.distance(File::_H), 7);
        assert_eq!(File::_B.distance(File::_A), 1);
        assert_eq!(File::_B.distance(File::_G), 5);
        assert_eq!(File::_B.distance(File::_H), 6);
        assert_eq!(File::_C.distance(File::_C), 0);
        assert_eq!(File::_G.distance(File::_A), 6);
        assert_eq!(File::_G.distance(File::_H), 1);
        assert_eq!(File::_H.distance(File::_H), 0);
    }

    #[test]
    fn file_bitor_rank() {
        for square in Square::into_iter() {
            assert_eq!(square.file() | square.rank(), square);
        }
    }
}
