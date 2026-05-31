//! Various fast lookups for consonant clusters and vowels.

/// A bitmask of valid consonant clusters.
pub const VALID_TABLE: [u32; 25] = [
    0b1_0001_0001_0001_1101_0010_0100, // b: bd bg bj bl bm bn br bv bz
    0b0_0000_0101_0101_1110_0001_0000, // c: cf ck cl cm cn cp cr ct
    0b1_0001_0001_0001_1101_0010_0001, // d: db dg dj dl dm dn dr dv dz
    0b0,                               // e
    0b0_0100_0111_0101_1110_0000_0010, // f: fc fk fl fm fn fp fr fs ft fx
    0b1_0001_0001_0001_1101_0000_0101, // g: gb gd gj gl gm gn gr gv gz
    0b0,                               // h
    0b0,                               // i
    0b0_0001_0001_0001_1100_0010_0101, // j: jb jd jg jl jm jn jr jv
    0b0_0000_0111_0101_1100_0001_0010, // k: kc kf kl km kn kp kr ks kt
    0b1_0101_0111_0101_1011_0011_0111, // l: lb lc ld lf lg lj lk lm ln lp lr ls lt lv lx lz
    0b0_0101_0111_0101_0111_0011_0111, // m: mb mc md mf mg mj mk ml mn mp mr ms mt mv mx
    0b1_0101_0111_0100_1111_0011_0111, // n: nb nc nd nf ng nj nk nl nm np nr ns nt nv nx nz
    0b0,                               // o
    0b0_0100_0111_0001_1110_0001_0010, // p: pc pf pk pl pm pn pr ps pt px
    0b0,                               // q
    0b1_0101_0110_0101_1111_0011_0111, // r: rb rc rd rf rg rj rk rl rm rn rp rs rt rv rx rz
    0b0_0100_0101_0101_1110_0001_0000, // s: sf sk sl sm sn sp sr st sx
    0b0_0100_0011_0101_1110_0001_0010, // t: tc tf tk tl tm tn tp tr ts tx
    0b0,                               // u
    0b1_0000_0001_0001_1101_0010_0101, // v: vb vd vg vj vl vm vn vr vz
    0b0,                               // w
    0b0_0000_0111_0101_1100_0001_0000, // x: xf xl xm xn xp xr xs xt
    0b0,                               // y
    0b0_0001_0001_0001_1100_0010_0101, // z: zb zd zg zl zm zn zr zv
];

/// Returns `true` if `s` is in [`VALID_TABLE`].
pub fn is_valid(s: &str) -> bool {
    let [x, y] = s.as_bytes() else { return false };
    let xi = x.wrapping_sub(b'b') as usize;
    let yi = y.wrapping_sub(b'b') as usize;
    if xi >= 25 || yi >= 25 {
        return false;
    }
    (VALID_TABLE[xi] >> yi) & 1 != 0
}

/// Returns `true` if `s` is in [`VALID_TABLE`], or is *mz*.
#[inline]
pub fn is_mz_valid(s: &str) -> bool {
    is_valid(s) || s == "mz"
}

/// A bitmask of valid word-initial consonant clusters.
pub const INITIAL_TABLE: [u32; 25] = [
    0b0_0000_0001_0000_0100_0000_0000, // b: bl br
    0b0_0000_0101_0101_1110_0001_0000, // c: cf ck cl cm cn cp cr ct
    0b1_0000_0001_0000_0001_0000_0000, // d: dj dr dz
    0b0,                               // e
    0b0_0000_0001_0000_0100_0000_0000, // f: fl fr
    0b0_0000_0001_0000_0100_0000_0000, // g: gl gr
    0b0_0000_0000_0000_0000_0000_0000, // h
    0b0,                               // i
    0b0_0001_0000_0000_1000_0010_0101, // j: jb jd jg jm jv
    0b0_0000_0001_0000_0100_0000_0000, // k: kl kr
    0b0,                               // l
    0b0_0000_0001_0000_0100_0000_0000, // m: ml mr
    0b0,                               // n
    0b0,                               // o
    0b0_0000_0001_0000_0100_0000_0000, // p: pl pr
    0b0,                               // q
    0b0,                               // r
    0b0_0000_0101_0101_1110_0001_0000, // s: sf sk sl sm sn sp sr st
    0b0_0000_0011_0000_0000_0000_0010, // t: tc tr ts
    0b0,                               // u
    0b0_0000_0001_0000_0100_0000_0000, // v: vl vr
    0b0,                               // w
    0b0_0000_0001_0000_0100_0000_0000, // x: xl xr
    0b0,                               // y
    0b0_0001_0000_0000_1000_0010_0101, // z: zb zd zg zm zv
];

/// Returns `true` if `s` is in [`INITIAL_TABLE`].
#[inline]
pub fn is_initial(s: &str) -> bool {
    let [x, y] = s.as_bytes() else { return false };
    let xi = x.wrapping_sub(b'b') as usize;
    let yi = y.wrapping_sub(b'b') as usize;
    if xi >= 25 || yi >= 25 {
        return false;
    }
    (INITIAL_TABLE[xi] >> yi) & 1 != 0
}

#[inline]
pub const fn is_zihevla_initial(s: &str) -> bool {
    matches!(
        s.as_bytes(),
        [b'b' | b'f' | b'g' | b'k' | b'm' | b'p' | b'v', b'l']
            | [b'b' | b'd' | b'f' | b'g' | b'k' | b'm' | b'p' | b't' | b'v', b'r']
    )
}

/// Returns `true` if `s` is one of the consonant triples banned by CLL: *ndj
/// ndz ntc nts*.
#[inline]
pub fn is_banned_triple(s: &str) -> bool {
    matches!(s, "ndj" | "ndz" | "ntc" | "nts")
}

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

/// Returns `true` if `s` is a permissible syllable onset.
pub fn is_hard_onset(s: &str) -> bool {
    match s.len() {
        0 => true,
        1 => s.chars().next().is_some_and(is_hard_consonant),
        2 => is_initial(s),
        3 => is_initial(&s[..2]) && is_zihevla_initial(&s[1..]),
        _ => false,
    }
}

/// Returns `true` if `c` is an annotated onglide: *q* or *w*.
#[inline]
pub const fn is_onglide(c: char) -> bool {
    matches!(c, 'q' | 'w')
}

/// Returns `true` if `c` is an annotated offglide: *ĭ* or *ŭ*.
#[inline]
pub const fn is_offglide(c: char) -> bool {
    matches!(c, 'ĭ' | 'ŭ')
}

/// Returns `true` if `c` is a sonorant: one of *l m n r*.
#[inline]
pub const fn is_sonorant(c: char) -> bool {
    matches!(c, 'l' | 'm' | 'n' | 'r')
}

/// Returns `true` if `c` is a vowel: *a e i o u y*.
#[inline]
pub const fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

/// Returns `true` if `c` is a diphthong: *ai ei oi au*.
#[inline]
pub fn is_diphthong(s: &str) -> bool {
    matches!(s, "ai" | "ei" | "oi" | "au")
}

/*
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
pub fn is_follow_vowel_cluster(s: &str) -> bool {
    matches!(s.chars().next(), Some('i' | 'u')) && is_start_vowel_cluster(&s[1..])
}
*/

/// Returns `true` if `s` is a lujvo hyphen, used to prevent cmavo-shaped rafsi
/// from falling off the start of a lujvo and to delimit zi'evla inside a lujvo.
#[inline]
pub fn is_hyphen(s: &str) -> bool {
    matches!(s, "r" | "n" | "y" | "'y" | "y'" | "'y'")
}
