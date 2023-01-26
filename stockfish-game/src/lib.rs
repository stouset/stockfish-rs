//! # `stockfish_game`

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
#![allow(clippy::missing_inline_in_public_items)]
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
#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::missing_docs_in_private_items)]

// Unstable Features
#![feature(const_discriminant)]
#![feature(derive_const)]
#![feature(let_chains)]
#![feature(rustdoc_missing_doc_code_examples)]
#![feature(strict_provenance)]

mod position;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::position::Position;
}

pub use prelude::*;
