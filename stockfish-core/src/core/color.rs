use crate::prelude::*;

use std::ops::Not;

enumeration! {
    /// A color of a chess token, white or black.
    pub Color, [ White, Black ]
}

impl Color {
    const RANKS: [Rank; Self::COUNT] = [ Rank::_1, Rank::_8 ];

    /// Returns [`true`] if this color is white.
    #[inline]
    #[must_use]
    pub fn is_white(self) -> bool {
        self == Self::White
    }

    /// Returns [`true`] if this color is black.
    #[inline]
    #[must_use]
    pub fn is_black(self) -> bool {
        self == Self::Black
    }

    /// The "home" [`Rank`] for the color. [`Rank::_1`] for [`Color::White`],
    /// [`Rank::_8`] for [`Color::Black`].
    #[inline]
    pub const fn rank(self) -> Rank {
        Self::RANKS[self]
    }

    /// Returns the direction the [`Color`]'s pawns move.
    #[inline]
    pub const fn direction(self) -> Direction {
        if self == Self::White { Direction::N } else { Direction::S }
    }
}

impl const Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        unsafe_optimization!(
            Self::from_u8(self.as_u8() ^ 1).unwrap(),
            Self::from_u8_unchecked(self.as_u8() ^ 1),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction() {
        assert_eq!(Direction::N, Color::White.direction());
        assert_eq!(Direction::S, Color::Black.direction());
    }

    #[test]
    fn not() {
        assert!(!Color::White.is_black());
        assert!(!Color::Black.is_white());
    }
}
