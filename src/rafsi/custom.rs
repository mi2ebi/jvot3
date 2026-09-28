//! Customization of rafsi lists.

use std::collections::HashMap;

use itertools::Itertools as _;

use crate::{
    jvofli::Jvofli::{self, LongRafsiAssignment, RafsiShapeTaken},
    rafsi::{
        Shape::{ArbitraryCmavo, Complete, Truncated},
        classify_rafsi,
    },
    settings::Settings,
    words::{Cmavo, Gismu, Zihevla},
};

#[derive(Debug)]
#[allow(missing_docs, reason = "obvious")]
/// A rafsi list.
pub struct Rafste {
    by_rafsi: HashMap<String, String>,
    by_word: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
#[allow(missing_docs, reason = "wrappers")]
/// Wrapper type for things that can have rafsi.
pub enum Selrafsi {
    Gismu(Gismu),
    Cmavo(Cmavo),
    Zihevla(Zihevla),
}

impl Rafste {
    #[must_use]
    /// Makes an empty rafsi list.
    pub fn empty() -> Self { Self { by_rafsi: HashMap::new(), by_word: HashMap::new() } }

    /// Adds a word without giving it any rafsi.
    pub fn add_word(&mut self, word: &str) { self.by_word.entry(word.to_string()).or_default(); }

    /// Tries to assign `rafsi` to `word`.
    ///
    /// # Errors
    ///
    /// If `rafsi` is a long rafsi, or `word` already has a rafsi of the same
    /// shape.
    pub fn assign(
        &mut self,
        rafsi: &str,
        word: &str,
        settings: Settings,
    ) -> Result<Option<String>, Jvofli> {
        Self::check_against(self.marafsi(word).unwrap_or(&[]), rafsi, word, settings)?;
        Ok(self.assign_unchecked(rafsi, word))
    }

    pub(crate) fn assign_unchecked(&mut self, rafsi: &str, word: &str) -> Option<String> {
        let old_word = self.by_rafsi.remove(rafsi);
        if let Some(ref old_word) = old_word {
            let Some(old_rafsi) = self.by_word.get_mut(old_word) else {
                unreachable!("[Rafste::assign_unchecked] both directions of rafste should match")
            };
            old_rafsi.retain(|r| r != rafsi);
        }
        self.by_rafsi.insert(rafsi.to_string(), word.to_string());
        self.by_word.entry(word.to_string()).or_default().push(rafsi.to_string());
        old_word
    }

    /// Checks that `attempted` is a short rafsi and that none of the `earlier`
    /// rafsi of `word` share its shape.
    fn check_against(
        earlier: &[String],
        attempted: &str,
        word: &str,
        settings: Settings,
    ) -> Result<(), Jvofli> {
        let shape = classify_rafsi(attempted, None, settings);
        if matches!(shape, Complete | Truncated | ArbitraryCmavo) {
            return Err(LongRafsiAssignment {
                word: word.to_string(),
                shape,
                attempted: attempted.to_string(),
            });
        }
        if let Some(existing) =
            earlier.iter().find(|r| **r != attempted && classify_rafsi(r, None, settings) == shape)
        {
            return Err(RafsiShapeTaken {
                word: word.to_string(),
                shape,
                existing: existing.clone(),
                attempted: attempted.to_string(),
            });
        }
        Ok(())
    }

    pub(crate) fn from_words(words: &[(&'static str, &'static [&'static str])]) -> Self {
        let mut by_rafsi = HashMap::new();
        let mut by_word = HashMap::new();
        for (word, rafsi) in words {
            let rafsi = rafsi.iter().map(ToString::to_string).collect::<Vec<_>>();
            for r in &rafsi {
                let old_word = by_rafsi.insert(r.clone(), word.to_string());
                debug_assert!(old_word.is_none(), "rafsi {{{r}}} assigned to multiple words");
            }
            by_word.insert(word.to_string(), rafsi);
        }
        Self { by_rafsi, by_word }
    }

    /// Checks every word's rafsi against the same rules [`Rafste::assign`]
    /// enforces.
    ///
    /// # Errors
    /// All violations found in word order (and within a word, in insertion
    /// order).
    pub fn validate(&self, settings: Settings) -> Result<(), Vec<Jvofli>> {
        let errors: Vec<Jvofli> = self
            .by_word
            .iter()
            .sorted_by_key(|(word, _)| *word)
            .flat_map(|(word, rafsi)| {
                rafsi
                    .iter()
                    .enumerate()
                    .map(move |(i, r)| Self::check_against(&rafsi[.. i], r, word, settings))
            })
            .filter_map(Result::err)
            .collect();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Removes `rafsi` from any word it's assigned to.
    pub fn remove(&mut self, rafsi: &str) {
        let Some(word) = self.by_rafsi.remove(rafsi) else {
            return;
        };
        let Some(rs) = self.by_word.get_mut(&word) else {
            unreachable!("[Rafste::remove] both directions of rafste should match")
        };
        rs.retain(|r| r != rafsi);
    }

    #[must_use]
    /// Gets the word `rafsi` is assigned to, if it exists.
    pub fn rafsima(&self, rafsi: &str) -> Option<&String> { self.by_rafsi.get(rafsi) }

    /// Tries to get a list of the rafsi `word` has.
    pub fn marafsi(&self, word: &str) -> Option<&[String]> {
        self.by_word.get(word).map(Vec::as_slice)
    }
}
