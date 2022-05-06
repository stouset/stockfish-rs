//! #stockfish-rs

// Lint Groups
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
//
// Individual Lints
#![warn(box_pointers)]
#![warn(deprecated_in_future)]
// #![warn(fuzzy_provenance_casts)]
// #![warn(lossy_provenance_casts)]
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
//
// Lint Extensions
#![warn(rustdoc::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
//
// Lint Exceptions
#![allow(unstable_features)]
#![allow(rustdoc::missing_doc_code_examples)]

#![feature(const_convert)]
#![feature(const_for)]
#![feature(const_intoiterator_identity)]
#![feature(const_mut_refs)]
#![feature(const_ops)]
#![feature(const_option_ext)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(generic_arg_infer)]
#![feature(mixed_integer_ops)]
#![feature(once_cell)]

pub mod bitboard;
pub mod command_line;
pub mod misc;
pub mod types;

use command_line::CommandLine;

use color_eyre::Report;

fn main() -> Result<(), Report> {
    let _command_line = CommandLine::init()?;

    Ok(())
}
