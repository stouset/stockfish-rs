macro_rules! c_style_enum {
    ($(#[$outer:meta])* $vis:vis $name:ident, $repr:ty, $count:literal; [$($var:ident),+ $(,)?]) => (
        $(#[$outer])*
        #[repr($repr)]
        #[derive(Copy, Clone, Debug, Eq)]
        #[must_use]
        $vis enum $name {
            $($var = ${index()}),+
        }

        impl $name {
            /// All of the variants of [`$name`]s, indexable by their
            /// discriminant.
            $vis const VARIANTS: [$name; $count] = [
                $(Self::$var,)+
            ];

            /// The total number of [`$name`]s.
            $vis const COUNT: usize = Self::VARIANTS.len();

            /// Converts the provided [`$repr`] discriminant to its
            /// corresponding [`$name`].
            ///
            /// # Safety
            ///
            /// This function is unsafe. You *must* guarantee that the input
            /// value is a real discriminant of this type.
            #[inline]
            #[allow(unsafe_code)]
            $vis const unsafe fn from_discriminant_unchecked(d: $repr) -> Self {
                std::mem::transmute(d)
            }

            /// Attempts to convert the provided [`$repr`] to its corresponding
            /// [`$name`].
            #[inline]
            #[must_use]
            pub fn try_from_discriminant(d: $repr) -> Option<Self> {
                Self::VARIANTS.get(d as usize).copied()
            }

            /// Returns an iterator through all [`$type`] variants.
            #[inline]
            #[must_use]
            pub const fn iter() -> std::array::IntoIter<Self, $count> {
                Self::VARIANTS.into_iter()
            }

            /// Returns the variant as its underlying discriminant.
            #[inline]
            #[must_use]
            pub const fn as_discriminant(self) -> $repr {
                self as $repr
            }
        }

        impl const PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                *self as $repr == *other as $repr
            }
        }

        impl<T> const std::ops::Index<$name> for [T; $count] {
            type Output = T;

            #[inline]
            #[must_use]
            fn index(&self, index: $name) -> &Self::Output {
                self.index(usize::from(index))
            }
        }

        impl<T> const std::ops::IndexMut<$name> for [T; $count] {
            #[inline]
            #[must_use]
            fn index_mut(&mut self, index: $name) -> &mut Self::Output {
                self.index_mut(usize::from(index))
            }
        }

        impl const From<$name> for $repr {
            #[inline]
            #[must_use]
            fn from(this: $name) -> Self {
                this as $repr
            }
        }

        impl const From<$name> for usize {
            #[inline]
            #[must_use]
            fn from(this: $name) -> Self {
                this as usize
            }
        }
    )
}
