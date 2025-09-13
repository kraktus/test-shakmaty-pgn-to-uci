use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use test_shakmaty_pgn_to_uci::{str_to_uci, EXAMPLE_PGN};


fn bench_method1(c: &mut Criterion) {
    c.bench_function("go", |b| b.iter(|| str_to_uci(black_box(EXAMPLE_PGN))));
}


criterion_group!(benches, bench_method1);
criterion_main!(benches);
