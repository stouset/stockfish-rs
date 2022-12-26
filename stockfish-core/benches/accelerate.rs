use stockfish_core::prelude::*;
use stockfish_core::accelerate::{cached, computed};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_square_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("accelerate::square_distance");
    let     s1    = Square::H7;
    let     s2    = Square::B3;

    group.bench_function("computed",         |b| b.iter(|| computed::square_distance(black_box(s1), black_box(s2)) ));
    group.bench_function("cached",           |b| b.iter(|| cached  ::square_distance(black_box(s1), black_box(s2)) ));
    group.bench_function("Square::distance", |b| b.iter(|| s1.distance(black_box(s2)) ));

    group.finish();
}

criterion_group!(
    benches,
    bench_square_distance,
);

criterion_main!(benches);
