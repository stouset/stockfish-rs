use super::Bitboard;
use crate::types::{Color, Direction, PieceType, Square};

// TODO: const-generic versions of functions that take enum-like structs (e.g.,
// PieceType, Color) once the `adt_const_params` feature is complete.

#[must_use]
pub(crate) const fn popcnt16(i: u16) -> u8 {
    match i.count_ones().try_into() {
        Ok(v)  => v,
        Err(_) => unreachable!(),
    }
}

#[must_use]
pub(crate) const fn popcnt64(i: u64) -> u8 {
    match i.count_ones().try_into() {
        Ok(v)  => v,
        Err(_) => unreachable!(),
    }
}

#[must_use]
pub(crate) const fn square(s: Square) -> Bitboard {
    Bitboard(1 << s.as_u8())
}

#[must_use]
pub(crate) const fn square_distance(s1: Square, s2: Square) -> u8 {
    let s1_file: u8 = s1.file().into();
    let s1_rank: u8 = s1.rank().into();
    let s2_file: u8 = s2.file().into();
    let s2_rank: u8 = s2.rank().into();

    let file_diff = s1_file.abs_diff(s2_file);
    let rank_diff = s1_rank.abs_diff(s2_rank);

    if file_diff > rank_diff { file_diff } else { rank_diff }
}

#[must_use]
pub(crate) fn pseudo_attacks(piece: PieceType, square: Square) -> Bitboard {
    match piece {
        PieceType::KNIGHT => {
            PieceType::KNIGHT_DIRECTIONS
                .into_iter()
                .map(|d| square + d)
                .fold(Bitboard::EMPTY, std::ops::BitOr::bitor)
        },

        PieceType::BISHOP => bishop_attacks(square, Bitboard::EMPTY),
        PieceType::ROOK   => rook_attacks(square, Bitboard::EMPTY),

        PieceType::QUEEN => {
            pseudo_attacks(PieceType::ROOK,   square) |
            pseudo_attacks(PieceType::BISHOP, square)
        },

        PieceType::KING => {
            PieceType::KING_DIRECTIONS
                .into_iter()
                .map(|d| square + d)
                .fold(Bitboard::EMPTY, std::ops::BitOr::bitor)
        },

        _ => unreachable!("pseudo attacks are not available for this piece")
    }
}

#[must_use]
pub(crate) const fn pawn_attacks(color: Color, square: Square) -> Bitboard {
    let board: Bitboard = square.into();

    match color {
        Color::White => (board + Direction::NW) | (board + Direction::NE),
        Color::Black => (board + Direction::SW) | (board + Direction::SE),
    }
}

#[must_use]
pub(crate) fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks    = Bitboard::EMPTY;
    let     directions = PieceType::BISHOP.sliding_directions();

    for dir in directions {
        let mut s = square;

        while (s + *dir).is_some() && (occupied & s).is_empty() {
            s = match s + *dir {
                Some(v) => v,
                None    => unreachable!(), // already tested is_some
            };

            attacks |= s;
        }
    }

    attacks
}

#[must_use]
pub(crate) fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks    = Bitboard::EMPTY;
    let     directions = PieceType::ROOK.sliding_directions();

    for dir in directions {
        let mut s = square;

        while (s + *dir).is_some() && (occupied & s).is_empty() {
            s = match s + *dir {
                Some(v) => v,
                None    => unreachable!(), // already tested is_some
            };

            attacks |= s;
        }
    }

    attacks
}
