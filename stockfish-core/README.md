# stockfish-core

Highly-optimized core building blocks for implementing a chess engine.

[![Crate][badge-crate]][project-crate] [![Build][badge-ci]][project-ci] [![Docs][badge-docs]][project-docs] [![License][badge-license]][project-license] [![Coverage][badge-coverage]][project-coverage]

# Overview

This is a component of [stockfish-rs][stockfish-rs], which is an attempt to port
the [Stockfish][stockfish] chess engine to Rust.

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

This project is licensed under the [GNU GPL, Version 3][project-license].

The choice of license is required due to this project's origins as a port of
[Stockfish][stockfish].

[project-ci]:       https://github.com/stouset/stockfish-rs/actions/workflows/rust.yml
[project-coverage]: https://app.codecov.io/gh/stouset/stockfish-rs/tree/main/stockfish-core
[project-crate]:    https://crates.io/crates/stockfish-core
[project-docs]:     https://docs.rs/stockfish-core
[project-license]:  https://github.com/stouset/stockfish-rs/blob/main/LICENSE

[badge-ci]:       https://img.shields.io/github/actions/workflow/status/stouset/stockfish-rs/rust.yml
[badge-coverage]: https://img.shields.io/codecov/c/github/stouset/stockfish-rs
[badge-crate]:    https://img.shields.io/crates/v/stockfish-core
[badge-docs]:     https://img.shields.io/docsrs/stockfish-core
[badge-license]:  https://img.shields.io/crates/l/stockfish-core

[stockfish]:    http://github.com/official-stockfish/Stockfish/
[stockfish-rs]: http://github.com/stouset/stockfish-rs
