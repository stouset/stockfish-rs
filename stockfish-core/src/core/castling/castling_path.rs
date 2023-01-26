use crate::prelude::*;

/// Encodes the full amount of infromation necessary to perform a castling
/// operation between a particular rook and king.
#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[must_use]
pub struct CastlingPath {
    /// The variety of castling to be performed.
    variety: CastlingVariety,

    /// The path between the king and the rook. This is compatible with both
    /// standard chess as well as Fischer random chess (Chess960). Includes all
    /// squares both the king and rook must transit to reach their destination.
    /// This is *inclusive* of their final resting squares and *exclusive* of
    /// their starting squares.
    path: Bitboard,

    king: Square,
    rook: Square,
}

impl CastlingPath {
    /// Constructs a new [`CastlingPath`] between a `king` and `rook` of a given
    /// `color` beginning on the given files. The pieces are assumed to be on
    /// the home row for their color.
    #[inline]
    #[must_use]
    pub const fn new(color: Color, king: File, rook: File) -> Option<Self> {
        let side    = CastlingSide::new(king, rook)?;
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

        Some(Self {
            variety,
            path,
            king,
            rook,
        })
    }

    /// Returns the color of the player involved in this castling operation.
    #[inline]
    pub const fn color(self) -> Color {
        self.variety.color()
    }

    /// Returns a bitboard containing all squares transited by the king and
    /// rook. This is inclusive of their destination squares but excludes their
    /// starting squares (unless the other piece transits that square).
    #[inline]
    pub const fn path(self) -> Bitboard {
        self.path
    }

    /// Returns the side of the board this castling operation will occur
    /// towards.
    #[inline]
    pub const fn side(self) -> CastlingSide {
        self.variety.side()
    }

    /// Returns the square the king begins on.
    #[inline]
    pub const fn king_origin(self) -> Square {
        self.king
    }

    /// Returns the square the king finishes on.
    #[inline]
    pub const fn king_destination(self) -> Square {
        self.variety.king_destination()
    }

    /// Returns the square the rook begins on.
    #[inline]
    pub const fn rook_origin(self) -> Square {
        self.rook
    }

    /// Returns the square the rook finishes on.
    #[inline]
    pub const fn rook_destination(self) -> Square {
        self.variety.rook_destination()
    }

    /// Returns the individual rights required for this castling operation to be
    /// eligible.
    #[inline]
    pub const fn rights(self) -> CastlingRights {
        self.variety.rights()
    }

    /// The variety of castling this path performs.
    #[inline]
    pub const fn variety(self) -> CastlingVariety {
        self.variety
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
    fn new_same_file() {
        assert_eq!(None, CastlingPath::new(Color::White, File::_D, File::_D));
    }

    #[test]
    fn color() {
        let color = Color::Black;
        let path  = CastlingPath::new(color, File::_A, File::_H).unwrap();

        assert_eq!(color, path.color());
    }

    #[test]
    fn side() {
        assert_eq!(
            CastlingSide::King,
            CastlingPath::new(Color::White, File::_A, File::_H).unwrap().side()
        );

        assert_eq!(
            CastlingSide::Queen,
            CastlingPath::new(Color::White, File::_B, File::_A).unwrap().side()
        );
    }

    #[test]
    fn origin() {
        let king = File::_E;
        let rook = File::_H;
        let path = CastlingPath::new(Color::White, king, rook);

        assert_eq!(king | Rank::_1, path.unwrap().king_origin());
        assert_eq!(rook | Rank::_1, path.unwrap().rook_origin());
    }

    #[test]
    fn destination() {
        let king = File::_E;
        let rook = File::_A;
        let path = CastlingPath::new(Color::Black, king, rook);

        assert_eq!(Square::C8, path.unwrap().king_destination());
        assert_eq!(Square::D8, path.unwrap().rook_destination());
    }

    #[test]
    fn rights() {
        let path = CastlingPath::new(Color::White, File::_A, File::_H);

        assert_eq!(CastlingRights::WHITE_OO, path.unwrap().rights());
    }
}
