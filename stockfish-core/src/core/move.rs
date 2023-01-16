use crate::prelude::*;

// A move needs 16 bits to be stored
//
// bit  0- 5: destination square (from 0 to 63)
// bit  6-11: origin square (from 0 to 63)
// bit 12-13: promotion piece type - 2 (from KNIGHT-2 to QUEEN-2)
// bit 14-15: special move flag: promotion (1), en passant (2), castling (3)
// NOTE: en passant bit is set only when a pawn can be captured
//
// Special cases are MOVE_NONE and MOVE_NULL. We can sneak these in because in
// any normal move destination square is always different from origin square
// while MOVE_NONE and MOVE_NULL have the same origin and destination square.
#[derive(Copy, Debug, Eq, PartialEq)]
#[derive_const(Clone)]
#[must_use]
pub struct Move(u16);

enumeration! {
    pub MoveType, [ Normal, Promotion, EnPassant, Castling ]
}

enumeration! {
    MovePromotion, [ Knight, Bishop, Rook, Queen ]
}

impl Move {
    const DESTINATION_BITS: u8 = 6;
    const ORIGIN_BITS:      u8 = 6;
    const PROMOTION_BITS:   u8 = 2;
    const MOVE_TYPE_BITS:   u8 = 2;

    const DESTINATION_SHIFT: u8 = 0;
    const ORIGIN_SHIFT:      u8 = Self::DESTINATION_SHIFT + Self::DESTINATION_BITS;
    const PROMOTION_SHIFT:   u8 = Self::ORIGIN_SHIFT      + Self::ORIGIN_BITS;
    const MOVE_TYPE_SHIFT:   u8 = Self::PROMOTION_SHIFT   + Self::PROMOTION_BITS;

    const DESTINATION_MASK: u8 = (1 << Self::DESTINATION_BITS) - 1;
    const ORIGIN_MASK:      u8 = (1 << Self::ORIGIN_BITS) - 1;
    const PROMOTION_MASK:   u8 = (1 << Self::PROMOTION_BITS) - 1;
    const MOVE_TYPE_MASK:   u8 = (1 << Self::MOVE_TYPE_BITS) - 1;

    pub const fn new(origin: Square, destination: Square) -> Self {
        // TODO: in release builds, if the origin is the destination, this will
        // produce logic errors; can we catch this without a performance
        // penalty?
        debug_assert!(origin != destination);

        // ensure that squares actually are the size we think they are since
        // we're going to bit-pack them
        debug_assert!(origin     .as_u8() < (1 << Self::ORIGIN_BITS));
        debug_assert!(destination.as_u8() < (1 << Self::DESTINATION_BITS));

        let bits_destination: u16 = destination.as_u8().into();
        let bits_origin:      u16 = origin     .as_u8().into();

        Self(
            (bits_destination          << Self::DESTINATION_SHIFT) |
            (bits_origin               << Self::ORIGIN_SHIFT)      |
            ((MoveType::Normal as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn new_promote_knight(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Knight as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn new_promote_bishop(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Bishop as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn new_promote_rook(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Rook as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn new_promote_queen(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Queen as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn new_en_passant(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MoveType::EnPassant as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn new_castling(king: Square, rook: Square) -> Self {
        Self(
            Self::new(king, rook).0
                | ((MoveType::Castling as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    pub const fn origin(self) -> Square {
        let bits = self.extract(Self::ORIGIN_SHIFT, Self::ORIGIN_MASK);

        unsafe_optimization!(
            Square::from_u8(bits).unwrap(),
            Square::from_u8_unchecked(bits),
        )
    }

    pub const fn destination(self) -> Square {
        let bits = self.extract(Self::DESTINATION_SHIFT, Self::DESTINATION_MASK);

        unsafe_optimization!(
            Square::from_u8(bits).unwrap(),
            Square::from_u8_unchecked(bits),
        )
    }

    pub const fn move_type(self) -> MoveType {
        let bits = self.extract(Self::MOVE_TYPE_SHIFT, Self::MOVE_TYPE_MASK);

        unsafe_optimization!(
            MoveType::from_u8(bits).unwrap(),
            MoveType::from_u8_unchecked(bits),
        )
    }

    pub const fn promotion(self) -> Piece {
        debug_assert!(MoveType::Promotion == self.move_type());

        let bits = self.extract(Self::PROMOTION_SHIFT, Self::PROMOTION_MASK) + 1;

        unsafe_optimization!(
            Piece::from_u8(bits).unwrap(),
            Piece::from_u8_unchecked(bits),
        )
    }

    const fn extract(self, shift: u8, mask: u8) -> u8 {
        #[allow(clippy::cast_possible_truncation)] {
            (self.0 >> shift) as u8 & mask
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mv = Move::new(Square::C3, Square::F7);

        assert_eq!(Square::C3,       mv.origin());
        assert_eq!(Square::F7,       mv.destination());
        assert_eq!(MoveType::Normal, mv.move_type());
    }

    #[test]
    fn new_promote_knight() {
        let mv = Move::new_promote_knight(Square::D6, Square::B1);

        assert_eq!(Square::D6,          mv.origin());
        assert_eq!(Square::B1,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Piece::Knight,       mv.promotion());
    }

    #[test]
    fn new_promote_bishop() {
        let mv = Move::new_promote_bishop(Square::C6, Square::C7);

        assert_eq!(Square::C6,          mv.origin());
        assert_eq!(Square::C7,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Piece::Bishop,       mv.promotion());
    }


    #[test]
    fn new_promote_rook() {
        let mv = Move::new_promote_rook(Square::A8, Square::A7);

        assert_eq!(Square::A8,          mv.origin());
        assert_eq!(Square::A7,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Piece::Rook,         mv.promotion());
    }


    #[test]
    fn new_promote_queen() {
        let mv = Move::new_promote_queen(Square::C5, Square::C4);

        assert_eq!(Square::C5,          mv.origin());
        assert_eq!(Square::C4,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Piece::Queen,        mv.promotion());
    }

    #[test]
    fn new_en_passant() {
        let mv = Move::new_en_passant(Square::G7, Square::E1);

        assert_eq!(Square::G7,          mv.origin());
        assert_eq!(Square::E1,          mv.destination());
        assert_eq!(MoveType::EnPassant, mv.move_type());
    }

    #[test]
    fn new_castling() {
        let mv = Move::new_castling(Square::E8, Square::H8);

        assert_eq!(Square::E8,         mv.origin());
        assert_eq!(Square::H8,         mv.destination());
        assert_eq!(MoveType::Castling, mv.move_type());
    }
}
