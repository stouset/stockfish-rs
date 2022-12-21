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
// #![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// #![warn(missing_docs)]
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
// #![warn(clippy::missing_docs_in_private_items)]
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
#![allow(rustdoc::missing_doc_code_examples)]

// Unstable Features
#![feature(const_convert)]
#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(const_slice_index)]
#![feature(macro_metavar_expr)]
#![feature(rustdoc_missing_doc_code_examples)]
#![feature(strict_provenance)]

macro_rules! enumeration {
    ($(#[$outer:meta])* $vis:vis $name:ident, $repr:ty, [$($var:ident),+ $(,)?]) => (
        $(#[$outer])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[repr($repr)]
        #[must_use]
        $vis enum $name {
            $($var = ${index()}),+
        }

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

            /// Converts the provided [`$repr`] discriminant to its
            /// corresponding [`$name`].
            ///
            /// # Safety
            ///
            /// This function is unsafe. You *must* guarantee that the input
            /// value is a real discriminant of this type.
            #[inline]
            #[allow(unsafe_code)]
            $vis const unsafe fn from_repr_unchecked(repr: $repr) -> Self {
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
            $vis const fn from_repr(repr: $repr) -> Option<Self> {
                Self::VARIANTS.get(repr as usize).copied()
            }

            /// Returns an iterator through all [`$type`] variants.
            #[inline]
            #[must_use]
            pub fn into_iter() -> std::array::IntoIter<Self, ${count(var)}> {
                Self::VARIANTS.into_iter()
            }

            /// Returns the name of the variant as a string.
            #[inline]
            #[must_use]
            pub fn name(self) -> &'static str {
                Self::NAMES[self.as_usize()]
            }

            /// Returns the variant as its underlying discriminant.
            #[inline]
            #[must_use]
            pub const fn as_repr(self) -> $repr {
                self as $repr
            }

            #[inline]
            #[must_use]
            pub const fn as_usize(self) -> usize {
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
    )
}

mod color;
mod file;
mod rank;
mod square;

pub use color::Color;
pub use file::File;
pub use rank::Rank;
pub use square::Square;

pub mod prelude {
    pub use crate::Color;
    pub use crate::File;
    pub use crate::Rank;
    pub use crate::Square;
}
