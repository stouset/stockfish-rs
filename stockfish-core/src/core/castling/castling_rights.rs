use crate::prelude::*;

use std::ops::BitAnd;

use bitflags::bitflags;

bitflags! {
    #[derive_const(Default)]
    #[must_use]
    pub struct CastlingRights: u8 {
        const NONE = 0;

        const WHITE_OO  = 1 << 0;
        const WHITE_OOO = 1 << 1;
        const BLACK_OO  = 1 << 2;
        const BLACK_OOO = 1 << 3;

        const WHITE      = Self::WHITE_OO .bits | Self::WHITE_OOO.bits;
        const BLACK      = Self::BLACK_OO .bits | Self::BLACK_OOO.bits;
        const KING_SIDE  = Self::WHITE_OO .bits | Self::BLACK_OO .bits;
        const QUEEN_SIDE = Self::WHITE_OOO.bits | Self::BLACK_OOO.bits;
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
