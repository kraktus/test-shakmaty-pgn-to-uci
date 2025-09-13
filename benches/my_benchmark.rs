use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use test_shakmaty_pgn_to_uci::{EXAMPLE_PGN, str_to_uci, str_to_uci_visitor};

fn bench_method1(c: &mut Criterion) {
    c.bench_function("adhoc parser", |b| {
        b.iter(|| str_to_uci(black_box(EXAMPLE_PGN)))
    });
    c.bench_function("pgn_reader parser", |b| {
        b.iter(|| str_to_uci_visitor(black_box(EXAMPLE_PGN)))
    });
}

criterion_group!(benches, bench_method1);
criterion_main!(benches);
