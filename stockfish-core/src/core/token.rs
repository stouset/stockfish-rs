use crate::prelude::*;

use core::ops::BitOr;

enumeration! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed an underscore to mimic those of [`Rank`].
    pub Token, [
        Pawn, Knight, Bishop, Rook, Queen, King,
    ]
}

impl Token {
    /// The single-step moves available to a given token.
    pub const STEPS: [&'static [Direction]; Self::COUNT] = [
        // pawn
        &[],

        // knight
        &[ Direction::NNW, Direction::NNE, Direction::ENE, Direction::ESE,
           Direction::SSE, Direction::SSW, Direction::WSW, Direction::WNW ],

        // bishop
        &[ Direction::NW, Direction::NE, Direction::SE, Direction::SW ],

        // rook
        &[ Direction::N,  Direction::E,  Direction::S,  Direction::W ],

        // queen
        &[ Direction::NW, Direction::N, Direction::NE, Direction::E,
           Direction::SE, Direction::S, Direction::SW, Direction::W ],

        // king
        &[ Direction::NW, Direction::N, Direction::NE, Direction::E,
           Direction::SE, Direction::S, Direction::SW, Direction::W ],
    ];

    /// Returns true if the [`Token`] may jump over other pieces on the board.
    #[inline]
    #[must_use]
    pub const fn is_jumping(self) -> bool {
        self == Self::Knight
    }

    /// Returns true if the [`Token`] slides multiple squares across the board.
    #[inline]
    #[must_use]
    pub const fn is_sliding(self) -> bool {
        self == Self::Bishop || self == Self::Rook || self == Self::Queen
    }

    /// Returns all possible moves of the token from a given `square` on the
    /// board.
    #[inline]
    pub const fn moves(self, square: Square) -> Bitboard {
        // TODO: ensure this optimizes correctly in release builds and doesn't
        // result in duplicated branching behind the function call
        match self {
            // pawns are the only piece that depend on their color for their
            // direction of travel
            Token::Pawn =>
                (Color::White | self).moves(square) |
                (Color::Black | self).moves(square),

            // all other pieces travel in a direction independent of their
            // color, so either color works
            _ =>
                (Color::White | self).moves(square),
        }
    }

    /// Returns a bitboard containing the squares this piece attacks from the
    /// given `square`, given an `occupancy` bitboard containing all of the
    /// squares with pieces on them that might interfere with its attack.
    #[inline]
    pub const fn attacks(self, square: Square, occupancy: Bitboard) -> Bitboard {
        // TODO: ensure this optimizes correctly in release builds and doesn't
        // result in duplicated branching behind the function call
        match self {
            // pawns are the only piece that depend on their color for their
            // direction of travel
            Token::Pawn =>
                (Color::White | self).attacks(square, occupancy) |
                (Color::Black | self).attacks(square, occupancy),

            // all other pieces travel in a direction independent of their
            // color, so either color works
            _ => (Color::White | self).attacks(square, occupancy),
        }
    }
}

impl const BitOr<Color> for Token {
    type Output = Piece;

    #[inline]
    fn bitor(self, color: Color) -> Self::Output {
        Piece::new(color, self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_jumping() {
        refute!(Token::Pawn  .is_jumping());
        assert!(Token::Knight.is_jumping());
        refute!(Token::Bishop.is_jumping());
        refute!(Token::Rook  .is_jumping());
        refute!(Token::Queen .is_jumping());
        refute!(Token::King  .is_jumping());
    }

    #[test]
    fn is_sliding() {
        refute!(Token::Pawn  .is_sliding());
        refute!(Token::Knight.is_sliding());
        assert!(Token::Bishop.is_sliding());
        assert!(Token::Rook  .is_sliding());
        assert!(Token::Queen .is_sliding());
        refute!(Token::King  .is_sliding());
    }

    #[test]
    fn bitor_color() {
        assert_eq!(Piece::WhiteKing, Token::King | Color::White);
        assert_eq!(Piece::BlackRook, Token::Rook | Color::Black);
    }
}
