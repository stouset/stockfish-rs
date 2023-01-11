#![allow(clippy::module_name_repetitions)]

/// Implements logic for castling-related behavior.
///
/// As it turns out, castling is annoyingly finicky. Breaking it apart into
/// smaller bits of functionality helps us use the type system to tease apart
/// some of its finer details.

mod castling_path;
mod castling_rights;
mod castling_side;
mod castling_variety;

pub use castling_path::CastlingPath;
pub use castling_rights::CastlingRights;
pub use castling_side::CastlingSide;
pub use castling_variety::CastlingVariety;
