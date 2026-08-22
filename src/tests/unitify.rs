use std::collections::VecDeque;

use crate::{
    jvofli::{
        Jvofli::{
            Invalid, InvalidStressPosition, MisplacedApostrophe, NonLojbanCharacter,
            NotEnoughSyllables, OnglideInCluster, Slinkuhi, StressOnUnstressable,
            UnstressablePreBrivlaEnd, UnstressablePreBrivlaStart,
        },
        What,
    },
    settings::Settings,
    syllables::{Coda, Nucleus, Onset, Syllable},
    units::{
        Unit::{Cmevla, Normal},
        unitify,
    },
};

// todo organize these

const CLL: Settings = Settings::CLL;

macro_rules! syllable {
    ($onset:literal, $nucleus:literal) => {
        Syllable {
            onset: Onset::new($onset).unwrap(),
            nucleus: Nucleus::new($nucleus).unwrap(),
            coda: None,
        }
    };
    ($onset:literal, $nucleus:literal, $coda:literal) => {
        Syllable {
            onset: Onset::new($onset).unwrap(),
            nucleus: Nucleus::new($nucleus).unwrap(),
            coda: Coda::new($coda),
        }
    };
}
macro_rules! vdq {
    [] => { VecDeque::new() };
    [$($item:expr),+ $(,)?] => { VecDeque::from([$($item),+]) };
}

#[test]
fn lehigerku() {
    assert_eq!(
        unitify("le'igerku", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("l", "e"),
                syllable!("'", "i"),
                syllable!("g", "é", 'r'),
                syllable!("k", "u")
            ],
            pre_brivla_start: Some(2)
        }])
    );
}

#[test]
fn ianai() {
    assert_eq!(
        unitify("ianai", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("i", "a"), syllable!("n", "ai")],
            pre_brivla_start: None
        }])
    );
}

#[test]
fn jehebzi() {
    assert_eq!(
        unitify("je'ebzi", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("j", "e"), syllable!("'", "é", 'b'), syllable!("z", "i")],
            pre_brivla_start: Some(0)
        }])
    );
}

#[test]
fn selojbonai() {
    assert_eq!(
        unitify("selojbonai", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("s", "e"),
                syllable!("l", "o"),
                syllable!("jb", "ó"),
                syllable!("n", "ai")
            ],
            pre_brivla_start: Some(2)
        }])
    );
}
#[test]
fn selojbónai() {
    assert_eq!(unitify("selojbónai", CLL), unitify("selojbonai", CLL));
}
#[test]
fn se_lojbonai() {
    assert_eq!(unitify("se lojbonai", CLL), unitify("selojbonai", CLL));
}
#[test]
fn selójbonai() {
    assert_eq!(
        unitify("selójbonai", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("s", "e"), syllable!("l", "ó"), syllable!("jb", "o")],
                pre_brivla_start: Some(1)
            },
            Normal { syllables: vdq![syllable!("n", "ai")], pre_brivla_start: None }
        ])
    );
}
#[test]
fn sélojbonai() {
    assert_eq!(
        unitify("sélojbonai", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("s", "é"),
                syllable!("l", "o"),
                syllable!("jb", "ó"),
                syllable!("n", "ai")
            ],
            pre_brivla_start: Some(2)
        }])
    );
}
#[test]
fn selojbonái() {
    assert_eq!(
        unitify("selojbonái", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("s", "e"), syllable!("l", "ó"), syllable!("jb", "o")],
                pre_brivla_start: Some(1)
            },
            Normal { syllables: vdq![syllable!("n", "ái")], pre_brivla_start: None }
        ])
    );
}
#[test]
fn lójbosélojbonai() {
    assert_eq!(
        unitify("lójbosélojbonai", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("l", "ó"), syllable!("jb", "o")],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![
                    syllable!("s", "é"),
                    syllable!("l", "o"),
                    syllable!("jb", "ó"),
                    syllable!("n", "ai")
                ],
                pre_brivla_start: Some(2)
            }
        ])
    );
}
#[test]
fn selojbonávahehyjboklu() {
    assert_eq!(
        unitify("selojbonáva'e'yjboklu", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("s", "e"), syllable!("l", "ó"), syllable!("jb", "o")],
                pre_brivla_start: Some(1)
            },
            Normal {
                syllables: vdq![
                    syllable!("n", "á"),
                    syllable!("v", "a"),
                    syllable!("'", "e"),
                    syllable!("'", "y"),
                    syllable!("jb", "ó"),
                    syllable!("kl", "u")
                ],
                pre_brivla_start: Some(1)
            },
        ])
    );
}

#[test]
fn xazdmru() {
    assert_eq!(
        unitify("xazdmru", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("x", "á", 'z'), syllable!("d", "m"), syllable!("r", "u")],
            pre_brivla_start: Some(0)
        }])
    );
}

#[test]
fn mi_do() {
    assert_eq!(
        unitify("mi do", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("m", "i"), syllable!("d", "o")],
            pre_brivla_start: None
        }])
    );
}

#[test]
fn ciai() {
    assert_eq!(unitify("ciai", CLL), Err(OnglideInCluster('i')));
}

#[test]
fn bytygau() {
    assert_eq!(unitify("bytygau", CLL), Err(UnstressablePreBrivlaStart("by".into())));
}

#[test]
fn krtyvla() {
    assert_eq!(unitify("krtyvla", CLL), Err(UnstressablePreBrivlaStart("kr".into())));
}

#[test]
fn mi_ihe() {
    assert_eq!(
        unitify("mi i'e", CLL),
        Ok(vec![Normal { syllables: vdq![syllable!("m", "i")], pre_brivla_start: None }, Normal {
            syllables: vdq![syllable!("", "i"), syllable!("'", "e")],
            pre_brivla_start: None
        }])
    );
}

#[test]
fn ai_iicmo() {
    assert_eq!(
        unitify("ai iicmo", CLL),
        Ok(vec![Normal { syllables: vdq![syllable!("", "ai")], pre_brivla_start: None }, Normal {
            syllables: vdq![syllable!("i", "í"), syllable!("cm", "o")],
            pre_brivla_start: Some(0)
        }])
    );
}

#[test]
fn n() {
    assert_eq!(unitify("n", CLL), Ok(vec![Cmevla("n".into())]));
}

#[test]
fn an() {
    assert_eq!(unitify("an", CLL), Ok(vec![Cmevla("an".into())]));
}

#[test]
fn ha() {
    assert_eq!(unitify("'a", CLL), Err(MisplacedApostrophe { before: None, after: Some('a') }));
}

#[test]
fn lojbónaiha() {
    assert_eq!(unitify("lojbónai'a", CLL), Err(InvalidStressPosition("jbó".into())));
}
#[test]
fn lojbónaihabla() {
    assert_eq!(unitify("lojbónai'abla", CLL), Err(InvalidStressPosition("jbó".into())));
}

#[test]
fn zba() {
    assert_eq!(unitify("zba", CLL), Err(NotEnoughSyllables("zba".into())));
}

#[test]
fn fyha() {
    assert_eq!(
        unitify("fy'a", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("f", "y"), syllable!("'", "a")],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn fyhahe() {
    assert_eq!(
        unitify("fy'a'e", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("f", "y"), syllable!("'", "a"), syllable!("'", "e")],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn fryha() {
    assert_eq!(unitify("fry'a", CLL), Err(UnstressablePreBrivlaStart("fry".into())));
}
#[test]
fn fryhahe() {
    assert_eq!(unitify("fry'a'e", CLL), Err(UnstressablePreBrivlaStart("fry".into())));
}
#[test]
fn fryhable() {
    assert_eq!(unitify("fry'able", CLL), Err(UnstressablePreBrivlaStart("fry".into())));
}
#[test]
fn frtahe() {
    assert_eq!(unitify("frta'e", CLL), Err(UnstressablePreBrivlaStart("fr".into())));
}
#[test]
fn fytahe() {
    assert_eq!(unitify("fyta'e", CLL), Err(UnstressablePreBrivlaStart("fy".into())));
}
#[test]
fn frtable() {
    assert_eq!(unitify("frtable", CLL), Err(UnstressablePreBrivlaStart("fr".into())));
}
#[test]
fn pafrtahe() {
    assert_eq!(
        unitify("pafrta'e", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("p", "a"),
                syllable!("f", "r"),
                syllable!("t", "á"),
                syllable!("'", "e")
            ],
            pre_brivla_start: Some(0)
        }])
    );
}
#[test]
fn pafrtable() {
    assert_eq!(
        unitify("pafrtable", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("p", "a"),
                syllable!("f", "r"),
                syllable!("t", "á"),
                syllable!("bl", "e")
            ],
            pre_brivla_start: Some(0)
        }])
    );
}

#[test]
fn bácrúda() {
    assert_eq!(
        unitify("bácrúda", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("b", "á"), syllable!("cr", "ú"), syllable!("d", "a")],
            pre_brivla_start: Some(1)
        }])
    );
}
#[test]
fn bácruda() {
    assert_eq!(
        unitify("bácruda", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("b", "á"), syllable!("cr", "u")],
                pre_brivla_start: Some(0)
            },
            Normal { syllables: vdq![syllable!("d", "a")], pre_brivla_start: None }
        ])
    );
}
#[test]
fn bácrudárno() {
    assert_eq!(
        unitify("bácrudárno", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("b", "á"), syllable!("cr", "u")],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![syllable!("d", "á", 'r'), syllable!("n", "o")],
                pre_brivla_start: Some(0)
            }
        ])
    );
}

#[test]
fn málblánu() {
    assert_eq!(unitify("málblánu", CLL), Err(InvalidStressPosition("mál".into())));
}

#[test]
fn cícozvátiti() {
    assert_eq!(
        unitify("cícozvátiti", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![
                    syllable!("c", "í"),
                    syllable!("c", "o"),
                    syllable!("zv", "á"),
                    syllable!("t", "i")
                ],
                pre_brivla_start: Some(2)
            },
            Normal { syllables: vdq![syllable!("t", "i")], pre_brivla_start: None }
        ])
    );
}

#[test]
fn máblanútrocícozvátiti() {
    assert_eq!(
        unitify("máblanútrocícozvátiti", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("m", "á"), syllable!("bl", "a")],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![syllable!("n", "ú"), syllable!("tr", "o")],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![
                    syllable!("c", "í"),
                    syllable!("c", "o"),
                    syllable!("zv", "á"),
                    syllable!("t", "i")
                ],
                pre_brivla_start: Some(2)
            },
            Normal { syllables: vdq![syllable!("t", "i")], pre_brivla_start: None }
        ])
    );
}
#[test]
fn máblánu() {
    assert_eq!(
        unitify("máblánu", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("m", "á"), syllable!("bl", "á"), syllable!("n", "u")],
            pre_brivla_start: Some(1)
        }])
    );
}

#[test]
fn mablaxekri() {
    assert_eq!(
        unitify("mablaxekri", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("m", "a"),
                syllable!("bl", "a"),
                syllable!("x", "é"),
                syllable!("kr", "i"),
            ],
            pre_brivla_start: Some(1)
        }])
    );
}
#[test]
fn ma_blaxekri() {
    assert_eq!(unitify("ma blaxekri", CLL), unitify("mablaxekri", CLL));
}
#[test]
fn máblaxekri() {
    assert_eq!(
        unitify("máblaxekri", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("m", "á"), syllable!("bl", "a")],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![syllable!("x", "é"), syllable!("kr", "i")],
                pre_brivla_start: Some(0)
            }
        ])
    );
}
#[test]
fn má_blaxekri() {
    assert_eq!(
        unitify("má blaxekri", CLL),
        Ok(vec![
            Normal { syllables: vdq![syllable!("m", "á")], pre_brivla_start: None },
            Normal {
                syllables: vdq![syllable!("bl", "a"), syllable!("x", "é"), syllable!("kr", "i")],
                pre_brivla_start: Some(0)
            }
        ])
    );
}
#[test]
fn má_bla() {
    assert_eq!(unitify("má bla", CLL), Err(NotEnoughSyllables("bla".into())));
}
#[test]
fn mába() {
    assert_eq!(
        unitify("mába", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("m", "á"), syllable!("b", "a")],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn má_ba() {
    assert_eq!(unitify("má ba", CLL), unitify("mába", CLL));
}
#[test]
fn mábá() {
    assert_eq!(
        unitify("mábá", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("m", "á"), syllable!("b", "á")],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn má_bá() {
    assert_eq!(unitify("má bá", CLL), unitify("mábá", CLL));
}

#[test]
fn mínelcido() {
    assert_eq!(
        unitify("mínelcido", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("m", "í"),
                syllable!("n", "e", 'l'),
                syllable!("c", "í"),
                syllable!("d", "o"),
            ],
            pre_brivla_start: Some(1)
        }])
    );
}
#[test]
fn mí_nelcido() {
    assert_eq!(unitify("mí nelcido", CLL), unitify("mínelcido", CLL));
}

#[test]
fn bangy() {
    assert_eq!(unitify("bangy", CLL), Err(UnstressablePreBrivlaEnd("gy".into())));
}
#[test]
fn vragy() {
    assert_eq!(unitify("vragy", CLL), Err(UnstressablePreBrivlaEnd("gy".into())));
}

#[test]
fn mi1() {
    assert_eq!(unitify("mi1", CLL), Err(NonLojbanCharacter('1')));
}

#[test]
fn akkan() {
    assert_eq!(unitify("akkan", CLL), Err(Invalid { what: What::Cluster, value: "kk".into() }));
}
#[test]
fn aan() {
    assert_eq!(unitify("aan", CLL), Err(Invalid { what: What::Nucleus, value: "aa".into() }));
}
#[test]
fn ahhan() {
    assert_eq!(
        unitify("a''an", CLL),
        Err(MisplacedApostrophe { before: Some('a'), after: Some('\'') })
    );
}

#[test]
fn blahi() {
    assert_eq!(unitify("bla'i", CLL), Err(Slinkuhi("bla'i".into())));
}

#[test]
fn íafak() {
    // non cmevla results in "{ía} is not a valid nucleus" instead
    assert_eq!(unitify("íafak", CLL), Err(StressOnUnstressable("í".into())));
}
