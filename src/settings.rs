//! Customization options.

/// Controls the allowed set of lujvo hyphens.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum HyphenSetting {
    /// *r* and *n* hyphens behave as in CLL.
    Standard,
    /// *y* hyphens are allowed in place of *r* and *n*.
    AllowY,
    /// *y* hyphens are required; using *r* or *n* hyphens creates a zi'evla.
    ForceY,
}

use HyphenSetting::{AllowY, Standard};

#[expect(clippy::struct_excessive_bools, reason = "there isn't a Settings::new()")]
#[derive(Clone, Copy, Debug)]
/// The settings!
pub struct Settings {
    /// Whether the lujvo should end in a consonant. This only affects
    /// generating lujvo, and has no effect when decomposing them.
    pub generate_cmevla: bool,
    /// What hyphens to allow.
    pub hyphens: HyphenSetting,
    /// Whether any cmavo not containing *y* may be a rafsi. This requires
    /// adding a glottal stop after every cmavo ending in *y* rather than
    /// just *Cy* cmavo.
    pub arbitrary_cmavo_rafsi: bool,
    /// Whether *mz* is considered a valid consonant cluster.
    pub allow_mz: bool,
    /// Whether slinku'i are valid words. If so e.g. *paslinku'i* is considered
    /// a tosmabru.
    pub no_slinkuhi: bool,
}

impl Settings {
    /// Settings that are as close as possible to CLL. Putting zi'evla in lujvo
    /// at all is still allowed.
    pub const CLL: Self = Self {
        generate_cmevla: false,
        hyphens: Standard,
        arbitrary_cmavo_rafsi: false,
        allow_mz: false,
        no_slinkuhi: false,
    };
    /// Settings that permit as many lujvo as possible.
    pub const PERMISSIVE: Self =
        Self { hyphens: AllowY, arbitrary_cmavo_rafsi: true, allow_mz: true, ..Self::CLL };
}

/// Constructs a new `Settings` from an existing one, but with the fields not
/// listed replaced by their values in [`Settings::CLL`].
#[macro_export]
macro_rules! extract_settings {
    ($settings:expr; $($field:ident),+) => {
        Settings { $($field: ($settings).$field),+, ..Settings::CLL }
    }
}
