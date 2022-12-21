enumeration! {
    /// A color of a chess piece, white or black.
    pub Color, u8, [ White, Black ]
}

impl Color {
    /// The underlying value of the [`Color`] as a [`u8`].
    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.as_repr()
    }
}
