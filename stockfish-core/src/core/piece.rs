use crate::prelude::*;

enumeration! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed an underscore to mimic those of [`Rank`].
    pub Piece, u8, [
        Pawn, Knight, Bishop, Rook, Queen, King,
    ]
}

impl Piece {
    /// The single-step moves available to a given piece.
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

    /// Returns true if the [`Piece`] may jump over other pieces.
    #[inline]
    #[must_use]
    pub const fn is_jumping(self) -> bool {
        self == Self::Knight
    }

    /// Returns true if the [`Piece`] slides multiple squares across the board.
    #[inline]
    #[must_use]
    pub const fn is_sliding(self) -> bool {
        self == Self::Bishop || self == Self::Rook || self == Self::Queen
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_jumping() {
        refute!(Piece::Pawn  .is_jumping());
        assert!(Piece::Knight.is_jumping());
        refute!(Piece::Bishop.is_jumping());
        refute!(Piece::Rook  .is_jumping());
        refute!(Piece::Queen .is_jumping());
        refute!(Piece::King  .is_jumping());
    }

    #[test]
    fn is_sliding() {
        refute!(Piece::Pawn  .is_sliding());
        refute!(Piece::Knight.is_sliding());
        assert!(Piece::Bishop.is_sliding());
        assert!(Piece::Rook  .is_sliding());
        assert!(Piece::Queen .is_sliding());
        refute!(Piece::King  .is_sliding());
    }
}
