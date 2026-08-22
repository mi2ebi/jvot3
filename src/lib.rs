//! MSRV: `1.88.0`.

#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    missing_debug_implementations,
    unreachable_pub,
    clippy::must_use_candidate,
    // clippy::unreachable,
    clippy::print_stderr
)]

pub mod jvofli;
pub mod phonology;
// pub mod rafsi;
pub mod settings;
pub mod syllables;
#[cfg(test)] mod tests;
pub mod units;
