
use super::{Square, TryFromPrimitiveError};

use std::iter::FusedIterator;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct File(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8);

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

    #[must_use]
    pub const fn is_ok(v: u8) -> bool {
        v == v & Self::MAX
    }

    #[must_use]
    pub fn iter() -> Iter {
        Iter(Self::MIN)
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self.0 {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            7 => "H",
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        u8::from(self).abs_diff(other.into())
    }
}

impl const From<File> for u8 {
    fn from(s: File) -> Self {
        s.0
    }
}

impl const From<Square> for File {
    fn from(s: Square) -> Self {
        Self(u8::from(s) & 7)
    }
}

impl const TryFrom<u8> for File {
    type Error = TryFromPrimitiveError<Self, u8>;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if !Self::is_ok(v) {
            return Err(TryFromPrimitiveError::new(v));
        }

        Ok(Self(v))
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
        if !Self::Item::is_ok(self.0) {
            return None;
        }

        let next = File(self.0);
        self.0  += 1;

        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (Self::Item::MAX - self.0 + 1) as usize;

        (size, Some(size))
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
