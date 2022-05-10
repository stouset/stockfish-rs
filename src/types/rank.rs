
use super::Square;

use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

/// A rank, 1 through 8, on a chess board. The variants for this enum are
/// prefixed an underscore due to constant names being unable to begin with
/// digits.
#[must_use]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Rank {
    /// The 1st rank.
    _1 = 0o00,

    /// The 2nd rank.
    _2 = 0o01,

    /// The 3rd rank.
    _3 = 0o02,

    /// The 4th rank.
    _4 = 0o03,

    /// The 5th rank.
    _5 = 0o04,

    /// The 6th rank.
    _6 = 0o05,

    /// The 7th rank.
    _7 = 0o06,

    /// The 8th rank.
    _8 = 0o07,
}

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

impl Rank {
    /// The first variant.
    pub const FIRST: Self = Self::_1;

    /// The first variant.
    pub const LAST: Self = Self::_8;

    /// The value of the first variant.
    pub const MIN: u8 = Self::FIRST.as_u8();

    /// The value of the second variant.
    pub const MAX: u8 = Self::LAST.as_u8();

    /// The number of enum variants.
    pub const COUNT: usize = Self::MAX as usize + 1;

    /// Converts the provided [`u8`] to its corresponding [`Rank`].
    ///
    /// # Safety
    ///
    /// This function is unsafe. You must guarantee that the input value maps to
    /// a real variant of this enum.
    #[inline]
    #[allow(unsafe_code)]
    pub const unsafe fn from_u8_unchecked(v: u8) -> Self {
        std::mem::transmute(v)
    }

    /// Attempts to convert the provided [`u8`] to its corresponding [`Rank`].
    #[inline]
    #[must_use]
    pub const fn try_from_u8(v: u8) -> Option<Self> {
        if v > Self::MAX {
            return None;
        }

        // the above check ensures that the value is valid
        #[allow(unsafe_code)]
        unsafe { Self::from_u8_unchecked(v) }.into()
    }

    /// Returns an iterator through all ranks A through H.
    #[inline]
    pub const fn iter() -> Iter {
        Iter(Self::MIN, Self::MAX + 1)
    }

    /// The number of steps it would take a king to move from one rank to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.into())
    }

    /// Converts the [`Rank`] to its underlying [`u8`] representation.
    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as _
    }
}

impl const From<Rank> for u8 {
    #[inline]
    #[must_use]
    fn from(r: Rank) -> Self {
        r.as_u8()
    }
}

impl const From<Rank> for usize {
    #[inline]
    #[must_use]
    fn from(r: Rank) -> Self {
        r.as_u8().into()
    }
}

impl const From<Square> for Rank {
    #[inline]
    fn from(s: Square) -> Self {
        // Masking against 0b0111 ensures that the input must be within a valid
        // range.
        #[allow(unsafe_code)]
        unsafe { Self::from_u8_unchecked(s.as_u8() >> 3) }
    }
}

impl<T> const Index<Rank> for [T; Rank::COUNT] {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: Rank) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<Rank> for [T; Rank::COUNT] {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl Iterator for Iter {
    type Item = Rank;

    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        // The above check ensures that the instantiated item is within the
        // valid range of possible discriminants.
        #[allow(unsafe_code)]
        let next = unsafe { Self::Item::from_u8_unchecked(self.0) };
        self.0  += 1;

        Some(next)
    }

    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.1 - self.0) as usize;

        (size, Some(size))
    }
}

impl DoubleEndedIterator for Iter {
    #[must_use]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        self.1 -= 1;

        // The above check ensures that the instantiated item is within the
        // valid range of possible discriminants.
        #[allow(unsafe_code)]
        Some(unsafe {
            Self::Item::from_u8_unchecked(self.1)
        })
    }
}

impl FusedIterator for Iter {}

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
    fn rank_try_from_u8_out_of_bounds() {
        assert_ne!(None, Rank::try_from_u8(Rank::MAX));
        assert_eq!(None, Rank::try_from_u8(Rank::MAX + 1));
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
