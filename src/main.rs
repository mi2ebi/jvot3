use std::time::{Duration, Instant};

use latkerlo_jvotci::prewords::syllabify;

fn avg(i: &str) -> Duration {
    let mut total = Duration::ZERO;
    let r = 3;
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
        let avg = avg(&input);
        println!(
            "[exp] n {high:7}, len {:8}, avg {:8.3}ms",
            input.len(),
            avg.as_secs_f32() * 1000.
        );
        if avg > TARGET {
            break;
        }
        low = high;
        high *= 2;
    }
    while low + 1 < high {
        let mid = low.midpoint(high);
        let input = p.repeat(mid);
        let avg = avg(&input);
        println!("[bin] n {mid:7}, len {:8}, avg {:8.3}ms", input.len(), avg.as_secs_f32() * 1000.);
        if avg > TARGET {
            high = mid;
        } else {
            low = mid;
        }
    }
}

fn main() {
    /*
    easy       7.22 MB/s
    hard       4.01 MB/s
    less hard  6.67 MB/s
    catgirl    7.54 MB/s
     */
    find("ua", "easy mode");
    find("xazdmru", "hard mode");
    find("xazblblblblblblblblblblna", "less hard mode");
    find("uu", "catgirl mode");
}
