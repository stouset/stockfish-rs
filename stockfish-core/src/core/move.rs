use crate::prelude::*;

/// Encodes a move from one square to another on a chess board.
///
/// Users of this struct must be aware of and observe the following rules and
/// behavior:
///
/// * a move *must not* go from an origin to the same destination
/// * castling is encoded as a move from the king's starting square to the
///   rook's starting square
/// * the [`Move::promotion`] must not be called and for moves whose type is
///   not [`MoveType::Promotion`]
#[derive(Copy, Debug, Eq, PartialEq)]
#[derive_const(Clone)]
#[must_use]
pub struct Move(u16);

enumeration! {
    /// Encodes the type of [`Move`] being made.
    ///
    /// Note that a [`MoveType::EnPassant`] represents the *capture* of a pawn
    /// en passant, not a two square pawn movement.
    pub MoveType, [ Normal, Promotion, EnPassant, Castling ]
}

enumeration! {
    /// Promotions can only result in four of the possible types of token, so we
    /// create an enum of them specifically for the sake of bit-packing.
    MovePromotion, [ Knight, Bishop, Rook, Queen ]
}

impl Move {
    // A move needs 16 bits to be stored.
    //
    // * bit  0- 5: destination square (from 0 to 63)
    // * bit  6-11: origin square (from 0 to 63)
    // * bit 12-13: promotion token type - 2 (from KNIGHT-2 to QUEEN-2)
    // * bit 14-15: special move flag: promotion (1), en passant (2), castling (3)
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

    /// Encodes a normal move from an `origin` square to a `destination` square.
    #[inline]
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

    /// Encodes a move of a pawn from an `origin` square to a `destination`
    /// square that results in promotion to a knight.
    #[inline]
    pub const fn new_promote_knight(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Knight as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    /// Encodes a move of a pawn from an `origin` square to a `destination`
    /// square that results in promotion to a bishop.
    #[inline]
    pub const fn new_promote_bishop(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Bishop as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    /// Encodes a move of a pawn from an `origin` square to a `destination`
    /// square that results in promotion to a rook.
    #[inline]
    pub const fn new_promote_rook(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Rook as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    /// Encodes a move of a pawn from an `origin` square to a `destination`
    /// square that results in promotion to a queen.
    #[inline]
    pub const fn new_promote_queen(origin: Square, destination: Square) -> Self {
        Self(
            Self::new(origin, destination).0
                | ((MovePromotion::Queen as u16) << Self::PROMOTION_SHIFT)
                | ((MoveType::Promotion   as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    /// Encodes the capture of a pawn en passant by a token starting on
    /// `origin` and ending on the `pawn`'s square.
    #[inline]
    pub const fn new_en_passant(origin: Square, pawn: Square) -> Self {
        Self(
            Self::new(origin, pawn).0
                | ((MoveType::EnPassant as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    /// Encodes castling between `king` and a `rook` on their respective
    /// starting squares. The destination squares must be inferred by the rules
    /// of castling.
    #[inline]
    pub const fn new_castling(king: Square, rook: Square) -> Self {
        Self(
            Self::new(king, rook).0
                | ((MoveType::Castling as u16) << Self::MOVE_TYPE_SHIFT)
        )
    }

    /// Returns the square the moving token began on.
    #[inline]
    pub const fn origin(self) -> Square {
        let bits = self.extract(Self::ORIGIN_SHIFT, Self::ORIGIN_MASK);

        unsafe_optimization!(
            Square::from_u8(bits).unwrap(),
            Square::from_u8_unchecked(bits),
        )
    }

    /// For all types of move other than [`MoveType::Castling`], returns the
    /// square the token finishes on.
    ///
    /// For castling, encodes the position the rook begins on. The actual
    /// destination square of the rook and king must be inferred by the rules of
    /// castling.
    #[inline]
    pub const fn destination(self) -> Square {
        let bits = self.extract(Self::DESTINATION_SHIFT, Self::DESTINATION_MASK);

        unsafe_optimization!(
            Square::from_u8(bits).unwrap(),
            Square::from_u8_unchecked(bits),
        )
    }

    /// Returns the type of move encoded.
    #[inline]
    pub const fn move_type(self) -> MoveType {
        let bits = self.extract(Self::MOVE_TYPE_SHIFT, Self::MOVE_TYPE_MASK);

        unsafe_optimization!(
            MoveType::from_u8(bits).unwrap(),
            MoveType::from_u8_unchecked(bits),
        )
    }

    /// Returns the type of token a pawn is being promoted to during this move.
    ///
    /// This method *must not* be called unless the move is a
    /// [`MoveType::Promotion`]. For all other move types, the result of this
    /// method is undefined and must not be expected to produce reliable
    /// behavior.
    #[inline]
    pub const fn promotion(self) -> Token {
        debug_assert!(MoveType::Promotion == self.move_type());

        // We exclude pawns and kings from the type of token that may be
        // promoted to (so that this information fits in two bits), so we need
        // to add the value of a knight in order to get the value of the token
        // being promoted to.
        //
        // This conversion could/should be implemented on the MovePromotion enum
        // itself, but given that it's *only* used here that's overkill.
        let bits = self.extract(Self::PROMOTION_SHIFT, Self::PROMOTION_MASK)
            + Token::Knight.as_u8();

        unsafe_optimization!(
            Token::from_u8(bits).unwrap(),
            Token::from_u8_unchecked(bits),
        )
    }

    #[inline]
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
    fn derives() {
        let mv = Move::new(Square::A1, Square::A2);

        assert_eq!(mv, mv.clone());
        assert_ne!("", format!("{mv:?}"));
    }

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
        assert_eq!(Token::Knight,       mv.promotion());
    }

    #[test]
    fn new_promote_bishop() {
        let mv = Move::new_promote_bishop(Square::C6, Square::C7);

        assert_eq!(Square::C6,          mv.origin());
        assert_eq!(Square::C7,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Token::Bishop,       mv.promotion());
    }


    #[test]
    fn new_promote_rook() {
        let mv = Move::new_promote_rook(Square::A8, Square::A7);

        assert_eq!(Square::A8,          mv.origin());
        assert_eq!(Square::A7,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Token::Rook,         mv.promotion());
    }


    #[test]
    fn new_promote_queen() {
        let mv = Move::new_promote_queen(Square::C5, Square::C4);

        assert_eq!(Square::C5,          mv.origin());
        assert_eq!(Square::C4,          mv.destination());
        assert_eq!(MoveType::Promotion, mv.move_type());
        assert_eq!(Token::Queen,        mv.promotion());
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
