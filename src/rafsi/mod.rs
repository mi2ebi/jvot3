//! Rafsi `lists`, `shapes`, and `custom`ization.

#![allow(clippy::multiple_inherent_impl, reason = "rafste presets")]

pub mod custom;
pub mod lists;
pub mod shapes;

pub use custom::Rafste;
pub use shapes::is_one_cmavo;
pub(crate) use shapes::{RafsiShape, classify_rafsi};
