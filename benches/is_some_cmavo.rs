use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use jvot3::prewords::is_some_cmavo;
use jvot3::settings::Settings;

fn bench_is_some_cmavo(c: &mut Criterion) {
    let cll: Settings = "".parse().unwrap();
    let r: Settings = "r".parse().unwrap();
    let len = 1000;

    let mut group = c.benchmark_group("is_some_cmavo");

    // single cmavo
    group.bench_function("vowel_1", |b| b.iter(|| is_some_cmavo(black_box("ie"), &cll)));
    group.bench_function("cv_1", |b| b.iter(|| is_some_cmavo(black_box("ko"), &cll)));
    group.bench_function("apo_1", |b| b.iter(|| is_some_cmavo(black_box("pa'e"), &cll)));

    // y-boundary logic (the interesting part)
    group.bench_function("ychain_4_cll", |b| b.iter(|| is_some_cmavo(black_box("bycydyfy"), &cll)));
    group.bench_function("ychain_4_r", |b| b.iter(|| is_some_cmavo(black_box("bycydyfy"), &r)));
    group.bench_function("yapo_cll", |b| b.iter(|| is_some_cmavo(black_box("te'yna'y"), &cll)));
    group.bench_function("yapo_r", |b| b.iter(|| is_some_cmavo(black_box("te'yna'y"), &r)));
    group.bench_function("rmode_1", |b| b.iter(|| is_some_cmavo(black_box("byna'y"), &r)));

    // failing cases
    group.bench_function("fail_syllabify", |b| b.iter(|| is_some_cmavo(black_box("anba"), &cll)));
    group.bench_function("fail_yboundary_cll", |b| {
        b.iter(|| is_some_cmavo(black_box("bavyteike'u"), &cll))
    });
    group.bench_function("fail_yboundary_r", |b| {
        b.iter(|| is_some_cmavo(black_box("bavyteike'u"), &r))
    });

    // long chains
    group.bench_function("cv_1000ch", |b| {
        b.iter(|| is_some_cmavo(black_box(&"ko".repeat(len / 2)), &cll))
    });
    group.bench_function("ychain_1000ch_cll", |b| {
        b.iter(|| is_some_cmavo(black_box(&"bycydyfy".repeat(len / 8)), &cll))
    });
    group.bench_function("ychain_1000ch_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&"bycydyfy".repeat(len / 8)), &r))
    });
    group.bench_function("apo_1000ch", |b| {
        b.iter(|| is_some_cmavo(black_box(&"pa'e".repeat(len / 4)), &cll))
    });
    group.bench_function("yapo_1000ch_cll", |b| {
        b.iter(|| is_some_cmavo(black_box(&"te'yna'y".repeat(len / 8)), &cll))
    });
    group.bench_function("yapo_1000ch_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&"te'yna'y".repeat(len / 8)), &r))
    });

    group.finish();
}

criterion_group!(benches, bench_is_some_cmavo);
criterion_main!(benches);
