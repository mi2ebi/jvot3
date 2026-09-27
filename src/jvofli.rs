//! Error types.

use std::fmt::{self, Debug, Display};

use thiserror::Error;

use crate::rafsi::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs, reason = "obvious")]
#[non_exhaustive]
/// Answers questions of the form "invalid what?" etc.
pub enum What {
    Cluster,
    Cmavo,
    ConsonantRun,
    ConsonantalSyllable,
    Diphthong,
    Gismu,
    Nucleus,
    Onset,
}

impl Display for What {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::enum_glob_use, reason = "no silly named wildcards")] use What::*;
        match self {
            Cluster => write!(f, "cluster"),
            ConsonantRun => write!(f, "consonant run"),
            ConsonantalSyllable => write!(f, "consonantal syllable"),
            Diphthong => write!(f, "diphthong"),
            Nucleus => write!(f, "nucleus"),
            Onset => write!(f, "onset"),
            Cmavo => write!(f, "cmavo"),
            Gismu => write!(f, "gismu"),
        }
    }
}

/// An error.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[allow(missing_docs, reason = "obvious")]
#[non_exhaustive]
pub enum Jvofli {
    #[error("{{{value}}} is not a valid {what}")]
    Invalid { what: What, value: String },
    #[error("no unit split makes {{{0}}} a valid stress location")]
    InvalidStressPosition(String),
    #[error("{{{0}}} may not be doubled when representing a glide both times")]
    LongGlide(char),
    #[error("{{{0}}} is a long rafsi, which can't be assigned arbitrarily")]
    LongRafsiAssignment(String),
    #[error(
        "{{'}} must be between two vowels, but has {} before it and {} after it",
        before.as_ref().map_or_else(|| "nothing".into(), |c| format!("{{{c}}}")),
        after.as_ref().map_or_else(|| "nothing".into(), |c| format!("{{{c}}}")),
    )]
    MisplacedApostrophe { before: Option<char>, after: Option<char> },
    #[error("{{{0}}} isn't a lojban character")]
    NonLojbanCharacter(char),
    #[error("{{{0}}} is only one syllable")]
    NotEnoughSyllables(String),
    #[error("{{{0}}} as an onglide can't be adjacent to consonants")]
    OnglideInCluster(char),
    #[error("{{{word}}} already has a {shape} rafsi")]
    RafsiShapeTaken { word: String, shape: Shape },
    #[error("{{{0}}} is a slinku'i")]
    Slinkuhi(String),
    #[error("{{{0}}} isn't stressable")]
    Unstressable(String),
    #[error("{{{0}}} is at the start of a pre-brivla but it isn't stressable")]
    UnstressablePreBrivlaStart(String),
    #[error("{{{0}}} is at the end of a pre-brivla but it isn't stressable")]
    UnstressablePreBrivlaEnd(String),
}

pub(crate) fn invalid_from_pair(w: What, x: char, y: char) -> Jvofli {
    Jvofli::Invalid {
        what: w,
        value: {
            let mut str = String::with_capacity(x.len_utf8() + y.len_utf8());
            str.push(x);
            str.push(y);
            str
        },
    }
}
pub(crate) fn invalid_cluster_from_triple(x: char, y: char, z: char) -> Jvofli {
    Jvofli::Invalid {
        what: What::Cluster,
        value: {
            let mut str = String::with_capacity(x.len_utf8() + y.len_utf8() + z.len_utf8());
            str.push(x);
            str.push(y);
            str.push(z);
            str
        },
    }
}
