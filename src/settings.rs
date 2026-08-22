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

use HyphenSetting::{AllowY, ForceY, Standard};

#[allow(clippy::struct_excessive_bools, reason = "there isn't a Settings::new()")]
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
    /// Whether slinku'i are valid words. If `true` it considers e.g.
    /// *paslinku'i* to be a tosmabru.
    pub no_slinkuhi: bool,
}

impl Settings {
    /// Settings that are as close as possible to the CLL. Putting zi'evla in
    /// lujvo at all is still allowed.
    pub const CLL: Self = Self {
        generate_cmevla: false,
        hyphens: Standard,
        arbitrary_cmavo_rafsi: false,
        allow_mz: false,
        no_slinkuhi: false,
    };
    /// Settings that permit as many lujvo as possible (`Arz`).
    pub const PERMISSIVE: Self =
        Self { hyphens: AllowY, arbitrary_cmavo_rafsi: true, allow_mz: true, ..Self::CLL };

    const fn is_settings_char(c: char) -> bool {
        matches!(c, 'x' | 'c' | 'S' | 'A' | 'F' | 'r' | 'z' | 'n')
    }

    /// Modifies `self` by toggling each character in `flags`.
    pub fn apply_flags(&mut self, flags: &str) -> Option<()> {
        macro_rules! toggle {
            ($field:ident, $on:ident) => {
                self.$field = if self.$field == $on { Settings::CLL.$field } else { $on }
            };
        }
        if flags.chars().any(|c| !Self::is_settings_char(c)) {
            return None;
        }
        for f in flags.chars() {
            match f {
                'x' => *self = Self::CLL,
                'c' => self.generate_cmevla ^= true,
                'r' => self.arbitrary_cmavo_rafsi ^= true,
                'z' => self.allow_mz ^= true,
                'n' => self.no_slinkuhi ^= true,
                'A' => toggle!(hyphens, AllowY),
                'F' => toggle!(hyphens, ForceY),
                'S' => self.hyphens = Standard,
                _ => return None,
            }
        }
        Some(())
    }
}

/// Constructs a new `Settings` from an existing one, but with the fields not
/// listed replaced by their values in [`Settings::CLL`].
#[macro_export]
macro_rules! extract_settings {
    ($settings:expr; $($field:ident),+) => {
        Settings { $($field: ($settings).$field),+, ..Settings::CLL }
    }
}
