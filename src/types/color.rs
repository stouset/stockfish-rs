use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

#[must_use]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

impl Color {
    pub const WHITE: Self = Self(0b0);
    pub const BLACK: Self = Self(0b1);

    pub const FIRST: Self  = Self::WHITE;
    pub const LAST:  Self  = Self::BLACK;
    pub const MIN:   u8    = Self::FIRST.0;
    pub const MAX:   u8    = Self::LAST.0;
    pub const COUNT: usize = Self::MAX as usize + 1;

    #[inline]
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v == v & Self::MAX { Some(Self(v)) } else { None }
    }

    #[inline]
    pub fn iter() -> Iter {
        Iter(Self::MIN, Self::MAX + 1)
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::WHITE => "white",
            Self::BLACK => "black",
            _           => unreachable!(),
        }
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl const From<Color> for u8 {
    #[inline]
    #[must_use]
    fn from(s: Color) -> Self {
        s.as_u8()
    }
}

impl const From<Color> for usize {
    #[inline]
    #[must_use]
    fn from(s: Color) -> Self {
        s.as_u8().into()
    }
}


impl<T> const Index<Color> for [T; 2] {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: Color) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<Color> for [T; 2] {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", std::any::type_name::<Self>(), self.name())
    }
}

impl Iterator for Iter {
    type Item = Color;

    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        let next = Self::Item::from_u8(self.0);
        self.0  += 1;

        next
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
        Self::Item::from_u8(self.1)
    }
}

impl FusedIterator for Iter {}
