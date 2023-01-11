/// An internal macro that creates a c-style enum of the style used by many of
/// the core types in this crate.
macro_rules! enumeration {
    ($(#[$outer:meta])* $vis:vis $name:ident, [$($var:ident),+ $(,)?]) => {
        $(#[$outer])*
        #[derive(Copy, Debug, Eq)]
        #[derive_const(Clone, PartialEq, Ord, PartialOrd)]
        #[must_use]
        #[repr(u8)]
        $vis enum $name {
            $($var = ${index()}),+
        }

        #[allow(dead_code)]
        impl $name {
            /// The total number of [`$name`]s.
            $vis const COUNT: usize = ${count(var)};

            /// All of the variants of [`$name`]s, indexable by their
            /// discriminant.
            $vis const VARIANTS: [$name; Self::COUNT] = [
                $(Self::$var,)+
            ];

            /// All of the variant names of [`$name`], indexable by their
            /// discriminant.
            $vis const NAMES: [&'static str; Self::COUNT] = [
                $(stringify!($var),)+
            ];

            /// Converts the provided [`u8`] discriminant to its corresponding
            /// [`$name`].
            ///
            /// # Safety
            ///
            /// This function is unsafe. You *must* guarantee that the input
            /// value is a real discriminant of this type.
            #[inline]
            #[allow(unsafe_code)]
            pub(crate) const unsafe fn from_u8_unchecked(repr: u8) -> Self {
                debug_assert!(Self::from_u8(repr).is_some());

                std::mem::transmute(repr)
            }

            /// Converts the provided [`u8`] discriminant to its corresponding
            /// [`$name`].
            ///
            /// # Panics
            ///
            /// This function panics if there is not a discriminant with the
            /// given value.
            #[inline]
            #[must_use]
            pub(crate) const fn from_u8(repr: u8) -> Option<Self> {
                Self::VARIANTS.get(repr as usize).copied()
            }

            /// Returns an iterator through all [`$type`] variants.
            #[inline]
            #[must_use]
            $vis fn iter() -> std::array::IntoIter<Self, ${count(var)}> {
                Self::VARIANTS.into_iter()
            }

            /// Returns the name of the variant as a string.
            #[inline]
            #[must_use]
            $vis fn name(self) -> &'static str {
                Self::NAMES[self.as_usize()]
            }

            /// Returns the variant as its underlying representation.
            #[inline]
            #[must_use]
            pub(crate) const fn as_u8(self) -> u8 {
                self as u8
            }

            /// Returns the variant as a usize.
            #[inline]
            #[must_use]
            pub(crate) const fn as_usize(self) -> usize {
                self as _
            }
        }

        impl const From<$name> for u8 {
            #[inline]
            #[must_use]
            fn from(this: $name) -> Self {
                this as _
            }
        }

        impl const From<$name> for usize {
            #[inline]
            #[must_use]
            fn from(this: $name) -> Self {
                this as usize
            }
        }

        impl<T> const std::ops::Index<$name> for [T; $name::COUNT] {
            type Output = T;

            fn index(&self, val: $name) -> &Self::Output {
                self.index(val.as_usize())
            }
        }

        impl<T> std::ops::IndexMut<$name> for [T; $name::COUNT] {
            fn index_mut(&mut self, val: $name) -> &mut Self::Output {
                self.index_mut(val.as_usize())
            }
        }

        #[cfg(test)]
        mod tests_enumerate_macro {
            use super::*;

            #[test]
            fn test_impl() {
                for v1 in $name::iter() {
                    assert_eq!(v1.clone(), v1);
                    assert_eq!(v1.name(),  format!("{:?}", v1));

                    for v2 in $name::iter() {
                        assert_eq!(
                            v1.partial_cmp(&v2).unwrap(),
                            v1.cmp(&v2)
                        );
                    }
                }
            }

            #[test]
            fn test_impl_from() {
                for (repr, variant) in $name::iter().enumerate() {
                    assert_eq!(repr as u8, u8   ::from(variant));
                    assert_eq!(repr,       usize::from(variant));
                }
            }

            #[test]
            fn test_from_u8() {
                for (repr, variant) in $name::iter().enumerate() {
                    assert_eq!(variant, unsafe_optimization!{
                        $name::from_u8(repr as _).unwrap(),
                        $name::from_u8_unchecked(repr as _)
                    })
                }
            }
        }
    }
}

mod board;
mod castling;
mod color;
mod direction;
mod file;
mod piece;
mod rank;
mod square;
mod token;

pub use board::Board;
pub use castling::Castling;
pub use color::Color;
pub use direction::Direction;
pub use file::File;
pub use piece::Piece;
pub use rank::Rank;
pub use square::Square;
pub use token::Token;
