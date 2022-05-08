use super::Direction;

use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

#[must_use]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PieceType(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

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

    #[inline]
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v <= Self::MAX { Some(Self(v)) } else { None }
    }

    #[inline]
    pub const fn iter() -> Iter {
        Iter(Self::MIN, Self::MAX + 1)
    }

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
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl const From<PieceType> for u8 {
    #[inline]
    #[must_use]
    fn from(pt: PieceType) -> Self {
        pt.as_u8()
    }
}

impl const From<PieceType> for usize {
    #[inline]
    #[must_use]
    fn from(pt: PieceType) -> Self {
        pt.as_u8().into()
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

impl<T> const Index<PieceType> for [T; PieceType::COUNT] {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: PieceType) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<PieceType> for [T; PieceType::COUNT] {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl Iterator for Iter {
    type Item = PieceType;

    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        let next = Self::Item::from_u8(self.0);
        self.0  += 1;

        next
    }

    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.1 - self.0) as usize;

        (size, Some(size))
    }
}

impl DoubleEndedIterator for Iter {
    #[must_use]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        self.1 -= 1;
        Self::Item::from_u8(self.1)
    }
}

impl FusedIterator for Iter {}
