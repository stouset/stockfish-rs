use crate::prelude::*;

enumeration! {
    /// A color of a chess piece, white or black.
    pub Color, [ White, Black ]
}

impl Color {
    const RANKS: [Rank; Self::COUNT] = [ Rank::_1, Rank::_8 ];

    /// The "home" [`Rank`] for the color. [`Rank::_1`] for [`Color::White`],
    /// [`Rank::_8`] for [`Color::Black`].
    #[inline]
    pub const fn rank(self) -> Rank {
        Self::RANKS[self]
    }
}
