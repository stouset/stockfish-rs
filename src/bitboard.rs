//! Bitboards for fast lookups.

#![allow(missing_docs)]
#![allow(clippy::identity_op)]

const FILES:   usize = crate::types::File::COUNT;
const SQUARES: usize = crate::types::Square::COUNT;

pub type Bitboard = u64;

pub const BB_ALL_SQUARES:  Bitboard = u64::MAX;
pub const BB_DARK_SQUARES: Bitboard = 0xAA_55_AA_55_AA_55_AA_55;

pub const BB_FILE_A: Bitboard = 0x01_01_01_01_01_01_01_01;
pub const BB_FILE_B: Bitboard = BB_FILE_A << 1;
pub const BB_FILE_C: Bitboard = BB_FILE_A << 2;
pub const BB_FILE_D: Bitboard = BB_FILE_A << 3;
pub const BB_FILE_E: Bitboard = BB_FILE_A << 4;
pub const BB_FILE_F: Bitboard = BB_FILE_A << 5;
pub const BB_FILE_G: Bitboard = BB_FILE_A << 6;
pub const BB_FILE_H: Bitboard = BB_FILE_A << 7;

pub const BB_RANK_1: Bitboard = 0xFF;
pub const BB_RANK_2: Bitboard = BB_RANK_1 << (8 * 1);
pub const BB_RANK_3: Bitboard = BB_RANK_1 << (8 * 2);
pub const BB_RANK_4: Bitboard = BB_RANK_1 << (8 * 3);
pub const BB_RANK_5: Bitboard = BB_RANK_1 << (8 * 4);
pub const BB_RANK_6: Bitboard = BB_RANK_1 << (8 * 5);
pub const BB_RANK_7: Bitboard = BB_RANK_1 << (8 * 6);
pub const BB_RANK_8: Bitboard = BB_RANK_1 << (8 * 7);

pub const BB_QUEEN_SIDE:   Bitboard = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
pub const BB_CENTER_FILES: Bitboard = BB_FILE_C | BB_FILE_D | BB_FILE_E | BB_FILE_F;
pub const BB_KING_SIDE:    Bitboard = BB_FILE_E | BB_FILE_F | BB_FILE_G | BB_FILE_H;
pub const BB_CENTER:       Bitboard = (BB_FILE_D | BB_FILE_E) & (BB_RANK_4 | BB_RANK_5);

pub const BB_KING_FLANK: [Bitboard; FILES] = [
    BB_QUEEN_SIDE ^ BB_FILE_D, BB_QUEEN_SIDE,
    BB_QUEEN_SIDE,             BB_CENTER_FILES,
    BB_CENTER_FILES,           BB_KING_SIDE,
    BB_KING_SIDE,              BB_KING_SIDE ^ BB_FILE_E,
];



pub const POPCNT16:        [u8; 1 << 16]            = cast(*include_bytes!(env!("STOCKFISH_RS_BB_POPCNT_16")));
pub const SQUARE_DISTANCE: [[u8; SQUARES]; SQUARES] = cast(*include_bytes!(env!("STOCKFISH_RS_BB_SQUARE_DISTANCE")));

pub const BB_SQUARE: [Bitboard; SQUARES] = cast(*include_bytes!(env!("STOCKFISH_RS_BB_SQUARE")));

// extern Bitboard BetweenBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard LineBB[SQUARE_NB][SQUARE_NB];
// extern Bitboard PseudoAttacks[PIECE_TYPE_NB][SQUARE_NB];
// extern Bitboard PawnAttacks[COLOR_NB][SQUARE_NB];

const fn cast<T: constmuck::Pod, U: constmuck::Pod>(from: T) -> U {
    constmuck::cast(from, constmuck::infer!())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Square;

    #[test]
    fn pop_cnt_16_is_correct() {
        for (i, v) in POPCNT16.iter().enumerate() {
            assert_eq!(v, &i.count_ones().try_into().unwrap());
        }
    }

    #[test]
    fn square_distance_is_correct() {
        for (i, s1) in Square::iter().enumerate() {
            for (j, s2) in Square::iter().enumerate() {
                assert_eq!(SQUARE_DISTANCE[i][j], std::cmp::max(
                    s1.distance_files(s2),
                    s1.distance_ranks(s2),
                ));
            }
        }
    }

    #[test]
    fn square_is_correct() {
        for (i, s) in Square::iter().enumerate() {
            assert_eq!(BB_SQUARE[i], 1 << u8::from(s));
        }
    }
}
