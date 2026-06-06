use itertools::Itertools as _;

use crate::{
    fli, flip,
    jvofli::{Jvofli, Jvoflikle::SyllableError},
    phonology::{
        is_banned_triple, is_diphthong, is_hard_consonant, is_hard_onset, is_initial, is_offglide,
        is_onglide, is_sonorant, is_valid, is_vowel,
    },
    settings::Settings,
    test_bytes,
};

/// Checks if `s` only contains Lojban letters and returns `Ok(())` if so.
///
/// # Errors
/// Returns a [`SyllableError`] if it doesn't.
pub fn check_lojban_only(s: &str) -> Result<(), Jvofli> {
    if let Some(bad) = s.chars().find(|&c| {
        !(is_hard_consonant(c) || is_vowel(c) || is_onglide(c) || is_offglide(c) || c == '\'')
    }) {
        flip!(SyllableError, "{{{s}}} contains non-lojban characters such as {{{bad}}}")
    }
    Ok(())
}

/// Respells *i u* when they are onglides (as *q w*) or offglides (as *ĭ ŭ*).
///
/// # Errors
/// Returns a [`SyllableError`] if any invalid falling diphthongs are used.
pub fn mark_glides(input: &str) -> Result<String, Jvofli> {
    let mut chars = input.chars().collect_vec();
    for i in (0..chars.len()).rev() {
        let c = chars[i];
        if matches!(c, 'i' | 'u') {
            let (on, off) = match c {
                'i' => ('q', 'ĭ'),
                'u' => ('w', 'ŭ'),
                _ => {
                    unreachable!("[mark_glides] only {{i/u}} should get here but this is a {{{c}}}")
                }
            };
            if i != chars.len() - 1 && matches!(chars[i + 1], 'a' | 'e' | 'i' | 'o' | 'u' | 'y') {
                chars[i] = on;
            } else if i != 0
                && let before = chars[i - 1]
                && matches!(before, 'a' | 'e' | 'o' | 'y')
            {
                if test_bytes!(is_diphthong(before, c)) {
                    chars[i] = off;
                    if chars.get(i + 1).is_some_and(|&c| c == on) {
                        flip!(SyllableError, "{{{off}{on}}} is invalid");
                    }
                } else {
                    flip!(SyllableError, "{{{before}{c}}} is not a valid diphthong");
                }
            }
        }
    }
    Ok(chars.iter().collect::<String>())
}

/// Extracts a coda from a list of consonants, and splits the rest into
/// consonantal syllables if possible.
///
/// # Errors
/// Returns a [`SyllableError`] if the cluster can't be split.
fn parse_previous_coda(chars: &[char]) -> Result<(Option<char>, Vec<String>), Jvofli> {
    if chars.is_empty() {
        return Ok((None, vec![]));
    }
    if (chars.len() - 1).is_multiple_of(2)
        && is_hard_consonant(chars[0])
        && let Ok(syllables) = as_consonantal_syllables(&chars[1..])
    {
        return Ok((Some(chars[0]), syllables));
    }
    if chars.len().is_multiple_of(2)
        && let Ok(syllables) = as_consonantal_syllables(chars)
    {
        return Ok((None, syllables));
    }
    flip!(
        SyllableError,
        "{{{}}} can't be parsed as an optional coda followed by some consonantal syllables",
        chars.iter().collect::<String>()
    );
}

/// Nicely packages consonantal syllables.
///
/// # Errors
/// Returns a [`SyllableError`] if there is anything that isn't a consonant
/// syllable.
fn as_consonantal_syllables(chars: &[char]) -> Result<Vec<String>, Jvofli> {
    if chars.is_empty() {
        return Ok(vec![]);
    }
    if !chars.len().is_multiple_of(2) {
        unreachable!(
            "[as_consonantal_syllables] odd number of chars in {{{}}}",
            String::from_iter(chars)
        );
    }
    let mut syllables = vec![];
    for chunk in chars.chunks(2) {
        let &[first, second] = chunk else {
            unreachable!("[as_consonantal_syllables] chunks must be length 2");
        };
        #[expect(clippy::suspicious_operation_groupings)]
        if is_hard_consonant(first) && is_sonorant(second) && first != second {
            syllables.push(chunk.iter().collect());
        } else {
            flip!(SyllableError, "{{{}}} is not a consonantal syllable", String::from_iter(chunk));
        }
    }
    Ok(syllables)
}

/// Applies a parsed coda to the syllable list.
///
/// # Errors
/// Returns a [`SyllableError`] if the cluster at the syllable boundary is
/// invalid, or if there is no previous syllable for a coda to attach to.
fn apply_coda(
    real: &mut Vec<String>,
    chars: &[char],
    next_consonant: Option<char>,
) -> Result<(), Jvofli> {
    if next_consonant.is_none()
        && let Some(c) = chars.iter().find(|&&c| is_onglide(c) || c == '\'')
    {
        flip!(SyllableError, "{{{c}}} can't appear in codas");
    }
    let (coda, consonant_syllables) = parse_previous_coda(chars)?;
    if let Some(coda) = coda {
        // regroup `coda` + the first consonant of what follows.
        // "what follows" is either the first char of the first consonantal syllable,
        // or if there aren't any, the first char of the onset we're about to push
        let next = consonant_syllables.first().and_then(|s| s.chars().next()).or(next_consonant);
        if let Some(c) = next
            && !test_bytes!(is_valid(coda, c))
        {
            // flip!(SyllableError, "{{{coda}{c}}} is an invalid cluster");
            unreachable!(
                "[apply_coda] scary case should have checked validity of {{{coda}{c}}} already"
            );
        }
        let Some(prev) = real.last_mut() else {
            let next = next.unwrap_or_else(|| {
                unreachable!(
                    "[apply_coda] `next` and `real.last_mut()` can't both be `None`: word-final \
                     codas always have a previous syllable, and if not it should be caught by \
                     'has no vowels'"
                )
            });
            flip!(SyllableError, "{{{coda}{next}}} is an invalid word-initial cluster");
        };
        prev.push(coda);
    }
    for cs in consonant_syllables {
        real.push(cs);
    }
    Ok(())
}

/// Splits some text after diphthongs and single vowels.
fn split_after_nuclei(s: &str) -> Vec<&str> {
    let mut pieces = vec![];
    let mut start = 0;
    let mut chars = s.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        let next = chars.peek().map(|&(_, c)| c);
        if is_vowel(c) && !next.is_some_and(is_offglide) || is_offglide(c) {
            let end = chars.peek().map_or(s.len(), |&(i, _)| i);
            pieces.push(&s[start..end]);
            start = end;
        }
    }
    if start < s.len() {
        pieces.push(&s[start..]);
    }
    pieces
}

/// Splits a unit in standard spelling into syllables.
///
/// # Errors
/// Returns a [`SyllableError`] if the input can't be split into valid
/// syllables.
#[allow(clippy::too_many_lines)]
pub fn syllabify(input: &str) -> Result<Vec<String>, Jvofli> {
    check_lojban_only(input)?;
    let annotated = mark_glides(input)?;
    if !annotated.chars().any(is_vowel) {
        flip!(SyllableError, "{{{input}}} has no vowels");
    }
    let fake = split_after_nuclei(&annotated);
    let mut real = vec![];
    for (i, piece) in fake.iter().enumerate() {
        let chars = piece.chars().collect_vec();
        // the last piece might be a coda since `POST_NUCLEUS` splits after nuclei
        if i == fake.len() - 1 && !chars.iter().any(|&c| is_vowel(c)) {
            apply_coda(&mut real, &chars, None)?;
            continue;
        }
        // in case someone wrote a misplaced ĭ/ŭ
        // (`mark_glides` shouldn't cause this)
        if let Some(&last) = chars.last()
            && matches!(last, 'ĭ' | 'ŭ')
            && (chars.len() < 2 || !is_vowel(chars[chars.len() - 2]))
        {
            flip!(SyllableError, "{{{last}}} must come after a vowel");
        }
        let nucleus_len = if matches!(chars.last(), Some('ĭ' | 'ŭ')) { 2 } else { 1 };
        let onset_chars = &chars[..chars.len() - nucleus_len];
        let nucleus = chars[chars.len() - nucleus_len..].iter().collect::<String>();
        match onset_chars {
            // null-onset syllables must be word-initial
            [] => {
                if i != 0 {
                    flip!(SyllableError, "{{{piece}}} lacks an onset");
                }
                real.push(nucleus);
            }
            // h-onset syllables must not
            ['\''] => {
                if i == 0 {
                    flip!(SyllableError, "{{'}} can't appear word-initially");
                }
                let mut s = String::with_capacity(1 + nucleus.len());
                s.push('\'');
                s.push_str(&nucleus);
                real.push(s);
            }
            // `mark_glides` validates these
            &[c] if is_onglide(c) => {
                let mut s = String::with_capacity(c.len_utf8() + nucleus.len());
                s.push(c);
                s.push_str(&nucleus);
                real.push(s);
            }
            // unambiguous hard onset
            o if let mut o = o.iter().collect::<String>()
                && is_hard_onset(&o) =>
            {
                o.push_str(&nucleus);
                real.push(o);
            }
            // evil q/w/'
            o if let Some(c) = o.iter().find(|&&c| is_onglide(c) || c == '\'') => {
                flip!(SyllableError, "{{{c}}} can't be adjacent to consonants");
            }
            // the scary case.
            // try treating each suffix of the onset (longest first) as a hard onset, assuming the
            // rest is the previous syllable's coda, and take the first split that works
            _ => {
                let mut best_err = None;
                let Some((suffix_len, hard_onset)) =
                    (1..=onset_chars.len().min(3)).rev().find_map(|suffix_len| {
                        let hard_onset: String =
                            onset_chars[onset_chars.len() - suffix_len..].iter().collect();
                        if !is_hard_onset(&hard_onset) {
                            return None;
                        }
                        let prefix = &onset_chars[..onset_chars.len() - suffix_len];
                        let (coda, consonantal_syllables) = match parse_previous_coda(prefix) {
                            Ok(x) => x,
                            Err(e) => {
                                best_err.get_or_insert(e);
                                return None;
                            }
                        };
                        if consonantal_syllables.is_empty()
                            && let (Some(coda), Some(c)) = (coda, hard_onset.chars().next())
                            && !test_bytes!(is_valid(coda, c))
                        {
                            best_err =
                                Some(fli!(SyllableError, "{{{coda}{c}}} is an invalid cluster"));
                            return None;
                        }
                        if !consonantal_syllables.is_empty()
                            && let (Some(&r), Some(c)) = (prefix.last(), hard_onset.chars().next())
                            && !test_bytes!(is_valid(r, c))
                        {
                            best_err =
                                Some(fli!(SyllableError, "{{{r}{c}}} is an invalid cluster"));
                            return None;
                        }
                        if let Some(coda) = coda
                            && hard_onset.len() >= 2
                        {
                            let mut chars = hard_onset.chars();
                            let oob = || unreachable!("[syllabify] `hard_onset` has a length < 2");
                            let first = chars.next().unwrap_or_else(oob);
                            let second = chars.next().unwrap_or_else(oob);
                            if test_bytes!(is_banned_triple(coda, first, second)) {
                                best_err = Some(fli!(
                                    SyllableError,
                                    "{{{coda}{first}{second}}} is a banned triple"
                                ));
                                return None;
                            }
                        }
                        Some((suffix_len, hard_onset))
                    })
                else {
                    return Err(best_err.unwrap_or_else(|| {
                        unreachable!("[syllabify] `best_err` should be `Some` by now")
                    }));
                };
                let prefix = &onset_chars[..onset_chars.len() - suffix_len];
                apply_coda(&mut real, prefix, hard_onset.chars().next())?;
                let mut hard_onset = hard_onset;
                hard_onset.push_str(&nucleus);
                real.push(hard_onset);
            }
        }
    }
    Ok(real)
}

#[allow(clippy::many_single_char_names)]
pub const fn is_gismu(s: &str) -> bool {
    let &[a, b, c, d, e] = s.as_bytes() else { return false };
    let [ab, cd] = [[a, b], [c, d]];
    let [Ok(ab), Ok(cd)] = [str::from_utf8(&ab), str::from_utf8(&cd)] else { return false };
    is_hard_consonant(a as char)
        && is_hard_consonant(d as char)
        && is_vowel(e as char)
        && (is_vowel(b as char) && is_valid(cd) || is_vowel(c as char) && is_initial(ab))
}

/// Returns `true` if the first syllable of `sylls` is CV or CF. It does not
/// consider the rest of the syllables at all.
fn starts_with_one_cmavo(sylls: &[String]) -> bool {
    let Some(first) = sylls.first() else { return false };
    first.chars().filter(|c| is_hard_consonant(*c)).nth(1).is_none()
        && first.ends_with(|c| is_vowel(c) || is_offglide(c))
}

/// Returns true if this syllable requires a cmavo ending in *y* after it. `arb`
/// indicates whether we care about all cmavo ending in *y* (`true`), or only
/// *Cy* cmavo like CLL says (`false`).
fn requires_y_next(syll: &str, arb: bool) -> bool {
    syll.ends_with('y')
        && (arb || syll.starts_with(|c| is_hard_consonant(c) || is_onglide(c) || c == 'y'))
}

/// Returns `true` if `sylls[i + 1]` is the first syllable of a cmavo ending in
/// *y*. `arb` indicates whether we care about all cmavo ending in *y* (`true`),
/// or only *Cy* cmavo like CLL says (`false`).
fn next_cmavo_ends_in_y(sylls: &[String], i: usize, arb: bool) -> bool {
    if arb {
        let end = sylls[i + 1..]
            .iter()
            .position(|s| !s.starts_with('\''))
            .map_or(sylls.len(), |p| i + 1 + p);
        sylls[i + 1..end]
            .iter()
            .all(|s| s.starts_with('\'') && s.ends_with(|c| is_vowel(c) || is_offglide(c)))
            && sylls[end - 1].ends_with('y')
    } else {
        sylls[i].len() == 2 && sylls[i].ends_with('y')
    }
}

/// Returns `true` if `s` is some cmavo. `settings.arbitrary_cmavo_rafsi`
/// affects which cmavo ending in *y* need pauses after them.
pub fn is_some_cmavo(s: &str, settings: &Settings) -> bool {
    let Ok(sylls) = syllabify(s) else { return false };
    let arb = settings.arbitrary_cmavo_rafsi;
    let mut sylls = &*sylls;
    while !sylls.is_empty() {
        if !starts_with_one_cmavo(sylls) {
            return false;
        }
        let next = sylls
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_, s)| s.starts_with(|c: char| is_hard_consonant(c) || is_onglide(c)))
            .map(|(i, _)| i);
        let Some(i) = next else { break };
        if requires_y_next(&sylls[i - 1], arb) && !next_cmavo_ends_in_y(sylls, i, arb) {
            return false;
        }
        sylls = &sylls[i..];
    }
    true
}

/// Tests.
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ok {
        (mark_glides; $in:literal => $out:literal) => {
            assert_eq!(mark_glides($in), Ok($out.to_string()));
        };
        (syllabify; $in:literal => $($out:literal),+) => {
            assert_eq!(syllabify($in), Ok([$($out),+].iter().map(ToString::to_string).collect_vec()));
        };
        (is_some_cmavo as $settings:literal; $in:literal) => {
            assert!(is_some_cmavo($in, &$settings.parse().unwrap()));
        };
    }
    macro_rules! err {
        (is_some_cmavo as $settings:literal; $in:literal) => {
            assert!(!is_some_cmavo($in, &$settings.parse().unwrap()));
        };
        ($f:ident; $in:literal) => {
            let res = $f($in);
            assert!(res.is_err(), "{res:?}");
        };
    }

    #[cfg(test)]
    mod mark_glides {
        use super::*;
        #[test]
        fn plukauaii_ok() {
            ok!(mark_glides; "plukauaii" => "plukawaqi");
        }
        #[test]
        fn u12_ok() {
            ok!(mark_glides; "uuuuuuuuuuuu" => "wuwuwuwuwuwu");
        }
        #[test]
        fn u13_ok() {
            ok!(mark_glides; "uuuuuuuuuuuuu" => "uwuwuwuwuwuwu");
        }
        #[test]
        fn auia_ok() {
            ok!(mark_glides; "auia" => "aŭqa");
        }
        #[test]
        fn aiia_err() {
            err!(mark_glides; "aiia");
        }
        #[test]
        fn eu_err() {
            err!(mark_glides; "eu");
        }
        #[test]
        fn eua_ok() {
            ok!(mark_glides; "eua" => "ewa");
        }
    }

    #[cfg(test)]
    mod syllabify {
        use super::*;
        #[test]
        fn latkerlo_ok() {
            ok!(syllabify; "latkerlo" => "lat", "ker", "lo");
        }
        #[test]
        fn sakprtlfmsngeha_ok() {
            ok!(syllabify; "sakprtlfmsnge'a" => "sak", "pr", "tl", "fm", "sn", "ge", "'a");
        }
        #[test]
        fn glek_ok() {
            ok!(syllabify; "glek" => "glek");
        }
        #[test]
        fn apba_err() {
            err!(syllabify; "apba");
        }
        #[test]
        fn apqa_err() {
            err!(syllabify; "apqa");
        }
        #[test]
        fn apyb_ok() {
            ok!(syllabify; "apyb" => "a", "pyb");
        }
        #[test]
        fn apb_err() {
            err!(syllabify; "apb");
        }
        #[test]
        fn aplbra_ok() {
            ok!(syllabify; "aplbra" => "a", "pl", "bra");
        }
        #[test]
        fn aplua_err() {
            err!(syllabify; "aplua");
        }
        #[test]
        fn an_ok() {
            ok!(syllabify; "an" => "an");
        }
        #[test]
        fn ant_err() {
            err!(syllabify; "ant"); // syllables can end in at most 1 consonant
        }
        #[test]
        fn antka_err() {
            err!(syllabify; "antka"); // ant,ka and an,tka are both illegal
        }
        #[test]
        fn cipnrstrigi_ok() {
            ok!(syllabify; "cipnrstrigi" => "cip", "nr", "stri", "gi");
        }
        #[test]
        fn apcnli_ok() {
            ok!(syllabify; "apcnli" => "ap", "cn", "li");
        }
        #[test]
        fn nondza_err() {
            err!(syllabify; "nondza");
        }
        #[test]
        fn djarrspageti_err() {
            err!(syllabify; "djarrspageti");
        }
        #[test]
        fn ivllava_err() {
            err!(syllabify; "ivllava");
        }
    }

    #[cfg(test)]
    mod is_some_cmavo {
        use super::*;
        #[test]
        fn iesai_ok() {
            ok!(is_some_cmavo as "x"; "iesai");
        }
        #[test]
        fn pahe_ok() {
            ok!(is_some_cmavo as "x"; "pa'e");
        }
        #[test]
        fn yhy_ok() {
            ok!(is_some_cmavo as "x"; "y'y");
        }
        #[test]
        fn anba_err() {
            err!(is_some_cmavo as "x"; "anba");
        }
        #[test]
        fn bycydyfy_cll_ok() {
            ok!(is_some_cmavo as "x"; "bycydyfy");
        }
        #[test]
        fn bycydyfy_r_ok() {
            ok!(is_some_cmavo as "r"; "bycydyfy");
        }
        #[test]
        fn tehynahy_cll_ok() {
            ok!(is_some_cmavo as "x"; "te'yna'y");
        }
        #[test]
        fn tehynahy_r_ok() {
            ok!(is_some_cmavo as "r"; "te'yna'y");
        }
        #[test]
        fn bynahy_cll_err() {
            err!(is_some_cmavo as "x"; "byna'y");
        }
        #[test]
        fn bynahy_r_ok() {
            ok!(is_some_cmavo as "r"; "byna'y");
        }
        #[test]
        fn bavyteikehu_cll_err() {
            err!(is_some_cmavo as "x"; "bavyteike'u");
        }
        #[test]
        fn bavyteikehu_r_err() {
            err!(is_some_cmavo as "r"; "bavyteike'u");
        }
        #[test]
        fn bahyteikehu_cll_ok() {
            ok!(is_some_cmavo as "x"; "ba'yteike'u");
        }
        #[test]
        fn bahyteikehu_r_err() {
            err!(is_some_cmavo as "r"; "ba'yteike'u");
        }
        #[test]
        fn zihybalvi_cll_err() {
            err!(is_some_cmavo as "x"; "zi'ybalvi");
        }
        #[test]
        fn zihybalvi_r_err() {
            err!(is_some_cmavo as "r"; "zi'ybalvi");
        }
    }
}
