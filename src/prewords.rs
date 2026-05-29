use itertools::Itertools as _;

use crate::{
    fli, flip,
    jvofli::{Jvofli, Jvoflikle::Jboraku},
    phonology::{
        is_diphthong, is_hard_consonant, is_hard_onset, is_offglide, is_onglide, is_sonorant,
        is_valid, is_vowel,
    },
};

/// Checks if `s` only contains Lojban letters and returns `Ok(())` if so.
///
/// # Errors
/// Returns a [`Jboraku`] if it doesn't.
pub fn check_lojban_only(s: &str) -> Result<(), Jvofli> {
    if let Some(bad) = s.chars().find(|&c| {
        !(is_hard_consonant(c) || is_vowel(c) || is_onglide(c) || is_offglide(c) || c == '\'')
    }) {
        flip!(Jboraku, "{{{s}}} contains non-lojban characters such as {{{bad}}}")
    }
    Ok(())
}

/// Respells *i u* when they are onglides (as *q w*) or offglides (as *ĭ ŭ*).
///
/// # Errors
/// Returns a [`Jboraku`] if any invalid falling diphthongs are used.
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
                if is_diphthong(&format!("{before}{c}")) {
                    chars[i] = off;
                    if chars.get(i + 1).is_some_and(|&c| c == on) {
                        flip!(Jboraku, "{{{off}{on}}} is invalid");
                    }
                } else {
                    flip!(Jboraku, "{{{before}{c}}} is not a valid diphthong");
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
/// Returns a [`Jboraku`] if the cluster can't be split.
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
        Jboraku,
        "{{{}}} can't be parsed as an optional coda followed by some consonantal syllables",
        chars.iter().collect::<String>()
    );
}

/// Nicely packages consonantal syllables.
///
/// # Errors
/// Returns a [`Jboraku`] if there is anything that isn't a consonant syllable.
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
        if is_hard_consonant(first) && is_sonorant(second) {
            syllables.push(chunk.iter().collect());
        } else {
            flip!(Jboraku, "{{{}}} is not a consonantal syllable", String::from_iter(chunk));
        }
    }
    Ok(syllables)
}

/// Applies a parsed coda to the syllable list.
///
/// # Errors
/// Returns a [`Jboraku`] if the cluster at the syllable boundary is invalid, or
/// if there is no previous syllable for a coda to attach to.
fn apply_coda(
    real: &mut Vec<String>,
    chars: &[char],
    next_consonant: Option<char>,
) -> Result<(), Jvofli> {
    if next_consonant.is_none()
        && let Some(c) = chars.iter().find(|&&c| is_onglide(c) || c == '\'')
    {
        flip!(Jboraku, "{{{c}}} can't appear in codas");
    }
    let (coda, consonant_syllables) = parse_previous_coda(chars)?;
    if let Some(coda) = coda {
        // regroup `coda` + the first consonant of what follows.
        // "what follows" is either the first char of the first consonantal syllable,
        // or if there aren't any, the first char of the onset we're about to push
        let next = consonant_syllables.first().and_then(|s| s.chars().next()).or(next_consonant);
        if let Some(c) = next
            && !is_valid(&format!("{coda}{c}"))
        {
            // flip!(Jboraku, "{{{coda}{c}}} is an invalid cluster");
            unreachable!(
                "[apply_coda] scary case should have checked validity of {{{coda}{c}}} already"
            );
        }
        let Some(prev) = real.last_mut() else {
            flip!(Jboraku, "{{{coda}}} is a word-initial coda");
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
/// Returns a [`Jboraku`] if the input can't be split into valid syllables.
pub fn syllabify(input: &str) -> Result<Vec<String>, Jvofli> {
    check_lojban_only(input)?;
    let annotated = mark_glides(input)?;
    if !annotated.chars().any(is_vowel) {
        flip!(Jboraku, "{{{input}}} has no vowels");
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
            flip!(Jboraku, "{{{last}}} must come after a vowel");
        }
        let nucleus_len = if matches!(chars.last(), Some('ĭ' | 'ŭ')) { 2 } else { 1 };
        let onset_chars = &chars[..chars.len() - nucleus_len];
        let nucleus = chars[chars.len() - nucleus_len..].iter().collect::<String>();
        match onset_chars {
            // null-onset syllables must be word-initial
            [] => {
                if i != 0 {
                    flip!(Jboraku, "{{{piece}}} lacks an onset");
                }
                real.push(nucleus);
            }
            // h-onset syllables must not
            ['\''] => {
                if i == 0 {
                    flip!(Jboraku, "{{'}} can't appear word-initially");
                }
                real.push(format!("'{nucleus}"));
            }
            // `mark_glides` validates these
            [c] if is_onglide(*c) => {
                real.push(format!("{c}{nucleus}"));
            }
            // unambiguous hard onset
            o if let o = o.iter().collect::<String>()
                && is_hard_onset(&o) =>
            {
                real.push(format!("{o}{nucleus}"));
            }
            // evil q/w/'
            o if let Some(c) = o.iter().find(|&&c| is_onglide(c) || c == '\'') => {
                flip!(Jboraku, "{{{c}}} can't be adjacent to consonants");
            }
            // the scary case.
            // try treating each suffix of the onset (longest first) as a hard onset, assuming the
            // rest is the previous syllable's coda, and take the first split that works
            _ => {
                let mut last_err = None;
                let Some((suffix_len, hard_onset)) =
                    (1..=onset_chars.len().min(3)).rev().find_map(|suffix_len| {
                        let hard_onset: String =
                            onset_chars[onset_chars.len() - suffix_len..].iter().collect();
                        if !is_hard_onset(&hard_onset) {
                            return None;
                        }
                        let prefix = &onset_chars[..onset_chars.len() - suffix_len];
                        let (coda, consonantal_syllables) =
                            parse_previous_coda(prefix).map_err(|e| last_err = Some(e)).ok()?;
                        if consonantal_syllables.is_empty()
                            && let (Some(coda), Some(c)) = (coda, hard_onset.chars().next())
                            && !is_valid(&format!("{coda}{c}"))
                        {
                            last_err = Some(fli!(Jboraku, "{{{coda}{c}}} is an invalid cluster"));
                            return None;
                        }
                        Some((suffix_len, hard_onset))
                    })
                else {
                    return Err(last_err.unwrap_or_else(|| {
                        unreachable!("[syllabify] `last_err` should be `Some` by now")
                    }));
                };
                let prefix = &onset_chars[..onset_chars.len() - suffix_len];
                apply_coda(&mut real, prefix, hard_onset.chars().next())?;
                real.push(format!("{hard_onset}{nucleus}"));
            }
        }
    }
    Ok(real)
}

/// Groups syllables into jboraku, runs of syllables where the first one has a
/// non-*'* onset and the rest have *'* onsets.
pub fn jborakufy(syllables: &[String]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for syll in syllables {
        if syll.starts_with('\'') {
            match result.last_mut() {
                Some(prev) => prev.push_str(syll),
                None => result.push(syll.clone()),
            }
        } else {
            result.push(syll.clone());
        }
    }
    result
}

/// Returns `true` if `s` is some cmavo (jboraku each with at most 1 hard
/// consonant).
pub fn is_some_cmavo(s: &str) -> bool {
    syllabify(s).is_ok_and(|y| {
        jborakufy(&y).iter().all(|r| r.chars().filter(|c| is_hard_consonant(*c)).count() <= 1)
    })
}

/// Tests.
mod tests {
    #![cfg(test)]
    use super::*;

    macro_rules! ok {
        (mark_glides; $in:literal => $out:literal) => {
            assert_eq!(mark_glides($in), Ok($out.to_string()));
        };
        ($f:ident; $in:literal => $($out:literal),+) => {
            assert_eq!($f($in), Ok([$($out),+].iter().map(ToString::to_string).collect_vec()));
        };
    }
    macro_rules! err {
        ($f:ident; $in:literal) => {
            let res = $f($in);
            assert!(res.is_err(), "{res:?}");
        };
    }

    #[test]
    fn t_markglides_plukauaii_ok() {
        ok!(mark_glides; "plukauaii" => "plukawaqi");
    }
    #[test]
    fn t_markglides_12u_ok() {
        ok!(mark_glides; "uuuuuuuuuuuu" => "wuwuwuwuwuwu");
    }
    #[test]
    fn t_markglides_13u_ok() {
        ok!(mark_glides; "uuuuuuuuuuuuu" => "uwuwuwuwuwuwu");
    }
    #[test]
    fn t_markglides_auia_ok() {
        ok!(mark_glides; "auia" => "aŭqa");
    }
    #[test]
    fn t_markglides_aiia_err() {
        err!(mark_glides; "aiia");
    }
    #[test]
    fn t_markglides_eu_err() {
        err!(mark_glides; "eu");
    }
    #[test]
    fn t_markglides_eua_ok() {
        ok!(mark_glides; "eua" => "ewa");
    }

    #[test]
    fn t_syllabify_latkerlo_ok() {
        ok!(syllabify; "latkerlo" => "lat", "ker", "lo");
    }
    #[test]
    fn t_syllabify_sakprtlfmsngeha_ok() {
        ok!(syllabify; "sakprtlfmsnge'a" => "sak", "pr", "tl", "fm", "sn", "ge", "'a");
    }
    #[test]
    fn t_syllabify_glek_ok() {
        ok!(syllabify; "glek" => "glek");
    }
    #[test]
    fn t_syllabify_apba_err() {
        err!(syllabify; "apba");
    }
    #[test]
    fn t_syllabify_apqa_err() {
        err!(syllabify; "apqa");
    }
    #[test]
    fn t_syllabify_apyb_ok() {
        ok!(syllabify; "apyb" => "a", "pyb");
    }
    #[test]
    fn t_syllabify_apb_err() {
        err!(syllabify; "apb");
    }
    #[test]
    fn t_syllabify_aplbra_ok() {
        ok!(syllabify; "aplbra" => "a", "pl", "bra");
    }
    #[test]
    fn t_syllabify_an_ok() {
        ok!(syllabify; "an" => "an");
    }
    #[test]
    fn t_syllabify_ant_err() {
        err!(syllabify; "ant"); // syllables can end in at most 1 consonant
    }
    #[test]
    fn t_syllabify_antka_err() {
        err!(syllabify; "antka"); // ant,ka and an,tka are both illegal
    }
    #[test]
    fn t_syllabify_cipnrstrigi_ok() {
        ok!(syllabify; "cipnrstrigi" => "cip", "nr", "stri", "gi");
    }
    #[test]
    fn t_syllabify_apcnli_ok() {
        ok!(syllabify; "apcnli" => "ap", "cn", "li");
    }
}
