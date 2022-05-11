use crate::types::{Color, PieceType, Square};
use super::{Bitboard, Magic};

macro_rules! bb {
    ( $env:tt ) => {
        constmuck::cast(
            *include_bytes!(env!(concat!("STOCKFISH_RS_BB_", $env))),
            constmuck::infer!()
        )
    }
}

/// The number of bits set for any given 16-bit value.
#[cfg(not(use_popcnt))]
const POPCNT16: [u8; 1 << 16] = bb!("POPCNT_16");

/// The number of moves necessary to walk a King from one square to the other.
const SQUARE_DISTANCE: [[u8; Square::COUNT]; Square::COUNT] = bb!("SQUARE_DISTANCE");

/// Conversion from a [`Square`] index to a [`Bitboard`] with only that
/// square set.
const SQUARE: [Bitboard; Square::COUNT] = bb!("SQUARE");

/// All of the squares on an entire line, edge-to-edge, connecting the two
/// squares (if such a line exists).
const LINE: [[Bitboard; Square::COUNT]; Square::COUNT] = bb!("LINE");

/// All of the squares on a line between the two squares, inclusive of the
/// second square. If no such line exists, just the second square.
const BETWEEN: [[Bitboard; Square::COUNT]; Square::COUNT] = bb!("BETWEEN");

/// The attacks for any type of piece assuming an empty board.
const PSEUDO_ATTACKS: [[Bitboard; Square::COUNT]; PieceType::COUNT] = bb!("PSEUDO_ATTACKS");

/// The attacks for pawns of each color.
const PAWN_ATTACKS: [[Bitboard; Square::COUNT]; Color::COUNT] = bb!("PAWN_ATTACKS");

/// A "magic bitboard" of Bishop attacks.
const BISHOP_MAGICS: Magic<0x1480> = Magic {
    magics:  bb!("BISHOP_MAGIC_MAGICS"),
    attacks: bb!("BISHOP_MAGIC_ATTACKS"),
};

/// A "magic bitboard" of Rook attacks.
const ROOK_MAGICS: Magic<0x19000> = Magic {
    magics:  bb!("ROOK_MAGIC_MAGICS"),
    attacks: bb!("ROOK_MAGIC_ATTACKS")
};

/// Counts the set bits in a [`u64`]. Uses a native instruction on architectures
/// that have it, and accelerates the operation with a 16-bit lookup table on
/// ones that don't.
#[inline]
#[must_use]
pub const fn popcnt64(i: u64) -> u8 {
    #[cfg(use_popcnt)] {
        // This cannot truncate as a u64 cannot possibly contain more
        // than 255 enabled bits.
        #[allow(clippy::cast_possible_truncation)] {
            i.count_ones() as _
        }
    }

    #[cfg(not(use_popcnt))] {
        let chunks: [u16; 4] =
            constmuck::cast(i, constmuck::infer!());

        POPCNT16[chunks[0]]
            + POPCNT16[chunks[1]]
            + POPCNT16[chunks[2]]
            + POPCNT16[chunks[3]]
    }
}

/// Returns the number of moves it would take for a king to move from the first
/// square to the second.
#[inline]
#[must_use]
pub const fn square_distance(s1: Square, s2: Square) -> u8 {
    SQUARE_DISTANCE[s1][s2]
}

/// Converts a square to a bitboard containing just that square.
#[inline]
#[must_use]
pub const fn square(s: Square) -> Bitboard {
    SQUARE[usize::from(s)]
}

/// Returns a bitboard representing an entire line (from board edge to board
/// edge) that intersects the two given squares. If the given squares are not
/// on a same file, rank, or diagonal, or if the squares are the same, returns
/// an empty bitboard.
///
/// For example, `line(Square::C4, Square::F7)` will return a bitboard with the
/// bits on the A2-G8 diagonal set.
///
/// # Example
///
/// ```rust
/// # use stockfish_rs::bitboard::{self, Bitboard};
/// # use stockfish_rs::types::Square;
///
/// assert_eq!(
///     Square::A2 | Square::B3 | Square::C4 | Square::D5 | Square::E6 | Square::F7 | Square::G8,
///     bitboard::line(Square::C4, Square::F7),
/// );
/// ```
#[inline]
#[must_use]
pub const fn line(s1: Square, s2: Square) -> Bitboard {
    LINE[s1][s2]
}

/// Returns a bitboard representing the squares in the semi-open segment between
/// the squares `s1` and `s2` (excluding `s1` but including `s2`). If the given
/// squares are not on a same file, rank, or diagonal, it returns `s2`.
///
/// For example, `between(Square::C4, Square::F7)` will return a bitboard with
/// squares D5, E6 and F7, but `between(Square::E6, Square::F8)` will return a
/// bitboard with the square F8. This trick allows to generate non-king evasion
/// moves faster: the defending piece must either interpose itself to cover the
/// check or capture the checking piece.
///
/// # Examples
///
/// ```rust
/// # use stockfish_rs::bitboard::{self, Bitboard};
/// # use stockfish_rs::types::Square;
///
/// assert_eq!(
///     Square::D5 | Square::E6 | Square::F7,
///     bitboard::between(Square::C4, Square::F7),
/// );
///
/// assert_eq!(
///     Bitboard::from(Square::F8),
///     bitboard::between(Square::E6, Square::F8),
/// );
/// ```
#[inline]
#[must_use]
pub const fn between(s1: Square, s2: Square) -> Bitboard {
    BETWEEN[s1][s2]
}

/// Returns a bitboard of valid moves for the piece given an empty board.
#[inline]
#[must_use]
pub const fn moves(color: Color, pt: PieceType, square: Square) -> Bitboard {
    match pt {
        PieceType::Pawn => PAWN_ATTACKS[color][square],
        _               => PSEUDO_ATTACKS[pt][square],
    }
}

/// Returns a bitboard of valid attacks given a board containing other pieces
/// that may interfere with its movements.
#[inline]
#[must_use]
pub const fn attacks(color: Color, pt: PieceType, square: Square, occupied: Bitboard) -> Bitboard {
    match pt {
        PieceType::Pawn   => PAWN_ATTACKS[color][square],
        PieceType::Bishop => BISHOP_MAGICS.attacks(square, occupied),
        PieceType::Rook   => ROOK_MAGICS  .attacks(square, occupied),
        PieceType::Queen  => BISHOP_MAGICS.attacks(square, occupied) |
                             ROOK_MAGICS  .attacks(square, occupied),
        _                 => PSEUDO_ATTACKS[pt][square]
    }
}
