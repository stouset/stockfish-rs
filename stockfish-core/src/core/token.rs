use crate::prelude::*;
use crate::accelerate;

enumeration! {
    /// A token to be placed on a chess board. Combines a color with a type of
    /// piece.
    ///
    /// Under the hood, the color is stored as the LSB while the type of piece
    /// is stored as the three next bits. The upper four bits of the byte are
    /// unused.
    pub Token, [
        WhitePawn,   BlackPawn,
        WhiteKnight, BlackKnight,
        WhiteBishop, BlackBishop,
        WhiteRook,   BlackRook,
        WhiteQueen,  BlackQueen,
        WhiteKing,   BlackKing,
    ]
}

impl Token {
    const FEN: [char; Self::COUNT] = [
        'P', 'p',
        'N', 'n',
        'B', 'b',
        'R', 'r',
        'Q', 'q',
        'K', 'k',
    ];

    /// Parses a FEN byte ('P' is a white pawn, `n` is a black knight, etc.)
    /// into a [`Token`]. Returns [`None`] if
    #[inline]
    #[must_use]
    pub const fn from_fen(byte: u8) -> Option<Self> {
        Some(match byte {
            b'P' => Self::WhitePawn,   b'p' => Self::BlackPawn,
            b'N' => Self::WhiteKnight, b'n' => Self::BlackKnight,
            b'B' => Self::WhiteBishop, b'b' => Self::BlackBishop,
            b'R' => Self::WhiteRook,   b'r' => Self::BlackRook,
            b'Q' => Self::WhiteQueen,  b'q' => Self::BlackQueen,
            b'K' => Self::WhiteKing,   b'k' => Self::BlackKing,

            _ => return None,
        })
    }

    /// Instantiates a [`Token`] from a [`Color`] and [`Piece`].
    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn new(color: Color, piece: Piece) -> Self {
        let repr = piece.as_u8() << 1 | color.as_u8();

        unsafe_optimization!(
            Self::from_u8(repr).unwrap(),
            Self::from_u8_unchecked(repr),
        )
    }

    /// Returns the [`Color`] of the [`Token`].
    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn color(self) -> Color {
        let color = self.as_u8() & 0b1;

        unsafe_optimization! {
            Color::from_u8(color).unwrap(),
            Color::from_u8_unchecked(color),
        }
    }

    /// Returns the type of [`Piece`] of the [`Token`].
    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn piece(self) -> Piece {
        let piece = self.as_u8() >> 1;

        unsafe_optimization! {
            Piece::from_u8(piece).unwrap(),
            Piece::from_u8_unchecked(piece),
        }
    }

    /// Returns a bitboard containing the squares the token attacks from the
    /// given `position`, given a `board` containing the pieces that might
    /// interfere with its attack.
    #[inline]
    pub const fn attacks(self, position: Square, board: Bitboard) -> Bitboard {
        accelerate::attacks(
            self.color(),
            self.piece(),
            position,
            board,
        )
    }
}

impl const From<Token> for char {
    #[inline]
    fn from(value: Token) -> Self {
        Token::FEN[value]
    }
}

impl const From<Token> for Color {
    #[inline]
    fn from(token: Token) -> Self {
        token.color()
    }
}

impl const From<Token> for Piece {
    #[inline]
    fn from(token: Token) -> Self {
        token.piece()
    }
}

