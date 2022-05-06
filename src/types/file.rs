
use super::Square;

use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct File(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

impl File {
    pub const _A: Self = Self(0o0);
    pub const _B: Self = Self(0o1);
    pub const _C: Self = Self(0o2);
    pub const _D: Self = Self(0o3);
    pub const _E: Self = Self(0o4);
    pub const _F: Self = Self(0o5);
    pub const _G: Self = Self(0o6);
    pub const _H: Self = Self(0o7);

    pub const FIRST: Self  = Self::_A;
    pub const LAST:  Self  = Self::_H;
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
            Self::_A => "A",
            Self::_B => "B",
            Self::_C => "C",
            Self::_D => "D",
            Self::_E => "E",
            Self::_F => "F",
            Self::_G => "G",
            Self::_H => "H",
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

impl const From<File> for u8 {
    #[inline]
    fn from(s: File) -> Self {
        s.as_u8()
    }
}

impl const From<File> for usize {
    #[inline]
    fn from(s: File) -> Self {
        s.as_u8().into()
    }
}

impl const From<Square> for File {
    #[inline]
    fn from(s: Square) -> Self {
        Self(s.as_u8() & 7)
    }
}

impl<T> const Index<File> for [T; 8] {
    type Output = T;

    #[inline]
    fn index(&self, index: File) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<File> for [T; 8] {
    #[inline]
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", std::any::type_name::<Self>(), self.name())
    }
}

impl Iterator for Iter {
    type Item = File;

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
    fn file_distance() {
        assert_eq!(File::_A.distance(File::_A), 0);
        assert_eq!(File::_A.distance(File::_B), 1);
        assert_eq!(File::_A.distance(File::_C), 2);
        assert_eq!(File::_A.distance(File::_D), 3);
        assert_eq!(File::_A.distance(File::_E), 4);
        assert_eq!(File::_A.distance(File::_F), 5);
        assert_eq!(File::_A.distance(File::_G), 6);
        assert_eq!(File::_A.distance(File::_H), 7);
        assert_eq!(File::_B.distance(File::_A), 1);
        assert_eq!(File::_B.distance(File::_G), 5);
        assert_eq!(File::_B.distance(File::_H), 6);
        assert_eq!(File::_C.distance(File::_C), 0);
        assert_eq!(File::_G.distance(File::_A), 6);
        assert_eq!(File::_G.distance(File::_H), 1);
        assert_eq!(File::_H.distance(File::_H), 0);
    }
}
