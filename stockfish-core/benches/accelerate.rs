use stockfish_core::prelude::*;
use stockfish_core::accelerate::{cached, computed};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_square_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("accelerate::square_distance");
    let     s1    = Square::H7;
    let     s2    = Square::B3;

    group.bench_function("computed",         |b| b.iter(|| computed::square_distance(black_box(s1), black_box(s2)) ));
    group.bench_function("cached",           |b| b.iter(|| cached  ::square_distance(black_box(s1), black_box(s2)) ));
    group.bench_function("Square::distance", |b| b.iter(|| s1.distance(black_box(s2)) ));

    group.finish();
}

fn bench_attacks(c: &mut Criterion) {
    let mut group = c.benchmark_group("accelerate::attacks");

    let square = Square::E4;

    let occupancy =
        Square::A7 | Square::B2 | Square::B3 | Square::C6 |
        Square::C4 | Square::D3 | Square::G6 | Square::H2 ;

    for color in Color::iter() {
        for piece in Piece::iter() {
            group.bench_with_input(
                BenchmarkId::new("computed", format!("{color:?} {piece:?}")),
                &(color, piece, square, occupancy),
                |b, i| b.iter(|| computed::attacks(i.0, i.1, i.2, i.3))
            );

            group.bench_with_input(
                BenchmarkId::new("cached", format!("{color:?} {piece:?}")),
                &(color, piece, square, occupancy),
                |b, i| b.iter(|| cached::attacks(i.0, i.1, i.2, i.3))
            );
        }
    }
}

criterion_group!(
    benches,
    bench_square_distance,
    bench_attacks,
);

criterion_main!(benches);
