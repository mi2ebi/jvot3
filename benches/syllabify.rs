#[macro_use]
mod common;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use jvot3::prewords::syllabify;

fn bench_syllabify(c: &mut Criterion) {
    let len = 1000;
    let zgifnzeha = "zgikemfi'inalka'esefsysajyke'ejvekemsefsyda'atoiflike'\
                     ejvejagborkemjilryjvesefsyborxenze'a";

    const G: &str = "syllabify";
    let mut group = c.benchmark_group(G);

    // single words
    bench!(group, G / "easy_1", |b| b.iter(|| syllabify(black_box("ua"))));
    bench!(group, G / "hard_1", |b| b.iter(|| syllabify(black_box("xazdmru"))));
    bench!(group, G / "22cons_1", |b| {
        b.iter(|| syllabify(black_box("xazblblblblblblblblblblna")))
    });
    bench!(group, G / "catgirl_1", |b| b.iter(|| syllabify(black_box("uu"))));
    bench!(group, G / "zgifnzeha_1", |b| b.iter(|| syllabify(black_box(zgifnzeha))));

    // long
    bench!(group, G / "easy_1kc", |b| b.iter(|| syllabify(black_box(&"ua".repeat(len / 2)))));
    bench!(group, G / "hard_1kc", |b| {
        b.iter(|| syllabify(black_box(&"xazdmru".repeat(len / 7))))
    });
    bench!(group, G / "22cons_1kc", |b| {
        b.iter(|| syllabify(black_box(&"xazblblblblblblblblblblna".repeat(len / 25))))
    });
    bench!(group, G / "catgirl_1kc", |b| {
        b.iter(|| syllabify(black_box(&"uu".repeat(len / 2))))
    });
    bench!(group, G / "zgifnzeha_1kc", |b| {
        b.iter(|| syllabify(black_box(&zgifnzeha.repeat(len / 90))))
    });

    group.finish();
}

criterion_group!(benches, bench_syllabify);
criterion_main!(benches);
