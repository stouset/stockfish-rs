use crate::prelude::*;

use std::ops::BitAnd;

use bitflags::bitflags;

bitflags! {
    /// A set of bitflags representing the ability to perform various castling
    /// operations.
    #[derive_const(Default)]
    #[must_use]
    pub struct CastlingRights: u8 {
        /// No castling is available.
        const NONE = 0;

        /// White may castle kingside.
        const WHITE_OO  = 1 << 0;

        /// White may castle queenside.
        const WHITE_OOO = 1 << 1;

        /// Black may castle kingside.
        const BLACK_OO  = 1 << 2;

        /// Black may castle queenside.
        const BLACK_OOO = 1 << 3;

        /// White may castle in either direction.
        const WHITE      = Self::WHITE_OO .bits | Self::WHITE_OOO.bits;

        /// Black may castle in either direction.
        const BLACK      = Self::BLACK_OO .bits | Self::BLACK_OOO.bits;

        /// Either side may castle kingside.
        const KING_SIDE  = Self::WHITE_OO .bits | Self::BLACK_OO .bits;

        /// Either side may castle queenside.
        const QUEEN_SIDE = Self::WHITE_OOO.bits | Self::BLACK_OOO.bits;

        /// Either side may castle in either direction.
        const ANY        = Self::WHITE    .bits | Self::BLACK    .bits;
    }
}

impl BitAnd<Color> for CastlingRights {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Color) -> Self::Output {
        match rhs {
            Color::White => self & Self::WHITE,
            Color::Black => self & Self::BLACK,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(CastlingRights::NONE, CastlingRights::default());
    }

    #[test]
    fn bitand_color() {
        assert_eq!(CastlingRights::WHITE_OOO, CastlingRights::QUEEN_SIDE & Color::White);
        assert_eq!(CastlingRights::BLACK_OO,  CastlingRights::KING_SIDE  & Color::Black);
    }
}
