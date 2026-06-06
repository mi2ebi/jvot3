#[macro_use]
mod common;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use jvot3::{prewords::is_some_cmavo, settings::Settings};

const G: &str = "is_some_cmavo";

fn two_long_y_cmavo(k: usize) -> String { format!("ko{}'y", "'a".repeat(k)).repeat(2) }

fn bench_is_some_cmavo(c: &mut Criterion) {
    let cll: Settings = "".parse().unwrap();
    let r: Settings = "r".parse().unwrap();
    let len = 1000;

    let mut group = c.benchmark_group(G);

    // single cmavo
    bench!(group, G / "vowel_1", |b| b.iter(|| is_some_cmavo(black_box("ie"), &cll)));
    bench!(group, G / "cv_1", |b| b.iter(|| is_some_cmavo(black_box("ko"), &cll)));
    bench!(group, G / "apo_1", |b| b.iter(|| is_some_cmavo(black_box("pa'e"), &cll)));

    // y boundaries
    bench!(group, G / "bcdf_cll", |b| b.iter(|| is_some_cmavo(black_box("bycydyfy"), &cll)));
    bench!(group, G / "bcdf_r", |b| b.iter(|| is_some_cmavo(black_box("bycydyfy"), &r)));
    bench!(group, G / "hy_cll", |b| b.iter(|| is_some_cmavo(black_box("te'yna'y"), &cll)));
    bench!(group, G / "hy_r", |b| b.iter(|| is_some_cmavo(black_box("te'yna'y"), &r)));
    bench!(group, G / "rmode_1", |b| b.iter(|| is_some_cmavo(black_box("byna'y"), &r)));

    // failing cases
    bench!(group, G / "fail_1st", |b| b.iter(|| is_some_cmavo(black_box("anba"), &cll)));
    bench!(group, G / "fail_y_x", |b| { b.iter(|| is_some_cmavo(black_box("bavyteike'u"), &cll)) });
    bench!(group, G / "fail_y_r", |b| { b.iter(|| is_some_cmavo(black_box("bavyteike'u"), &r)) });

    // long chains
    bench!(group, G / "cv_1kc", |b| {
        b.iter(|| is_some_cmavo(black_box(&"ko".repeat(len / 2)), &cll));
    });
    bench!(group, G / "ych_1kc_x", |b| {
        b.iter(|| is_some_cmavo(black_box(&"bycydyfy".repeat(len / 8)), &cll));
    });
    bench!(group, G / "ych_1kc_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&"bycydyfy".repeat(len / 8)), &r));
    });
    bench!(group, G / "apo_1kc", |b| {
        b.iter(|| is_some_cmavo(black_box(&"pa'e".repeat(len / 4)), &cll));
    });
    bench!(group, G / "hy_1kc_x", |b| {
        b.iter(|| is_some_cmavo(black_box(&"te'yna'y".repeat(len / 8)), &cll));
    });
    bench!(group, G / "hy_1kc_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&"te'yna'y".repeat(len / 8)), &r));
    });

    // really long cmavo in r mode
    bench!(group, G / "2y_10s_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&two_long_y_cmavo(8)), &r));
    });
    bench!(group, G / "2y_100s_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&two_long_y_cmavo(98)), &r));
    });
    bench!(group, G / "2y_1ks_r", |b| {
        b.iter(|| is_some_cmavo(black_box(&two_long_y_cmavo(998)), &r));
    });
    bench!(group, G / "2y_1ks_x", |b| {
        b.iter(|| is_some_cmavo(black_box(&two_long_y_cmavo(998)), &cll));
    });

    group.finish();
}

criterion_group!(benches, bench_is_some_cmavo);
criterion_main!(benches);
