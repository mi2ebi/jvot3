use crate::{
    jvofli::{
        Jvofli::{Invalid, Unstressable},
        What,
    },
    syllables::{
        Coda,
        Nucleus::{self, Diphthong, Sonorant, StressableMonophthong, Y},
        Onset::{self, Empty, H, Onglide, Pair, Single, Triple},
    },
};

#[test]
fn onset_b() {
    assert_eq!(Onset::new("b"), Ok(Single('b')));
}
#[test]
fn onset_bl() {
    assert_eq!(Onset::new("bl"), Ok(Pair(['b', 'l'])));
}
#[test]
fn onset_jvl() {
    assert_eq!(Onset::new("jvl"), Ok(Triple(['j', 'v', 'l'])));
}
#[test]
fn onset_h() {
    assert_eq!(Onset::new("'"), Ok(H));
}
#[test]
fn onset_() {
    assert_eq!(Onset::new(""), Ok(Empty));
}
#[test]
fn onset_u() {
    assert_eq!(Onset::new("u"), Ok(Onglide('u')));
}
#[test]
fn onset_bd() {
    assert_eq!(Onset::new("bd"), Err(Invalid { what: What::Onset, value: "bd".into() }));
}

#[test]
fn nucleus_a() {
    assert_eq!(Nucleus::new("a"), Ok(StressableMonophthong { vowel: 'a', stressed: false }));
}
#[test]
fn nucleus_á() {
    assert_eq!(Nucleus::new("á"), Ok(StressableMonophthong { vowel: 'a', stressed: true }));
}
#[test]
fn nucleus_aa() {
    assert_eq!(Nucleus::new("aa"), Err(Invalid { what: What::Nucleus, value: "aa".into() }));
}
#[test]
fn nucleus_ai() {
    assert_eq!(Nucleus::new("ai"), Ok(Diphthong { first: 'a', second: 'i', stressed: false }));
}
#[test]
fn nucleus_ái() {
    assert_eq!(Nucleus::new("ái"), Ok(Diphthong { first: 'a', second: 'i', stressed: true }));
}
#[test]
fn nucleus_r() {
    assert_eq!(Nucleus::new("r"), Ok(Sonorant('r')));
}
#[test]
fn nucleus_y() {
    assert_eq!(Nucleus::new("y"), Ok(Y));
}
#[test]
fn nucleus_ý() {
    assert_eq!(Nucleus::new("ý"), Err(Unstressable("y".into())));
}

#[test]
fn coda_q() {
    assert_eq!(Coda::new('q'), None);
}

#[test]
fn coda_p() {
    assert_eq!(Coda::new('p'), Some(Coda('p')));
}
