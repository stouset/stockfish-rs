# stockfish-core

Highly-optimized the core building blocks for implementing a chess engine.

![Crate][badge-crate] ![Docs][badge-docs] ![License][badge-license] ![CI][badge-ci]

# Overview

This is a component of `stockfish_rs`, which is an attempt to port the
[Stockfish][stockfish] chess engine to Rust.

Unlike other similar efforts, the goal of this project is to express as much of
the logic in idiomatic Rust as possible. To this end, significant creative
liberties have been taken to rewrite components as they're brought over to
leverage as much of the Rust type system as possible.

This project also aims to break apart conceptual components of the engine to
enable others to build their own engines and chess programs without duplicating
effort.

# Supported Rust Versions

This crate requires many nightly-only features and so will only build only on
nightly Rust. As features or stabilized or alternative non-unstable
implementations are settled upon, we anticipate this crate to build on a future
stable version of Rust.

# License

This project is licensed under the [GNU GPL, Version 3][licence-gpl3].

The choice of license is required due to this project's origins as a port of
[Stockfish][stockfish].

[badge-ci]:      https://img.shields.io/codecov/c/github/stouset/stockfish-rs
[badge-crate]:   https://img.shields.io/crates/v/stockfish_core
[badge-docs]:    https://img.shields.io/docsrs/stockfish_core
[badge-license]: https://img.shields.io/crates/l/stockfish_core

[license-gpl3]: https://github.com/stouset/stockfish-rs/blob/main/LICENSE

[stockfish]: http://github.com/official-stockfish/Stockfish/
