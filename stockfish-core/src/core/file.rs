use crate::prelude::*;

use std::ops::{BitOr, Not};

enumeration! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed with an underscore to mimic those of [`Rank`].
    #[derive_const(Ord, PartialOrd)]
    pub File, [ _A, _B, _C, _D, _E, _F, _G, _H ]
}

impl File {
    /// Returns a [`File`] parsed from a FEN name. `None` if it isn't a valid
    /// file name.
    #[inline]
    #[must_use]
    pub const fn from_fen(byte: u8) -> Option<Self> {
        match byte {
            b'A'..=b'H' => Self::from_u8(byte - b'A'),
            b'a'..=b'h' => Self::from_u8(byte - b'a'),
            _           => None,
        }
    }

    /// The number of steps it would take a king to move from one file to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.into())
    }
}

impl IntoIterator for File {
    type Item     = Square;
    type IntoIter = std::array::IntoIter<Square, 8>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        use Square as S;

        match self {
            Self::_A => [S::A1, S::A2, S::A3, S::A4, S::A5, S::A6, S::A7, S::A8],
            Self::_B => [S::B1, S::B2, S::B3, S::B4, S::B5, S::B6, S::B7, S::B8],
            Self::_C => [S::C1, S::C2, S::C3, S::C4, S::C5, S::C6, S::C7, S::C8],
            Self::_D => [S::D1, S::D2, S::D3, S::D4, S::D5, S::D6, S::D7, S::D8],
            Self::_E => [S::E1, S::E2, S::E3, S::E4, S::E5, S::E6, S::E7, S::E8],
            Self::_F => [S::F1, S::F2, S::F3, S::F4, S::F5, S::F6, S::F7, S::F8],
            Self::_G => [S::G1, S::G2, S::G3, S::G4, S::G5, S::G6, S::G7, S::G8],
            Self::_H => [S::H1, S::H2, S::H3, S::H4, S::H5, S::H6, S::H7, S::H8],
        }.into_iter()
    }
}

impl const From<File> for char {
    #[inline]
    fn from(value: File) -> Self {
        (value.as_u8() + b'A') as _
    }
}

impl const From<Square> for File {
    #[inline]
    fn from(s: Square) -> Self {
        unsafe_optimization!(
            Self::from_u8(s.file_index()).unwrap(),
            Self::from_u8_unchecked(s.file_index()),
        )
    }
}

impl const BitOr<Self> for File {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard::from(self) | rhs
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
        for square in Square::iter() {
            assert_eq!(square.file() | square.rank(), square);
        }
    }

    #[test]
    fn file_into_iter() {
        for file in File::iter() {
            assert_eq!(8, file.into_iter().count());
            assert!(file.into_iter().is_sorted_by_key(Square::file));
        }
    }
}
