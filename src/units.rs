//! Types and parsers for units, strings of Lojban that match `cmavo*
//! prebrivla?`. (See the jargon docs for more details.)

use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    iter::Rev,
    str::CharIndices,
};

use itertools::Itertools as _;

use crate::{
    extract_settings,
    jvofli::{
        Jvofli::{
            self, Invalid, InvalidStressPosition, LongGlide, MisplacedApostrophe,
            NonLojbanCharacter, NotEnoughSyllables, OnglideInCluster, Slinkuhi,
            StressOnUnstressable, UnstressablePreBrivlaEnd, UnstressablePreBrivlaStart,
        },
        What, invalid_cluster_from_triple, invalid_from_pair,
    },
    phonology::{
        deannotate_glide, is_annotated_offglide, is_annotated_onglide, is_banned_triple_chars,
        is_consonant, is_diphthong_chars, is_hard_consonant, is_sonorant, is_valid_chars, is_vowel,
        strip_stress_accent,
    },
    settings::Settings,
    syllables::{
        Coda,
        Nucleus::{self, Sonorant, Y},
        Onset::{self, Empty, Onglide, Pair, Single},
        Syllable,
    },
};

// todo maybe split this (perhaps according to the `// - ... -` comments)

/// Mostly a list of Lojban syllables. Units are the result of taking some
/// Lojban text and splitting it at mandatory pauses and after brivla.
#[derive(Clone, PartialEq, Eq)]
pub enum Unit {
    /// A normal unit consists of a possibly empty sequence of cmavo followed by
    /// an optional pre-brivla.
    Normal {
        /// The list of syllables.
        syllables: VecDeque<Syllable>,
        /// The index in `syllables` that marks the start of the pre-brivla, if
        /// it's present; i.e. this is `Some(0)` if there are no cmavo and
        /// `None` if there is no pre-brivla.
        pre_brivla_start: Option<usize>,
    },
    /// Cmevla are not required to consist of syllables at all, so we don't
    /// bother trying to syllabify them.
    Cmevla(String),
}

impl Unit {
    /// Returns whether this unit is entirely cmavo.
    #[inline]
    #[must_use]
    pub const fn is_cmavo_only(&self) -> bool {
        matches!(self, Self::Normal { pre_brivla_start: None, .. })
    }

    /// Returns whether this unit is entirely a pre-brivla.
    #[inline]
    #[must_use]
    pub const fn is_pre_brivla_only(&self) -> bool {
        matches!(self, Self::Normal { pre_brivla_start: Some(0), .. })
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal { syllables, pre_brivla_start } => {
                for (i, s) in syllables.iter().enumerate() {
                    write!(
                        f,
                        "{}{}{s}",
                        if i > 0 { "," } else { "" },
                        if *pre_brivla_start == Some(i) { "|" } else { "" }
                    )?;
                }
                Ok(())
            }
            Self::Cmevla(c) => write!(f, ".{c}."),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{self:?}");
        s.retain(|c| is_consonant(c) || is_vowel(c));
        write!(f, "{s}")
    }
}

// - consonantal syllables -

/// Tries to package `chars` (rtl) into consonantal [`Syllable`]s (ltr).
fn pair_consonantal(chars: &[char]) -> Result<Vec<Syllable>, Jvofli> {
    let mut chars = chars.to_vec();
    chars.reverse();
    let (chunks, []) = chars.as_chunks::<2>() else {
        unreachable!("[pair_consonantal] odd number of chars");
    };
    #[expect(
        clippy::suspicious_operation_groupings,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17143"
    )]
    chunks
        .iter()
        .map(|&[a, b]| {
            if is_hard_consonant(a) && is_sonorant(b) && a != b {
                Ok(Syllable { onset: Single(a), nucleus: Sonorant(b), coda: None })
            } else {
                Err(invalid_from_pair(What::ConsonantalSyllable, a, b))
            }
        })
        .collect()
}

/// Tries to split `chars` (rtl) into an optional coda and maybe some
/// consonantal syllables (ltr).
fn split_into_coda_and_consonantal(
    chars: &[char],
    allow_coda: bool,
) -> Result<(Option<Coda>, Vec<Syllable>), Jvofli> {
    if chars.is_empty() {
        return Ok((None, vec![]));
    }
    if allow_coda
        && !chars.len().is_multiple_of(2)
        && let Some(&last) = chars.last()
        && let Some(coda) = Coda::new(last)
        && let Ok(pairs) = pair_consonantal(&chars[.. chars.len() - 1])
    {
        return Ok((Some(coda), pairs));
    }
    if chars.len().is_multiple_of(2)
        && let Ok(pairs) = pair_consonantal(chars)
    {
        return Ok((None, pairs));
    }
    Err(Invalid { what: What::ConsonantRun, value: chars.iter().rev().collect::<String>() })
}

// - cmavo detection -

/// Returns a list of, for each index `i`, the length of the maximal run of
/// cmavo continuation syllables starting at `i`, or 0 if `syllables[i]` doesn't
/// itself qualify.
fn cmavo_tail_lengths(syllables: &[Syllable]) -> Vec<usize> {
    let n = syllables.len();
    let mut out = vec![0_usize; n];
    for i in (0 .. n).rev() {
        if syllables[i].could_continue_cmavo() {
            out[i] = 1 + out.get(i + 1).copied().unwrap_or(0);
        }
    }
    out
}

/// Tries to consume a single cmavo starting at `syllables[i]` and return
/// `Some` index after it.
#[inline]
fn try_one_cmavo(syllables: &[Syllable], cmavo_tail_lens: &[usize], i: usize) -> Option<usize> {
    if i >= syllables.len() || !syllables[i].could_start_cmavo() {
        return None;
    }
    let tail = cmavo_tail_lens.get(i + 1).copied().unwrap_or(0);
    Some(i + 1 + tail)
}

/// Returns whether a cmavo may immediately follow a *y*-ending cmavo
/// without an intervening pause.
fn cmavo_permitted_after_y(
    syllables: &[Syllable],
    cmavo_tail_lens: &[usize],
    pos: usize,
    end: usize,
    settings: Settings,
) -> bool {
    let settings = extract_settings!(settings; arbitrary_cmavo_rafsi);
    debug_assert!(
        matches!(syllables[end - 1].nucleus, Y),
        "[ccpay] caller should check syllables[end-1] = {} has y nucleus",
        syllables[end - 1]
    );
    let is_cy_shaped =
        end - pos == 1 && matches!(syllables[pos].onset, Empty | Single(_) | Onglide(_));
    if !(settings.arbitrary_cmavo_rafsi || is_cy_shaped) {
        return true;
    }
    let Some(next_end) = try_one_cmavo(syllables, cmavo_tail_lens, end) else { return false };
    matches!(syllables[next_end - 1].nucleus, Y)
}

fn cmavo_boundaries_from(
    syllables: &[Syllable],
    cmavo_tail_lens: &[usize],
    settings: Settings,
    start: usize,
    len: usize,
) -> Vec<usize> {
    let mut boundaries = vec![0_usize];
    let mut pos = start;
    let mut cached_next: Option<Option<usize>> = None;
    while pos < start + len {
        let end =
            cached_next.take().unwrap_or_else(|| try_one_cmavo(syllables, cmavo_tail_lens, pos));
        let Some(end) = end else { break };
        if end < start + len && matches!(syllables[end - 1].nucleus, Y) {
            let next_end = try_one_cmavo(syllables, cmavo_tail_lens, end);
            if !cmavo_permitted_after_y(syllables, cmavo_tail_lens, pos, end, settings) {
                break;
            }
            cached_next = Some(next_end);
        }
        pos = end;
        boundaries.push(pos - start);
    }
    boundaries
}

/// Returns whether `syllables` is a valid cmavo sequence.
fn is_cmavo_sequence(
    syllables: &[Syllable],
    cmavo_tail_lens: &[usize],
    settings: Settings,
) -> bool {
    if syllables.is_empty() {
        return true;
    }
    let boundaries =
        cmavo_boundaries_from(syllables, cmavo_tail_lens, settings, 0, syllables.len());
    boundaries.last().copied() == Some(syllables.len())
}

/// Returns, for each index `i`, the start of the cmavo `syllables[i]`
/// belongs to, if any.
fn cmavo_starts(syllables: &[Syllable]) -> Vec<usize> {
    let mut start = Vec::with_capacity(syllables.len());
    for i in 0 .. syllables.len() {
        start.push(if i > 0 && syllables[i].could_continue_cmavo() { start[i - 1] } else { i });
    }
    start
}

// - precomputing stuff: hard consonants -

/// Returns prefix sums of hard consonant counts: `prefix[k]` is the total hard
/// consonants in `syllables[0..k]`.
fn hard_consonant_prefix_sums(syllables: &[Syllable]) -> Vec<usize> {
    let mut prefix = Vec::with_capacity(syllables.len() + 1);
    prefix.push(0);
    let mut running = 0;
    for s in syllables {
        running += s.hard_consonant_count();
        prefix.push(running);
    }
    prefix
}

// - precomputing stuff: stress -

/// Returns, for each index `i`, the largest `j <= i` where `syllables[j]` is
/// stressable, or `None` if there is no such `j`.
fn nearest_stressable_at_or_before(syllables: &[Syllable]) -> Vec<Option<usize>> {
    let mut out = Vec::with_capacity(syllables.len());
    let mut last = None;
    for (i, s) in syllables.iter().enumerate() {
        if s.nucleus.is_stressable() {
            last = Some(i);
        }
        out.push(last);
    }
    out
}

/// For each index `i`, returns the nearest stressable syllable strictly after
/// `i`, or `n` if none exists.
fn next_stressable_after(syllables: &[Syllable]) -> Vec<usize> {
    let n = syllables.len();
    let mut out = vec![n; n];
    for i in (0 .. n.saturating_sub(1)).rev() {
        out[i] = if syllables[i + 1].nucleus.is_stressable() { i + 1 } else { out[i + 1] };
    }
    out
}

// - precomputing stuff: brivla evidence -

/// Precomputes the leftmost brivla evidence target for every start position.
fn evidence_target_from(
    syllables: &[Syllable],
    cmavo_tail_lens: &[usize],
    settings: Settings,
) -> Vec<Option<usize>> {
    let n = syllables.len();
    let cmavo_starts = cmavo_starts(syllables);
    let nucleus_evidence = |i: usize| {
        matches!(syllables[i].nucleus, Sonorant(_))
            || matches!(syllables[i].nucleus, Y) && i + 1 < n && {
                let pos = cmavo_starts[i];
                let end = i + 1;
                !cmavo_permitted_after_y(syllables, cmavo_tail_lens, pos, end, settings)
            }
    };
    let periphery_evidence =
        |i: usize| syllables[i].coda.is_some() || syllables[i].onset.hard_consonant_count() >= 2;
    let passes_evidence = |i: usize| syllables[i].has_h_onset() || nucleus_evidence(i);
    let mut prev_blocker = vec![None; n];
    let mut last = None;
    for (i, item) in prev_blocker.iter_mut().enumerate().take(n) {
        *item = last;
        if !passes_evidence(i) {
            last = Some(i);
        }
    }
    let mut next_blocker = vec![None; n + 1];
    let mut next_periphery = vec![None; n + 1];
    let mut next_nucleus = vec![None; n + 1];
    for i in (0 .. n).rev() {
        next_blocker[i] = if passes_evidence(i) { next_blocker[i + 1] } else { Some(i) };
        next_periphery[i] = if periphery_evidence(i) { Some(i) } else { next_periphery[i + 1] };
        next_nucleus[i] = if nucleus_evidence(i) { Some(i) } else { next_nucleus[i + 1] };
    }
    (0 .. n)
        .map(|start| {
            let local_target =
                [next_periphery[start], next_nucleus[start]].into_iter().flatten().min();
            let remote_target = next_blocker[start].and_then(|b| next_nucleus[b + 1]).map(|i| {
                prev_blocker[i].unwrap_or_else(|| {
                    unreachable!("[evidence_target_from] a blocker >= `start` precedes `i`")
                })
            });
            match (local_target, remote_target) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, b) => b,
            }
        })
        .collect()
}

// - grouping syllables -

/// Tries to split `syllables` into multiple units using stress locations.
fn resolve_stress_and_split(
    syllables: &[Syllable],
    settings: Settings,
) -> Result<Vec<Unit>, Jvofli> {
    let n = syllables.len();
    if n == 0 {
        return Ok(vec![]);
    }
    // precompute a bunch of stuff
    let cmavo_tail_lens = cmavo_tail_lengths(syllables);
    let boundaries = cmavo_boundaries_from(syllables, &cmavo_tail_lens, settings, 0, n);
    if boundaries.last().copied() == Some(n) {
        // early exit if only cmavo
        return Ok(vec![Unit::Normal {
            syllables: syllables.to_vec().into(),
            pre_brivla_start: None,
        }]);
    }
    let nearest_abs = nearest_stressable_at_or_before(syllables);
    let nearest_rel = |start: usize, rel: usize| {
        nearest_abs
            .get(start + rel)
            .copied()
            .flatten()
            .and_then(|j| (j >= start).then(|| j - start))
    };
    let mut next_explicit_stress = vec![None; n + 1];
    for i in (0 .. n).rev() {
        next_explicit_stress[i] =
            if syllables[i].nucleus.is_stressed() { Some(i) } else { next_explicit_stress[i + 1] };
    }
    let hc_prefix = hard_consonant_prefix_sums(syllables);
    let hc_rel = |start: usize, rel: usize| hc_prefix[start + rel] - hc_prefix[start];
    let next_stressable = next_stressable_after(syllables);
    let evidence_targets = evidence_target_from(syllables, &cmavo_tail_lens, settings);
    // scan time
    let mut units = Vec::new();
    let mut start = 0;
    while start < n {
        let seg = &syllables[start ..];
        let seg_len = seg.len();
        let boundaries =
            cmavo_boundaries_from(syllables, &cmavo_tail_lens, settings, start, seg_len);
        if boundaries.last().copied() == Some(seg_len) {
            units.push(Unit::Normal { syllables: seg.to_vec().into(), pre_brivla_start: None });
            break;
        }
        let evidence_target = evidence_targets[start]
            .map(|abs| abs - start)
            .ok_or_else(|| unreachable!("[rs&s] there's gotta be brivla evidence here right?"))?;
        if seg_len < 2 {
            return Err(NotEnoughSyllables(seg[0].to_string()));
        }
        let default_start_search = boundaries.iter().copied().enumerate().rev().find(|&(_, l)| {
            l <= evidence_target
                && seg_len - l >= 2
                && !seg[l].has_h_onset()
                && seg[l].nucleus.is_stressable()
        });
        let Some((default_idx, default_start)) = default_start_search else {
            return Err(UnstressablePreBrivlaStart(seg[0].to_string()));
        };
        let is_ccvhv = seg_len - default_start == 2
            && matches!(seg[default_start].onset, Pair(_))
            && seg[default_start].coda.is_none()
            && seg[default_start + 1].has_h_onset()
            && seg[default_start + 1].coda.is_none();
        let default_start = if is_ccvhv {
            match default_idx.checked_sub(1) {
                Some(prev_idx) => boundaries[prev_idx],
                None => return Err(Slinkuhi(seg[default_start ..].iter().join(""))), /* even under noslinku'i! */
            }
        } else {
            default_start
        };
        if !seg[seg_len - 1].nucleus.is_stressable() {
            return Err(UnstressablePreBrivlaEnd(seg[seg_len - 1].to_string()));
        }
        let stress_idx = next_explicit_stress[start]
            .and_then(|abs| (abs < start + seg_len).then(|| abs - start));
        let natural_stress =
            nearest_rel(start, seg_len.saturating_sub(2)).filter(|&i| i >= default_start);
        let Some(stress_idx) = stress_idx else {
            let natural = natural_stress.unwrap_or_else(|| {
                unreachable!(
                    "[rs&s] `default_start` is only accepted by `default_start_search` if \
                     `seg[default_start].nucleus.is_stressable()`, and it's always `<= seg_len - \
                     2` (penult), so the globally-nearest stressable syllable at-or-before the \
                     penult must be `>= default_start`. The CCVHV branch below only ever moves \
                     `default_start` earlier, which can only loosen this. So `natural_stress` \
                     should always resolve."
                )
            });
            let mut syllables_vec: VecDeque<_> =
                syllables[start .. start + seg_len].to_vec().into();
            syllables_vec[natural].nucleus.set_stressed(true);
            units.push(Unit::Normal {
                syllables: syllables_vec,
                pre_brivla_start: Some(default_start),
            });
            break;
        };
        if natural_stress == Some(stress_idx) {
            units.push(Unit::Normal {
                syllables: seg.to_vec().into(),
                pre_brivla_start: Some(default_start),
            });
            break;
        }
        let stress_idx_abs = start + stress_idx;
        let r_hi = seg_len.min(next_stressable[stress_idx_abs] - start + 1);
        let r_lo = (stress_idx + 2).max(evidence_target + 1);
        let r_candidates: Vec<usize> = if r_lo <= r_hi {
            (r_lo ..= r_hi)
                .filter(|&r| {
                    seg[r - 1].coda.is_none()
                        && seg.get(r).is_none_or(|s| !s.has_h_onset())
                        && (stress_idx + 1 .. r).all(|j| !seg[j].nucleus.is_stressed())
                })
                .collect()
        } else {
            vec![]
        };
        let mut ptr = r_candidates.len();
        let cmavo_stress_err = None;
        let mut split_at = None;
        for (_, &l) in boundaries.iter().enumerate().rev() {
            if l > stress_idx || l > evidence_target {
                continue;
            }
            let threshold = hc_rel(start, l) + 2;
            while ptr > 0 && hc_rel(start, r_candidates[ptr - 1]) >= threshold {
                ptr -= 1;
            }
            if let Some(&r) = r_candidates.get(ptr) {
                split_at = Some((l, r));
                break;
            }
        }
        if let Some((l, r)) = split_at {
            if !seg[r - 1].nucleus.is_stressable() {
                return Err(UnstressablePreBrivlaEnd(seg[r - 1].to_string()));
            }
            units.push(Unit::Normal {
                syllables: seg[.. r].to_vec().into(),
                pre_brivla_start: Some(l),
            });
            start += r;
            continue;
        }
        if let Some(e) = cmavo_stress_err {
            return Err(e);
        }
        if stress_idx < default_start
            && is_cmavo_sequence(
                &seg[.. default_start],
                &cmavo_tail_lens[start .. start + default_start],
                settings,
            )
        {
            let mut rest = resolve_stress_and_split(&seg[default_start ..], settings)?;
            let Some(Unit::Normal { syllables, pre_brivla_start }) = rest.first_mut() else {
                unreachable!(
                    "[rs&s] resolve_stress_and_split never produces no units or any cmevla"
                )
            };
            for &s in seg[.. default_start].iter().rev() {
                syllables.push_front(s);
            }
            *pre_brivla_start = Some(pre_brivla_start.unwrap_or(0) + default_start);
            units.extend(rest);
            break;
        }
        return Err(InvalidStressPosition(seg[stress_idx].to_string()));
    }
    Ok(units)
}

// - merging units -

/// Tries to merge `l` into `r`. Returns `Err(r)` if it can't.
fn try_merging_units(l: &Unit, mut r: Unit, settings: Settings) -> Result<Unit, Unit> {
    let Unit::Normal { syllables: l_syl, pre_brivla_start: None } = l else {
        return Err(r);
    };
    let Unit::Normal { syllables: r_syl, pre_brivla_start: r_pbs } = &mut r else {
        return Err(r);
    };
    let Some(r_first) = r_syl.front().copied() else { return Err(r) };
    if r_first.has_empty_onset() {
        return Err(r);
    }
    let Some(l_last) = l_syl.back().copied() else { return Err(r) };
    if let (Some(off), Onglide(on)) = (l_last.nucleus.get_offglide(), r_first.onset)
        && off == on
    {
        return Err(r);
    }
    if matches!(l_last.nucleus, Y) {
        let l_vec: Vec<Syllable> = l_syl.iter().copied().collect();
        let start = cmavo_starts(&l_vec)[l_vec.len() - 1];
        let mut window: Vec<Syllable> = l_vec[start ..].to_vec();
        let end = window.len();
        window.extend(r_syl.iter().copied());
        let cmavo_tail_lens = cmavo_tail_lengths(&window);
        if !cmavo_permitted_after_y(&window, &cmavo_tail_lens, 0, end, settings) {
            return Err(r);
        }
    }
    if l_last.nucleus.is_stressed() && *r_pbs == Some(0) && r_first.coda.is_none() {
        return Err(r);
    }
    let l_len = l_syl.len();
    for syl in l_syl.iter().rev().copied() {
        r_syl.push_front(syl);
    }
    *r_pbs = r_pbs.map(|i| i + l_len);
    Ok(r)
}

// - cmevla -

/// Checks that `input` only contains Lojban letters, then annotates glides so
/// that *i* and *u* are unambiguous.
fn annotate_glides(input: &str) -> Result<String, Jvofli> {
    if let Some(bad) =
        input.chars().find(|&c| !(is_consonant(c) || is_vowel(c) || is_annotated_offglide(c)))
    {
        return Err(NonLojbanCharacter(bad));
    }
    if input.find(['i', 'u', 'í', 'ú']).is_none() {
        return Ok(input.to_string());
    }
    let mut chars = input.chars().collect_vec();
    for i in (0 .. chars.len()).rev() {
        let c = chars[i];
        let (base, stressed) = strip_stress_accent(c);
        if !matches!(base, 'i' | 'u') {
            continue;
        }
        let (on, off) = if base == 'i' { ('q', 'ĭ') } else { ('w', 'ŭ') };
        let next_is_vowel = chars.get(i + 1).is_some_and(|&n| is_vowel(n));
        if next_is_vowel {
            if stressed {
                return Err(StressOnUnstressable(c.to_string()));
            }
            chars[i] = on;
            continue;
        }
        let prev_is_diphthong_first = i != 0
            && chars
                .get(i - 1)
                .is_some_and(|&p| matches!(p, 'a' | 'á' | 'e' | 'é' | 'o' | 'ó' | 'y' | 'ý'));
        if prev_is_diphthong_first {
            if is_diphthong_chars(chars[i - 1], c) {
                if stressed {
                    return Err(StressOnUnstressable(c.to_string()));
                }
                chars[i] = off;
                if chars.get(i + 1) == Some(&on) {
                    return Err(LongGlide(base));
                }
            } else {
                return Err(invalid_from_pair(What::Diphthong, chars[i - 1], c));
            }
        }
    }
    Ok(chars.into_iter().collect())
}

/// Checks that `pg` (already known to end in a hard consonant) is a plausible
/// cmevla.
fn check_cmevla(pg: &str, settings: Settings) -> Result<(), Jvofli> {
    let annotated = annotate_glides(pg)?;
    let chars: Vec<char> = annotated.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        let before = i.checked_sub(1).map(|j| chars[j]);
        let after = chars.get(i + 1).copied();
        if c != '\'' && after == Some(c) {
            return Err(invalid_from_pair(
                if is_consonant(c) { What::Cluster } else { What::Nucleus },
                c,
                c,
            ));
        }
        if is_hard_consonant(c)
            && let Some(n) = after
            && is_hard_consonant(n)
            && !is_valid_chars(c, n, settings)
        {
            return Err(invalid_from_pair(What::Cluster, c, n));
        }
        if is_vowel(c)
            && let Some(n) = after
            && is_vowel(n)
        {
            return Err(invalid_from_pair(What::Nucleus, c, n));
        }
        match c {
            '\'' => {
                let before_ok = before.is_some_and(|p| is_vowel(p) || is_annotated_offglide(p));
                let after_ok = after.is_some_and(is_vowel);
                if !(before_ok && after_ok) {
                    return Err(MisplacedApostrophe { before, after });
                }
            }
            c if is_annotated_onglide(c)
                && (before.is_some_and(is_consonant) || after.is_some_and(is_consonant)) =>
            {
                return Err(OnglideInCluster(c));
            }
            _ => {}
        }
    }
    Ok(())
}

// - state -

struct Unitifier<'a> {
    input: &'a str,
    /// Remaining chars of `input`.
    chars: Rev<CharIndices<'a>>,
    /// Completed units, pushed rtl.
    units: Vec<Unit>,
    settings: Settings,
    pg_end: usize,
    in_cmevla: bool,
    consonantal_syllable_buffer: Vec<Syllable>,
    // all of these vecs are rtl:
    pending_unit: Vec<Syllable>,
    pending_nucleus: Vec<char>,
    pending_coda: Option<Coda>,
    pending_consonants: Vec<char>,
}

// - actually doing things -

impl<'a> Unitifier<'a> {
    // - begin -

    fn new(input: &'a str, settings: Settings) -> Self {
        Self {
            input,
            chars: input.char_indices().rev(),
            units: vec![],
            settings,
            pg_end: input.len(),
            in_cmevla: false,
            consonantal_syllable_buffer: Vec::with_capacity(8),
            pending_unit: vec![],
            pending_nucleus: vec![],
            pending_coda: None,
            pending_consonants: vec![],
        }
    }

    const fn pg_is_fresh(&self) -> bool {
        self.pending_unit.is_empty()
            && self.pending_nucleus.is_empty()
            && self.pending_consonants.is_empty()
    }

    fn reset_pending(&mut self) {
        self.pending_unit.clear();
        self.pending_nucleus.clear();
        self.pending_coda = None;
        self.pending_consonants.clear();
        self.in_cmevla = false;
    }

    // - consonants -

    /// Accumulates a consonant and sets `in_cmevla` if the `pg_is_fresh`.
    fn push_consonant(&mut self, c: char) {
        if self.pg_is_fresh() {
            self.in_cmevla = true;
        }
        self.pending_consonants.push(c);
    }

    /// Tries to split `pending_consonants` into some combination of onset,
    /// coda, and consonantal syllables. The onset and coda are returned and
    /// the consonantal syllables are written to
    /// `self.consonantal_syllable_buffer`.
    fn resolve_pending_consonants(
        &mut self,
        may_try_coda: bool,
        left_neighbor: Option<char>,
    ) -> Result<(Onset, Option<Coda>), Jvofli> {
        if self.pending_consonants.len() > 1
            && let Some(pos) = self.pending_consonants.iter().position(|&c| c == '\'')
        {
            let before = self.pending_consonants.get(pos + 1).copied().or(left_neighbor);
            let after = if pos > 0 {
                self.pending_consonants.get(pos - 1).copied()
            } else {
                self.pending_nucleus.first().copied()
            };
            return Err(MisplacedApostrophe { before, after });
        }
        let n = self.pending_consonants.len();
        let mut best_err = None;
        let max_suffix = n.min(3);
        for suffix_len in (1 ..= max_suffix).rev() {
            let onset_chars: Vec<char> =
                self.pending_consonants[.. suffix_len].iter().rev().copied().collect();
            let onset_str: String = onset_chars.iter().collect();
            if let Err(e) = Onset::new(&onset_str) {
                let leftover = &self.pending_consonants[suffix_len ..];
                if split_into_coda_and_consonantal(leftover, may_try_coda).is_ok() {
                    best_err.get_or_insert(e);
                }
                continue;
            }
            let leftover = &self.pending_consonants[suffix_len ..];
            let (coda, pairs) = match split_into_coda_and_consonantal(leftover, may_try_coda) {
                Ok(x) => x,
                Err(e) => {
                    best_err.get_or_insert(e);
                    continue;
                }
            };
            let left_of_onset = pairs
                .last()
                .and_then(|s| s.nucleus.get_sonorant())
                .or_else(|| coda.map(|c| c.it()));
            if let Some(l) = left_of_onset
                && let Some(&r) = onset_chars.first()
                && !is_valid_chars(l, r, self.settings)
            {
                best_err.get_or_insert_with(|| invalid_from_pair(What::Cluster, l, r));
                continue;
            }
            if let Some(pre) = left_of_onset
                && onset_chars.len() >= 2
                && is_banned_triple_chars(pre, onset_chars[0], onset_chars[1])
            {
                best_err.get_or_insert_with(|| {
                    invalid_cluster_from_triple(pre, onset_chars[0], onset_chars[1])
                });
                continue;
            }
            let onset = Onset::new(&onset_str)?;
            let new_coda = coda;
            self.consonantal_syllable_buffer.clear();
            self.consonantal_syllable_buffer.extend(pairs.into_iter().rev());
            self.pending_consonants.clear();
            return Ok((onset, new_coda));
        }
        Err(best_err.unwrap_or_else(|| Invalid {
            what: What::ConsonantRun,
            value: self.pending_consonants.iter().rev().collect(),
        }))
    }

    // - vowels -

    fn take_nucleus(&mut self) -> Result<Nucleus, Jvofli> {
        let chars = std::mem::take(&mut self.pending_nucleus);
        let s: String = chars.into_iter().rev().collect();
        Nucleus::new(&s)
    }

    /// Handles a vowel (and in doing so also the `pending_consonants`).
    fn handle_vowel(&mut self, c: char) -> Result<(), Jvofli> {
        if self.in_cmevla {
            return Ok(());
        }
        if !self.pending_consonants.is_empty() {
            // resolve them
            let old_coda = self.pending_coda.take();
            let (onset, new_coda) = self.resolve_pending_consonants(true, Some(c))?;
            if self.pending_nucleus.is_empty()
                && let Some(&Syllable { onset: Onglide(g), .. }) = self.pending_unit.last()
            {
                return Err(OnglideInCluster(g));
            }
            let nucleus = self.take_nucleus()?;
            self.pending_unit.push(Syllable { onset, nucleus, coda: old_coda });
            self.pending_unit.extend(&self.consonantal_syllable_buffer);
            self.pending_coda = new_coda;
            self.pending_nucleus.push(c);
            return Ok(());
        }
        let Some(&existing) = self.pending_nucleus.last() else {
            self.pending_nucleus.push(c);
            return Ok(());
        };
        // diphthong time
        if is_diphthong_chars(c, existing) {
            if let Some(&Syllable { onset: Onglide(g), .. }) = self.pending_unit.last()
                && g == self.pending_nucleus[0]
            {
                return Err(LongGlide(g));
            }
            self.pending_nucleus.push(c);
            return Ok(());
        }
        // onglide time
        if matches!(c, 'i' | 'u') {
            let nucleus = self.take_nucleus()?;
            self.pending_unit.push(Syllable {
                onset: Onglide(c),
                nucleus,
                coda: self.pending_coda.take(),
            });
            return Ok(());
        }
        Err(invalid_from_pair(What::Nucleus, c, existing))
    }

    // - pause groups / boundaries -

    /// Tries to resolve any leftover `pending_consonants`/`pending_nucleus` and
    /// emit a `Unit`.
    fn flush_group(&mut self, pg_start: usize) -> Result<(), Jvofli> {
        if self.in_cmevla {
            let text = &self.input[pg_start .. self.pg_end];
            check_cmevla(text, self.settings)?;
            self.units.push(Unit::Cmevla(text.to_string()));
            self.reset_pending();
            return Ok(());
        }
        if !self.pending_consonants.is_empty() {
            let old_coda = self.pending_coda.take();
            let (onset, _) = self.resolve_pending_consonants(false, None)?;
            if !self.pending_nucleus.is_empty() {
                let nucleus = self.take_nucleus()?;
                self.pending_unit.push(Syllable { onset, nucleus, coda: old_coda });
                self.pending_unit.extend(&self.consonantal_syllable_buffer);
            } else if let Some(&Syllable { onset: Onglide(g), .. }) = self.pending_unit.last() {
                return Err(OnglideInCluster(g));
            }
        } else if !self.pending_nucleus.is_empty() {
            let coda = self.pending_coda.take();
            let nucleus = self.take_nucleus()?;
            self.pending_unit.push(Syllable { onset: Empty, nucleus, coda });
        }
        if self.pending_unit.is_empty() {
            self.reset_pending();
            return Ok(());
        }
        let mut syllables = std::mem::take(&mut self.pending_unit);
        syllables.reverse();
        if let Some(first) = syllables.first()
            && first.has_h_onset()
        {
            return Err(MisplacedApostrophe {
                before: None,
                after: first.nucleus.to_string().chars().next(),
            });
        }
        let resolved = resolve_stress_and_split(&syllables, self.settings)?;
        let mut resolved = resolved.into_iter().rev();
        if let Some(new) = resolved.next() {
            let to_push = if let Some(old) = self.units.pop() {
                match try_merging_units(&new, old, self.settings) {
                    Ok(merged) => merged,
                    Err(old) => {
                        self.units.push(old);
                        new
                    }
                }
            } else {
                new
            };
            self.units.push(to_push);
        }
        self.units.extend(resolved);
        self.reset_pending();
        Ok(())
    }

    /// Tries to flush the current pause group.
    fn handle_boundary(&mut self, i: usize) -> Result<(), Jvofli> {
        self.flush_group(i + 1)?;
        self.pg_end = i;
        Ok(())
    }

    fn handle_boundary_at_start(&mut self) -> Result<(), Jvofli> { self.flush_group(0) }

    // - orchestrator -

    /// Walks `input` rtl trying to unitify it and returns the units ltr.
    fn run(mut self) -> Result<Vec<Unit>, Jvofli> {
        while let Some((i, c)) = self.chars.next() {
            let c = deannotate_glide(c);
            match c {
                ' ' | '.' => self.handle_boundary(i)?,
                v if is_vowel(v) => self.handle_vowel(v)?,
                c if is_consonant(c) => self.push_consonant(c),
                x => return Err(NonLojbanCharacter(x)),
            }
        }
        self.handle_boundary_at_start()?;
        self.units.reverse();
        Ok(self.units)
    }
}

// - entry point -

/// Tries to convert a string to a list of [`Unit`]s. `settings` is used for
/// `allow_mz` and `arbitrary_cmavo_rafsi`.
///
/// This isn't done via [`FromStr`](std::str::FromStr) for two reasons:
/// - in the `Ok` case we return a `Vec` rather than one `Unit`
/// - `from_str(s)` can also be written as `s.parse()`, which could be
///   interpreted as meaning Lojban *grammar* parsing
///
/// The input is assumed to be 100% Lojban, so e.g. uses of *la'oi .whatever.*
/// and *zoizoi. whatever .zoi* will probably error in some way. (We can't
/// handle these without considering grammar, which jvot3 isn't concerned with,
/// so this is a wontfix.)
///
/// # Errors
///
/// Propogates errors from all the internal functions. For example:
/// - [`NonLojbanCharacter`]
/// - [`Invalid`] (see [`What`])
/// - [`LongGlide`]: same letter used as both offglide and onglide
/// - [`OnglideInCluster`]: annotated glide adjacent to a consonant
/// - [`MisplacedApostrophe`]
/// - [`NotEnoughSyllables`]
/// - [`InvalidStressPosition`]: explicit stress doesn't fall on a valid penult
/// - [`Slinkuhi`]: specifically for CCV'V units, as those are banned even under
///   `settings.no_slinkuhi`
pub fn unitify(s: &str, settings: Settings) -> Result<Vec<Unit>, Jvofli> {
    Unitifier::new(s, settings).run()
}
