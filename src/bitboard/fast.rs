use crate::types::Square;
use super::{Bitboard, Magic};

pub(crate) const fn bb<T: bytemuck::Pod, U: bytemuck::Pod>(from: T) -> U {
    constmuck::cast(from, constmuck::infer!())
}

/// The number of bits set for any given 16-bit value.
///
/// TODO: detect at compile time whether or not we need to use this or
/// if we can rely on the `popcnt` instruction.
const POPCNT16: [u8; 1 << 16] = bb(*include_bytes!(env!("STOCKFISH_RS_BB_POPCNT_16")));

/// The number of moves necessary to walk a King from one square to the other.
const SQUARE_DISTANCE: [[u8; Square::COUNT]; Square::COUNT] = bb(*include_bytes!(env!("STOCKFISH_RS_BB_SQUARE_DISTANCE")));

/// Conversion from a [`Square`] index to a [`Bitboard`] with only that
/// square set.
const SQUARE: [Bitboard; Square::COUNT] = bb(*include_bytes!(env!("STOCKFISH_RS_BB_SQUARE")));

// pub const BB_BETWEEN: [[Bitboard; SQUARES]; SQUARES] = cast(*include_bytes!(env!("STOCKFISH_RS_BB_BETWEEN")));
// extern Bitboard BetweenBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard LineBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard PseudoAttacks[PIECE_TYPE_NB][SQUARE_NB];
// extern Bitboard PawnAttacks[COLOR_NB][SQUARE_NB];

const BISHOP_MAGICS: Magic::<0x1480> = Magic {
    magics:  bb(*include_bytes!(env!("STOCKFISH_RS_BB_BISHOP_MAGIC_MAGICS"))),
    attacks: bb(*include_bytes!(env!("STOCKFISH_RS_BB_BISHOP_MAGIC_ATTACKS"))),
};

const ROOK_MAGICS: Magic::<0x19000> = Magic {
    magics:  bb(*include_bytes!(env!("STOCKFISH_RS_BB_ROOK_MAGIC_MAGICS"))),
    attacks: bb(*include_bytes!(env!("STOCKFISH_RS_BB_ROOK_MAGIC_ATTACKS"))),
};

#[inline]
#[must_use]
pub const fn popcnt16(i: u16) -> u8 {
    POPCNT16[i as usize]
}

#[inline]
#[must_use]
pub const fn popcnt64(i: u64) -> u8 {
    if cfg!(use_popcnt) {
        // This cannot truncate as a u64 cannot possible contain more
        // than 255 enabled bits.
        #[allow(clippy::cast_possible_truncation)]
        { i.count_ones() as u8 }
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
    let magic = BISHOP_MAGICS.magics[square];
    let index = magic.index(occupied);

    BISHOP_MAGICS.attacks[index]
}

#[inline]
#[must_use]
pub const fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
// return super::slow::rook_attacks(square, occupied);

    let magic = ROOK_MAGICS.magics[square];
    let index = magic.index(occupied);

    ROOK_MAGICS.attacks[index]
}
