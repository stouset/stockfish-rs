use crate::prelude::*;

enumeration! {
    /// Represents a complete type of castling, including the color to castle
    /// and the side to which castling will occur.
    pub CastlingVariety, [
        WhiteKingside, WhiteQueenside,
        BlackKingside, BlackQueenside,
    ]
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

    const SIDES: [CastlingSide; Self::COUNT] = [
        CastlingSide::King, CastlingSide::Queen,
        CastlingSide::King, CastlingSide::Queen,
    ];

    const RIGHTS: [CastlingRights; Self::COUNT] = [
        CastlingRights::WHITE_OO, CastlingRights::WHITE_OOO,
        CastlingRights::BLACK_OO, CastlingRights::BLACK_OOO,
    ];

    /// Returns the variety of castling for the given `color` and `side`.
    #[inline]
    pub const fn new(color: Color, side: CastlingSide) -> Self {
        Self::KINDS[color][side]
    }

    /// Returns the color of the player involved in this castling variety
    #[inline]
    pub const fn color(self) -> Color {
        Self::COLORS[self]
    }

    /// Returns the side of the board this castling variety will occur towards.
    #[inline]
    pub const fn side(self) -> CastlingSide {
        Self::SIDES[self]
    }

    /// Returns the square the king finishes on.
    #[inline]
    pub const fn king_destination(self) -> Square {
        Self::KING_DESTINATIONS[self]
    }

    /// Returns the square the rook finishes on.
    #[inline]
    pub const fn rook_destination(self) -> Square {
        Self::ROOK_DESTINATIONS[self]
    }

    /// Returns the individual rights required for this castling variety to be
    /// eligible.
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
        assert!(CastlingVariety::WhiteKingside .color().is_white());
        assert!(CastlingVariety::WhiteQueenside.color().is_white());
        assert!(CastlingVariety::BlackKingside .color().is_black());
        assert!(CastlingVariety::BlackQueenside.color().is_black());
    }

    #[test]
    fn side() {
        assert_eq!(CastlingSide::King,  CastlingVariety::WhiteKingside .side());
        assert_eq!(CastlingSide::Queen, CastlingVariety::WhiteQueenside.side());
        assert_eq!(CastlingSide::King,  CastlingVariety::BlackKingside .side());
        assert_eq!(CastlingSide::Queen, CastlingVariety::BlackQueenside.side());
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
