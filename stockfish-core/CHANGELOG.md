# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project
adheres to [Semantic Versioning][semver].

## [Unreleased](https://github.com/stouset/stockfish-rs/tree/main/stockfish-core)

### Fixed

- Fixed errors in CHANGELOG formatting
- Added some missing `#[inline]` attributes to functions

### Changed

- `Bitboard::into_some_square` is now `Into::<Option<Square>>::into`
- `Piece::attacks` no longer requires the square to be outside the occupancy
  bitboard.
- `CastlingPath::path` and `CastlingPath::variety` are now functions rather than
  direct field accesses

### Added

- `impl Default for Bitboard`
- `impl IntoIterator<Output = Square> for Bitboard`
- `impl BitOr<Color, Output = Piece> for Token`
- `impl BitOr<Token, Output = Piece> for Color`
- `Token::attacks()` and `Token::moves()`

### Removed

- `impl Shl<u8> for Bitboard`

## [0.2.0](https://github.com/stouset/stockfish-rs/tree/stockfish-core-v0.2.0/stockfish-core) - 2023-01-16

### Changed

- `Token` and `Piece` have had their meanings swapped to better reflect the
  naming conventions in the wider chess programming community as well as the
  upstream Stockfish project.

  A `Token` is now the abstract concept of a type of piece such as a king or a
  knight. A `Piece` is now a physical, colored piece that may be placed on a
  chess board such as a white pawn or a black queen.
- `File` and `Rank` now *both* allow iteration over the squares they contain,
  but the type of their `IntoIterator` is changed to
  `std::array::IntoIter<Square, 8>`

### Removed

- `Square` no longer implements `std::iter::Step`
- Core types other than `File` and `Rank` no longer implement `PartialOrd` and
  `Ord`

### Added

- `File` and `Rank` now implement `BitOr` against themselves
- `Bitboard` can now `BitAnd`, `BitOr`, and `BitXor` against `File` and `Rank`
- `Move` core type added, which encodes details about a move between two
  squares.
- `CastlingRights::iter` now allows iterating over all possible variants.
- An implementation of generating `Key`s through `Zobrist` hashing is now
  available.

### Fixed

- Stricter hygeine for `unsafe_optimization` and `enumeration` macros

## [0.1.1](https://github.com/stouset/stockfish-rs/tree/stockfish-core-v0.1.1/stockfish-core) - 2023-01-13

### Fixed

- Various fixes to the project README

## [0.1.0](https://github.com/stouset/stockfish-rs/tree/stockfish-core-v0.1.0/stockfish-core) - 2023-01-13

### Added

- Project exists!

[keepachangelog]: https://keepachangelog.com/en/1.0.0/
[semver]:         https://semver.org/spec/v2.0.0.html
