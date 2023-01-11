use crate::prelude::*;

#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[must_use]
pub struct CastlingPath {
    pub variety: CastlingVariety,
    pub path:    Bitboard,

    king: Square,
    rook: Square,
}

impl CastlingPath {
    #[inline]
    pub const fn new(color: Color, king: File, rook: File) -> Self {
        let side    = CastlingSide::detect(king, rook);
        let variety = CastlingVariety::new(color, side);

        // reassign king and rook to their actual square
        let rank = color.rank();
        let king = rank | king;
        let rook = rank | rook;

        // the complete path is the intersection of the path the king takes and
        // the path the rook takes, excluding either origin square
        let path = (
            crate::accelerate::between(king, variety.king_destination()) |
            crate::accelerate::between(rook, variety.rook_destination())
        ) & !(king | rook);

        Self {
            variety,
            path,
            king,
            rook,
        }
    }

    #[inline]
    pub const fn king_origin(self) -> Square {
        self.king
    }

    #[inline]
    pub const fn king_destination(self) -> Square {
        self.variety.king_destination()
    }

    #[inline]
    pub const fn rook_origin(self) -> Square {
        self.rook
    }

    #[inline]
    pub const fn rook_destination(self) -> Square {
        self.variety.rook_destination()
    }

    #[inline]
    pub const fn rights(self) -> CastlingRights {
        self.variety.rights()
    }
}
