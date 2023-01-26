mod fen;

use stockfish_core::prelude::*;

use core::ops::Index;

#[allow(missing_copy_implementations)] // type is too large to want copying
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    // basic board state
    ruleset:   Ruleset,
    turn:      Color,
    board:     Board,
    ply:       u8,
    halfmoves: u8,

    // castling state
    castling_paths:     [Option<CastlingPath>; CastlingVariety::COUNT],
    castling_by_square: [CastlingRights; Square::COUNT],

    // internal bitboards
    bb_all:      Bitboard,
    bb_by_color: [Bitboard; Color::COUNT],
    bb_by_piece: [Bitboard; Token::COUNT],

    // internal metrics
    count_by_color: [u8; Color::COUNT],
    count_by_token: [u8; Piece::COUNT],

    // TODO: stuff from the StateInfo stockfish struct that eventually doesn't
    // go here
    castling_rights: CastlingRights,
    en_passant:      Option<Square>,
}

impl Position {
    pub fn empty(ruleset: Ruleset) -> Self {
        Self {
            ruleset,
            turn:      Color::White,
            board:     Board::EMPTY,
            ply:       0,
            halfmoves: 0,

            castling_paths:     Default::default(),
            castling_by_square: [CastlingRights::default(); Square::COUNT],

            bb_all:      Bitboard::EMPTY,
            bb_by_color: [Bitboard::EMPTY; Color::COUNT],
            bb_by_piece: [Bitboard::EMPTY; Token::COUNT],

            count_by_token: [0; Piece::COUNT],
            count_by_color: [0; Color::COUNT],

            castling_rights: CastlingRights::NONE,
            en_passant:      None,
        }
    }

    #[inline]
    pub fn emplace(&mut self, piece: Piece, square: Square) {
        self.board[square] = Some(piece);

        self.bb_all                     |= square;
        self.bb_by_color[piece.color()] |= square;
        self.bb_by_piece[piece.token()] |= square;

        self.count_by_color[piece.color()] += 1;
        self.count_by_token[piece]         += 1;

        // TODO: piece-square tables
        // psq += PSQT::psq[pc][s];
    }

    #[inline]
    pub fn remove(&mut self, square: Square) -> Option<Piece> {
        let piece = self.board[square].take()?;

        self.bb_all                     ^= square;
        self.bb_by_piece[piece.token()] ^= square;
        self.bb_by_color[piece.color()] ^= square;

        self.count_by_color[piece.color()] -= 1;
        self.count_by_token[piece]         -= 1;

        // TODO: piece-square tables
        // psq -= PSQT::psq[pc][s];

        Some(piece)
    }

    #[inline]
    pub const fn bitboard(&self) -> Bitboard {
        self.bb_all
    }

    #[inline]
    pub fn bitboard_for_color(&self, color: Color) -> Bitboard {
        self.bb_by_color[color]
    }

    #[inline]
    pub fn bitboard_for_piece(&self, token: Token) -> Bitboard {
        self.bb_by_piece[token]
    }

    #[inline]
    pub fn bitboard_for_token(&self, piece: Piece) -> Bitboard {
        self.bb_by_color[piece.color()] & self.bb_by_piece[piece.token()]
    }
}

impl Index<Square> for Position {
    type Output = Option<Piece>;

    #[inline]
    fn index(&self, index: Square) -> &Self::Output {
        self.board.index(index)
    }
}
