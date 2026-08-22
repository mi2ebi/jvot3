//! Constructors for syllables.

use std::fmt::{self, Display, Formatter};

use crate::{
    jvofli::{
        Jvofli::{self, Invalid, StressOnUnstressable},
        What,
    },
    phonology::{
        add_stress_accent, could_be_glide, is_diphthong_chars, is_hard_consonant, is_hard_onset,
        is_sonorant, is_vowel, strip_stress_accent,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs, reason = "nostly obvious")]
/// An onset. It can either be empty, an apostrophe, an onglide, or a [hard
/// onset](`is_hard_onset`).
pub enum Onset {
    /// "Nothing" (really *denpa bu* = glottal stop).
    Empty,
    /// An apostrophe.
    H,
    Onglide(char),
    /// A hard consonant.
    Single(char),
    Pair([char; 2]),
    Triple([char; 3]),
}
use Onset::{Empty, H, Onglide, Pair, Single, Triple};

impl Onset {
    /// Tries to create an `Onset`.
    ///
    /// # Errors
    /// Returns [`Invalid`] with [`What::Onset`] if `s` is not a valid onset.
    pub fn new(s: &str) -> Result<Self, Jvofli> {
        if s.len() > 3 {
            return Err(Invalid { what: What::Onset, value: s.into() });
        }
        if s.is_empty() {
            return Ok(Empty);
        }
        if s == "'" {
            return Ok(H);
        }
        let mut chars = ['\0'; 3];
        let mut len = 0;
        for c in s.chars() {
            chars[len] = c;
            len += 1;
        }
        if len == 1 && could_be_glide(chars[0]) {
            return Ok(Onglide(chars[0]));
        }
        if !is_hard_onset(s) {
            return Err(Invalid { what: What::Onset, value: s.into() });
        }
        match chars {
            [a, '\0', '\0'] => Ok(Single(a)),
            [a, b, '\0'] => Ok(Pair([a, b])),
            [a, b, c] => Ok(Triple([a, b, c])),
        }
    }

    /// Returns the number of hard consonants in this onset.
    #[inline]
    #[must_use]
    pub const fn hard_consonant_count(&self) -> usize {
        match self {
            Single(_) => 1,
            Pair(_) => 2,
            Triple(_) => 3,
            _ => 0,
        }
    }

    /// Returns whether this onset is [`H`].
    #[inline]
    #[must_use]
    pub const fn is_h(&self) -> bool { matches!(self, H) }

    /// Returns whether this onset is [`Empty`].
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { matches!(self, Empty) }
}

impl Display for Onset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Empty => Ok(()),
            H => write!(f, "'"),
            Onglide(c) | Self::Single(c) => write!(f, "{c}"),
            Pair([a, b]) => write!(f, "{a}{b}"),
            Triple([a, b, c]) => write!(f, "{a}{b}{c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs, reason = "obvious")]
/// A syllable nucleus. It can either be a stressable monophthong (*a e i o u*),
/// a diphthong (*ai au ei oi*), a sonorant (*l m n r*), or the letter *y*.
pub enum Nucleus {
    StressableMonophthong { vowel: char, stressed: bool },
    Y,
    Diphthong { first: char, second: char, stressed: bool },
    Sonorant(char),
}
use Nucleus::{Diphthong, Sonorant, StressableMonophthong, Y};

impl Nucleus {
    /// Tries to parse a `Nucleus`.
    ///
    /// # Errors
    /// - [`Invalid`] with [`What::Nucleus`] if `s` is not a valid nucleus.
    /// - [`StressOnUnstressable`] if `s` has explicit stress on *y* or a
    ///   sonorant.
    pub fn new(s: &str) -> Result<Self, Jvofli> {
        let mut raw = s.chars();
        let Some(first) = raw.next() else {
            return Err(Invalid { what: What::Nucleus, value: s.into() });
        };
        let (first, stressed) = strip_stress_accent(first);
        let second = raw.next();
        if raw.next().is_some() {
            return Err(Invalid { what: What::Nucleus, value: s.into() });
        }
        match (first, second, stressed) {
            ('y', None, false) => Ok(Y),
            ('y', None, true) => Err(StressOnUnstressable(s.into())),
            (c, None, st) if is_vowel(c) => Ok(StressableMonophthong { vowel: c, stressed: st }),
            (c, None, false) if is_sonorant(c) => Ok(Sonorant(c)),
            (c, None, true) if is_sonorant(c) => Err(StressOnUnstressable(s.into())),
            (a, Some(b), st) if is_diphthong_chars(a, b) => {
                Ok(Diphthong { first: a, second: b, stressed: st })
            }
            _ => Err(Invalid { what: What::Nucleus, value: s.into() }),
        }
    }

    #[must_use]
    /// Returns whether this syllable is stressable, i.e. it's a
    /// [`StressableMonophthong`] or [`Diphthong`].
    pub const fn is_stressable(&self) -> bool { !matches!(self, Y | Sonorant(_)) }

    /// Marks this nucleus as stressed. Does nothing if `!self.is_stressable()`.
    pub const fn set_stressed(&mut self, value: bool) {
        match self {
            StressableMonophthong { stressed, .. } | Diphthong { stressed, .. } => {
                *stressed = value;
            }
            _ => {}
        }
    }

    #[must_use]
    /// Returns whether this nucleus is stressed.
    pub const fn is_stressed(&self) -> bool {
        match self {
            StressableMonophthong { stressed, .. } | Diphthong { stressed, .. } => *stressed,
            _ => false,
        }
    }

    #[must_use]
    /// Tries to return the offglide of a [`Diphthong`].
    pub const fn get_offglide(&self) -> Option<char> {
        match self {
            &Diphthong { second, .. } => Some(second),
            _ => None,
        }
    }

    #[must_use]
    /// Tries to return the character in a [`Sonorant`].
    pub const fn get_sonorant(&self) -> Option<char> {
        match self {
            Sonorant(c) => Some(*c),
            _ => None,
        }
    }

    #[must_use]
    /// Returns whether the nucleus is a [`Sonorant`].
    pub const fn is_consonantal(&self) -> bool { matches!(self, Sonorant(_)) }

    #[must_use]
    /// Returns whether the nucleus *isn't* a [`Sonorant`].
    pub const fn is_vocalic(&self) -> bool { !self.is_consonantal() }
}

impl Display for Nucleus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Y => write!(f, "y"),
            Sonorant(c) => write!(f, "{c}"),
            &StressableMonophthong { vowel, stressed: false } => write!(f, "{vowel}"),
            &StressableMonophthong { vowel, stressed: true } => {
                let accented = add_stress_accent(vowel).unwrap_or(vowel);
                write!(f, "{accented}")
            }
            &Diphthong { first, second, stressed: false } => write!(f, "{first}{second}"),
            &Diphthong { first, second, stressed: true } => {
                let accented = add_stress_accent(first).unwrap_or(first);
                write!(f, "{accented}{second}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A coda, containing one hard consonant.
pub struct Coda(pub char);

impl Coda {
    #[must_use]
    /// Tries to parse a `Coda`.
    ///
    /// Unlike `Onset` and `Nucleus`, this doesn't ever return an error, instead
    /// returning `None` if it's invalid.
    pub const fn new(c: char) -> Option<Self> {
        if is_hard_consonant(c) { Some(Self(c)) } else { None }
    }

    #[must_use]
    /// Returns the character inside.
    pub const fn it(&self) -> char { self.0 }
}

impl Display for Coda {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs, reason = "obvious")]
/// A parsed syllable.
pub struct Syllable {
    pub onset: Onset,
    pub nucleus: Nucleus,
    pub coda: Option<Coda>,
}

impl Syllable {
    #[must_use]
    /// Returns whether the nucleus is a [`Sonorant`].
    pub const fn is_consonantal(&self) -> bool { self.nucleus.is_consonantal() }

    /// Counts hard consonants in this syllable.
    #[inline]
    #[must_use]
    pub const fn hard_consonant_count(&self) -> usize {
        self.onset.hard_consonant_count()
            + self.is_consonantal() as usize
            + self.coda.is_some() as usize
    }

    /// Returns whether this syllable's onset is [`H`].
    #[inline]
    #[must_use]
    pub const fn has_h_onset(&self) -> bool { matches!(self.onset, H) }

    /// Returns whether this syllable's onset is [`Empty`].
    #[inline]
    #[must_use]
    pub const fn has_empty_onset(&self) -> bool { matches!(self.onset, Empty) }

    /// Returns whether this syllable is a monosyllabic cmavo.
    #[inline]
    pub(crate) const fn could_start_cmavo(&self) -> bool {
        matches!(self.onset, Empty | Onglide(_) | Single(_))
            && self.coda.is_none()
            && self.nucleus.is_vocalic()
    }

    /// Returns whether this syllable could continue a cmavo that's already
    /// started.
    #[inline]
    pub(crate) const fn could_continue_cmavo(&self) -> bool {
        self.has_h_onset() && self.coda.is_none() && self.nucleus.is_vocalic()
    }
}

impl Display for Syllable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.onset, self.nucleus)?;
        if let Some(c) = &self.coda {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}
