use crate::prelude::*;

enumeration! {
    pub CastlingSide, [ Kingside, Queenside ]
}

impl CastlingSide {
    #[inline]
    pub const fn detect(king: File, rook: File) -> Self {
        debug_assert!(king != rook);

        if king < rook { Self::Kingside } else { Self::Queenside }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect() {
        assert_eq!(CastlingSide::Kingside, CastlingSide::detect(File::_F, File::_G));
        assert_eq!(CastlingSide::Kingside, CastlingSide::detect(File::_A, File::_B));

        assert_eq!(CastlingSide::Queenside, CastlingSide::detect(File::_H, File::_C));
        assert_eq!(CastlingSide::Queenside, CastlingSide::detect(File::_F, File::_E));
    }
}
