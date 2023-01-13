use crate::prelude::*;

enumeration! {
    pub CastlingSide, [ King, Queen ]
}

impl CastlingSide {
    #[inline]
    pub const fn new(king: File, rook: File) -> Self {
        debug_assert!(king != rook);

        if king < rook { Self::King } else { Self::Queen }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect() {
        assert_eq!(CastlingSide::King,  CastlingSide::new(File::_F, File::_G));
        assert_eq!(CastlingSide::King,  CastlingSide::new(File::_A, File::_B));
        assert_eq!(CastlingSide::Queen, CastlingSide::new(File::_H, File::_C));
        assert_eq!(CastlingSide::Queen, CastlingSide::new(File::_F, File::_E));
    }
}
