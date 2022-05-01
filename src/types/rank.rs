use super::{Square, TryFromPrimitiveError};

use std::iter::FusedIterator;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rank(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8);

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
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        u8::from(self).abs_diff(other.into())
    }
}

impl const From<Rank> for u8 {
    fn from(r: Rank) -> Self {
        r.0
    }
}

impl const From<Square> for Rank {
    fn from(s: Square) -> Self {
        Self(u8::from(s) >> 3)
    }
}

impl const TryFrom<u8> for Rank {
    type Error = TryFromPrimitiveError<Self, u8>;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if !Self::is_ok(v) {
            return Err(TryFromPrimitiveError::new(v));
        }

        Ok(Self(v))
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
        if !Self::Item::is_ok(self.0) {
            return None;
        }

        let next = Rank(self.0);
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
