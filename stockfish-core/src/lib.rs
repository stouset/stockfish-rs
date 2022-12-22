//! # `stockfish_core`

// Lint Groups
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]

// Individual Lints
#![warn(deprecated_in_future)]
#![warn(fuzzy_provenance_casts)]
#![warn(lossy_provenance_casts)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

// Lint Extensions
#![warn(rustdoc::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

// Additional Clippy Lints
#![warn(clippy::dbg_macro)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exit)]
#![warn(clippy::expect_used)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::string_slice)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::use_debug)]

// Additional Clippy Lints Excluded During Testing
// #![cfg_attr(not(test), warn(clippy::unwrap_in_result))]
// #![cfg_attr(not(test), warn(clippy::unwrap_used))]
// #![cfg_attr(not(test), warn(clippy::unreachable))]

// Lint Exceptions
#![allow(unstable_features)]

// TODO: remove
#![allow(missing_docs)]
#![allow(meta_variable_misuse)] // false positive with `count()`
#![allow(unused_macro_rules)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::missing_docs_in_private_items)]

// Unstable Features
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_discriminant)]
#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(const_slice_index)]
#![feature(derive_const)]
#![feature(macro_metavar_expr)]
#![feature(rustdoc_missing_doc_code_examples)]
#![feature(strict_provenance)]

macro_rules! unsafe_optimization {
    ($safe:expr, $unsafe:expr) => {{
        #[allow(unsafe_code)]
        unsafe {
            debug_assert!($safe == $unsafe);
            $unsafe
        }
    }};
}

macro_rules! enumeration {
    ($(#[$outer:meta])* $vis:vis $name:ident, $repr:ty, [$($var:ident),+ $(,)?]) => {
        $(#[$outer])*
        #[derive(Copy, Debug, Eq)]
        #[derive_const(Clone, PartialEq, Ord, PartialOrd)]
        #[must_use]
        #[repr($repr)]
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

            paste::paste!{
                /// Converts the provided [`$repr`] discriminant to its
                /// corresponding [`$name`].
                ///
                /// # Safety
                ///
                /// This function is unsafe. You *must* guarantee that the input
                /// value is a real discriminant of this type.
                #[inline]
                #[allow(unsafe_code)]
                pub(crate) const unsafe fn [<from_ $repr _unchecked>](repr: $repr) -> Self {
                    debug_assert!(Self::from_u8(repr).is_some());

                    *Self::VARIANTS.get_unchecked(repr as usize)
                }

                /// Converts the provided [`$repr`] discriminant to its
                /// corresponding [`$name`].
                ///
                /// # Panics
                ///
                /// This function panics if there is not a discriminant with the
                /// given value.
                #[inline]
                #[must_use]
                pub(crate) const fn [<from_ $repr>](repr: $repr) -> Option<Self> {
                    Self::VARIANTS.get(repr as usize).copied()
                }
            }

            /// Returns an iterator through all [`$type`] variants.
            #[inline]
            #[must_use]
            $vis fn into_iter() -> std::array::IntoIter<Self, ${count(var)}> {
                Self::VARIANTS.into_iter()
            }

            /// Returns the name of the variant as a string.
            #[inline]
            #[must_use]
            $vis fn name(self) -> &'static str {
                Self::NAMES[self.as_usize()]
            }

            paste::paste! {
                /// Returns the variant as its underlying representation.
                #[inline]
                #[must_use]
                pub(crate) const fn [<as_ $repr>](self) -> $repr {
                    self as $repr
                }
            }

            /// Returns the variant as a usize.
            #[inline]
            #[must_use]
            pub(crate) const fn as_usize(self) -> usize {
                self as _
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

        #[cfg(test)]
        mod tests_enumerate_macro {
            use super::*;

            #[test]
            fn test_impl() {
                for v1 in $name::into_iter() {
                    assert_eq!(v1.clone(), v1);
                    assert_eq!(v1.name(),  format!("{:?}", v1));

                    for v2 in $name::into_iter() {
                        assert_eq!(
                            v1.partial_cmp(&v2).unwrap(),
                            v1.cmp(&v2)
                        );
                    }
                }
            }

            #[test]
            fn test_impl_from() {
                for (repr, variant) in $name::into_iter().enumerate() {
                    assert_eq!(repr as u8, u8   ::from(variant));
                    assert_eq!(repr,       usize::from(variant));
                }
            }

            #[test]
            fn test_from_u8() {
                for (repr, variant) in $name::into_iter().enumerate() {
                    assert_eq!(variant, unsafe_optimization!{
                        $name::from_u8(repr as _).unwrap(),
                        $name::from_u8_unchecked(repr as _)
                    })
                }
            }
        }
    }
}

#[cfg(test)]
macro_rules! refute {
    ($cond:expr $(,)?)        => { assert!(!$cond) };
    ($cond:expr, $($arg:tt)+) => { assert!(!$cond, $($arg)+) };
}

mod color;
mod direction;
mod file;
mod piece;
mod rank;
mod square;

pub mod prelude {
    pub use crate::color::Color;
    pub use crate::direction::Direction;
    pub use crate::file::File;
    pub use crate::piece::Piece;
    pub use crate::rank::Rank;
    pub use crate::square::Square;
}

pub use prelude::*;
