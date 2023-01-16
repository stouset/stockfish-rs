# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project
adheres to [Semantic Versioning][semver].

## [Unreleased][https://github.com/stouset/stockfish-rs/tree/main]

### Added

- `File` and `Rank` now implement `BitOr` against themselves
- `Bitboard` can now `BitAnd`, `BitOr`, and `BitXor` against `File` and `Rank`

### Changed

- `File` and `Rank` now *both* allow iteration over the squares they contain,
  but the type of their `IntoIterator` is changed to
  `std::array::IntoIter<Square, 8>`

### Fixed

- Stricter hygeine for `unsafe_optimization` and `enumeration` macros

### Removed

- `Square` no longer implements `std::iter::Step`
- Core types other than `File` and `Rank` no longer implement `PartialOrd` and
  `Ord`

## [0.1.1][https://github.com/stouset/stockfish-rs/tree/stockfish-core-v0.1.1] - 2023-01-13

### Fixed

- Various fixes to the project README

## [0.1.0][https://github.com/stouset/stockfish-rs/tree/stockfish-core-v0.1.0] - 2023-01-13

### Added

- Project exists!

[keepachangelog]: https://keepachangelog.com/en/1.0.0/
[semver]:         https://semver.org/spec/v2.0.0.html
