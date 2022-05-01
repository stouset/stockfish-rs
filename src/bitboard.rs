//! Bitboards for fast lookups.

#![allow(missing_docs)]
#![allow(clippy::identity_op)]

use self::constmuck::bb;
use crate::types::{File, Square};

pub const BB_NO_SQUARES:   Bitboard = 0.into();
pub const BB_ALL_SQUARES:  Bitboard = u64::MAX.into();
pub const BB_DARK_SQUARES: Bitboard = 0xAA_55_AA_55_AA_55_AA_55.into();

pub const BB_FILE_A: Bitboard = 0x01_01_01_01_01_01_01_01.into();
pub const BB_FILE_B: Bitboard = BB_FILE_A << 1;
pub const BB_FILE_C: Bitboard = BB_FILE_A << 2;
pub const BB_FILE_D: Bitboard = BB_FILE_A << 3;
pub const BB_FILE_E: Bitboard = BB_FILE_A << 4;
pub const BB_FILE_F: Bitboard = BB_FILE_A << 5;
pub const BB_FILE_G: Bitboard = BB_FILE_A << 6;
pub const BB_FILE_H: Bitboard = BB_FILE_A << 7;

pub const BB_RANK_1: Bitboard = 0xFF.into();
pub const BB_RANK_2: Bitboard = BB_RANK_1 << (8 * 1);
pub const BB_RANK_3: Bitboard = BB_RANK_1 << (8 * 2);
pub const BB_RANK_4: Bitboard = BB_RANK_1 << (8 * 3);
pub const BB_RANK_5: Bitboard = BB_RANK_1 << (8 * 4);
pub const BB_RANK_6: Bitboard = BB_RANK_1 << (8 * 5);
pub const BB_RANK_7: Bitboard = BB_RANK_1 << (8 * 6);
pub const BB_RANK_8: Bitboard = BB_RANK_1 << (8 * 7);

pub const BB_QUEEN_SIDE:   Bitboard = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
pub const BB_CENTER_FILES: Bitboard = BB_FILE_C | BB_FILE_D | BB_FILE_E | BB_FILE_F;
pub const BB_KING_SIDE:    Bitboard = BB_FILE_E | BB_FILE_F | BB_FILE_G | BB_FILE_H;
pub const BB_CENTER:       Bitboard = (BB_FILE_D | BB_FILE_E) & (BB_RANK_4 | BB_RANK_5);

pub const BB_KING_FLANK: [Bitboard; File::COUNT] = [
    BB_QUEEN_SIDE ^ BB_FILE_D, BB_QUEEN_SIDE,
    BB_QUEEN_SIDE,             BB_CENTER_FILES,
    BB_CENTER_FILES,           BB_KING_SIDE,
    BB_KING_SIDE,              BB_KING_SIDE ^ BB_FILE_E,
];

/// The number of bits set for any given 16-bit value.
pub const POPCNT16: [u8; 1 << 16] = bb(*include_bytes!(env!("STOCKFISH_RS_BB_POPCNT_16")));

/// The number of moves necessary to walk a King from one square to the other.
pub const SQUARE_DISTANCE: [[u8; Square::COUNT]; Square::COUNT] = bb(*include_bytes!(env!("STOCKFISH_RS_BB_SQUARE_DISTANCE")));

/// Conversion from a [`Square`] index to a [`Bitboard`] with only that
/// square set.
pub const BB_SQUARE: [Bitboard; Square::COUNT] = bb(*include_bytes!(env!("STOCKFISH_RS_BB_SQUARE")));

// pub const BB_BETWEEN: [[Bitboard; SQUARES]; SQUARES] = cast(*include_bytes!(env!("STOCKFISH_RS_BB_BETWEEN")));
// extern Bitboard BetweenBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard LineBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard PseudoAttacks[PIECE_TYPE_NB][SQUARE_NB];
// extern Bitboard PawnAttacks[COLOR_NB][SQUARE_NB];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Bitboard(u64);

impl Bitboard {
    /// Returns [`true`] if the [`Bitboard`] does not contain any spaces.
    #[inline]
    #[must_use]
    pub fn is_none(self) -> bool {
        self == BB_NO_SQUARES
    }

    //// Returns [`true`] if the [`Bitboard`] contains any spaces.
    #[inline]
    #[must_use]
    pub fn is_any(self) -> bool {
        !self.is_none()
    }

    /// Returns [`true`] if the [`Bitboard`] contains more than one space.
    #[inline]
    #[must_use]
    pub fn is_many(self) -> bool {
        // If more than one bit is set, subtracting one will flip the
        // lowest set bit and any bits lower than it. But any *higher*
        // set bits will be unchanged.
        //
        // In the case of zero, all bits will be flipped.
        self.0 & (self.0 - 1) != 0
    }
}

impl const From<u64> for Bitboard {
    #[inline]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl const From<Square> for Bitboard {
    #[inline]
    fn from(s: Square) -> Self {
        BB_SQUARE[u8::from(s) as usize]
    }
}

impl const From<Bitboard> for u64 {
    #[inline]
    fn from(bb: Bitboard) -> Self {
        bb.0
    }
}

impl const std::ops::BitAnd<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.0.bitand(rhs.0).into()
    }
}

impl const std::ops::BitAnd<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Square) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl const std::ops::BitOr<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.0.bitor(rhs.0).into()
    }
}

impl const std::ops::BitOr<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Square) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl const std::ops::BitXor<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl const std::ops::BitXor<Square> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Square) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl const std::ops::Shl<u8> for Bitboard {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        (self.0 << rhs).into()
    }
}

// Implementing these traits is unsafe as Bitboard is a simple newtype
// around u64.
#[allow(unsafe_code)]
mod constmuck {
    unsafe impl constmuck::Zeroable for super::Bitboard {}
    unsafe impl constmuck::Pod      for super::Bitboard {}

    pub(crate) const fn bb<T: constmuck::Pod, U: constmuck::Pod>(from: T) -> U {
        constmuck::cast(from, constmuck::infer!())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Square;

    #[test]
    fn pop_cnt_16_is_correct() {
        for (i, v) in POPCNT16.iter().enumerate() {
            assert_eq!(v, &i.count_ones().try_into().unwrap());
        }
    }

    #[test]
    fn square_distance_is_correct() {
        for (i, s1) in Square::iter().enumerate() {
            for (j, s2) in Square::iter().enumerate() {
                assert_eq!(SQUARE_DISTANCE[i][j], std::cmp::max(
                    s1.distance_files(s2),
                    s1.distance_ranks(s2),
                ));
            }
        }
    }

    #[test]
    fn square_is_correct() {
        for (i, s) in Square::iter().enumerate() {
            assert_eq!(BB_SQUARE[i], (1 << u8::from(s)).into());
        }
    }
}
