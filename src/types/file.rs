
use super::Square;

use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

/// A file, A through H, on a chess board. The variants for this enum are
/// prefixed an underscore to mimic those of [`Rank`].
#[must_use]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum File {
    /// The A file.
    _A = 0o00,

    /// The B file.
    _B = 0o01,

    /// The C file.
    _C = 0o02,

    /// The D file.
    _D = 0o03,

    /// The E file.
    _E = 0o04,

    /// The F file.
    _F = 0o05,

    /// The G file.
    _G = 0o06,

    /// The H file.
    _H = 0o07,
}

// implementing Copy on Iterator is a footgun
#[allow(missing_copy_implementations)]
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iter(u8, u8);

impl File {
    /// The first variant.
    pub const FIRST: Self = Self::_A;

    /// The first variant.
    pub const LAST: Self = Self::_H;

    /// The value of the first variant.
    pub const MIN: u8 = Self::FIRST.as_u8();

    /// The value of the second variant.
    pub const MAX: u8 = Self::LAST.as_u8();

    /// The number of enum variants.
    pub const COUNT: usize = Self::MAX as usize + 1;

    /// Converts the provided [`u8`] to its corresponding [`File`].
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

    /// Attempts to convert the provided [`u8`] to its corresponding [`File`].
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

    /// Returns an iterator through all files A through H.
    #[inline]
    pub const fn iter() -> Iter {
        Iter(Self::MIN, Self::MAX + 1)
    }

    /// The number of steps it would take a king to move from one file to the
    /// other.
    #[inline]
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.as_u8().abs_diff(other.into())
    }

    /// Converts the [`File`] to its underlying [`u8`] representation.
    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as _
    }
}

impl const From<File> for u8 {
    #[inline]
    #[must_use]
    fn from(f: File) -> Self {
        f.as_u8()
    }
}

impl const From<File> for usize {
    #[inline]
    #[must_use]
    fn from(f: File) -> Self {
        f.as_u8().into()
    }
}

impl const From<Square> for File {
    #[inline]
    fn from(s: Square) -> Self {
        // Masking against 0b0111 ensures that the input must be within a valid
        // range.
        #[allow(unsafe_code)]
        unsafe { Self::from_u8_unchecked(s.as_u8() & 0b0111) }
    }
}

impl<T> const Index<File> for [T; File::COUNT] {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: File) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> const IndexMut<File> for [T; File::COUNT] {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}

impl Iterator for Iter {
    type Item = File;

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
    fn file_clone() {
        for file in File::iter() {
            assert_eq!(file, file.clone());
        }
    }

    #[test]
    fn file_debug() {
        assert_ne!("", format!("{:?}", File::_G));
    }

    #[test]
    fn file_try_from_u8_out_of_bounds() {
        assert_ne!(None, File::try_from_u8(File::MAX));
        assert_eq!(None, File::try_from_u8(File::MAX + 1));
    }

    #[test]
    fn file_array_index() {
        let mut a = [0; File::COUNT];

        a[File::_C] = 3;
        a[File::_H] = 4;

        assert_eq!(0, a[File::_A]);
        assert_eq!(3, a[File::_C]);
        assert_eq!(4, a[File::_H]);
    }

    #[test]
    fn file_iter() {
        let files: Vec<File> = File::iter().collect();

        assert_eq!(files, vec![
            File::_A, File::_B, File::_C, File::_D,
            File::_E, File::_F, File::_G, File::_H,
        ]);
    }

    #[test]
    fn file_iter_rev() {
        let files: Vec<File> = File::iter().rev().collect();

        assert_eq!(files, vec![
            File::_H, File::_G, File::_F, File::_E,
            File::_D, File::_C, File::_B, File::_A,
        ]);
    }

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
