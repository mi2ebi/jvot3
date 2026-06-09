//! Various fast lookups for consonant clusters and vowels.

/// A bitmask of valid consonant clusters.
const VALID_TABLE: [u32; 25] = [
    0b1_0001_0001_0001_1101_0010_0100, // bd bg bj bl bm bn br bv bz
    0b0_0000_0101_0101_1110_0001_0000, // cf ck cl cm cn cp cr ct
    0b1_0001_0001_0001_1101_0010_0001, // db dg dj dl dm dn dr dv dz
    0,
    0b0_0100_0111_0101_1110_0000_0010, // fc fk fl fm fn fp fr fs ft fx
    0b1_0001_0001_0001_1101_0000_0101, // gb gd gj gl gm gn gr gv gz
    0,
    0,
    0b0_0001_0001_0001_1100_0010_0101, // jb jd jg jl jm jn jr jv
    0b0_0000_0111_0101_1100_0001_0010, // kc kf kl km kn kp kr ks kt
    0b1_0101_0111_0101_1011_0011_0111, // lb lc ld lf lg lj lk lm ln lp lr ls lt lv lx lz
    0b0_0101_0111_0101_0111_0011_0111, // mb mc md mf mg mj mk ml mn mp mr ms mt mv mx
    0b1_0101_0111_0100_1111_0011_0111, // nb nc nd nf ng nj nk nl nm np nr ns nt nv nx nz
    0,
    0b0_0100_0111_0001_1110_0001_0010, // pc pf pk pl pm pn pr ps pt px
    0,
    0b1_0101_0110_0101_1111_0011_0111, // rb rc rd rf rg rj rk rl rm rn rp rs rt rv rx rz
    0b0_0100_0101_0101_1110_0001_0000, // sf sk sl sm sn sp sr st sx
    0b0_0100_0011_0101_1110_0001_0010, // tc tf tk tl tm tn tp tr ts tx
    0,
    0b1_0000_0001_0001_1101_0010_0101, // vb vd vg vj vl vm vn vr vz
    0,
    0b0_0000_0111_0101_1100_0001_0000, // xf xl xm xn xp xr xs xt
    0,
    0b0_0001_0001_0001_1100_0010_0101, // zb zd zg zl zm zn zr zv
];

/// Returns `true` if `s` is a valid consonant cluster.
pub const fn is_valid(s: &str) -> bool {
    let [x, y] = s.as_bytes() else { return false };
    let xi = x.wrapping_sub(b'b') as usize;
    let yi = y.wrapping_sub(b'b') as usize;
    if xi >= 25 || yi >= 25 {
        return false;
    }
    (VALID_TABLE[xi] >> yi) & 1 != 0
}

/// Returns `true` if `s` is a valid consonant cluster or if it's *mz*.
#[inline]
pub const fn is_mz_valid(s: &str) -> bool { is_valid(s) || matches!(s.as_bytes(), b"mz") }

/// A bitmask of valid word-initial consonant clusters.
const INITIAL_TABLE: [u32; 25] = [
    0b0_0000_0001_0000_0100_0000_0000, // bl br
    0b0_0000_0101_0101_1110_0001_0000, // cf ck cl cm cn cp cr ct
    0b1_0000_0001_0000_0001_0000_0000, // dj dr dz
    0,
    0b0_0000_0001_0000_0100_0000_0000, // fl fr
    0b0_0000_0001_0000_0100_0000_0000, // gl gr
    0,
    0,
    0b0_0001_0000_0000_1000_0010_0101, // jb jd jg jm jv
    0b0_0000_0001_0000_0100_0000_0000, // kl kr
    0,
    0b0_0000_0001_0000_0100_0000_0000, // ml mr
    0,
    0,
    0b0_0000_0001_0000_0100_0000_0000, // pl pr
    0,
    0,
    0b0_0000_0101_0101_1110_0001_0000, // sf sk sl sm sn sp sr st
    0b0_0000_0011_0000_0000_0000_0010, // tc tr ts
    0,
    0b0_0000_0001_0000_0100_0000_0000, // vl vr
    0,
    0b0_0000_0001_0000_0100_0000_0000, // xl xr
    0,
    0b0_0001_0000_0000_1000_0010_0101, // zb zd zg zm zv
];

/// Returns `true` if `s` is a valid word-initial consonant cluster.
#[inline]
pub const fn is_initial(s: &str) -> bool {
    let [x, y] = s.as_bytes() else { return false };
    let xi = x.wrapping_sub(b'b') as usize;
    let yi = y.wrapping_sub(b'b') as usize;
    if xi >= 25 || yi >= 25 {
        return false;
    }
    (INITIAL_TABLE[xi] >> yi) & 1 != 0
}

/// Returns `true` if `s` is one of the consonant triples banned by CLL: *ndj
/// ndz ntc nts*.
#[inline]
pub const fn is_banned_triple(s: &str) -> bool {
    matches!(s.as_bytes(), b"ndj" | b"ndz" | b"ntc" | b"nts")
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

#[macro_export]
macro_rules! test_bytes {
    ($funcs:ident ($($char:expr),+)) => {{
        if let Ok(s) = str::from_utf8(&[$($char as u8),+]) { $funcs(s) } else { false }
    }}
}

/// Returns `true` if `s` is a permissible syllable onset.
#[inline]
pub const fn is_hard_onset(s: &str) -> bool {
    match s.as_bytes() {
        [] => true,
        &[x] => is_hard_consonant(x as char),
        &[_, _] => is_initial(s),
        &[x, y, z] => {
            matches!(x, b'c' | b'j' | b's' | b'z')
                && test_bytes!(is_initial(x, y))
                && test_bytes!(is_initial(y, z))
                && matches!(z, b'l' | b'r')
        }
        _ => false,
    }
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
pub const fn is_diphthong(s: &str) -> bool { matches!(s.as_bytes(), b"ai" | b"ei" | b"oi" | b"au") }

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
pub const fn is_hyphen(s: &str) -> bool {
    matches!(s.as_bytes(), b"r" | b"n" | b"y" | b"'y" | b"y'" | b"'y'")
}
