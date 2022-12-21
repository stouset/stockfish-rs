use crate::prelude::*;

enumeration! {
    /// A rank, 1 through 8, on a chess board. The variants for this enum are
    /// prefixed with an underscore since identifiers may not begin with a
    /// number.
    pub Rank, u8, [
        _1, _2, _3, _4, _5, _6, _7, _8,
    ]
}

impl Rank {
    /// The underlying value of the [`Rank`] as a [`u8`].
    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.as_repr()
    }

    /// The number of steps it would take a king to move from one rank to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.as_u8())
    }
}

impl const From<Square> for Rank {
    #[inline]
    fn from(s: Square) -> Self {
        // Masking against 0b0111 ensures that the input must be within a valid
        // range.
        #[allow(unsafe_code)]
        unsafe { Self::from_repr_unchecked(s.as_u8() >> 3) }
    }
}

impl const std::ops::BitOr<File> for Rank {
    type Output = Square;

    fn bitor(self, rhs: File) -> Self::Output {
        Square::new(rhs, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_clone() {
        for rank in Rank::into_iter() {
            assert_eq!(rank, rank.clone());
        }
    }

    #[test]
    fn rank_debug() {
        assert_ne!("", format!("{:?}", Rank::_7));
    }

    #[test]
    fn rank_try_from_repr_out_of_bounds() {
        assert_ne!(None, Rank::from_repr(7));
        assert_eq!(None, Rank::from_repr(8));
    }

    #[test]
    fn rank_iter() {
        let ranks: Vec<Rank> = Rank::into_iter().collect();

        assert_eq!(ranks, vec![
            Rank::_1, Rank::_2, Rank::_3, Rank::_4,
            Rank::_5, Rank::_6, Rank::_7, Rank::_8,
        ]);
    }

    #[test]
    fn rank_iter_rev() {
        let ranks: Vec<Rank> = Rank::into_iter().rev().collect();

        assert_eq!(ranks, vec![
            Rank::_8, Rank::_7, Rank::_6, Rank::_5,
            Rank::_4, Rank::_3, Rank::_2, Rank::_1,
        ]);
    }

    #[test]
    fn rank_distance() {
        assert_eq!(Rank::_1.distance(Rank::_1), 0);
        assert_eq!(Rank::_1.distance(Rank::_2), 1);
        assert_eq!(Rank::_1.distance(Rank::_3), 2);
        assert_eq!(Rank::_1.distance(Rank::_4), 3);
        assert_eq!(Rank::_1.distance(Rank::_5), 4);
        assert_eq!(Rank::_1.distance(Rank::_6), 5);
        assert_eq!(Rank::_1.distance(Rank::_7), 6);
        assert_eq!(Rank::_1.distance(Rank::_8), 7);
        assert_eq!(Rank::_2.distance(Rank::_1), 1);
        assert_eq!(Rank::_2.distance(Rank::_7), 5);
        assert_eq!(Rank::_2.distance(Rank::_8), 6);
        assert_eq!(Rank::_3.distance(Rank::_3), 0);
        assert_eq!(Rank::_7.distance(Rank::_1), 6);
        assert_eq!(Rank::_7.distance(Rank::_8), 1);
        assert_eq!(Rank::_8.distance(Rank::_8), 0);
    }
}
