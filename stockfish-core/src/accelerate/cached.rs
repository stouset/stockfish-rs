use crate::prelude::*;
use crate::bitboard::magic::Magic;

macro_rules! cached {
    ( $name:literal ) => {{
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
        for (s1, s2) in Square::into_iter().zip(Square::into_iter()) {
            assert_eq!(
                computed::square_distance(s1, s2),
                cached  ::square_distance(s1, s2),
            );
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
