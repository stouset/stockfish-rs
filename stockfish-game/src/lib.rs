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
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::string_slice)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::use_debug)]

// Lint Exceptions
#![allow(unstable_features)]
#![cfg_attr(test,             allow(clippy::missing_const_for_fn))]
#![cfg_attr(debug_assertions, allow(clippy::missing_panics_doc))]

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
