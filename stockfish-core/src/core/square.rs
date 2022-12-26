use crate::prelude::*;

enumeration! {
    pub Square, u8, [
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8,
    ]
}

impl Square {
    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn new(file: File, rank: Rank) -> Self {
        let f: u8 = file.into();
        let r: u8 = rank.into();
        let s: u8 = (r << 3) + f;
        unsafe_optimization!(
            Self::from_u8(s).unwrap(),
            Self::from_u8_unchecked(s)
        )
    }

    #[inline]
    #[must_use]
    pub(crate) const fn file_index(self) -> u8 {
        self.as_u8() & 0b0111
    }

    #[inline]
    pub const fn file(self) -> File {
        self.into()
    }

    #[inline]
    #[must_use]
    pub(crate) const fn rank_index(self) -> u8 {
        self.as_u8() >> 3
    }

    #[inline]
    pub const fn rank(self) -> Rank {
        self.into()
    }

    // TODO: implement off bitboards
    // #[inline]
    // pub const fn is_dark(self) -> bool {
    //     (Bitboard::from(self) & Bitboard::DARK_SQUARES).is_any()
    // }

    // TODO: implement off bitboards
    // #[inline]
    // pub const fn is_light(self) -> bool {
    //     !self.is_dark()
    // }

    #[inline]
    pub const fn flip_file(self) -> Self {
        self ^ Square::H1
    }

    #[inline]
    pub const fn flip_rank(self) -> Self {
        self ^ Square::A8
    }

    /// If `color` is [`Color::White`], returns the original square. If `color`
    /// is [`Color::Black`], returns the square on the same file but whose rank
    /// is from the black player's perspective.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stockfish_core::prelude::*;
    ///
    /// assert_eq!(Square::B3, Square::B3.from_perspective(Color::White));
    /// assert_eq!(Square::B6, Square::B3.from_perspective(Color::Black));
    /// assert_eq!(Square::C1, Square::C8.from_perspective(Color::Black));
    /// ```
    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn from_perspective(self, color: Color) -> Self {
        // flip all the bits in the rank portion of the square if the color
        // is black, otherwise XOR with 0 is a no-op
        let s = self.as_u8() ^ (color.as_u8() * 0b0011_1000);

        unsafe_optimization!(
            Self::from_u8(s).unwrap(),
            Self::from_u8_unchecked(s)
        )
    }

    #[inline]
    #[must_use]
    pub const fn distance_files(self, other: Self) -> u8 {
        self.file().distance(other.file())
    }

    #[inline]
    #[must_use]
    pub const fn distance_ranks(self, other: Self) -> u8 {
        self.rank().distance(other.rank())
    }

    #[inline]
    #[must_use]
    pub const fn distance(self, rhs: Self) -> u8 {
        crate::accelerate::square_distance(self, rhs)
    }
}

// TODO: implement off bitboards
// impl const std::ops::BitAnd for Square {
//     type Output = Bitboard;

//     #[inline]
//     #[must_use]
//     fn bitand(self, rhs: Self) -> Self::Output {
//         Bitboard::from(self) & rhs
//     }
// }

// TODO: implement off bitboards
// impl const std::ops::BitOr for Square {
//     type Output = Bitboard;

//     #[inline]
//     #[must_use]
//     fn bitor(self, rhs: Self) -> Self::Output {
//         Bitboard::from(self) | rhs
//     }
// }

impl const std::ops::BitXor for Square {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let s = self.as_u8() ^ rhs.as_u8();

        unsafe_optimization! {
            Self::from_u8(s).unwrap(),
            Self::from_u8_unchecked(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_new() {
        assert_eq!(Square::A1, Square::new(File::_A, Rank::_1));
        assert_eq!(Square::B3, Square::new(File::_B, Rank::_3));
        assert_eq!(Square::B4, Square::new(File::_B, Rank::_4));
        assert_eq!(Square::C7, Square::new(File::_C, Rank::_7));
        assert_eq!(Square::D2, Square::new(File::_D, Rank::_2));
        assert_eq!(Square::E6, Square::new(File::_E, Rank::_6));
        assert_eq!(Square::F5, Square::new(File::_F, Rank::_5));
        assert_eq!(Square::G8, Square::new(File::_G, Rank::_8));
        assert_eq!(Square::H8, Square::new(File::_H, Rank::_8));
    }

    #[test]
    fn square_file_rank() {
        for square in Square::into_iter() {
            let file = File::from_u8(square.as_u8() & 7) .unwrap();
            let rank = Rank::from_u8(square.as_u8() >> 3).unwrap();

            assert_eq!(square.file(), file);
            assert_eq!(square.rank(), rank);
        }
    }

    #[test]
    fn square_flip_file() {
        assert_eq!(Square::A2.flip_file(), Square::H2);
        assert_eq!(Square::D1.flip_file(), Square::E1);
        assert_eq!(Square::G7.flip_file(), Square::B7);
        assert_eq!(Square::H6.flip_file(), Square::A6);
    }

    #[test]
    fn square_flip_file_reflexive() {
        for s in Square::into_iter() {
            assert_eq!(s, s.flip_file().flip_file());
        }
    }

    #[test]
    fn square_flip_rank() {
        assert_eq!(Square::A2.flip_rank(), Square::A7);
        assert_eq!(Square::D1.flip_rank(), Square::D8);
        assert_eq!(Square::G7.flip_rank(), Square::G2);
        assert_eq!(Square::H6.flip_rank(), Square::H3);
    }

    #[test]
    fn square_flip_rank_reflexive() {
        for s in Square::into_iter() {
            assert_eq!(s, s.flip_rank().flip_rank());
        }
    }

    #[test]
    fn square_flip_around_the_world() {
        for s in Square::into_iter() {
            assert_eq!(s, s.flip_file().flip_rank().flip_file().flip_rank());
            assert_eq!(s.flip_file(), s.flip_rank().flip_file().flip_rank());
            assert_eq!(s.flip_file().flip_rank(), s.flip_rank().flip_file());

            assert_eq!(s, s.flip_rank().flip_file().flip_rank().flip_file());
            assert_eq!(s.flip_rank(), s.flip_file().flip_rank().flip_file());
            assert_eq!(s.flip_rank().flip_file(), s.flip_file().flip_rank());
        }
    }

    #[test]
    fn square_distance() {
        for s1 in Square::into_iter() {
            for s2 in Square::into_iter() {
                assert_eq!(s1.distance(s2), std::cmp::max(
                    s1.distance_files(s2),
                    s1.distance_ranks(s2),
                ));

                assert_eq!(s1.distance(s2), s2.distance(s1));
            }
        }
    }

    #[test]
    fn square_from_perspective() {
        assert_eq!(Square::H8, Square::H1.from_perspective(Color::Black));
        assert_eq!(Square::C4, Square::C5.from_perspective(Color::Black));
        assert_eq!(Square::D7, Square::D7.from_perspective(Color::White));
    }

    #[test]
    fn square_distance_files() {
        assert_eq!(3, Square::H4.distance_files(Square::E1));
        assert_eq!(0, Square::G3.distance_files(Square::G8));
        assert_eq!(7, Square::A7.distance_files(Square::H7));
    }

    #[test]
    fn square_distance_ranks() {
        assert_eq!(3, Square::H4.distance_ranks(Square::E1));
        assert_eq!(0, Square::G3.distance_ranks(Square::D3));
        assert_eq!(7, Square::A1.distance_ranks(Square::A8));
    }
}
