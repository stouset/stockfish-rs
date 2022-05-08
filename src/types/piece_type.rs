use super::{Direction, Square};
use crate::bitboard::{self, Bitboard};

#[must_use]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PieceType(u8);

impl PieceType {
    pub const PAWN:   Self = Self(0);
    pub const KNIGHT: Self = Self(1);
    pub const BISHOP: Self = Self(2);
    pub const ROOK:   Self = Self(3);
    pub const QUEEN:  Self = Self(4);
    pub const KING:   Self = Self(5);

    pub const FIRST: Self  = Self::PAWN;
    pub const LAST:  Self  = Self::KING;
    pub const MIN:   u8    = Self::FIRST.0;
    pub const MAX:   u8    = Self::LAST.0;
    pub const COUNT: usize = Self::MAX as usize + 1;

    pub const KNIGHT_DIRECTIONS: [Direction; 8] = [
        Direction::NNW,
        Direction::NNE,
        Direction::ENE,
        Direction::ESE,
        Direction::SSE,
        Direction::SSW,
        Direction::WSW,
        Direction::WNW,
    ];

    pub const BISHOP_DIRECTIONS: [Direction; 4] = [
        Direction::NW,
        Direction::NE,
        Direction::SE,
        Direction::SW,
    ];

    pub const ROOK_DIRECTIONS: [Direction; 4] = [
        Direction::N,
        Direction::S,
        Direction::E,
        Direction::W,
    ];

    pub const QUEEN_DIRECTIONS: [Direction; 8] = [
        Direction::NW,
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
    ];

    pub const KING_DIRECTIONS: [Direction; 8] = Self::QUEEN_DIRECTIONS;

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PAWN   => "pawn",
            Self::KNIGHT => "knight",
            Self::BISHOP => "bishop",
            Self::ROOK   => "rook",
            Self::QUEEN  => "queen",
            Self::KING   => "king",
            _            => unreachable!(),
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_sliding(self) -> bool {
        self.0 == Self::BISHOP.0
            || self.0 == Self::ROOK.0
            || self.0 == Self::QUEEN.0
    }

    pub const fn sliding_directions(self) -> &'static [Direction] {
        match self {
            Self::ROOK   => Self::ROOK_DIRECTIONS  .as_slice(),
            Self::BISHOP => Self::BISHOP_DIRECTIONS.as_slice(),
            Self::QUEEN  => Self::QUEEN_DIRECTIONS .as_slice(),
            _            => &[],
        }
    }

    #[must_use]
    pub fn sliding_attacks(
        self,
        square:   Square,
        occupied: Bitboard
    ) -> Bitboard {
        debug_assert!(self.is_sliding(),
            "{:?} is not capable of sliding attacks", self);

        debug_assert!((occupied & square).is_empty(),
            "{:?} must not be in the occupancy board {:?}", self, occupied);

        match self {
            Self::ROOK   => bitboard::rook_attacks(square, occupied),
            Self::BISHOP => bitboard::bishop_attacks(square, occupied),
            Self::QUEEN  => (
                Self::ROOK  .sliding_attacks(square, occupied) |
                Self::BISHOP.sliding_attacks(square, occupied)
            ),
            _ => Bitboard::EMPTY,
        }
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", std::any::type_name::<Self>(), self.name())
    }
}
