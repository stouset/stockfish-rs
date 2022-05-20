use super::Bitboard;
use crate::types::{Color, Direction, PieceType, Square};

// TODO: const-generic versions of functions that take enum-like structs (e.g.,
// PieceType, Color) once the `adt_const_params` feature is complete.

#[must_use]
pub(crate) const fn popcnt64(i: u64) -> u8 {
    // this cannot truncate as a u64 cannot possibly contain more
    // than 255 enabled bits
    #[allow(clippy::cast_possible_truncation)] {
        i.count_ones() as _
    }
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
pub(crate) const fn square(s: Square) -> Bitboard {
    Bitboard(1 << s.as_u8())
}

#[must_use]
pub(crate) fn line(s1: Square, s2: Square) -> Bitboard {
    for pt in [PieceType::Bishop, PieceType::Rook] {
        if pseudo_attacks(pt, s1).contains(s2) {
            return pseudo_attacks(pt, s1) & pseudo_attacks(pt, s2) | s1 | s2
        }
    }

    Bitboard::EMPTY
}

#[must_use]
pub(crate) fn between(s1: Square, s2: Square) -> Bitboard {
    for pt in [PieceType::Bishop, PieceType::Rook] {
        if pseudo_attacks(pt, s1).contains(s2) {
            return Bitboard::from(s2) | (
                attacks(Color::White, pt, s1, s2.into()) &
                attacks(Color::White, pt, s2, s1.into())
             );
        }
    }

    s2.into()
}

#[inline]
#[must_use]
pub(crate) fn moves(color: Color, pt: PieceType, square: Square) -> Bitboard {
    match pt {
        PieceType::Pawn => pawn_attacks(color, square),
        _               => pseudo_attacks(pt, square),
    }
}

#[must_use]
pub(crate) fn attacks(color: Color, pt: PieceType, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((occupied & square).is_empty(),
        "occupancy bitboard must not contain the attacking piece");

    match pt {
        PieceType::Pawn                     => pawn_attacks(color, square),
        PieceType::Knight | PieceType::King => pseudo_attacks(pt, square),
        _                                   => sliding_attacks(pt, square, occupied),
    }
}

#[must_use]
pub(crate) fn pseudo_attacks(pt: PieceType, square: Square) -> Bitboard {
    // pawns require a color to know which direction they attack in
    debug_assert!(pt != PieceType::Pawn,
        "pawns do not have pseudo-attacks on them");

    // punt to `sliding_attacks` on an empty board for pieces which slide along
    // the board (bishop, rook, queen)
    if pt.is_sliding() {
        return sliding_attacks(pt, square, Bitboard::EMPTY);
    }

    // if the piece doesn't slide, (knight or king), OR together any single
    // movements that land on a valid square
    PieceType::STEPS[pt]
        .iter()
        .map(|d| square + *d)
        .fold(Bitboard::EMPTY, std::ops::BitOr::bitor)
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
pub(crate) fn sliding_attacks(pt: PieceType, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!(pt.is_sliding(),
        "{:?} is not capable of sliding attacks", pt);

    let mut attacks    = Bitboard::EMPTY;
    let     directions = PieceType::STEPS[pt];

    for dir in directions {
        let mut s = square;

        while (s + *dir).is_some() && !occupied.contains(s) {
            s = match s + *dir {
                Some(v) => v,
                None    => unreachable!(), // already tested is_some
            };

            attacks |= s;
        }
    }

    attacks
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::{Color, PieceType, Square};

    #[test]
    #[should_panic(expected = "must not contain the attacking piece")]
    fn attacks_detects_accidental_misuse() {
        let _ = attacks(
            Color::Black,
            PieceType::King,
            Square::C3,
            Bitboard::FILE_C
        );
    }
}
