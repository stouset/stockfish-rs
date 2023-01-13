mod fen;

use stockfish_core::prelude::*;

#[allow(missing_copy_implementations)] // type is too large to want copying
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    // basic board state
    ruleset: Ruleset,
    turn:    Color,
    board:   Board,

    // castling state
    castling_paths:     [Option<CastlingPath>; CastlingVariety::COUNT],
    castling_by_square: [CastlingRights; Square::COUNT],

    // internal bitboards
    bb_all:      Bitboard,
    bb_by_piece: [Bitboard; Piece::COUNT],
    bb_by_color: [Bitboard; Color::COUNT],

    // internal metrics
    count_by_token: [u8; Token::MAX + 1],
    count_by_color: [u8; Color::COUNT],

    // TODO: stuff from the StateInfo stockfish struct that eventually doesn't
    // go here
    castling_rights: CastlingRights,
    en_passant:      Option<Square>,
}

impl Position {
    pub fn empty(ruleset: Ruleset) -> Self {
        Self {
            ruleset,
            turn:  Color::White,
            board: Board::EMPTY,

            castling_paths:     Default::default(),
            castling_by_square: [CastlingRights::default(); Square::COUNT],

            bb_all:      Bitboard::EMPTY,
            bb_by_piece: [Bitboard::EMPTY; Piece::COUNT],
            bb_by_color: [Bitboard::EMPTY; Color::COUNT],

            count_by_token: [0; Token::MAX + 1],
            count_by_color: [0; Color::COUNT],

            castling_rights: CastlingRights::NONE,
            en_passant:      None,
        }
    }

    #[inline]
    pub fn emplace(&mut self, token: Token, square: Square) {
        self.board[square] = Some(token);

        self.bb_all                     |= square;
        self.bb_by_piece[token.piece()] |= square;
        self.bb_by_color[token.color()] |= square;

        self.count_by_token[token]         += 1;
        self.count_by_color[token.color()] += 1;

        // TODO: piece-square tables
        // psq += PSQT::psq[pc][s];
    }

    #[inline]
    pub fn bitboard(&self) -> Bitboard {
        self.bb_all
    }

    #[inline]
    pub fn bitboard_for_color(&self, color: Color) -> Bitboard {
        self.bb_by_color[color]
    }

    #[inline]
    pub fn bitboard_for_piece(&self, piece: Piece) -> Bitboard {
        self.bb_by_piece[piece]
    }

    #[inline]
    pub fn bitboard_for_token(&self, token: Token) -> Bitboard {
        self.bb_by_color[token.color()] & self.bb_by_piece[token.piece()]
    }

    #[inline]
    pub fn as_bitboard(&self) -> Bitboard {
        self.bb_all
    }
}
