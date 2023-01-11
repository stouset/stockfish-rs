use crate::prelude::*;

enumeration! {
    pub CastlingSide, [ Kingside, Queenside ]
}

impl CastlingSide {
    #[inline]
    pub const fn detect(king: File, rook: File) -> Self {
        debug_assert!(king != rook);

        if king < rook { Self::Kingside } else { Self::Queenside }
    }
}

