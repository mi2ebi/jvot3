//! Rafsi shapes, and ways to determine them.

use std::fmt::Display;

use crate::{
    phonology::{is_diphthong, is_hard_consonant, is_stressable_vowel},
    settings::Settings,
    syllables::Onset,
    units::{Unit, unitify},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A rafsi shape. The variant names are inspired by [Mati's better-rafsi-list
/// spreadsheet][it].
///
/// [it]: https://docs.google.com/spreadsheets/d/e/2PACX-1vRr_nD2khJ-JdMAP9qnmuXz6c_veVaypf6ExFtKdf4H95wug2dN21ReE9gTpBnywo1S0Kj2ocAEWMF2/pubhtml
pub enum Shape {
    /// A cmavo that isn't an `End` rafsi, for `arbitrary_cmavo_rafsi`. Unlike
    /// `End`, these can only be followed by *y* hyphens, not *r*/*n*.
    ArbitraryCmavo,
    /// A full brivla.
    Complete,
    /// A brivla without its final vowel. For gismu, these are "4-letter" rafsi.
    Truncated,
    /// CVC. Depending on what follows, we might need to add a *y* hyphen after.
    Prefix,
    /// CCV. This is "nice" because we never need to add a hyphen after.
    Nice,
    /// CF and CV'V. If they're at the start of a lujvo, we probably need to
    /// insert a hyphen after. (Which hyphen specifically is controlled by
    /// [`Settings`].)
    End {
        /// Whether there's an apostrophe.
        h: bool,
    },
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ArbitraryCmavo => "cmavo",
            Complete => "complete-brivla",
            Truncated => "truncated-brivla",
            Prefix => "prefix (CVC)",
            Nice => "nice (CCV)",
            End { h } =>
                if *h {
                    "hiatus (CV'V)"
                } else {
                    "diphthong (CF)"
                },
        })
    }
}

use Shape::{ArbitraryCmavo, Complete, End, Nice, Prefix, Truncated};

#[must_use]
/// Returns whether `s` is exactly one cmavo.
pub fn is_one_cmavo(s: &str) -> bool {
    // choice of settings doesn't matter here
    let Ok(units) = unitify(s, Settings::CLL) else {
        return false;
    };
    let [Unit::Normal { syllables, pre_brivla_start: None }] = units.as_slice() else {
        return false;
    };
    syllables.iter().filter(|s| !s.onset.is_h()).nth(1).is_none()
}

pub(crate) fn classify_rafsi(text: &str, next_char: Option<char>, settings: Settings) -> Shape {
    debug_assert!(text.is_ascii(), "[classify_rafsi] {text} has non ascii");
    let elided = next_char == Some('y');
    let bytes = text.as_bytes();
    if let &[b0, b1, b2] = bytes {
        let c0 = b0 as char;
        let c1 = b1 as char;
        let c2 = b2 as char;
        if is_hard_consonant(c0) {
            if is_hard_consonant(c1) {
                if is_stressable_vowel(c2)
                    && matches!(Onset::new(&text[.. 2], settings), Ok(Onset::Pair(_)))
                {
                    return Nice;
                }
            } else if is_stressable_vowel(c1) {
                if is_hard_consonant(c2) {
                    return Prefix;
                }
                if is_diphthong(c1, c2) {
                    if elided {
                        return Truncated;
                    }
                    return End { h: false };
                }
            }
        }
    }
    if let &[b0, b1, b2, b3] = bytes {
        let c0 = b0 as char;
        let c1 = b1 as char;
        let c2 = b2 as char;
        let c3 = b3 as char;
        if is_hard_consonant(c0) && is_stressable_vowel(c1) && c2 == '\'' && is_stressable_vowel(c3)
        {
            return if elided { Truncated } else { End { h: true } };
        }
    }
    let Some(&last) = bytes.last() else {
        unreachable!("[classify_rafsi] requires nonempty text");
    };
    if elided && is_stressable_vowel(last as char) {
        return Truncated;
    }
    if text.chars().filter(|&c| is_hard_consonant(c)).nth(1).is_none() && is_one_cmavo(text) {
        return ArbitraryCmavo;
    }
    if !is_stressable_vowel(last as char) {
        return Truncated;
    }
    Complete
}

#[cfg(test)]
mod tests {
    use super::*;
    const CLL: Settings = Settings::CLL;
    #[test]
    fn nice() {
        assert_eq!(classify_rafsi("bla", None, CLL), Nice, "bla");
        assert_eq!(classify_rafsi("gri", None, CLL), Nice, "gri");
        assert_eq!(classify_rafsi("sfa", None, CLL), Nice, "sfa");
    }
    #[test]
    fn end_diphthong() {
        assert_eq!(classify_rafsi("tei", None, CLL), End { h: false }, "tei");
        assert_eq!(classify_rafsi("lau", None, CLL), End { h: false }, "lau");
    }
    #[test]
    fn end_diphthongh() {
        assert_eq!(classify_rafsi("tei", Some('\''), CLL), End { h: false }, "tei'");
        assert_eq!(classify_rafsi("lau", Some('\''), CLL), End { h: false }, "lau'");
    }
    #[test]
    fn truncated_cvgy() {
        // these should all fail downstream
        assert_eq!(classify_rafsi("tei", Some('y'), CLL), Truncated, "teiy");
        assert_eq!(classify_rafsi("lau", Some('y'), CLL), Truncated, "lauy");
        assert_eq!(classify_rafsi("peu", Some('y'), CLL), Truncated, "peuy");
        assert_eq!(classify_rafsi("iau", Some('y'), CLL), Truncated, "iauy");
        assert_eq!(classify_rafsi("le", Some('y'), CLL), Truncated, "ley");
    }
    #[test]
    fn end_hiatus() {
        assert_eq!(classify_rafsi("te'i", None, CLL), End { h: true }, "te'i");
        assert_eq!(classify_rafsi("la'u", None, CLL), End { h: true }, "la'u");
        assert_eq!(classify_rafsi("pe'u", None, CLL), End { h: true }, "pe'u");
    }
    #[test]
    fn prefix() {
        assert_eq!(classify_rafsi("bal", None, CLL), Prefix, "bal");
        assert_eq!(classify_rafsi("lan", None, CLL), Prefix, "lan");
    }
    #[test]
    fn truncated() {
        assert_eq!(classify_rafsi("anj", None, CLL), Truncated, "anj");
        assert_eq!(classify_rafsi("fi'ikc", None, CLL), Truncated, "fi'ikc");
        assert_eq!(classify_rafsi("gism", None, CLL), Truncated, "gism");
    }
    #[test]
    fn complete() {
        assert_eq!(classify_rafsi("anji", None, CLL), Complete, "anji");
        assert_eq!(classify_rafsi("fi'ikca", None, CLL), Complete, "fi'ikca");
        assert_eq!(classify_rafsi("gismu", None, CLL), Complete, "gismu");
    }
    #[test]
    fn brivla_diphthong() {
        assert_eq!(classify_rafsi("plukauai", None, CLL), Complete, "plauakai");
        assert_eq!(classify_rafsi("plukauai", Some('\''), CLL), Complete, "plukauai'");
        assert_eq!(classify_rafsi("plukauai", Some('y'), CLL), Truncated, "plukauaiy");
    }
    #[test]
    fn cmavo() {
        assert_eq!(classify_rafsi("mi", None, CLL), ArbitraryCmavo, "mi");
        assert_eq!(classify_rafsi("fi'i'e", None, CLL), ArbitraryCmavo, "fi'i'e");
        assert_eq!(classify_rafsi("te'y", None, CLL), ArbitraryCmavo, "te'y"); // should fail later
        assert_eq!(classify_rafsi("iau", None, CLL), ArbitraryCmavo, "iau");
        assert_eq!(classify_rafsi("iu'a", None, CLL), ArbitraryCmavo, "iu'a");
    }
}
