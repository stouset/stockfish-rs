/// The cardinal directions on a chessboard plus the other individual steps a
/// piece is capable of taking across a chess board.
#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[must_use]
pub struct Direction(i8);

impl Direction {
    pub const NONE: Self = Self(0);

    pub const N: Self = Self(8);
    pub const E: Self = Self(1);
    pub const S: Self = -Self::N;
    pub const W: Self = -Self::E;

    pub const NW: Self = Self(Self::N.0 + Self::W.0);
    pub const NN: Self = Self(Self::N.0 + Self::N.0);
    pub const NE: Self = Self(Self::N.0 + Self::E.0);
    pub const EE: Self = Self(Self::E.0 + Self::E.0);
    pub const SE: Self = Self(Self::S.0 + Self::E.0);
    pub const SS: Self = Self(Self::S.0 + Self::S.0);
    pub const SW: Self = Self(Self::S.0 + Self::W.0);
    pub const WW: Self = Self(Self::W.0 + Self::W.0);

    pub const NNW: Self = Self(Self::NN.0 + Self::W.0);
    pub const NNE: Self = Self(Self::NN.0 + Self::E.0);
    pub const ENE: Self = Self(Self::EE.0 + Self::N.0);
    pub const ESE: Self = Self(Self::EE.0 + Self::S.0);
    pub const SSE: Self = Self(Self::SS.0 + Self::E.0);
    pub const SSW: Self = Self(Self::SS.0 + Self::W.0);
    pub const WSW: Self = Self(Self::WW.0 + Self::S.0);
    pub const WNW: Self = Self(Self::WW.0 + Self::N.0);

    /// The maximum distance a Direction can cover.
    pub const MAX: u8 = 2;

    // /// Returns a bitboard of files that will be pushed off the board by
    // /// shifting it in this direction.
    // ///
    // /// # Panics
    // ///
    // /// This function will panic in debug builds if it encounters a Direction
    // /// with too large a distance. This should not be encountered in practice.
    // #[inline]
    // #[must_use]
    // pub const fn discarded_files(self) -> Bitboard {
    //     // Distances currently rely on the assumption that they unambiguously
    //     // encode a shift in some direction. However if distances get large
    //     // (above 3), it's no longer possible to uniquely distinguish them.
    //     //
    //     // For example, one step NW (`Distance(8 + -1)`) is a left shift by
    //     // seven while masking the contents of the A file. But seven steps E
    //     // (`Distance(7)`) is encoded the same way, but requires a left shift by
    //     // seven while masking the contents of files B through H.
    //     #[cfg(debug_assertions)] {
    //         #![allow(clippy::assertions_on_constants)]
    //         assert!(Direction::MAX_DISTANCE < 4,
    //             "distance logic is no longer sound");

    //         assert!(self.0 & 0b111 != 0b100,
    //             "directional shifts must be 3 or fewer places east/west");

    //         // this check only works because we've capped e/w shifts to
    //         // 3 in the above check
    //         assert!(self.0 <= (32 + 3) && self.0 >= (-32 - 3),
    //             "directional shifts must be 3 or fewer places north/south");
    //     }

    //     match self.0 & 0b111 {
    //         // north/south-only shifts
    //         0b000 => Bitboard::EMPTY,

    //         // eastward (left) shifts
    //         0b001 => Bitboard::FILE_H,
    //         0b010 => Bitboard::FILE_H | Bitboard::FILE_G,
    //         0b011 => Bitboard::FILE_H | Bitboard::FILE_G | Bitboard::FILE_F,

    //         // westward (right) shifts
    //         0b111 => Bitboard::FILE_A,
    //         0b110 => Bitboard::FILE_A | Bitboard::FILE_B,
    //         0b101 => Bitboard::FILE_A | Bitboard::FILE_B | Bitboard::FILE_C,

    //         // invalid (> 3 shift); we indicate all fields will be masked to
    //         // avoid having a panicking branch in release builds, but this
    //         // should be caught by the debug assertions above
    //         0b100 => Bitboard::ALL,

    //         // the above patterns are exhaustive for an 0b111 mask
    //         #[allow(unsafe_code)]
    //         _ => unsafe { unreachable_unchecked() },
    //     }
    // }

    // fn as_i8(self) -> i8 {
    //     self.into()
    // }
}

impl const From<Direction> for i8 {
    fn from(value: Direction) -> Self {
        value.0
    }
}

// impl const std::ops::Add<Direction> for Square {
//     type Output = Option<Self>;

//     #[must_use]
//     fn add(self, rhs: Direction) -> Self::Output {
//         let from = self.as_u8();
//         let step = rhs .as_i8();
//         let to   = Self::from_u8(from.wrapping_add_signed(step))?;

//         if self.distance(to) <= Direction::MAX {
//             return Some(to);
//         }

//         None
//     }
// }

// impl const std::ops::Add<i8> for Square {
//     type Output = Self;

//     #[must_use]
//     fn add(self, rhs: i8) -> Self::Output {
//         let from = self.as_u8();
//         let to   = from.wrapping_add_signed(rhs) %
//             (Square::SQUARES[Square::COUNT - 1].as_u8() + 1);

//         Self::from_u8_unchecked(to)
//     }
// }

// impl const std::ops::AddAssign<i8> for Square {
//     fn add_assign(&mut self, rhs: i8) {
//         *self = self.add(rhs);
//     }
// }

impl const std::ops::Neg for Direction {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_clone() {
        assert_eq!(Direction::W, Direction::W.clone());
    }

    #[test]
    fn direction_debug() {
        assert_eq!("Direction(0)",  format!("{:?}", Direction::NONE));
        assert_eq!("Direction(-7)", format!("{:?}", Direction::SE));
    }

    #[test]
    fn direction_neg() {
        assert_eq!(Direction::E, -Direction::W);
        assert_eq!(Direction::SSW, -Direction::NNE);
    }

    #[allow(clippy::double_neg)]
    #[test]
    fn direction_neg_twice() {
        assert_eq!(Direction::SE, --Direction::SE);
    }
}
