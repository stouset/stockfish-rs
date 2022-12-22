enumeration! {
    /// A color of a chess piece, white or black.
    pub Color, u8, [ White, Black ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u8() {
        assert_eq!(0, Color::White.as_u8());
        assert_eq!(1, Color::Black.as_u8());
    }
}
