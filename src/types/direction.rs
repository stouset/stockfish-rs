use super::Square;

use std::ops::{Add, Neg};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Direction(i8);

impl Direction {
    pub const NORTH: Self = Self(8);
    pub const EAST:  Self = Self(1);
    pub const SOUTH: Self = -Self::NORTH;
    pub const WEST:  Self = -Self::EAST;

    // TODO: Can these be initialized by composing bitwise operations?
    // Implementing [`ops::Add`] is potentially dangerous as it's not
    // guaranteed to give us valid output moves.
    pub const NORTH_EAST: Self = Self(Self::NORTH.0 + Self::EAST.0);
    pub const NORTH_WEST: Self = Self(Self::NORTH.0 + Self::WEST.0);
    pub const SOUTH_EAST: Self = Self(Self::SOUTH.0 + Self::EAST.0);
    pub const SOUTH_WEST: Self = Self(Self::SOUTH.0 + Self::WEST.0);

    // TODO: safe implementations of bitwise operations, in a way that
    // the set of operations is closed over only legal directions

    #[inline]
    #[must_use]
    const fn name(self) -> &'static str {
        match self {
            Self::NORTH      => "North",
            Self::EAST       => "East",
            Self::SOUTH      => "South",
            Self::WEST       => "West",
            Self::NORTH_EAST => "Northeast",
            Self::NORTH_WEST => "Northwest",
            Self::SOUTH_EAST => "Southeast",
            Self::SOUTH_WEST => "Southwest",
            _                => unreachable!(),
        }
    }

    #[inline]
    #[must_use]
    pub const fn as_i8(self) -> i8 {
        self.0
    }
}

impl const Add<Direction> for Square {
    type Output = Option<Self>;

    fn add(self, rhs: Direction) -> Self::Output {
        let from = self.as_u8();
        let out   = Self::from_u8(from.wrapping_add_signed(rhs.as_i8()));

        if let Some(to) = out {
            if self.distance(to) == 1 {
                return out;
            }
        }

        None
    }
}

impl const Neg for Direction {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", std::any::type_name::<Self>(), self.name())
    }
}
