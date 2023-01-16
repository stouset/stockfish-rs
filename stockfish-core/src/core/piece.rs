use crate::prelude::*;
use crate::accelerate;

enumeration! {
    /// A piece to be placed on a chess board. Combines a color with a type of
    /// token.
    ///
    /// Under the hood, the color is stored as the LSB while the type of token
    /// is stored as the three next bits. The upper four bits of the byte are
    /// unused.
    pub Piece, [
        WhitePawn,   BlackPawn,
        WhiteKnight, BlackKnight,
        WhiteBishop, BlackBishop,
        WhiteRook,   BlackRook,
        WhiteQueen,  BlackQueen,
        WhiteKing,   BlackKing,
    ]
}

impl Piece {
    const FEN: [char; Self::COUNT] = [
        'P', 'p',
        'N', 'n',
        'B', 'b',
        'R', 'r',
        'Q', 'q',
        'K', 'k',
    ];

    /// Parses a FEN byte ('P' is a white pawn, `n` is a black knight, etc.)
    /// into a [`Piece`]. Returns [`None`] if
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

    /// Instantiates a [`Piece`] from a [`Color`] and [`Token`].
    #[inline]
    pub const fn new(color: Color, token: Token) -> Self {
        let repr = token.as_u8() << 1 | color.as_u8();

        unsafe_optimization!(
            Self::from_u8(repr).unwrap(),
            Self::from_u8_unchecked(repr),
        )
    }

    /// Returns the [`Color`] of the [`Piece`].
    #[inline]
    pub const fn color(self) -> Color {
        let color = self.as_u8() & 0b1;

        unsafe_optimization! {
            Color::from_u8(color).unwrap(),
            Color::from_u8_unchecked(color),
        }
    }

    /// Returns the type of [`Token`] of the [`Piece`].
    #[inline]
    pub const fn token(self) -> Token {
        let token = self.as_u8() >> 1;

        unsafe_optimization! {
            Token::from_u8(token).unwrap(),
            Token::from_u8_unchecked(token),
        }
    }

    /// Returns a bitboard containing all for a piece on the given `square`.
    /// This is equivalent to computing the piece's `attacks` on an empty
    /// board.
    #[inline]
    pub const fn moves(self, square: Square) -> Bitboard {
        self.attacks(square, Bitboard::EMPTY)
    }

    /// Returns a bitboard containing the squares the piece attacks from the
    /// given `square`, given a `board` containing all of the squares with
    /// pieces on them that might interfere with its attack.
    #[inline]
    pub const fn attacks(self, square: Square, board: Bitboard) -> Bitboard {
        accelerate::attacks(
            self.color(),
            self.token(),
            square,
            board,
        )
    }
}

impl const From<Piece> for char {
    #[inline]
    fn from(value: Piece) -> Self {
        Piece::FEN[value]
    }
}

impl const From<Piece> for Color {
    #[inline]
    fn from(piece: Piece) -> Self {
        piece.color()
    }
}

impl const From<Piece> for Token {
    #[inline]
    fn from(piece: Piece) -> Self {
        piece.token()
    }
}


// impl const From<Token> for usize {
//     fn from(p: Token) -> Self {
//         let c:  usize = p.color().into();
//         let pt: usize = p.piece_type().into();

//         PieceType::COUNT * c + pt
//     }
// }

// impl<T> const std::ops::Index<Token> for [T; Token::COUNT] {
//     type Output = T;

//     #[inline]
//     #[must_use]
//     fn index(&self, index: Token) -> &Self::Output {
//         self.index(usize::from(index))
//     }
// }

// impl<T> const std::ops::IndexMut<Token> for [T; Token::COUNT] {
//     #[inline]
//     #[must_use]
//     fn index_mut(&mut self, index: Token) -> &mut Self::Output {
//         self.index_mut(usize::from(index))
//     }
// }
