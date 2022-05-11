
use super::Square;

c_style_enum! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed an underscore to mimic those of [`Rank`].
    pub Rank, u8, 8; [
        _1, _2, _3, _4, _5, _6, _7, _8,
    ]
}

impl Rank {
    /// The number of steps it would take a king to move from one rank to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_discriminant().abs_diff(other.into())
    }
}

impl const From<Square> for Rank {
    #[inline]
    fn from(s: Square) -> Self {
        // Masking against 0b0111 ensures that the input must be within a valid
        // range.
        #[allow(unsafe_code)]
        unsafe { Self::from_discriminant_unchecked(s.as_u8() >> 3) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_clone() {
        for rank in Rank::iter() {
            assert_eq!(rank, rank.clone());
        }
    }

    #[test]
    fn rank_debug() {
        assert_ne!("", format!("{:?}", Rank::_7));
    }

    #[test]
    fn rank_try_from_discriminant_out_of_bounds() {
        assert_ne!(None, Rank::try_from_discriminant(7));
        assert_eq!(None, Rank::try_from_discriminant(8));
    }

    #[test]
    fn rank_array_index() {
        let mut a = [0; Rank::COUNT];

        a[Rank::_3] = 3;
        a[Rank::_8] = 4;

        assert_eq!(0, a[Rank::_1]);
        assert_eq!(3, a[Rank::_3]);
        assert_eq!(4, a[Rank::_8]);
    }

    #[test]
    fn rank_iter() {
        let ranks: Vec<Rank> = Rank::iter().collect();

        assert_eq!(ranks, vec![
            Rank::_1, Rank::_2, Rank::_3, Rank::_4,
            Rank::_5, Rank::_6, Rank::_7, Rank::_8,
        ]);
    }

    #[test]
    fn rank_iter_rev() {
        let ranks: Vec<Rank> = Rank::iter().rev().collect();

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
