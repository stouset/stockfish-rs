use super::{File, Rank};

use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Square(u8);

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8);

impl Square {
    pub const A1: Self = Self(0o00);
    pub const B1: Self = Self(0o01);
    pub const C1: Self = Self(0o02);
    pub const D1: Self = Self(0o03);
    pub const E1: Self = Self(0o04);
    pub const F1: Self = Self(0o05);
    pub const G1: Self = Self(0o06);
    pub const H1: Self = Self(0o07);
    pub const A2: Self = Self(0o10);
    pub const B2: Self = Self(0o11);
    pub const C2: Self = Self(0o12);
    pub const D2: Self = Self(0o13);
    pub const E2: Self = Self(0o14);
    pub const F2: Self = Self(0o15);
    pub const G2: Self = Self(0o16);
    pub const H2: Self = Self(0o17);
    pub const A3: Self = Self(0o20);
    pub const B3: Self = Self(0o21);
    pub const C3: Self = Self(0o22);
    pub const D3: Self = Self(0o23);
    pub const E3: Self = Self(0o24);
    pub const F3: Self = Self(0o25);
    pub const G3: Self = Self(0o26);
    pub const H3: Self = Self(0o27);
    pub const A4: Self = Self(0o30);
    pub const B4: Self = Self(0o31);
    pub const C4: Self = Self(0o32);
    pub const D4: Self = Self(0o33);
    pub const E4: Self = Self(0o34);
    pub const F4: Self = Self(0o35);
    pub const G4: Self = Self(0o36);
    pub const H4: Self = Self(0o37);
    pub const A5: Self = Self(0o40);
    pub const B5: Self = Self(0o41);
    pub const C5: Self = Self(0o42);
    pub const D5: Self = Self(0o43);
    pub const E5: Self = Self(0o44);
    pub const F5: Self = Self(0o45);
    pub const G5: Self = Self(0o46);
    pub const H5: Self = Self(0o47);
    pub const A6: Self = Self(0o50);
    pub const B6: Self = Self(0o51);
    pub const C6: Self = Self(0o52);
    pub const D6: Self = Self(0o53);
    pub const E6: Self = Self(0o54);
    pub const F6: Self = Self(0o55);
    pub const G6: Self = Self(0o56);
    pub const H6: Self = Self(0o57);
    pub const A7: Self = Self(0o60);
    pub const B7: Self = Self(0o61);
    pub const C7: Self = Self(0o62);
    pub const D7: Self = Self(0o63);
    pub const E7: Self = Self(0o64);
    pub const F7: Self = Self(0o65);
    pub const G7: Self = Self(0o66);
    pub const H7: Self = Self(0o67);
    pub const A8: Self = Self(0o70);
    pub const B8: Self = Self(0o71);
    pub const C8: Self = Self(0o72);
    pub const D8: Self = Self(0o73);
    pub const E8: Self = Self(0o74);
    pub const F8: Self = Self(0o75);
    pub const G8: Self = Self(0o76);
    pub const H8: Self = Self(0o77);

    pub const FIRST: Self  = Self::A1;
    pub const LAST:  Self  = Self::H8;
    pub const MIN:   u8    = Self::FIRST.0;
    pub const MAX:   u8    = Self::LAST.0;
    pub const COUNT: usize = Self::MAX as usize + 1;

    pub const NAMES: [&'static str; Self::COUNT] = [
        "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
        "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2",
        "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3",
        "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4",
        "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5",
        "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6",
        "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7",
        "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8",
    ];

    #[inline]
    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Self {
        let f: u8 = file.into();
        let r: u8 = rank.into();
        let s: u8 = (r << 3) + f;

        Self(s)
    }

    #[inline]
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v == v & Self::MAX { Some(Self(v)) } else { None }
    }

    #[inline]
    #[must_use]
    pub fn iter() -> Iter {
        Iter(Self::MIN)
    }

    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        Self::NAMES[self.0 as usize]
    }

    #[inline]
    #[must_use]
    pub const fn file(self) -> File {
        self.into()
    }

    #[inline]
    #[must_use]
    pub const fn rank(self) -> Rank {
        self.into()
    }

    #[inline]
    #[must_use]
    pub const fn flip_file(self) -> Self {
        self ^ Square::H1
    }

    #[inline]
    #[must_use]
    pub const fn flip_rank(self) -> Self {
       self ^ Square::A8
    }

    #[inline]
    #[must_use]
    pub const fn distance_files(self, other: Self) -> u8 {
        self.file().distance(other.file())
    }

    #[inline]
    #[must_use]
    pub const fn distance_ranks(self, other: Self) -> u8 {
        self.rank().distance(other.rank())
    }

    #[inline]
    #[must_use]
    pub const fn distance(self, rhs: Self) -> u8 {
        crate::bitboard::square_distance(self, rhs)
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl const std::ops::BitXor for Square {
    type Output = Square;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.as_u8() ^ rhs.as_u8())
    }
}

impl const From<Square> for u8 {
    #[inline]
    fn from(s: Square) -> Self {
        s.as_u8()
    }
}

impl const From<Square> for usize {
    #[inline]
    fn from(s: Square) -> Self {
        s.as_u8().into()
    }
}

impl<T> const Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", std::any::type_name::<Self>(), self.name())
    }
}

impl Iterator for Iter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Self::Item::from_u8(self.0);
        self.0  += 1;

        next
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
    fn square_new() {
        assert_eq!(Square::A1, Square::new(File::_A, Rank::_1));
        assert_eq!(Square::B3, Square::new(File::_B, Rank::_3));
        assert_eq!(Square::B4, Square::new(File::_B, Rank::_4));
        assert_eq!(Square::C7, Square::new(File::_C, Rank::_7));
        assert_eq!(Square::D2, Square::new(File::_D, Rank::_2));
        assert_eq!(Square::E6, Square::new(File::_E, Rank::_6));
        assert_eq!(Square::F5, Square::new(File::_F, Rank::_5));
        assert_eq!(Square::G8, Square::new(File::_G, Rank::_8));
        assert_eq!(Square::H8, Square::new(File::_H, Rank::_8));
    }

    #[test]
    fn square_from_u8() {
        assert_eq!(Square::A1, Square::from_u8(0o00).unwrap());
        assert_eq!(Square::B3, Square::from_u8(0o21).unwrap());
        assert_eq!(Square::B4, Square::from_u8(0o31).unwrap());
        assert_eq!(Square::C7, Square::from_u8(0o62).unwrap());
        assert_eq!(Square::D2, Square::from_u8(0o13).unwrap());
        assert_eq!(Square::E6, Square::from_u8(0o54).unwrap());
        assert_eq!(Square::F5, Square::from_u8(0o45).unwrap());
        assert_eq!(Square::G8, Square::from_u8(0o76).unwrap());
        assert_eq!(Square::H8, Square::from_u8(0o77).unwrap());
    }

    #[test]
    fn square_name() {
        assert_eq!(Square::A1.name(), "A1");
        assert_eq!(Square::C4.name(), "C4");
        assert_eq!(Square::G2.name(), "G2");
        assert_eq!(Square::H8.name(), "H8");
    }

    #[test]
    fn square_file_rank() {
        for s in 0..64 {
            let square = Square(s);
            let file   = File::from_u8(s & 7) .unwrap();
            let rank   = Rank::from_u8(s >> 3).unwrap();

            assert_eq!(square.file(), file);
            assert_eq!(square.rank(), rank);
        }
    }

    #[test]
    fn square_flip_file() {
        assert_eq!(Square::A2.flip_file(), Square::H2);
        assert_eq!(Square::D1.flip_file(), Square::E1);
        assert_eq!(Square::G7.flip_file(), Square::B7);
        assert_eq!(Square::H6.flip_file(), Square::A6);
    }

    #[test]
    fn square_flip_file_reflexive() {
        for i in 0..64 {
            let s = Square(i);

            assert_eq!(s, s.flip_file().flip_file());
        }
    }

    #[test]
    fn square_flip_rank() {
        assert_eq!(Square::A2.flip_rank(), Square::A7);
        assert_eq!(Square::D1.flip_rank(), Square::D8);
        assert_eq!(Square::G7.flip_rank(), Square::G2);
        assert_eq!(Square::H6.flip_rank(), Square::H3);
    }

    #[test]
    fn square_flip_rank_reflexive() {
        for i in 0..64 {
            let s = Square(i);

            assert_eq!(s, s.flip_rank().flip_rank());
        }
    }

    #[test]
    fn square_flip_around_the_world() {
        for i in 0..64 {
            let s = Square(i);

            assert_eq!(s, s.flip_file().flip_rank().flip_file().flip_rank());
            assert_eq!(s.flip_file(), s.flip_rank().flip_file().flip_rank());
            assert_eq!(s.flip_file().flip_rank(), s.flip_rank().flip_file());

            assert_eq!(s, s.flip_rank().flip_file().flip_rank().flip_file());
            assert_eq!(s.flip_rank(), s.flip_file().flip_rank().flip_file());
            assert_eq!(s.flip_rank().flip_file(), s.flip_file().flip_rank());
        }
    }

    #[test]
    fn square_distance() {
        for i in 0..64 {
            for j in 0..64 {
                let s1 = Square(i);
                let s2 = Square(j);

                assert_eq!(s1.distance(s2), std::cmp::max(
                    s1.distance_files(s2),
                    s1.distance_ranks(s2),
                ));

                assert_eq!(s1.distance(s2), s2.distance(s1));
            }
        }
    }
}
