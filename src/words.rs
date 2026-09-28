//! Types for words.

use crate::{
    jvofli::{
        Jvofli::{self, Invalid},
        What,
    },
    phonology::{
        is_hard_consonant, is_initial, is_stressable_vowel, is_stressed, is_valid, strip_all_stress,
    },
    rafsi::is_one_cmavo,
    settings::Settings,
};

#[derive(Debug, Clone)]
/// A gismu.
pub struct Gismu(String);

impl Gismu {
    /// Tries to construct a `Gismu`.
    ///
    /// # Errors
    /// [`Invalid`] with [`What::Gismu`] if `s` isn't CVCCV or CCVCV shaped.
    #[allow(clippy::many_single_char_names, reason = "letters")]
    pub fn new(text: &str, settings: Settings) -> Result<Self, Jvofli> {
        let chars = text.chars().collect::<Vec<_>>();
        let text = strip_all_stress(text);
        let shaped = match chars[..] {
            [a, b, c, d, e] => {
                is_hard_consonant(a)
                    && (is_hard_consonant(b)
                        && is_stressable_vowel(c)
                        && is_initial(a, b, settings)
                        || is_stressable_vowel(b)
                            && is_hard_consonant(c)
                            && is_valid(c, d, settings))
                    && is_hard_consonant(d)
                    && is_stressable_vowel(e)
                    && !is_stressed(e)
            }
            _ => false,
        };
        if !shaped {
            return Err(Invalid { what: What::Gismu, value: text });
        }
        Ok(Self(text))
    }

    #[must_use]
    /// Returns the gismu text.
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone)]
/// A cmavo.
pub struct Cmavo(String);

impl Cmavo {
    /// Tries to construct a `Cmavo`.
    ///
    /// # Errors
    /// [`Invalid`] with [`What::Cmavo`] if `s` isn't a single cmavo.
    pub fn new(s: &str) -> Result<Self, Jvofli> {
        let text = strip_all_stress(s);
        if !is_one_cmavo(&text) {
            return Err(Invalid { what: What::Cmavo, value: text });
        }
        Ok(Self(text))
    }

    #[must_use]
    /// Returns the cmavo text.
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone)]
/// A zi'evla. These can only be constructed by `Brivla::analyze`.
pub struct Zihevla(String);

impl Zihevla {
    #[must_use]
    /// Returns the zi'evla text.
    pub fn as_str(&self) -> &str { &self.0 }
}
