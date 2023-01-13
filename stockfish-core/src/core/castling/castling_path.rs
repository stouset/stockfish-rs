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
        let side    = CastlingSide::new(king, rook);
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
    pub const fn color(self) -> Color {
        self.variety.color()
    }

    #[inline]
    pub const fn side(self) -> CastlingSide {
        self.variety.side()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives() {
        let path = CastlingPath::new(Color::Black, File::_A, File::_H);

        assert_eq!(path, path.clone());
        assert_ne!("", format!("{path:?}"));
    }

    #[test]
    fn color() {
        let color = Color::Black;
        let path  = CastlingPath::new(color, File::_A, File::_H);

        assert_eq!(color, path.color());
    }

    #[test]
    fn side() {
        assert_eq!(
            CastlingSide::King,
            CastlingPath::new(Color::White, File::_A, File::_H).side()
        );

        assert_eq!(
            CastlingSide::Queen,
            CastlingPath::new(Color::White, File::_B, File::_A).side()
        );
    }

    #[test]
    fn origin() {
        let king = File::_E;
        let rook = File::_H;
        let path = CastlingPath::new(Color::White, king, rook);

        assert_eq!(king | Rank::_1, path.king_origin());
        assert_eq!(rook | Rank::_1, path.rook_origin());
    }

    #[test]
    fn destination() {
        let king = File::_E;
        let rook = File::_A;
        let path = CastlingPath::new(Color::Black, king, rook);

        assert_eq!(Square::C8, path.king_destination());
        assert_eq!(Square::D8, path.rook_destination());
    }

    #[test]
    fn rights() {
        let path = CastlingPath::new(Color::White, File::_A, File::_H);

        assert_eq!(CastlingRights::WHITE_OO, path.rights());
    }
}
