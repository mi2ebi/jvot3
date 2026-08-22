//! Various fast lookups for consonant clusters and vowels.

use std::sync::LazyLock;

use bimap::BiHashMap;

use crate::{extract_settings, settings::Settings};

// - tables -

const B: u32 = 1 << 0;
const C: u32 = 1 << 1;
const D: u32 = 1 << 2;
const F: u32 = 1 << 4;
const G: u32 = 1 << 5;
const J: u32 = 1 << 8;
const K: u32 = 1 << 9;
const L: u32 = 1 << 10;
const M: u32 = 1 << 11;
const N: u32 = 1 << 12;
const P: u32 = 1 << 14;
const R: u32 = 1 << 16;
const S: u32 = 1 << 17;
const T: u32 = 1 << 18;
const V: u32 = 1 << 20;
const X: u32 = 1 << 22;
const Z: u32 = 1 << 24;

const VOICED: u32 = B | D | G | J | V | Z;
const VOICELESS: u32 = C | F | K | P | S | T | X;
const SONORANT: u32 = L | M | N | R;
const ALL: u32 = VOICED | VOICELESS | SONORANT;
const LIQUID: u32 = L | R;
const SIBILANT: u32 = C | J | S | Z;

const VALID_TABLE: [u32; 25] = [
    /* b */ VOICED ^ B | SONORANT,
    /* c */ VOICELESS & !(SIBILANT | X) | SONORANT,
    /* d */ VOICED ^ D | SONORANT,
    0,
    /* f */ VOICELESS ^ F | SONORANT,
    /* g */ VOICED ^ G | SONORANT,
    0,
    0,
    /* j */ VOICED & !SIBILANT | SONORANT,
    /* k */ VOICELESS ^ (K | X) | SONORANT,
    /* l */ ALL ^ L,
    /* m */ ALL ^ (M | Z), // lojban...
    /* n */ ALL ^ N,
    0,
    /* p */ VOICELESS ^ P | SONORANT,
    0,
    /* r */ ALL ^ R,
    /* s */ VOICELESS & !SIBILANT | SONORANT,
    /* t */ VOICELESS ^ T | SONORANT,
    0,
    /* v */ VOICED ^ V | SONORANT,
    0,
    /* x */ VOICELESS ^ (C | K | X) | SONORANT,
    0,
    /* z */ VOICED & !SIBILANT | SONORANT,
];

const INITIAL_TABLE: [u32; 25] = [
    /* b */ LIQUID,
    /* c */ VOICELESS & !(SIBILANT | X) | SONORANT,
    /* d */ VOICED & SIBILANT | R,
    0,
    /* f */ LIQUID,
    /* g */ LIQUID,
    0,
    0,
    /* j */ VOICED & !SIBILANT | M,
    /* k */ LIQUID,
    0,
    /* m */ LIQUID,
    0,
    0,
    /* p */ LIQUID,
    0,
    0,
    /* s */ VOICELESS & !(SIBILANT | X) | SONORANT,
    /* t */ VOICELESS & SIBILANT | R,
    0,
    /* v */ LIQUID,
    0,
    /* x */ LIQUID,
    0,
    /* z */ VOICED & !SIBILANT | M,
];

// - vowels -

/// Returns whether `c` is a stressable vowel (including when actually
/// stressed): *a e i o u*.
#[inline]
#[must_use]
pub const fn is_stressable_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'á' | 'é' | 'í' | 'ó' | 'ú')
}

/// Returns whether `c` is a vowel, i.e. including *y*, optionally with a stress
/// mark (even for *y*, to produce better error messages).
#[inline]
#[must_use]
pub const fn is_vowel(c: char) -> bool { is_stressable_vowel(c) || c == 'y' || c == 'ý' }

/// Returns whether `x` and `y` form a diphthong: *ai au ei oi*.
#[inline]
#[must_use]
pub const fn is_diphthong_chars(x: char, y: char) -> bool {
    matches!([x, y], ['a' | 'á' | 'e' | 'é' | 'o' | 'ó', 'i'] | ['a' | 'á', 'u'])
}

#[inline]
#[must_use]
pub(crate) const fn is_annotated_onglide(c: char) -> bool { matches!(c, 'q' | 'w') }

#[inline]
#[must_use]
pub(crate) const fn is_annotated_offglide(c: char) -> bool { matches!(c, 'ĭ' | 'ŭ') }

/// Returns whether `c` is *i* or *u*.
#[inline]
#[must_use]
pub const fn could_be_glide(c: char) -> bool { matches!(c, 'i' | 'u') }

/// Normalizes an annotated glide character back to the plain vowel it
/// represents (*q*/*ĭ* → *i*, *w*/*ŭ* → *u*).
#[inline]
#[must_use]
pub const fn deannotate_glide(c: char) -> char {
    match c {
        'q' | 'ĭ' => 'i',
        'w' | 'ŭ' => 'u',
        _ => c,
    }
}

static STRESS: LazyLock<BiHashMap<char, char>> = LazyLock::new(|| {
    let mut m = BiHashMap::new();
    m.insert('á', 'a');
    m.insert('é', 'e');
    m.insert('í', 'i');
    m.insert('ó', 'o');
    m.insert('ú', 'u');
    m.insert('ý', 'y');
    m
});

/// Removes the stress from a vowel, returning the plain vowel and whether it
/// was stressed.
pub fn strip_stress_accent(c: char) -> (char, bool) {
    STRESS.get_by_left(&c).map_or((c, false), |&plain| (plain, true))
}

/// Adds explicit stress to a vowel.
pub fn add_stress_accent(c: char) -> Option<char> { STRESS.get_by_right(&c).copied() }

// - consonants -

/// Returns whether `c` is in the given bitset, where bit `i` corresponds to
/// the letter `b'b' + i`.
#[inline]
const fn is_in_set(c: u8, set: u32) -> bool {
    let idx = (c as u32).wrapping_sub(b'b' as u32);
    idx < 25 && (set >> idx) & 1 != 0
}

/// Returns whether `c` is a consonant, *excluding* apostrophe.
#[inline]
#[must_use]
pub const fn is_hard_consonant(c: char) -> bool { is_in_set(c as u8, ALL) }

/// Returns whether `c` is a consonant, *including* apsotrophe.
#[inline]
#[must_use]
pub const fn is_consonant(c: char) -> bool { is_hard_consonant(c) || c == '\'' }

/// Returns whether `c` is one of *l m n r*.
#[inline]
#[must_use]
pub const fn is_sonorant(c: char) -> bool { is_in_set(c as u8, SONORANT) }

// - clusters -

/// Returns whether the byte pair `(x, y)` is allowed by `table`.
#[inline]
const fn check_pair(x: u8, y: u8, table: &[u32; 25]) -> bool {
    let xi = x.wrapping_sub(b'b') as usize;
    let yi = y.wrapping_sub(b'b') as usize;
    if xi >= 25 || yi >= 25 {
        return false;
    }
    (table[xi] >> yi) & 1 != 0
}

/// Returns whether `x` and `y` form a valid consonant cluster. `settings` is
/// used for `allow_mz`.
#[inline]
#[must_use]
pub const fn is_valid_chars(x: char, y: char, settings: Settings) -> bool {
    let settings = extract_settings!(settings; allow_mz);
    check_pair(x as u8, y as u8, &VALID_TABLE) || settings.allow_mz && x == 'm' && y == 'z'
}

/// Returns whether `x` and `y` form a valid word-initial consonant cluster.
#[inline]
#[must_use]
pub const fn is_initial_chars(x: char, y: char) -> bool {
    check_pair(x as u8, y as u8, &INITIAL_TABLE)
}

/// Returns whether `x`, `y`, and `z` form one of the consonant triples banned
/// by CLL: *ndj ndz ntc nts*.
#[inline]
#[must_use]
pub const fn is_banned_triple_chars(x: char, y: char, z: char) -> bool {
    x == 'n' && matches!([y, z], ['d', 'j' | 'z'] | ['t', 'c' | 's'])
}

/// Returns whether `s` is a hard onset (an onset consisting only of characters
/// that [`is_hard_consonant`]).
#[inline]
#[must_use]
pub const fn is_hard_onset(s: &str) -> bool {
    match *s.as_bytes() {
        [x] => is_in_set(x, ALL),
        [x, y] => check_pair(x, y, &INITIAL_TABLE),
        [x, y, z] => {
            is_in_set(x, SIBILANT)
                && is_in_set(z, LIQUID)
                && check_pair(x, y, &INITIAL_TABLE)
                && check_pair(y, z, &INITIAL_TABLE)
        }
        _ => false,
    }
}

// - hyphens -

/// Returns whether `s` is a lujvo hyphen, used to prevent cmavo-shaped rafsi
/// from falling off the start of a lujvo and to delimit zi'evla inside a lujvo.
#[inline]
#[must_use]
pub const fn is_hyphen(s: &str) -> bool {
    matches!(s.as_bytes(), b"r" | b"n" | b"y" | b"'y" | b"y'" | b"'y'")
}
