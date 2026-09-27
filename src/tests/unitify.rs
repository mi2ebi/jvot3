use std::collections::VecDeque;

use crate::{
    jvofli::{
        Jvofli::{
            Invalid, InvalidStressPosition, MisplacedApostrophe, NonLojbanCharacter,
            NotEnoughSyllables, OnglideInCluster, Slinkuhi, Unstressable, UnstressablePreBrivlaEnd,
            UnstressablePreBrivlaStart,
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
const PERMISSIVE: Settings = Settings::PERMISSIVE;

macro_rules! syllable {
    ($onset:literal, $nucleus:literal; $settings:expr) => {
        Syllable {
            onset: Onset::new($onset, $settings).unwrap(),
            nucleus: Nucleus::new($nucleus).unwrap(),
            coda: None,
        }
    };
    ($onset:literal, $nucleus:literal, $coda:literal; $settings:expr) => {
        Syllable {
            onset: Onset::new($onset, $settings).unwrap(),
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
                syllable!("l", "e"; CLL),
                syllable!("'", "i"; CLL),
                syllable!("g", "é", 'r'; CLL),
                syllable!("k", "u"; CLL)
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
            syllables: vdq![syllable!("i", "a"; CLL), syllable!("n", "ai"; CLL)],
            pre_brivla_start: None
        }])
    );
}

#[test]
fn jehebzi() {
    assert_eq!(
        unitify("je'ebzi", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("j", "e"; CLL),
                syllable!("'", "é", 'b'; CLL),
                syllable!("z", "i"; CLL)
            ],
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
                syllable!("s", "e"; CLL),
                syllable!("l", "o"; CLL),
                syllable!("jb", "ó"; CLL),
                syllable!("n", "ai"; CLL)
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
                syllables: vdq![
                    syllable!("s", "e"; CLL),
                    syllable!("l", "ó"; CLL),
                    syllable!("jb", "o"; CLL)
                ],
                pre_brivla_start: Some(1)
            },
            Normal { syllables: vdq![syllable!("n", "ai"; CLL)], pre_brivla_start: None }
        ])
    );
}
#[test]
fn sélojbonai() {
    assert_eq!(
        unitify("sélojbonai", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("s", "é"; CLL),
                syllable!("l", "o"; CLL),
                syllable!("jb", "ó"; CLL),
                syllable!("n", "ai"; CLL)
            ],
            pre_brivla_start: Some(2)
        }])
    );
}
#[test]
fn selojbonái() {
    assert_eq!(unitify("selojbonái", CLL), Err(InvalidStressPosition("nái".into())));
}
#[test]
fn lójbosélojbonai() {
    assert_eq!(
        unitify("lójbosélojbonai", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("l", "ó"; CLL), syllable!("jb", "o"; CLL)],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![
                    syllable!("s", "é"; CLL),
                    syllable!("l", "o"; CLL),
                    syllable!("jb", "ó"; CLL),
                    syllable!("n", "ai"; CLL)
                ],
                pre_brivla_start: Some(2)
            }
        ])
    );
}

#[test]
fn xazdmru() {
    assert_eq!(
        unitify("xazdmru", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("x", "á", 'z'; CLL),
                syllable!("d", "m"; CLL),
                syllable!("r", "u"; CLL)
            ],
            pre_brivla_start: Some(0)
        }])
    );
}

#[test]
fn mi_do() {
    assert_eq!(
        unitify("mi do", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("m", "i"; CLL), syllable!("d", "o"; CLL)],
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
        Ok(vec![
            Normal { syllables: vdq![syllable!("m", "i"; CLL)], pre_brivla_start: None },
            Normal {
                syllables: vdq![syllable!("", "i"; CLL), syllable!("'", "e"; CLL)],
                pre_brivla_start: None
            }
        ])
    );
}

#[test]
fn ai_iicmo() {
    assert_eq!(
        unitify("ai iicmo", CLL),
        Ok(vec![
            Normal { syllables: vdq![syllable!("", "ai"; CLL)], pre_brivla_start: None },
            Normal {
                syllables: vdq![syllable!("i", "í"; CLL), syllable!("cm", "o"; CLL)],
                pre_brivla_start: Some(0)
            }
        ])
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
            syllables: vdq![syllable!("f", "y"; CLL), syllable!("'", "a"; CLL)],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn fyhahe() {
    assert_eq!(
        unitify("fy'a'e", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("f", "y"; CLL),
                syllable!("'", "a"; CLL),
                syllable!("'", "e"; CLL)
            ],
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
                syllable!("p", "a"; CLL),
                syllable!("f", "r"; CLL),
                syllable!("t", "á"; CLL),
                syllable!("'", "e"; CLL)
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
                syllable!("p", "a"; CLL),
                syllable!("f", "r"; CLL),
                syllable!("t", "á"; CLL),
                syllable!("bl", "e"; CLL)
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
            syllables: vdq![
                syllable!("b", "á"; CLL),
                syllable!("cr", "ú"; CLL),
                syllable!("d", "a"; CLL)
            ],
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
                syllables: vdq![syllable!("b", "á"; CLL), syllable!("cr", "u"; CLL)],
                pre_brivla_start: Some(0)
            },
            Normal { syllables: vdq![syllable!("d", "a"; CLL)], pre_brivla_start: None }
        ])
    );
}
#[test]
fn bácrudárno() {
    assert_eq!(
        unitify("bácrudárno", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("b", "á"; CLL), syllable!("cr", "u"; CLL)],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![syllable!("d", "á", 'r'; CLL), syllable!("n", "o"; CLL)],
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
                    syllable!("c", "í"; CLL),
                    syllable!("c", "o"; CLL),
                    syllable!("zv", "á"; CLL),
                    syllable!("t", "i"; CLL)
                ],
                pre_brivla_start: Some(2)
            },
            Normal { syllables: vdq![syllable!("t", "i"; CLL)], pre_brivla_start: None }
        ])
    );
}

#[test]
fn máblanútrocícozvátiti() {
    assert_eq!(
        unitify("máblanútrocícozvátiti", CLL),
        Ok(vec![
            Normal {
                syllables: vdq![syllable!("m", "á"; CLL), syllable!("bl", "a"; CLL)],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![syllable!("n", "ú"; CLL), syllable!("tr", "o"; CLL)],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![
                    syllable!("c", "í"; CLL),
                    syllable!("c", "o"; CLL),
                    syllable!("zv", "á"; CLL),
                    syllable!("t", "i"; CLL)
                ],
                pre_brivla_start: Some(2)
            },
            Normal { syllables: vdq![syllable!("t", "i"; CLL)], pre_brivla_start: None }
        ])
    );
}
#[test]
fn máblánu() {
    assert_eq!(
        unitify("máblánu", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("m", "á"; CLL),
                syllable!("bl", "á"; CLL),
                syllable!("n", "u"; CLL)
            ],
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
                syllable!("m", "a"; CLL),
                syllable!("bl", "a"; CLL),
                syllable!("x", "é"; CLL),
                syllable!("kr", "i"; CLL),
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
                syllables: vdq![syllable!("m", "á"; CLL), syllable!("bl", "a"; CLL)],
                pre_brivla_start: Some(0)
            },
            Normal {
                syllables: vdq![syllable!("x", "é"; CLL), syllable!("kr", "i"; CLL)],
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
            Normal { syllables: vdq![syllable!("m", "á"; CLL)], pre_brivla_start: None },
            Normal {
                syllables: vdq![
                    syllable!("bl", "a"; CLL),
                    syllable!("x", "é"; CLL),
                    syllable!("kr", "i"; CLL)
                ],
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
            syllables: vdq![syllable!("m", "á"; CLL), syllable!("b", "a"; CLL)],
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
            syllables: vdq![syllable!("m", "á"; CLL), syllable!("b", "á"; CLL)],
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
                syllable!("m", "í"; CLL),
                syllable!("n", "e", 'l'; CLL),
                syllable!("c", "í"; CLL),
                syllable!("d", "o"; CLL),
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
    assert_eq!(unitify("íafak", CLL), Err(Unstressable("i".into())));
}

#[test]
fn gy() {
    assert_eq!(
        unitify("gy", CLL),
        Ok(vec![Normal { syllables: vdq![syllable!("g", "y"; CLL)], pre_brivla_start: None }])
    );
}
#[test]
fn jegy() {
    assert_eq!(
        unitify("jegy", CLL),
        Ok(vec![Normal {
            syllables: vdq![syllable!("j", "e"; CLL), syllable!("g", "y"; CLL)],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn gyje() {
    assert_eq!(unitify("gyje", CLL), Err(UnstressablePreBrivlaStart("gy".into())));
}

#[test]
fn pahyva_cll() {
    assert_eq!(
        unitify("pa'yva", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("p", "a"; CLL),
                syllable!("'", "y"; CLL),
                syllable!("v", "a"; CLL)
            ],
            pre_brivla_start: None
        }])
    );
}
#[test]
fn pahyva_permissive() {
    assert_eq!(
        unitify("pa'yva", PERMISSIVE),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("p", "á"; PERMISSIVE),
                syllable!("'", "y"; PERMISSIVE),
                syllable!("v", "a"; PERMISSIVE)
            ],
            pre_brivla_start: Some(0)
        }])
    );
}
#[test]
fn pahyvalsi_cll() {
    assert_eq!(
        unitify("pa'yvalsi", CLL),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("p", "a"; CLL),
                syllable!("'", "y"; CLL),
                syllable!("v", "á", 'l'; CLL),
                syllable!("s", "i"; CLL)
            ],
            pre_brivla_start: Some(2)
        }])
    );
}
#[test]
fn pahyvalsi_permissive() {
    assert_eq!(
        unitify("pa'yvalsi", PERMISSIVE),
        Ok(vec![Normal {
            syllables: vdq![
                syllable!("p", "a"; PERMISSIVE),
                syllable!("'", "y"; PERMISSIVE),
                syllable!("v", "á", 'l'; PERMISSIVE),
                syllable!("s", "i"; PERMISSIVE)
            ],
            pre_brivla_start: Some(0)
        }])
    );
}
#[test]
fn pahy_valsi_cll() {
    assert_eq!(unitify("pa'y valsi", CLL), unitify("pa'yvalsi", CLL));
}
#[test]
fn pahy_valsi_permissive() {
    assert_ne!(unitify("pa'y valsi", PERMISSIVE), unitify("pa'yvalsi", PERMISSIVE));
}

#[test]
fn gy_pabroda() {
    assert_eq!(
        unitify("gy pabroda", CLL),
        Ok(vec![
            Normal { syllables: vdq![syllable!("g", "y"; CLL)], pre_brivla_start: None },
            Normal {
                syllables: vdq![
                    syllable!("p", "a"; CLL),
                    syllable!("br", "ó"; CLL),
                    syllable!("d", "a"; CLL)
                ],
                pre_brivla_start: Some(1)
            }
        ])
    );
}
#[test]
fn gy_gybroda() {
    assert_eq!(unitify("gy gybroda", CLL), unitify("gygybroda", CLL));
}
