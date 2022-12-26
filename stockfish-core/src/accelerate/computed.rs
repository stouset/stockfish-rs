// use crate::Bitboard;

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

// /// Returns a [`Bitboard`] representing only the provided [`Square`].
// pub const fn square(s: Square) -> Bitboard {
//     Bitboard::from(1 << s.as_u8())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_distance() {
        assert_eq!(0, square_distance(Square::D7, Square::D7));
        assert_eq!(7, square_distance(Square::A1, Square::H8));
        assert_eq!(4, square_distance(Square::G3, Square::G7));
        assert_eq!(4, square_distance(Square::B1, Square::F1));
        assert_eq!(5, square_distance(Square::H2, Square::C1));
    }

    // #[test]
    // fn test_square() {
    //     assert_eq!(1, square(Square::B3).count());

    //     assert!(square(Square::D2).overlaps(Bitboard::FILE_D));
    //     assert!(square(Square::D2).overlaps(Bitboard::RANK_2));
    // }
}
