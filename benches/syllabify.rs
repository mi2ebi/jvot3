use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use latkerlo_jvotci::prewords::syllabify;

fn bench_syllabify(c: &mut Criterion) {
    let mut group = c.benchmark_group("syllabify");
    let len = 1000;
    let zgifnzeha = "zgikemfi'inalka'esefsysajyke'ejvekemsefsyda'atoiflike'ejvejagborkemjilryjvesefsyborxenze'a";
    group.bench_function("easy_1", |b| b.iter(|| syllabify(black_box("ua"))));
    group.bench_function("hard_1", |b| b.iter(|| syllabify(black_box("xazdmru"))));
    group.bench_function("less_hard_1", |b| {
        b.iter(|| syllabify(black_box("xazblblblblblblblblblblna")))
    });
    group.bench_function("catgirl_1", |b| b.iter(|| syllabify(black_box("uu"))));
    group.bench_function("zgifnzeha_1", |b| b.iter(|| syllabify(black_box(zgifnzeha))));
    group.bench_function("easy_1000ch", |b| b.iter(|| syllabify(black_box(&"ua".repeat(len / 2)))));
    group.bench_function("hard_1000ch", |b| {
        b.iter(|| syllabify(black_box(&"xazdmru".repeat(len / 7))))
    });
    group.bench_function("less_hard_1000ch", |b| {
        b.iter(|| syllabify(black_box(&"xazblblblblblblblblblblna".repeat(len / 25))))
    });
    group.bench_function("catgirl_1000ch", |b| {
        b.iter(|| syllabify(black_box(&"uu".repeat(len / 2))))
    });
    group.bench_function("zgifnzeha_1000ch", |b| {
        b.iter(|| syllabify(black_box(&zgifnzeha.repeat(len / 90))))
    });
    group.finish();
}

criterion_group!(benches, bench_syllabify);
criterion_main!(benches);
