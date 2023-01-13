use crate::prelude::*;
use crate::bitboard::magic::Magic;

// TODO: rewrite this entire approach to figuring out the filename for
// architecture-dependent cached computations
macro_rules! cached {
    ( $name:literal ) => {{
        cached!($name, "", "")
    }};

    ( $name:literal, $tag:literal ) => {{
        cached!($name, "-", $tag)
    }};

    ( $name:literal, $sep:literal, $tag:literal ) => {{
        // TODO: replace with a const version of bytemuck::from_bytes to better
        // ensure this is actually safe
        #[allow(unsafe_code)]
        unsafe { std::mem::transmute(*include_bytes!(cached_filename!($name, $sep, $tag))) }
    }};
}

#[cfg(all(target_pointer_width = "64", target_endian = "little"))]
macro_rules! cached_filename {
    ( $name:literal, $sep:literal, $tag:literal ) => {
        concat!("../../share/cached/", $name, ".le64", $sep, $tag, ".bin")
    };
}

#[cfg(all(target_pointer_width = "64", target_endian = "big"))]
macro_rules! cached_filename {
    ( $name:literal, $sep:literal, $tag:literal ) => {
        concat!("../../share/cached/", $name, ".be64", $sep, $tag, ".bin")
    }
}

#[cfg(all(target_pointer_width = "32", target_endian = "little"))]
macro_rules! cached_filename {
    ( $name:literal, $sep:literal, $tag:literal ) => {
        concat!("../../share/cached/", $name, ".le32", $sep, $tag, ".bin")
    }
}

#[cfg(all(target_pointer_width = "32", target_endian = "big"))]
macro_rules! cached_filename {
    ( $name:literal, $sep:literal, $tag:literal ) => {
        concat!("../../share/cached/", $name, ".be32", $sep, $tag, ".bin")
    }
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
    magics:  cached!("bishop_magic_numbers", "pext_off"),
    attacks: cached!("bishop_magic_attacks", "pext_off"),
};

/// Precomputed "magic bitboard" of Rook attacks.
const ROOK_MAGICS: Magic<0x19000> = Magic {
    magics:  cached!("rook_magic_numbers", "pext_off"),
    attacks: cached!("rook_magic_attacks", "pext_off"),
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
        for s1 in Square::iter() {
            for s2 in Square::iter() {
                assert_eq!(
                    computed::square_distance(s1, s2),
                    cached  ::square_distance(s1, s2),
                );
            }
        }
    }

    #[test]
    fn line() {
        for s1 in Square::iter() {
            for s2 in Square::iter() {
                assert_eq!(
                    computed::line(s1, s2),
                    cached  ::line(s1, s2),
                );
            }
        }
    }

    #[test]
    fn between() {
        for s1 in Square::iter() {
            for s2 in Square::iter() {
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

        for color in Color::iter() {
            for piece in Piece::iter() {
                for square in Square::iter() {
                    assert_eq!(
                        computed::attacks(color, piece, square, occupied & !square),
                        cached  ::attacks(color, piece, square, occupied & !square),
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "must not contain the attacking piece")]
    fn attacks_includes_origin_square() {
        let _ = cached::attacks(Color::White, Piece::King, Square::C7, Square::C7.into());
    }
}
