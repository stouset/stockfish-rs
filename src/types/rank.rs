use super::Square;

use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rank(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

impl Rank {
    pub const _1: Self = Self(0o0);
    pub const _2: Self = Self(0o1);
    pub const _3: Self = Self(0o2);
    pub const _4: Self = Self(0o3);
    pub const _5: Self = Self(0o4);
    pub const _6: Self = Self(0o5);
    pub const _7: Self = Self(0o6);
    pub const _8: Self = Self(0o7);

    pub const FIRST: Self  = Self::_1;
    pub const LAST:  Self  = Self::_8;
    pub const MIN:   u8    = Self::FIRST.0;
    pub const MAX:   u8    = Self::LAST.0;
    pub const COUNT: usize = Self::MAX as usize + 1;

    #[inline]
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v == v & Self::MAX { Some(Self(v)) } else { None }
    }

    #[inline]
    #[must_use]
    pub fn iter() -> Iter {
        Iter(Self::MIN, Self::MAX + 1)
    }

    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::_1 => "1",
            Self::_2 => "2",
            Self::_3 => "3",
            Self::_4 => "4",
            Self::_5 => "5",
            Self::_6 => "6",
            Self::_7 => "7",
            Self::_8 => "8",
            _ => unreachable!(),
        }
    }

    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.into())
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl const From<Rank> for u8 {
    #[inline]
    fn from(r: Rank) -> Self {
        r.as_u8()
    }
}

impl const From<Rank> for usize {
    #[inline]
    fn from(r: Rank) -> Self {
        r.as_u8().into()
    }
}

impl const From<Square> for Rank {
    #[inline]
    fn from(s: Square) -> Self {
        Self(s.as_u8() >> 3)
    }
}

impl<T> const Index<Rank> for [T; 8] {
    type Output = T;

    #[inline]
    fn index(&self, index: Rank) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<Rank> for [T; 8] {
    #[inline]
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", std::any::type_name::<Self>(), self.name())
    }
}

impl Iterator for Iter {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        let next = Self::Item::from_u8(self.0);
        self.0  += 1;

        next
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.1 - self.0) as usize;

        (size, Some(size))
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        self.1 -= 1;
        Self::Item::from_u8(self.1)
    }
}

impl FusedIterator for Iter {}

#[cfg(test)]
mod tests {
    use super::*;

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
