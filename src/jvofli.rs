use std::fmt::{self, Debug, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Jvoflikle {
    Jboraku,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Jvofli {
    kind: Jvoflikle,
    message: String,
}
impl Jvofli {
    pub const fn new(kind: Jvoflikle, msg: String) -> Self { Self { kind, message: msg } }
}
impl Debug for Jvofli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}(\"{}\")", self.kind, self.message)
    }
}
impl Display for Jvofli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.message) }
}

#[macro_export]
macro_rules! fli {
    ($k: ident, $($f: expr),*) => { Jvofli::new($k, format!($($f),*)) };
}

#[macro_export]
macro_rules! flip {
    ($k:ident, $($f: expr),*) => { return Err(fli!($k, $($f),*)) };
}
