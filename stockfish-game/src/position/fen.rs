use crate::prelude::*;
use stockfish_core::prelude::*;

impl Position {
    /// Parses a `fen` (Forsyth-Edward Notation) string into a [`Position`].
    ///
    /// The FEN string is assumed to be valid and meaningful. If it is not, we
    /// try to do our best, but no guarantee is made that the board state will
    /// be legal or consistent.
    pub fn from_fen(fen: &[u8], ruleset: Ruleset) -> Self {
        // A FEN string defines a particular position using only the ASCII
        // character set.
        //
        // A FEN string contains six fields separated by a space. The fields
        // are:
        //
        // 1) Piece placement (from white's perspective). Each rank is
        //    described, starting with rank 8 and ending with rank 1. Within
        //    each rank, the contents of each square are described from file A
        //    through file H. Following the Standard Algebraic Notation (SAN),
        //    each piece is identified by a single letter taken from the
        //    standard English names. White pieces are designated using
        //    upper-case letters ("PNBRQK") whilst Black uses lowercase
        //    ("pnbrqk"). Blank squares are noted using digits 1 through 8 (the
        //    number of blank squares), and "/" separates ranks.
        //
        // 2) Active color. "w" means white moves next, "b" means black.
        //
        // 3) Castling availability. If neither side can castle, this is "-".
        //    Otherwise, this has one or more letters: "K" (White can castle
        //    kingside), "Q" (White can castle queenside), "k" (Black can castle
        //    kingside), and/or "q" (Black can castle queenside).
        //
        // 4) En passant target square (in algebraic notation). If there's no en
        //    passant target square, this is "-". If a pawn has just made a
        //    2-square move, this is the position "behind" the pawn. Following
        //    X-FEN standard, this is recorded only if there is a pawn in
        //    position to make an en passant capture, and if there really is a
        //    pawn that might have advanced two squares.
        //
        // 5) Halfmove clock. This is the number of halfmoves since the last
        //    pawn advance or capture. This is used to determine if a draw can
        //    be claimed under the fifty-move rule.
        //
        // 6) Fullmove number. The number of the full move. It starts at 1, and
        //    is incremented after Black's move.
        let mut position = Position::empty(ruleset);
        let mut fields   = fen.split(|b| *b == b' ');

        let board    = parse_board(fields.next().unwrap_or_default());
        let turn     = parse_turn(fields.next().unwrap_or_default());
        let castling = parse_castling(fields.next().unwrap_or_default(), &board);

        for (square, token) in board.iter() {
            position.emplace(token, square);
        }

        position.turn           = turn;
        position.castling_paths = castling;

        for path in castling.iter().flatten() {
            position.castling_by_square[path.king_origin()] |= path.rights();
            position.castling_by_square[path.rook_origin()] |= path.rights();
        }

        position
    }
}

fn parse_board(bytes: &[u8]) -> Board {
    let mut board = Board::EMPTY;

    // TODO: This requires breaking the abstraction of a Square by using
    // intimate knowldege of the underlying numerical representation. This is
    // partially a consequence of Direction not being able to encode (by design)
    // arbitrary-distance jumps on the board.
    let _ = bytes.iter().copied().fold((56_usize, 56_usize), |(origin, square), byte| {
        // TODO: not overflowing the board or a rank should be validated on
        // release builds by returning a result and not by panicking
        debug_assert!(origin          <= 64);
        debug_assert!(square          <= 64);
        debug_assert!(square - origin <=  8);

        match byte {
            // a slash indicates the end of a rank, so we reset to the first
            // file one rank lower
            b'/' => (origin - 8, origin - 8),

            // 1-8 indicates that number of empty squares, so we skip that
            // number of files
            b'1'..=b'8' => (origin, square + (byte - b'0') as usize),

            // any other token should be treated as a piece
            _ => {
                // TODO: not overflowing the board or rank should be
                // validated on release builds by returning a result
                debug_assert!(square          < 64);
                debug_assert!(square - origin < 8);

                board[square] = Token::from_fen(byte);

                (origin, square + 1)
            }
        }
    });

    board
}

fn parse_turn(bytes: &[u8]) -> Color {
    debug_assert_eq!(1, bytes.len());
    debug_assert!(bytes[0] == b'w' || bytes[0] == b'b');

    match bytes.first().copied().unwrap_or(b'w') {
        b'w' => Color::White,
        b'b' => Color::Black,
        _    => unreachable!(),
    }
}

fn parse_castling(bytes: &[u8], board: &Board) -> [Option<CastlingPath>; 4] {
    let mut paths = [None; 4];

    for byte in bytes {
        let color = if byte.is_ascii_uppercase() { Color::White } else { Color::Black };
        let king  = Token::new(color, Piece::King);
        let rook  = Token::new(color, Piece::Rook);
        let rank  = color.rank();

        // TODO: this iterates over the board which is probably fine for setting
        // up a FEN position, but ideally we'd have already constructed the
        // position's bitboards and could look it up in O(1)
        let k_file = board.search(rank.into_iter(), king).map(Square::file);

        // search for a rook on the home file starting from the relevant side
        //
        // TODO: stop looking once we hit the square the king is on
        // TODO: actually confirm the rook exists on the square for X-FEN files
        //
        // (doing both of the above is surprisingly annoying and not really
        // worth it right now)
        let r_file = match byte {
            b'K' | b'k' => board.search(rank.into_iter().rev(), rook).map(Square::file),
            b'Q' | b'q' => board.search(rank.into_iter(),       rook).map(Square::file),

            b'A'..=b'H' | b'a'..= b'h' => File::from_fen(*byte),

            _ => continue,
        };

        let Some(k) = k_file else { continue };
        let Some(r) = r_file else { continue };

        let path    = CastlingPath::new(color, k, r);
        let variety = path.variety;

        paths[variety] = Some(path);
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board_empty_position() {
        let fen   = b"8/8/8/8/8/8/8/8";
        let board = parse_board(fen);

        assert_eq!(Board::EMPTY, board);
    }

    #[test]
    fn parse_board_start_position() {
        let fen   = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board = parse_board(fen);

        assert_eq!(board!(
            r n b q k b n r
            p p p p p p p p
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            P P P P P P P P
            R N B Q K B N R
        ), board);
    }

    #[test]
    fn parse_board_example_board_1() {
        let fen   = b"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R";
        let board = parse_board(fen);

        assert_eq!(board!(
            r n b q k b n r
            p p _ p p p p p
            _ _ _ _ _ _ _ _
            _ _ p _ _ _ _ _
            _ _ _ _ P _ _ _
            _ _ _ _ _ N _ _
            P P P P _ P P P
            R N B Q K B _ R
        ), board);
    }

    #[test]
    fn parse_board_example_board_najdorf() {
        let fen   = b"rn1q1rk1/1p2bppp/p2pbn2/4p3/4P3/1NN1BP2/PPPQ2PP/2KR1B1R";
        let board = parse_board(fen);

        assert_eq!(board!(
            r n _ q _ r k _
            _ p _ _ b p p p
            p _ _ p b n _ _
            _ _ _ _ p _ _ _
            _ _ _ _ P _ _ _
            _ N N _ B P _ _
            P P P Q _ _ P P
            _ _ K R _ B _ R
        ), board);
    }

    #[test]
    fn parse_board_sets_none_on_invalid_piece() {
        let fen   = b"!b rxN";
        let board = parse_board(fen);

        assert_eq!(board!(
            _ b _ r _ N _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        ), board);
    }

    #[test]
    #[should_panic]
    fn parse_board_panics_on_wide_rank() {
        let fen = b"rnbqkbnrr/8/8/8/8/8/8/8";
        let _   = parse_board(fen);
    }

    #[test]
    #[should_panic]
    fn parse_board_panics_on_wide_partial_rank() {
        let fen = b"p8/8/8/8/8/8/8/8";
        let _   = parse_board(fen);
    }

    #[test]
    fn parse_board_allows_short_rank() {
        let fen   = b"r/8/8/8/8/8/8/k";
        let board = parse_board(fen);

        assert_eq!(board!(
            r _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            k _ _ _ _ _ _ _
        ), board);
    }

    #[test]
    fn parse_board_allows_short_partial_rank() {
        let fen   = b"r2/8/8/8/8/8/8/k";
        let board = parse_board(fen);

        assert_eq!(board!(
            r _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            k _ _ _ _ _ _ _
        ), board);
    }

    #[test]
    fn parse_board_allows_partial_rank() {
        let fen   = b"8/1Q5Q/k3NP2/8/8/7p/6P1/8";
        let board = parse_board(fen);

        assert_eq!(board!(
            _ _ _ _ _ _ _ _
            _ Q _ _ _ _ _ Q
            k _ _ _ N P _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ p
            _ _ _ _ _ _ P _
            _ _ _ _ _ _ _ _
        ), board);
    }

    #[test]
    fn parse_turn_white() {
        let fen  = b"w";
        let turn = parse_turn(fen);

        assert_eq!(Color::White, turn);
    }

    #[test]
    fn parse_turn_black() {
        let fen  = b"b";
        let turn = parse_turn(fen);

        assert_eq!(Color::Black, turn);
    }

    #[test]
    #[should_panic]
    fn parse_turn_too_short() {
        let fen = b"";
        let _   = parse_turn(fen);
    }

    #[test]
    #[should_panic]
    fn parse_turn_too_long() {
        let fen = b"bw";
        let _   = parse_turn(fen);
    }

    #[test]
    fn parse_castling_all() {
        let fen_b    = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let fen_c    = b"KQkq";
        let board    = parse_board(fen_b);
        let castling = parse_castling(fen_c, &board);

        assert_eq!(castling[CastlingVariety::WhiteKingside],  Some(CastlingPath::new(Color::White, File::_E, File::_H)));
        assert_eq!(castling[CastlingVariety::WhiteQueenside], Some(CastlingPath::new(Color::White, File::_E, File::_A)));
        assert_eq!(castling[CastlingVariety::BlackKingside],  Some(CastlingPath::new(Color::Black, File::_E, File::_H)));
        assert_eq!(castling[CastlingVariety::BlackQueenside], Some(CastlingPath::new(Color::Black, File::_E, File::_A)));
    }

    #[test]
    fn parse_castling_none() {
        let fen_b    = b"rn1q1rk1/1p2bppp/p2pbn2/4p3/4P3/1NN1BP2/PPPQ2PP/2KR1B1R";
        let fen_c    = b"";
        let board    = parse_board(fen_b);
        let castling = parse_castling(fen_c, &board);

        assert_eq!(castling[CastlingVariety::WhiteKingside],  None);
        assert_eq!(castling[CastlingVariety::WhiteQueenside], None);
        assert_eq!(castling[CastlingVariety::BlackKingside],  None);
        assert_eq!(castling[CastlingVariety::BlackQueenside], None);
    }

    #[test]
    fn parse_castling_x_fen_chess960_1() {
        let fen_b    = b"nrk12r1/ppp1pp1p/3p2p1/5bn1/P7/2N2B2/1PPPPP2/2KBN1RR";
        let fen_c    = b"Gkq";
        let board    = parse_board(fen_b);
        let castling = parse_castling(fen_c, &board);

        assert_eq!(castling[CastlingVariety::WhiteKingside],  Some(CastlingPath::new(Color::White, File::_C, File::_G)));
        assert_eq!(castling[CastlingVariety::WhiteQueenside], None);
        assert_eq!(castling[CastlingVariety::BlackKingside],  Some(CastlingPath::new(Color::Black, File::_C, File::_G)));
        assert_eq!(castling[CastlingVariety::BlackQueenside], Some(CastlingPath::new(Color::Black, File::_C, File::_B)));
    }

    #[test]
    fn parse_castling_x_fen_chess960_2() {
        let fen_b    = b"nrk121r/ppp1pp1p/3p2p1/5bn1/P7/2N2B2/1PPPPP2/2KBN1RR";
        let fen_c    = b"Hkq";
        let board    = parse_board(fen_b);
        let castling = parse_castling(fen_c, &board);

        assert_eq!(castling[CastlingVariety::WhiteKingside],  Some(CastlingPath::new(Color::White, File::_C, File::_H)));
        assert_eq!(castling[CastlingVariety::WhiteQueenside], None);
        assert_eq!(castling[CastlingVariety::BlackKingside],  Some(CastlingPath::new(Color::Black, File::_C, File::_H)));
        assert_eq!(castling[CastlingVariety::BlackQueenside], Some(CastlingPath::new(Color::Black, File::_C, File::_B)));
    }
}
