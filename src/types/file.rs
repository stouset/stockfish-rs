use super::Square;

c_style_enum! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed an underscore to mimic those of [`Rank`].
    pub File, u8, 8; [
        _A, _B, _C, _D, _E, _F, _G, _H,
    ]
}

impl File {
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

impl const From<Square> for File {
    #[inline]
    fn from(s: Square) -> Self {
        // Masking against 0b0111 ensures that the input must be within a valid
        // range.
        #[allow(unsafe_code)]
        unsafe { Self::from_discriminant_unchecked(s.as_u8() & 0b0111) }
    }
}

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
    fn file_try_from_discriminant_out_of_bounds() {
        assert_ne!(None, File::try_from_discriminant(7));
        assert_eq!(None, File::try_from_discriminant(8));
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
