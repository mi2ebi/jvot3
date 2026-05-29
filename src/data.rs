//! Various fast lookups for consonant clusters and vowels.

use phf::phf_set;

/// The consonant clusters permitted by CLL.
pub static VALID: phf::Set<&'static str> = phf_set![
    "bd", "bg", "bj", "bl", "bm", "bn", "br", "bv", "bz", "cf", "ck", "cl", "cm", "cn", "cp", "cr",
    "ct", "db", "dg", "dj", "dl", "dm", "dn", "dr", "dv", "dz", "fc", "fk", "fl", "fm", "fn", "fp",
    "fr", "fs", "ft", "fx", "gb", "gd", "gj", "gl", "gm", "gn", "gr", "gv", "gz", "jb", "jd", "jg",
    "jl", "jm", "jn", "jr", "jv", "kc", "kf", "kl", "km", "kn", "kp", "kr", "ks", "kt", "lb", "lc",
    "ld", "lf", "lg", "lj", "lk", "lm", "ln", "lp", "lr", "ls", "lt", "lv", "lx", "lz", "mb", "mc",
    "md", "mf", "mg", "mj", "mk", "ml", "mn", "mp", "mr", "ms", "mt", "mv", "mx", "nb", "nc", "nd",
    "nf", "ng", "nj", "nk", "nl", "nm", "np", "nr", "ns", "nt", "nv", "nx", "nz", "pc", "pf", "pk",
    "pl", "pm", "pn", "pr", "ps", "pt", "px", "rb", "rc", "rd", "rf", "rg", "rj", "rk", "rl", "rm",
    "rn", "rp", "rs", "rt", "rv", "rx", "rz", "sf", "sk", "sl", "sm", "sn", "sp", "sr", "st", "sx",
    "tc", "tf", "tk", "tl", "tm", "tn", "tp", "tr", "ts", "tx", "vb", "vd", "vg", "vj", "vl", "vm",
    "vn", "vr", "vz", "xf", "xl", "xm", "xn", "xp", "xr", "xs", "xt", "zb", "zd", "zg", "zl", "zm",
    "zn", "zr", "zv"
];

/// Returns `true` if `s` is in [`VALID`].
#[inline]
pub fn is_valid(s: &str) -> bool { VALID.contains(s) }

#[inline]
/// Returns `true` if `s` is in [`VALID`], or is *mz*.
pub fn is_mz_valid(s: &str) -> bool { s == "mz" || is_valid(s) }

/// The consonant clusters permitted word-initially by CLL.
pub static INITIAL: phf::Set<&'static str> = phf_set![
    "bl", "br", "cf", "ck", "cl", "cm", "cn", "cp", "cr", "ct", "dj", "dr", "dz", "fl", "fr", "gl",
    "gr", "jb", "jd", "jg", "jm", "jv", "kl", "kr", "ml", "mr", "pl", "pr", "sf", "sk", "sl", "sm",
    "sn", "sp", "sr", "st", "tc", "tr", "ts", "vl", "vr", "xl", "xr", "zb", "zd", "zg", "zm", "zv"
];

/// Returns `true` if `s` is in [`INITIAL`].
#[inline]
pub fn is_initial(s: &str) -> bool { INITIAL.contains(s) }

#[inline]
#[rustfmt::skip]
pub fn is_zihevla_initial(s: &str) -> bool {
    matches!(
        s,
        "bl" | "br" | "dr" | "fl" | "fr" | "gl" | "gr" | "kl" | "kr" | "ml" | "mr" | "pl" | "pr"
        | "tr" | "vl" | "vr"
    )
}

/// Returns `true` if `s` is one of the consonant triples banned by CLL: *ndj
/// ndz ntc nts*.
#[inline]
pub fn is_banned_triple(s: &str) -> bool { matches!(s, "ndj" | "ndz" | "ntc" | "nts") }

/// Returns `true` if `c` is a hard consonant (any consonant except *'* and
/// *.*).
#[inline]
#[rustfmt::skip]
pub const fn is_hard_consonant(c: char) -> bool {
    matches!(
        c,
        'b' | 'c' | 'd' | 'f' | 'g' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'r' | 's' | 't' | 'v'
        | 'x' | 'z'
    )
}

/// Returns `true` if `c` is an annotated onglide: *q* or *w*.
#[inline]
pub const fn is_onglide(c: char) -> bool { matches!(c, 'q' | 'w') }

/// Returns `true` if `c` is an annotated offglide: *ĭ* or *ŭ*.
#[inline]
pub const fn is_offglide(c: char) -> bool { matches!(c, 'ĭ' | 'ŭ') }

/// Returns `true` if `c` is a sonorant: one of *l m n r*.
#[inline]
pub const fn is_sonorant(c: char) -> bool { matches!(c, 'l' | 'm' | 'n' | 'r') }

/// Returns `true` if `c` is a vowel: *a e i o u y*.
#[inline]
pub const fn is_vowel(c: char) -> bool { matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y') }

/// Returns `true` if `c` is a diphthong: *ai ei oi au*.
#[inline]
pub fn is_diphthong(s: &str) -> bool { matches!(s, "ai" | "ei" | "oi" | "au") }

/// Returns `true` if `s` is a single vowel (other than *y*) or a diphthong. As
/// standalone syllables, these always require a glottal stop before them.
#[inline]
pub fn is_start_vowel_cluster(s: &str) -> bool {
    match s.as_bytes() {
        [b] => *b != b'y' && is_vowel(*b as char),
        _ => is_diphthong(s),
    }
}

/// Returns `true` if `s` is a syllable nucleus starting with a glide.
#[inline]
#[rustfmt::skip]
pub fn is_follow_vowel_cluster(_s: &str) -> bool {
    todo!("should these have q/w instead of i/u?");
    /*
    matches!(
        s,
        "ia" | "ie" | "ii" | "io" | "iu" | "iau" | "iai" | "iei" | "ioi" | "ua" | "ue" | "ui"
        | "uo" | "uu" | "uau" | "uai" | "uei" | "uoi"
    )
    */
}

/// Returns `true` if `s` is a lujvo hyphen, used to prevent cmavo-shaped rafsi
/// from falling off the start of a lujvo and to delimit zi'evla inside a lujvo.
#[inline]
pub fn is_hyphen(s: &str) -> bool { matches!(s, "r" | "n" | "y" | "'y" | "y'" | "'y'") }
