//! Rafsi `lists`, `shapes`, and `custom`ization.

#![allow(clippy::multiple_inherent_impl, reason = "rafste presets")]

pub mod custom;
pub mod lists;
pub mod shapes;

pub use custom::Rafste;
pub(crate) use shapes::classify_rafsi;
pub use shapes::{RafsiShape, is_one_cmavo};
