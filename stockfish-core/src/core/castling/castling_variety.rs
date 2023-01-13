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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color() {
        assert_eq!(Color::White, CastlingVariety::WhiteKingside .color());
        assert_eq!(Color::White, CastlingVariety::WhiteQueenside.color());
        assert_eq!(Color::Black, CastlingVariety::BlackKingside .color());
        assert_eq!(Color::Black, CastlingVariety::BlackQueenside.color());
    }


    #[test]
    fn king_destination() {
        assert_eq!(Square::G1, CastlingVariety::WhiteKingside .king_destination());
        assert_eq!(Square::C1, CastlingVariety::WhiteQueenside.king_destination());
        assert_eq!(Square::G8, CastlingVariety::BlackKingside .king_destination());
        assert_eq!(Square::C8, CastlingVariety::BlackQueenside.king_destination());
    }

    #[test]
    fn rook_destination() {
        assert_eq!(Square::F1, CastlingVariety::WhiteKingside .rook_destination());
        assert_eq!(Square::D1, CastlingVariety::WhiteQueenside.rook_destination());
        assert_eq!(Square::F8, CastlingVariety::BlackKingside .rook_destination());
        assert_eq!(Square::D8, CastlingVariety::BlackQueenside.rook_destination());
    }

    #[test]
    fn rights() {
        assert_eq!(CastlingRights::WHITE_OO,  CastlingVariety::WhiteKingside .rights());
        assert_eq!(CastlingRights::WHITE_OOO, CastlingVariety::WhiteQueenside.rights());
        assert_eq!(CastlingRights::BLACK_OO,  CastlingVariety::BlackKingside .rights());
        assert_eq!(CastlingRights::BLACK_OOO, CastlingVariety::BlackQueenside.rights());
    }
}
