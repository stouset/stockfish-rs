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
#![warn(clippy::restriction)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

// Additional Clippy Lints
#![warn(clippy::missing_const_for_fn)]

// Lint Exceptions
#![allow(unstable_features)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::as_conversions)]
#![allow(clippy::as_underscore)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::pub_use)]
#![allow(clippy::self_named_module_files)]
#![allow(clippy::separated_literal_suffix)]
#![allow(clippy::shadow_reuse)]
#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::wildcard_enum_match_arm)]
#![cfg_attr(test,             allow(clippy::missing_const_for_fn))]
#![cfg_attr(debug_assertions, allow(clippy::missing_panics_doc))]
#![cfg_attr(debug_assertions, allow(clippy::unwrap_used))]

// TODO: remove
#![allow(meta_variable_misuse)] // false positive with `count()`
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::missing_docs_in_private_items)]

// Unstable Features
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_default_impls)]
#![feature(const_discriminant)]
#![feature(const_mut_refs)]
#![feature(const_ops)]
#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(const_slice_index)]
#![feature(derive_const)]
#![feature(is_sorted)]
#![feature(macro_metavar_expr)]
#![feature(no_coverage)]
#![feature(rustdoc_missing_doc_code_examples)]
#![feature(strict_provenance)]

#[cfg(test)]
use criterion as _;

macro_rules! unsafe_optimization {
    ($safe:expr, $unsafe:expr $(,)?) => {{
        #[allow(clippy::undocumented_unsafe_blocks)]
        #[allow(unsafe_code)]
        unsafe {
            ::core::debug_assert!($safe == $unsafe);
            $unsafe
        }
    }};
}

#[cfg(test)]
macro_rules! refute {
    ($cond:expr $(,)?)        => { assert!(!$cond) };
    // ($cond:expr, $($arg:tt)+) => { assert!(!$cond, $($arg)+) };
}

mod misc;

#[doc(hidden)]
pub mod arch;

#[doc(hidden)]
pub mod accelerate;

pub mod bitboard;
pub mod core;
pub mod hash;

/// The `stockfish_core` prelude. Re-exports most types useful for implementing
/// the fundamental workings of a chess engine.
pub mod prelude {
    pub use crate::{board, board_pieces};

    #[doc(no_inline)]
    pub use crate::core::Board;

    #[doc(no_inline)]
    pub use crate::core::CastlingPath;

    #[doc(no_inline)]
    pub use crate::core::CastlingRights;

    #[doc(no_inline)]
    pub use crate::core::CastlingSide;

    #[doc(no_inline)]
    pub use crate::core::CastlingVariety;

    #[doc(no_inline)]
    pub use crate::core::Color;

    #[doc(no_inline)]
    pub use crate::core::Direction;

    #[doc(no_inline)]
    pub use crate::core::File;

    #[doc(no_inline)]
    pub use crate::core::Move;

    #[doc(no_inline)]
    pub use crate::core::MoveType;

    #[doc(no_inline)]
    pub use crate::core::Piece;

    #[doc(no_inline)]
    pub use crate::core::Rank;

    #[doc(no_inline)]
    pub use crate::core::Ruleset;

    #[doc(no_inline)]
    pub use crate::core::Square;

    #[doc(no_inline)]
    pub use crate::core::Token;

    #[doc(no_inline)]
    pub use crate::bitboard::Bitboard;
}

pub use prelude::*;
