use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use latkerlo_jvotci::prewords::syllabify;

fn bench_syllabify(c: &mut Criterion) {
    let mut group = c.benchmark_group("syllabify");
    let len = 1000;
    let easy_input = "ua".repeat(len / 2);
    let hard_input = "xazdmru".repeat(len / 7);
    let less_hard_input = "xazblblblblblblblblblblna".repeat(len / 25);
    let catgirl_input = "uu".repeat(len / 2);
    group.bench_function("easy_mode", |b| b.iter(|| syllabify(black_box(&easy_input))));
    group.bench_function("hard_mode", |b| b.iter(|| syllabify(black_box(&hard_input))));
    group.bench_function("less_hard_mode", |b| b.iter(|| syllabify(black_box(&less_hard_input))));
    group.bench_function("catgirl_mode", |b| b.iter(|| syllabify(black_box(&catgirl_input))));
    group.finish();
}

criterion_group!(benches, bench_syllabify);
criterion_main!(benches);
