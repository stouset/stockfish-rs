use super::Direction;

c_style_enum! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed an underscore to mimic those of [`Rank`].
    pub PieceType, u8, 6; [
        Pawn, Knight, Bishop, Rook, Queen, King,
    ]
}

impl PieceType {
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

    #[inline]
    #[must_use]
    pub const fn is_sliding(self) -> bool {
        self == Self::Bishop
            || self == Self::Rook
            || self == Self::Queen
    }
}
