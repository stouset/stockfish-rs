use crate::prelude::*;

use std::ops::{Index, IndexMut};

/// A token placed on a chess board. Combines a piece with its color.
///
/// To keep this in a single byte, we pack both the color and the type of piece
/// directly into an enum. Making a struct of two values or an enum with
/// `White(piece)` and `Black(piece)` variants would both push it to two bytes.
#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[must_use]
#[repr(u8)]
pub enum Token {
    WhitePawn = 0, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing,
    BlackPawn = 8, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing,
}

impl Token {
    // TODO: `COUNT` feels wrong for expressing what this needs to since there
    // aren't actually 14 variants, but `MAX` requires adding one; is there a
    // better word for "the number of array spaces needed to hold all variants"
    pub const MAX: usize = Self::BlackKing.as_u8() as _;

    #[allow(unsafe_code)]
    const unsafe fn from_u8_unchecked(repr: u8) -> Self {
        debug_assert!(
            (repr >= Self::WhitePawn.as_u8() && repr <= Self::WhiteKing.as_u8()) ||
            (repr >= Self::BlackPawn.as_u8() && repr <= Self::BlackKing.as_u8())
        );

        std::mem::transmute(repr)
    }

    #[allow(unsafe_code)]
    const fn from_u8(repr: u8) -> Option<Self> {
        Some(match repr {
            0x0 => Self::WhitePawn,
            0x1 => Self::WhiteKnight,
            0x2 => Self::WhiteBishop,
            0x3 => Self::WhiteRook,
            0x4 => Self::WhiteQueen,
            0x5 => Self::WhiteKing,
            0x8 => Self::BlackPawn,
            0x9 => Self::BlackKnight,
            0xa => Self::BlackBishop,
            0xb => Self::BlackRook,
            0xc => Self::BlackQueen,
            0xd => Self::BlackKing,

            _   => return None,
        })
    }

    /// Parses a FEN byte ('P' is a white pawn, `n` is a black knight, etc.)
    /// into a [`Token`]. Returns [`None`] if
    #[inline]
    #[must_use]
    pub const fn from_fen(byte: u8) -> Option<Self> {
        Some(match byte {
            b'P' => Self::WhitePawn,
            b'N' => Self::WhiteKnight,
            b'B' => Self::WhiteBishop,
            b'R' => Self::WhiteRook,
            b'Q' => Self::WhiteQueen,
            b'K' => Self::WhiteKing,
            b'p' => Self::BlackPawn,
            b'n' => Self::BlackKnight,
            b'b' => Self::BlackBishop,
            b'r' => Self::BlackRook,
            b'q' => Self::BlackQueen,
            b'k' => Self::BlackKing,

            _ => return None,
        })
    }


    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn new(color: Color, piece: Piece) -> Self {
        let repr = color.as_u8() << 3 | piece.as_u8();

        unsafe_optimization!(
            Self::from_u8(repr).unwrap(),
            Self::from_u8_unchecked(repr),
        )
    }

    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn color(self) -> Color {
        let color = self.as_u8() >> 3;

        unsafe_optimization! {
            Color::from_u8(color).unwrap(),
            Color::from_u8_unchecked(color),
        }
    }

    #[allow(clippy::missing_panics_doc)] // false positive
    #[inline]
    pub const fn piece(self) -> Piece {
        let piece = self.as_u8() & 0b111;

        unsafe_optimization! {
            Piece::from_u8(piece).unwrap(),
            Piece::from_u8_unchecked(piece),
        }
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as _
    }

    #[inline]
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as _
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

impl<T> const Index<Token> for [T; Token::MAX + 1] {
    type Output = T;

    fn index(&self, index: Token) -> &Self::Output {
        self.index(index.as_usize())
    }
}

impl<T> const IndexMut<Token> for [T; Token::MAX + 1] {
    fn index_mut(&mut self, index: Token) -> &mut Self::Output {
        self.index_mut(index.as_usize())
    }
}
