use crate::prelude::*;

enumeration! {
    pub CastlingVariety, [ WhiteKingside, WhiteQueenside, BlackKingside, BlackQueenside ]
}

impl CastlingVariety {
    const COLORS: [Color; Self::COUNT] = [
        Color::White, Color::White,
        Color::Black, Color::Black,
    ];

    const KING_DESTINATIONS: [Square; Self::COUNT] = [
        Square::G1, Square::C1,
        Square::G8, Square::C8,
    ];

    const ROOK_DESTINATIONS: [Square; Self::COUNT] = [
        Square::F1, Square::D1,
        Square::F8, Square::D8,
    ];

    const KINDS: [[Self; CastlingSide::COUNT]; Color::COUNT] = [
        [ Self::WhiteKingside, Self::WhiteQueenside ],
        [ Self::BlackKingside, Self::BlackQueenside ],
    ];

    const RIGHTS: [CastlingRights; Self::COUNT] = [
        CastlingRights::WHITE_OO, CastlingRights::WHITE_OOO,
        CastlingRights::BLACK_OO, CastlingRights::BLACK_OOO,
    ];

    #[inline]
    pub const fn new(color: Color, side: CastlingSide) -> Self {
        Self::KINDS[color][side]
    }

    #[inline]
    pub const fn color(self) -> Color {
        Self::COLORS[self]
    }

    #[inline]
    pub const fn king_destination(self) -> Square {
        Self::KING_DESTINATIONS[self]
    }

    #[inline]
    pub const fn rook_destination(self) -> Square {
        Self::ROOK_DESTINATIONS[self]
    }

    #[inline]
    pub const fn rights(self) -> CastlingRights {
        Self::RIGHTS[self]
    }
}
