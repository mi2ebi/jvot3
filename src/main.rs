use latkerlo_jvotci::prewords::syllabify;

fn the(s: &str) {
    println!("{s} -> {:?}", syllabify(s));
}

fn main() {
    the("mtsxetas");
    the("beiao");
    the("a4a");
}
