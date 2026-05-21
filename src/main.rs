#![feature(duration_millis_float)]

use std::time::{Duration, Instant};

use latkerlo_jvotci::prewords::syllabify;

fn avg(i: &str, r: u32) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..r {
        let start = Instant::now();
        let _ = syllabify(i);
        total += start.elapsed();
    }
    total / r
}
const TARGET: Duration = Duration::from_secs(1);
fn find(p: &str, n: &str) {
    println!("\x1b[1m{n}:\x1b[m");
    let mut low = 0_usize;
    let mut high = 1_usize;
    loop {
        let input = p.repeat(high);
        let avg = avg(&input, 3);
        println!("[exp] n {high:7}, len {:7}, avg {:8.3?}ms", input.len(), avg.as_millis_f32());
        if avg > TARGET {
            break;
        }
        low = high;
        high *= 2;
    }
    while low + 1 < high {
        let mid = low.midpoint(high);
        let input = p.repeat(mid);
        let avg = avg(&input, 3);
        println!("[bin] n {mid:7}, len {:7}, avg {:8.3?}ms", input.len(), avg.as_millis_f32());
        if avg > TARGET {
            high = mid;
        } else {
            low = mid;
        }
    }
}

fn main() {
    /*
                 regex   manual
    ---------------------------
    easy       |  2.49     6.02
    hard       |  1.95     3.17
    impossible |  2.96     4.70
     */
    find("ua", "easy mode");
    find("xazdmru", "hard mode");
    find("xazblblblblblblblblblblna", "impossible mode?");
}
