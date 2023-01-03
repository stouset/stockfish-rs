use crate::prelude::*;
use crate::bitboard::magic::Magic;

macro_rules! cached {
    ( $name:literal ) => {{
        // TODO: replace with a const version of bytemuck::from_bytes to better
        // ensure this is actually safe
        #[allow(unsafe_code)]
        unsafe {
            std::mem::transmute(*include_bytes!(
                concat!("../../share/cached/", $name, ".bin")
            ))
        }
    }}
}

/// Precomputed disatnces between [`Square`]s.
const SQUARE_DISTANCE: [[u8; Square::COUNT]; Square::COUNT] = cached!("square_distance");

/// Precomputed lines containing [`Square`]s.
const LINE: [[Bitboard; Square::COUNT]; Square::COUNT] = cached!("line");

/// Precomputed lines between [`Square`]s.
const BETWEEN: [[Bitboard; Square::COUNT]; Square::COUNT] = cached!("between");

/// Precomputed attacks for any type of piece on an empty board.
const PSEUDO_ATTACKS: [[Bitboard; Square::COUNT]; Piece::COUNT] = cached!("pseudo_attacks");

/// Precomputed attacks for pawns of each color.
const PAWN_ATTACKS: [[Bitboard; Square::COUNT]; Color::COUNT] = cached!("pawn_attacks");

/// Precomputed "magic bitboard" of Bishop attacks.
const BISHOP_MAGICS: Magic<0x1480> = Magic {
    magics:  cached!("bishop_magic_numbers"),
    attacks: cached!("bishop_magic_attacks"),
};

/// Precomputed "magic bitboard" of Rook attacks.
const ROOK_MAGICS: Magic<0x19000> = Magic {
    magics:  cached!("rook_magic_numbers"),
    attacks: cached!("rook_magic_attacks")
};

/// Returns the number of moves a king would require to move from the origin
/// square to the destination square.
#[inline]
#[must_use]
pub const fn square_distance(s1: Square, s2: Square) -> u8 {
    SQUARE_DISTANCE[s1][s2]
}

/// Returns a [`Bitboard`] containing all the [`Square`]s on the same file,
/// rank, or diagonal as both `s1` and `s2`. Includes `s1` and `s2`.
#[inline]
pub const fn line(s1: Square, s2: Square) -> Bitboard {
    LINE[s1][s2]
}

/// Returns a [`Bitboard`] containing all of the [`Square`]s between `s1` and
/// `s2` exclusive of `s1` and inclusive of `s2`. If `s1` and `s2` are not on
/// the same rank, file, or diagonal, returns `s2`.
///
/// This can allow us to generate non-king evasion moves faster: a defending
/// piece must either interpose itself to cover the check or capture the
/// checking piece.
pub const fn between(s1: Square, s2: Square) -> Bitboard {
    BETWEEN[s1][s2]
}

/// Returns a bitboard of valid attacks given a board containing other pieces
/// that may interfere with its movements.
///
/// TODO: relax requirements on occupancy bitboards so this function cannot
/// produce incorrect results in release builds
#[inline]
pub const fn attacks(color: Color, piece: Piece, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((occupied & square).is_empty(),
        "occupancy bitboard must not contain the attacking piece");

    match piece {
        Piece::Pawn   => PAWN_ATTACKS[color][square],
        Piece::Bishop => BISHOP_MAGICS.attacks(square, occupied),
        Piece::Rook   => ROOK_MAGICS  .attacks(square, occupied),
        Piece::Queen  => BISHOP_MAGICS.attacks(square, occupied) |
                         ROOK_MAGICS  .attacks(square, occupied),
        _             => PSEUDO_ATTACKS[piece][square]
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::super::{cached, computed};

    #[test]
    fn square_distance() {
        for s1 in Square::into_iter() {
            for s2 in Square::into_iter() {
                assert_eq!(
                    computed::square_distance(s1, s2),
                    cached  ::square_distance(s1, s2),
                );
            }
        }
    }

    #[test]
    fn line() {
        for s1 in Square::into_iter() {
            for s2 in Square::into_iter() {
                assert_eq!(
                    computed::line(s1, s2),
                    cached  ::line(s1, s2),
                );
            }
        }
    }

    #[test]
    fn between() {
        for s1 in Square::into_iter() {
            for s2 in Square::into_iter() {
                assert_eq!(
                    computed::between(s1, s2),
                    cached  ::between(s1, s2),
                );
            }
        }
    }

    #[test]
    fn attacks() {
        let occupied =
            Square::A1 | Square::B1 | Square::D1 | Square::F1 |
            Square::E2 | Square::G2 |
            Square::C3 | Square::D3 |
            Square::H5 |
            Square::A6 | Square::C6 |
            Square::A7 | Square::H7 |
            Square::B8 | Square::D8 | Square::F8 | Square::G8 | Square::H8;

        for color in Color::into_iter() {
            for piece in Piece::into_iter() {
                for square in Square::into_iter() {
                    assert_eq!(
                        computed::attacks(color, piece, square, occupied & !square),
                        cached  ::attacks(color, piece, square, occupied & !square),
                    );
                }
            }
        }
    }
}
