use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

/// An error kind.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Jvoflikle {
    Jboraku,
}

/// An error.
#[derive(PartialEq, Eq, Clone)]
pub struct Jvofli {
    kind: Jvoflikle,
    message: String,
}

/// Constructs a `Jvofli`.
#[macro_export]
macro_rules! fli {
    ($k: expr, $($f: expr),*) => { Jvofli::new($k, format!($($f),*)) };
}
/// Constructs and returns a `Jvofli`.
#[macro_export]
macro_rules! flip {
    ($k: expr, $($f: expr),*) => { return Err(fli!($k, $($f),*)) };
}

impl Jvofli {
    /// Constructs a `Jvofli`. It's recommended to use one of the macros
    /// [`fli!`] or [`flip!`] instead.
    pub const fn new(kind: Jvoflikle, msg: String) -> Self {
        Self { kind, message: msg }
    }
}
impl Debug for Jvofli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}(\"{}\")", self.kind, self.message)
    }
}
impl Display for Jvofli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for Jvofli {}
