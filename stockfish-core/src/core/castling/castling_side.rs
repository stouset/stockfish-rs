use crate::prelude::*;

enumeration! {
    /// Represents a side to which castling may occur, kingside or queenside.
    pub CastlingSide, [ King, Queen ]
}

impl CastlingSide {
    /// Determines the appropriate [`CastlingSide`] from the [`File`]s the king and
    /// rook are on. If the king is to the left of the rook,
    #[inline]
    #[must_use]
    pub const fn new(king: File, rook: File) -> Option<Self> {
        match king.cmp(&rook) {
            core::cmp::Ordering::Less    => Some(Self::King),
            core::cmp::Ordering::Equal   => None,
            core::cmp::Ordering::Greater => Some(Self::Queen),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect() {
        assert_eq!(CastlingSide::King,  CastlingSide::new(File::_F, File::_G).unwrap());
        assert_eq!(CastlingSide::King,  CastlingSide::new(File::_A, File::_B).unwrap());
        assert_eq!(CastlingSide::Queen, CastlingSide::new(File::_H, File::_C).unwrap());
        assert_eq!(CastlingSide::Queen, CastlingSide::new(File::_F, File::_E).unwrap());
    }
}
