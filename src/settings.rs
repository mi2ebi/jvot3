use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// Controls the allowed set of lujvo hyphens.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HyphenSetting {
    /// *r* and *n* behave as in CLL.
    Standard,
    /// *y*-hyphens are allowed in place of *r* and *n*.
    AllowY,
    /// *y*-hyphens are required; using *r* or *n* hyphens creates a zi'evla.
    ForceY,
}

/// Controls the minimum consonant requirement.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConsonantSetting {
    /// Brivla must contain a consonant cluster.
    Cluster,
    /// Brivla must contain two consonants even if they're not adjacent.
    TwoCons,
    /// Brivla must contain at least one consonant.
    OneCons,
}

use ConsonantSetting::{Cluster, OneCons, TwoCons};
use HyphenSetting::{AllowY, ForceY, Standard};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
pub struct Settings {
    /// Whether the lujvo should end in a consonant. This only affects
    /// generating lujvo, and has no effect when decomposing them.
    pub generate_cmevla: bool,
    /// What hyphens to allow.
    pub hyphens: HyphenSetting,
    /// Minimum consonant requirements.
    pub minimum_consonants: ConsonantSetting,
    /// Allows any cmavo not containing *y* to be a rafsi. This requires adding
    /// a glottal stop after every cmavo ending in *y* rather than just *Cy*
    /// cmavo.
    pub arbitrary_cmavo_rafsi: bool,
    /// Whether *q* and *w* are treated as consonants. Together with
    /// `arbitrary_cmavo_rafsi` and `minimum_consonants` this can produce lujvo
    /// with zero hard consonants, e.g. *qa'ywa*.
    pub onglides_are_brivla_consonants: bool,
    /// Whether *mz* is considered a valid consonant cluster.
    pub allow_mz: bool,
}

impl Display for Settings {
    /// Displays a `Settings`. `crgz` indicate `generate_cmevla`,
    /// `arbitrary_cmavo_rafsi`, `onglides_are_brivla_consonants`, and
    /// `allow_mz` respectively. `AF` and `21` select [`HyphenSetting`] and
    /// [`ConsonantSetting`]. In particular, [`CLL`](Self::CLL) produces `x` as
    /// it would otherwise result in the empty string.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut s = String::new();
        if self.generate_cmevla {
            s.push('c');
        }
        match self.hyphens {
            Standard => {}
            AllowY => s.push('A'),
            ForceY => s.push('F'),
        }
        match self.minimum_consonants {
            Cluster => {}
            TwoCons => s.push('2'),
            OneCons => s.push('1'),
        }
        if self.arbitrary_cmavo_rafsi {
            s.push('r');
        }
        if self.onglides_are_brivla_consonants {
            s.push('g');
        }
        if self.allow_mz {
            s.push('z');
        }
        if s.is_empty() { write!(f, "x") } else { write!(f, "{s}") }
    }
}

#[derive(Debug)]
pub struct SettingsErr;
impl FromStr for Settings {
    type Err = SettingsErr;
    /// Tries to return a `Settings` that `Display`s as the input. `S` can be
    /// used for `HyphenSetting::Standard`, `C` can be used for
    /// `ConsonantSetting::Cluster`, and `x` by itself or the empty string
    /// result in [`Settings::CLL`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        if s == b"x" {
            return Ok(Self::CLL);
        }
        if b"crgz".iter().any(|x| s.iter().filter(|c| *c == x).nth(1).is_some())
            || s.iter().filter(|c| matches!(c, b'S' | b'A' | b'F')).nth(1).is_some()
            || s.iter().filter(|c| matches!(c, b'C' | b'2' | b'1')).nth(1).is_some()
            || s.iter().any(|c| !Self::is_settings_char(*c as char) || *c == b'x')
        {
            return Err(SettingsErr);
        }
        Ok(Self {
            generate_cmevla: s.contains(&b'c'),
            hyphens: if s.contains(&b'A') {
                AllowY
            } else if s.contains(&b'F') {
                ForceY
            } else {
                Standard
            },
            minimum_consonants: if s.contains(&b'2') {
                TwoCons
            } else if s.contains(&b'1') {
                OneCons
            } else {
                Cluster
            },
            arbitrary_cmavo_rafsi: s.contains(&b'r'),
            onglides_are_brivla_consonants: s.contains(&b'g'),
            allow_mz: s.contains(&b'z'),
        })
    }
}

impl Settings {
    /// Settings that are as close as possible to the CLL. Putting zi'evla in
    /// lujvo at all is still allowed.
    pub const CLL: Self = Self {
        generate_cmevla: false,
        hyphens: Standard,
        minimum_consonants: Cluster,
        arbitrary_cmavo_rafsi: false,
        onglides_are_brivla_consonants: false,
        allow_mz: false,
    };
    /// Settings that permit as many lujvo as possible (`A1rg`).
    pub const PERMISSIVE: Self = Self {
        hyphens: AllowY,
        minimum_consonants: OneCons,
        arbitrary_cmavo_rafsi: true,
        onglides_are_brivla_consonants: true,
        ..Self::CLL
    };
    const fn is_settings_char(c: char) -> bool {
        matches!(c, 'x' | 'c' | 'S' | 'A' | 'F' | 'C' | '2' | '1' | 'r' | 'g' | 'z')
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
                'g' => self.onglides_are_brivla_consonants ^= true,
                'z' => self.allow_mz ^= true,
                'A' => toggle!(hyphens, AllowY),
                'F' => toggle!(hyphens, ForceY),
                'S' => self.hyphens = Standard,
                '1' => toggle!(minimum_consonants, OneCons),
                '2' => toggle!(minimum_consonants, TwoCons),
                'C' => self.minimum_consonants = Cluster,
                _ => return None,
            }
        }
        Some(())
    }
}
