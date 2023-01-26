//! Types for representing fundamental aspects of chess.

/// An internal macro that creates a c-style enum of the style used by many of
/// the core types in this crate.
macro_rules! enumeration {
    ($(#[$outer:meta])* $vis:vis $name:ident, [$($var:ident),+ $(,)?]) => {
        $(#[$outer])*
        #[derive(::core::cmp::Eq)]
        #[derive(::core::fmt::Debug)]
        #[derive(::core::marker::Copy)]
        #[derive_const(::core::cmp::PartialEq)]
        #[derive_const(::core::clone::Clone)]
        #[must_use]
        #[repr(u8)]
        $vis enum $name {
            $(#[allow(missing_docs)] $var = ${index()}),+
        }

        #[allow(dead_code)]
        impl $name {
            /// The total number of [`$name`]s.
            $vis const COUNT: ::core::primitive::usize = ${count(var)};

            /// All of the variants of [`$name`]s, indexable by their
            /// discriminant.
            $vis const VARIANTS: [$name; Self::COUNT] = [
                $(Self::$var,)+
            ];

            /// All of the variant names of [`$name`], indexable by their
            /// discriminant.
            $vis const NAMES: [&'static ::core::primitive::str; Self::COUNT] = [
                $(::core::stringify!($var),)+
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
            pub(crate) const unsafe fn from_u8_unchecked(repr: ::core::primitive::u8) -> Self {
                ::core::debug_assert!(Self::from_u8(repr).is_some());

                ::core::mem::transmute(repr)
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
            pub(crate) const fn from_u8(repr: ::core::primitive::u8) -> ::core::option::Option<Self> {
                Self::VARIANTS.get(repr as usize).copied()
            }

            /// Returns an iterator through all [`$type`] variants.
            #[inline]
            #[must_use]
            $vis fn iter() -> ::core::array::IntoIter<Self, ${count(var)}> {
                ::core::iter::IntoIterator::into_iter(Self::VARIANTS)
            }

            /// Returns the name of the variant as a string.
            #[inline]
            #[must_use]
            $vis fn name(self) -> &'static ::core::primitive::str {
                Self::NAMES[self.as_usize()]
            }

            /// Returns the variant as its underlying representation.
            #[inline]
            #[must_use]
            pub(crate) const fn as_u8(self) -> ::core::primitive::u8 {
                self as _
            }

            /// Returns the variant as a usize.
            #[inline]
            #[must_use]
            pub(crate) const fn as_usize(self) -> ::core::primitive::usize {
                self as _
            }
        }

        impl const ::core::convert::From<$name> for ::core::primitive::u8 {
            #[inline]
            #[must_use]
            fn from(this: $name) -> Self {
                this as _
            }
        }

        impl const ::core::convert::From<$name> for ::core::primitive::usize {
            #[inline]
            #[must_use]
            fn from(this: $name) -> Self {
                this as _
            }
        }

        impl<T> const ::core::ops::Index<$name> for [T; $name::COUNT] {
            type Output = T;

            #[inline]
            fn index(&self, val: $name) -> &Self::Output {
                self.index(val.as_usize())
            }
        }

        impl<T> ::core::ops::IndexMut<$name> for [T; $name::COUNT] {
            #[inline]
            fn index_mut(&mut self, val: $name) -> &mut Self::Output {
                self.index_mut(val.as_usize())
            }
        }

        #[cfg(test)]
        ::paste::paste! {
            mod [<tests _ $name:lower>] {
                use super::*;

                use ::core::assert_eq;
                use ::core::clone::Clone;
                use ::core::convert::From;
                use ::core::iter::Iterator;

                use ::std::format;

                #[test]
                fn test_impl() {
                    for v1 in $name::iter() {
                        assert_eq!(v1.clone(), v1);
                        assert_eq!(v1.name(),  format!("{:?}", v1));
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
}

#[cfg(test)]
mod tests_enumeration_macro {
    #![no_implicit_prelude]

    enumeration!(X, [A, B, C, D]);
}

mod board;
mod castling;
mod color;
mod direction;
mod file;
mod r#move;
mod piece;
mod rank;
mod ruleset;
mod square;
mod token;

pub use board::Board;
pub use castling::{CastlingVariety, CastlingPath, CastlingRights, CastlingSide};
pub use color::Color;
pub use direction::Direction;
pub use file::File;
pub use r#move::{Move, MoveType};
pub use piece::Piece;
pub use rank::Rank;
pub use ruleset::Ruleset;
pub use square::Square;
pub use token::Token;
