use crate::prelude::*;

/// Returns the number of moves a king would require to move from the origin
/// square to the destination square.
#[must_use]
pub const fn square_distance(s1: Square, s2: Square) -> u8 {
    let s1_file: u8 = s1.file().into();
    let s1_rank: u8 = s1.rank().into();
    let s2_file: u8 = s2.file().into();
    let s2_rank: u8 = s2.rank().into();

    let file_diff = s1_file.abs_diff(s2_file);
    let rank_diff = s1_rank.abs_diff(s2_rank);

    std::cmp::max(file_diff, rank_diff)
}

/// Returns a [`Bitboard`] containing all the [`Square`]s on the same file,
/// rank, or diagonal as both `s1` and `s2`. Includes `s1` and `s2`.
pub const fn line(s1: Square, s2: Square) -> Bitboard {
    if pseudo_attacks(Token::Bishop, s1).contains(s2) {
        return (s1 | s2) | (
            pseudo_attacks(Token::Bishop, s1) &
            pseudo_attacks(Token::Bishop, s2)
        );
    }

    if pseudo_attacks(Token::Rook, s1).contains(s2) {
        return (s1 | s2) | (
            pseudo_attacks(Token::Rook, s1) &
            pseudo_attacks(Token::Rook, s2)
        );
    }

    Bitboard::EMPTY
}

/// Returns a [`Bitboard`] containing all of the [`Square`]s between `s1` and
/// `s2` exclusive of `s1` and inclusive of `s2`. If `s1` and `s2` are not on
/// the same rank, file, or diagonal, returns `s2`.
///
/// This can allow us to generate non-king evasion moves faster: a defending
/// token must either interpose itself to cover the check or capture the
/// checking token.
pub const fn between(s1: Square, s2: Square) -> Bitboard {
    if pseudo_attacks(Token::Bishop, s1).contains(s2) {
        return
            sliding_attacks(Token::Bishop, s1, s2.into()) &
            sliding_attacks(Token::Bishop, s2, s1.into()) |
            s2;
    }

    if pseudo_attacks(Token::Rook, s1).contains(s2) {
        return
            sliding_attacks(Token::Rook, s1, s2.into()) &
            sliding_attacks(Token::Rook, s2, s1.into()) |
            s2;
    }

    s2.into()
}

pub const fn attacks(color: Color, token: Token, square: Square, occupied: Bitboard) -> Bitboard {
    // TODO: at some point I was convinced this was necessary, but it appears
    // not to be, identify where this belief came from and verify
    //
    // debug_assert!((occupied & square).is_empty(),
    //     "occupancy bitboard must not contain the attacking token");

    match token {
        Token::Pawn                 => pawn_attacks(color, square),
        Token::Knight | Token::King => pseudo_attacks(token, square),
        _                           => sliding_attacks(token, square, occupied),
    }
}

pub const fn pseudo_attacks(token: Token, square: Square) -> Bitboard {
    // pawns require a color to know which direction they attack in
    debug_assert!(token != Token::Pawn,
        "pawns do not have pseudo-attacks defined on them");

    // punt to `sliding_attacks` on an empty board for tokens which slide along
    // the board (bishop, rook, queen)
    if token.is_sliding() {
        return sliding_attacks(token, square, Bitboard::EMPTY);
    }

    // if the token doesn't slide, (knight or king), OR together any single
    // movements that land on a valid square
    let mut i     = 0;
    let mut bb    = Bitboard::EMPTY;
    let     steps = Token::STEPS[token];

    while i < steps.len() {
        if let Some(s) = square + steps[i] {
            bb |= s;
        };

        i += 1;
    }

    bb
}

pub const fn pawn_attacks(color: Color, square: Square) -> Bitboard {
    let board: Bitboard = square.into();

    match color {
        Color::White => (board + Direction::NW) | (board + Direction::NE),
        Color::Black => (board + Direction::SW) | (board + Direction::SE),
    }
}

pub const fn sliding_attacks(token: Token, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!(token.is_sliding(),
        "token is not capable of sliding attacks");

    let mut attacks    = Bitboard::EMPTY;
    let     directions = Token::STEPS[token];

    let mut i = 0;

    while i < directions.len() {
        let     dir = directions[i];
        let mut s   = square;

        while !occupied.contains(s) {
            s = match s + dir {
                Some(v) => v,
                None    => break,
            };

            attacks |= s;
        }

        i += 1;
    }

    attacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_distance_computation() {
        assert_eq!(0, square_distance(Square::D7, Square::D7));
        assert_eq!(7, square_distance(Square::A1, Square::H8));
        assert_eq!(4, square_distance(Square::G3, Square::G7));
        assert_eq!(4, square_distance(Square::B1, Square::F1));
        assert_eq!(5, square_distance(Square::H2, Square::C1));
    }

    #[test]
    fn line_same_square() {
        for s in Square::iter() {
            assert_eq!(Bitboard::EMPTY, line(s, s));
        }
    }

    #[test]
    fn line_disjoint() {
        for s1 in Square::iter() {
            let moves = pseudo_attacks(Token::Queen, s1);

            for s2 in Square::iter().filter(|s| !moves.contains(*s)) {
                assert_eq!(Bitboard::EMPTY, line(s1, s2));
            }
        }
    }

    #[test]
    fn line_bishop_moves() {
        for s1 in Square::iter() {
            let moves = pseudo_attacks(Token::Bishop, s1);

            for s2 in Square::iter().filter(|s| moves.contains(*s)) {
                assert!(line(s1, s2).count() >  1);
                assert!(line(s1, s2).count() <= 8);
                assert!(line(s1, s2).overlaps(Bitboard::EDGES));
            }
        }
    }

    #[test]
    fn line_rook_moves() {
        for s1 in Square::iter() {
            let moves = pseudo_attacks(Token::Rook, s1);

            for s2 in Square::iter().filter(|s| moves.contains(*s)) {
                assert_eq!(8, line(s1, s2).count());
            }
        }
    }

    #[test]
    fn between_same_square() {
        for s in Square::iter() {
            assert_eq!(Bitboard::from(s), between(s, s));
        }
    }

    #[test]
    fn between_disjoint() {
        for s1 in Square::iter() {
            let moves = pseudo_attacks(Token::Queen, s1);

            for s2 in Square::iter().filter(|s| !moves.contains(*s)) {
                assert_eq!(Bitboard::from(s2), between(s1, s2));
            }
        }
    }

    #[test]
    fn between_overlapping() {
        for token in [Token::Bishop, Token::Rook] {
            for s1 in Square::iter() {
                let moves = pseudo_attacks(token, s1);

                for s2 in Square::iter().filter(|s| moves.contains(*s)) {
                    assert_eq!(s1.distance(s2) as usize, between(s1, s2).count());
                }
            }
        }
    }

    #[test]
    fn attacks_pawn() {
        assert_eq!(
            Bitboard::EMPTY,
            attacks(Color::White, Token::Pawn, Square::D8, Bitboard::EMPTY),
        );

        assert_eq!(
            Bitboard::from(Square::B3),
            attacks(Color::White, Token::Pawn, Square::A2, Bitboard::EMPTY),
        );

        assert_eq!(
            Square::D1 | Square::F1,
            attacks(Color::Black, Token::Pawn, Square::E2, Bitboard::EMPTY),
        );

        assert_eq!(
            Square::G4 | Square::E4,
            attacks(Color::White, Token::Pawn, Square::F3, Bitboard::EMPTY | Square::E4 | Square::G4),
        );
    }

    #[test]
    fn attacks_knight() {
        assert_eq!(
            Square::D2 | Square::F2 | Square::C3 | Square::G3 |
            Square::C5 | Square::G5 | Square::D6 | Square::F6,
            attacks(Color::White, Token::Knight, Square::E4, Bitboard::ALL ^ Square::E4),
        );

        assert_eq!(
            Square::B3 | Square::C2,
            attacks(Color::Black, Token::Knight, Square::A1, Bitboard::EMPTY)
        );
    }

    #[test]
    fn attacks_bishop() {
        assert_eq!(
            Square::D3 | Square::F3 | Square::D5 | Square::F5,
            attacks(Color::White, Token::Bishop, Square::E4, Bitboard::ALL ^ Square::E4),
        );

        assert_eq!(
            Square::A1 | Square::C1 | Square::A3 | Square::C3 |
            Square::D4 | Square::E5 | Square::F6 | Square::G7 |
            Square::H8,
            attacks(Color::Black, Token::Bishop, Square::B2, Bitboard::EMPTY),
        );
    }

    #[test]
    fn attacks_rook() {
        assert_eq!(
            Square::F1 | Square::F2 | Square::F3 | Square::F4 |
            Square::F5 | Square::F6 |              Square::F8 |
            Square::A7 | Square::B7 | Square::C7 | Square::D7 |
            Square::E7 |              Square::G7 | Square::H7,
            attacks(Color::White, Token::Rook, Square::F7, Bitboard::EMPTY),
        );

        assert_eq!(
            Square::F3 | Square::F4 | Square::F5 | Square::F6 |
            Square::D7 | Square::E7 | Square::G7 | Square::H7 |
            Square::F8,
            attacks(Color::White, Token::Rook, Square::F7, Bitboard::FILE_D | Bitboard::RANK_3)
        );
    }

    #[test]
    fn pseudo_attacks_with_sliding_piece() {
        assert_eq!(
            sliding_attacks(Token::Bishop, Square::A1, Bitboard::EMPTY),
            pseudo_attacks(Token::Bishop, Square::A1),
        );

        assert_eq!(
            sliding_attacks(Token::Rook, Square::H8, Bitboard::EMPTY),
            pseudo_attacks(Token::Rook, Square::H8),
        );
    }

    #[test]
    #[should_panic(expected = "pawns do not have pseudo-attacks defined")]
    fn pseudo_attacks_with_pawn() {
        let _ = pseudo_attacks(Token::Pawn, Square::D1);
    }

    #[test]
    #[should_panic(expected = "not capable of sliding attacks")]
    fn sliding_attacks_must_slide() {
        let _ = sliding_attacks(Token::Knight, Square::D4, Bitboard::EMPTY);
    }
}
