use crate::prelude::*;

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

const SQUARE_DISTANCE: [[u8; Square::COUNT]; Square::COUNT] = cached!("square_distance");
// const SQUARE: [Bitboard; Square::COUNT] = cached!("square");

/// Returns the number of moves a king would require to move from the origin
/// square to the destination square.
#[inline]
#[must_use]
pub const fn square_distance(s1: Square, s2: Square) -> u8 {
    SQUARE_DISTANCE[s1][s2]
}

/// Returns a [`Bitboard`] representing only the provided [`Square`].
///
/// TODO: is this actually faster? check the ASM
// #[inline]
// pub const fn square(s: Square) -> Bitboard {
//     SQUARE[s]
// }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::super::{cached, computed};

    #[test]
    fn test_square_distance() {
        for (s1, s2) in Square::into_iter().zip(Square::into_iter()) {
            assert_eq!(
                computed::square_distance(s1, s2),
                cached  ::square_distance(s1, s2),
            );
        }
    }

    // #[test]
    // fn test_square() {
    //     for s in Square::into_iter() {
    //         assert_eq!(
    //             computed::square(s),
    //             cached  ::square(s),
    //         );
    //     }
    // }
}
