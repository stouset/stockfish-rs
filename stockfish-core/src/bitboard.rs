use crate::prelude::*;

#[derive(Copy, Debug, Eq)]
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

    #[inline]
    #[must_use]
    pub fn into_some_square(self) -> Option<Square> {
        let z: usize = self.0.trailing_zeros().try_into().unwrap();
        let s        = Square::VARIANTS.get(z);

        println!("{self:?} {z} {s:?}");

        s.copied()
    }

    // Returns the number of squares in the bitboard.
    #[inline]
    #[must_use]
    pub const fn count(self) -> usize {
        self.0.count_ones() as _
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

impl const std::ops::Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        (!self.0).into()
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

impl const std::ops::BitXor<Self> for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl const std::ops::Shl<u8> for Bitboard {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        (self.0 << rhs).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_some_square() {
        for s in Square::into_iter() {
            assert_eq!(Some(s), Bitboard::from(s).into_some_square());
        }

        assert_eq!(None, Bitboard::EMPTY.into_some_square());
        assert!(Bitboard::DARK_SQUARES.into_some_square().is_some());
    }
}
