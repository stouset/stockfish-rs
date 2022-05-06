use crate::types::{Color, Square};
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
const POPCNT16: [u8; 1 << 16] = bb!("POPCNT_16");

/// The number of moves necessary to walk a King from one square to the other.
const SQUARE_DISTANCE: [[u8; Square::COUNT]; Square::COUNT] = bb!("SQUARE_DISTANCE");

/// Conversion from a [`Square`] index to a [`Bitboard`] with only that
/// square set.
const SQUARE: [Bitboard; Square::COUNT] = bb!("SQUARE");

// pub const BB_BETWEEN: [[Bitboard; SQUARES]; SQUARES] = cast(*include_bytes!(env!("STOCKFISH_RS_BB_BETWEEN")));
// extern Bitboard BetweenBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard LineBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard PseudoAttacks[PIECE_TYPE_NB][SQUARE_NB];

const PAWN_ATTACKS: [[Bitboard; Square::COUNT]; Color::COUNT] = bb!("PAWN_ATTACKS");

const BISHOP_MAGICS: Magic<0x1480> = Magic {
    magics:  bb!("BISHOP_MAGIC_MAGICS"),
    attacks: bb!("BISHOP_MAGIC_ATTACKS"),
};

const ROOK_MAGICS: Magic<0x19000> = Magic {
    magics:  bb!("ROOK_MAGIC_MAGICS"),
    attacks: bb!("ROOK_MAGIC_ATTACKS")
};

#[inline]
#[must_use]
pub const fn popcnt16(i: u16) -> u8 {
    if cfg!(use_popcnt) {
        // This cannot truncate as a u16 cannot possibly contain more
        // than 255 enabled bits.
        #[allow(clippy::cast_possible_truncation)] {
            i.count_ones() as _
        }
    } else {
        POPCNT16[i as usize]
    }
}

#[inline]
#[must_use]
pub const fn popcnt64(i: u64) -> u8 {
    if cfg!(use_popcnt) {
        // This cannot truncate as a u64 cannot possibly contain more
        // than 255 enabled bits.
        #[allow(clippy::cast_possible_truncation)] {
            i.count_ones() as _
        }
    } else {
        let chunks: [u16; 4] =
            constmuck::cast(i, constmuck::infer!());

        popcnt16(chunks[0])
            + popcnt16(chunks[1])
            + popcnt16(chunks[2])
            + popcnt16(chunks[3])
    }
}

#[inline]
#[must_use]
pub const fn square(s: Square) -> Bitboard {
    SQUARE[usize::from(s)]
}

#[inline]
#[must_use]
pub const fn square_distance(s1: Square, s2: Square) -> u8 {
    SQUARE_DISTANCE[usize::from(s1)][usize::from(s2)]
}

#[inline]
#[must_use]
pub const fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    BISHOP_MAGICS.attacks(square, occupied)
}

#[inline]
#[must_use]
pub const fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    ROOK_MAGICS.attacks(square, occupied)
}

#[inline]
#[must_use]
pub const fn pawn_attacks(color: Color, square: Square) -> Bitboard {
    PAWN_ATTACKS[color][square]
}
