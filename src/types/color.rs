use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

#[must_use]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

impl Color {
    pub const FIRST: Color = Self::White;
    pub const LAST:  Color = Self::Black;
    pub const MIN:   u8    = Self::FIRST.as_u8();
    pub const MAX:   u8    = Self::LAST.as_u8();
    pub const COUNT: usize = Self::MAX as usize + 1;

    #[inline]
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::White),
            1 => Some(Self::Black),
            _ => None,
        }
    }

    #[inline]
    pub const fn iter() -> Iter {
        Iter(Self::MIN, Self::MAX + 1)
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl const From<Color> for u8 {
    #[inline]
    #[must_use]
    fn from(c: Color) -> Self {
        c.as_u8()
    }
}

impl const From<Color> for usize {
    #[inline]
    #[must_use]
    fn from(c: Color) -> Self {
        c.as_u8().into()
    }
}


impl<T> const Index<Color> for [T; Color::COUNT] {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: Color) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<Color> for [T; Color::COUNT] {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        self.index_mut(usize::from(index))
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
