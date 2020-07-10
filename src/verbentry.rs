use super::types::{Conjugated, TenseVoiceMoodVariant};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VerbEntry {
    present: Option<String>,
    future: Option<String>,
    aorist: Option<String>,
    #[serde(rename = "Aorist_Passive")]
    aorist_passive: Option<String>,
}

impl VerbEntry {
    pub fn check_alternates(&self) -> VerbAlternates {
        let pres = match &self.present {
            Some(v) => Some(v.to_string()),
            None => None,
        };
        let (fut, fut_alt) = match &self.future {
            Some(v) => {
                if v.contains("/") {
                    let v_s: Vec<&str> = v.split("/").collect();
                    (Some(v_s[0].to_string()), Some(v_s[1].to_string()))
                } else {
                    (Some(v.to_string()), None)
                }
            }
            _ => (None, None),
        };
        let (aor, aor_alt) = match &self.aorist {
            Some(v) => {
                if v.contains("/") {
                    let v_s: Vec<&str> = v.split("/").collect();
                    (Some(v_s[0].to_string()), Some(v_s[1].to_string()))
                } else {
                    (Some(v.to_string()), None)
                }
            }
            _ => (None, None),
        };
        let (aor_pass, aor_pass_alt) = match &self.aorist_passive {
            Some(v) => {
                if v.contains("/") {
                    let v_s: Vec<&str> = v.split("/").collect();
                    (Some(v_s[0].to_string()), Some(v_s[1].to_string()))
                } else {
                    (Some(v.to_string()), None)
                }
            }
            _ => (None, None),
        };
        VerbAlternates {
            present: pres,
            future: fut,
            future_alt: fut_alt,
            aorist: aor,
            aorist_alt: aor_alt,
            aorist_passive: aor_pass,
            aorist_passive_alt: aor_pass_alt,
        }
    }
}

#[derive(Debug)]
pub struct VerbAlternates {
    present: Option<String>,
    future: Option<String>,
    future_alt: Option<String>,
    aorist: Option<String>,
    aorist_alt: Option<String>,
    aorist_passive: Option<String>,
    aorist_passive_alt: Option<String>,
}

impl VerbAlternates {
    pub fn parse(&self) -> VerbStemSet {
        let pres: Option<TenseVoiceMoodVariant> = match &self.present {
            Some(pp) => {
                let mut tmp = pp.to_string();
                if tmp.ends_with("αω") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresActIndAlp(tmp))
                } else if tmp.ends_with("εω") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresActIndEps(tmp))
                } else if tmp.ends_with("οω") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresActIndOmi(tmp))
                } else if tmp.ends_with("ω") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresActIndReg(tmp))
                } else if tmp.ends_with("αομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresPassIndAlp(tmp))
                } else if tmp.ends_with("εομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresPassIndEps(tmp))
                } else if tmp.ends_with("οομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresPassIndOmi(tmp))
                } else if tmp.ends_with("ομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::PresPassIndReg(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        let fut: Option<TenseVoiceMoodVariant> = match &self.future {
            Some(pp) => {
                let mut tmp = pp.to_string();
                if tmp.ends_with("σω") || tmp.ends_with("ξω") || tmp.ends_with("ψω") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutActIndReg(tmp))
                } else if tmp.ends_with("θησομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutPassIndReg(tmp))
                } else if tmp.ends_with("σομαι") || tmp.ends_with("ξομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutMidIndReg(tmp))
                } else if tmp.ends_with("ω") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutActIndEps(tmp))
                } else if tmp.ends_with("ουμαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutMidIndEps(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        let fut2: Option<TenseVoiceMoodVariant> = match &self.future_alt {
            Some(pp) => {
                let mut tmp = pp.to_string();
                if tmp.ends_with("σω") || tmp.ends_with("ξω") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutActIndReg(tmp))
                } else if tmp.ends_with("θησομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutPassIndReg(tmp))
                } else if tmp.ends_with("σομαι") || tmp.ends_with("ξομαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutMidIndReg(tmp))
                } else if tmp.ends_with("ω") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutActIndEps(tmp))
                } else if tmp.ends_with("ουμαι") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::FutMidIndEps(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        let aor: Option<TenseVoiceMoodVariant> = match &self.aorist {
            Some(pp) => {
                let mut tmp = pp.to_string();
                if tmp.ends_with("α") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorActIndWk(tmp))
                } else if tmp.ends_with("αμην") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorMidIndWk(tmp))
                } else if tmp.ends_with("ομην") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorMidIndStr(tmp))
                } else if tmp.ends_with("ην") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorPassIndWk(tmp))
                } else if tmp.ends_with("ον") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorActIndStr(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        let aor2: Option<TenseVoiceMoodVariant> = match &self.aorist_alt {
            Some(pp) => {
                let mut tmp = pp.to_string();
                if tmp.ends_with("α") {
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorActIndWk(tmp))
                } else if tmp.ends_with("αμην") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorMidIndWk(tmp))
                } else if tmp.ends_with("ομην") {
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorMidIndStr(tmp))
                } else if tmp.ends_with("ην") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorPassIndWk(tmp))
                } else if tmp.ends_with("ον") {
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorActIndStr(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        let ap: Option<TenseVoiceMoodVariant> = match &self.aorist_passive {
            Some(pp) => {
                if pp.ends_with("ην") {
                    let mut tmp = pp.to_string();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorPassIndWk(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        let ap2: Option<TenseVoiceMoodVariant> = match &self.aorist_passive_alt {
            Some(pp) => {
                if pp.ends_with("ην") {
                    let mut tmp = pp.to_string();
                    tmp.pop();
                    tmp.pop();
                    Some(TenseVoiceMoodVariant::AorPassIndWk(tmp))
                } else {
                    None
                }
            }
            None => None,
        };
        VerbStemSet {
            pres_stem: pres,
            fut_stem: fut,
            fut_stem_alt: fut2,
            aorist_stem: aor,
            aorist_stem_alt: aor2,
            aor_pass_stem: ap,
            aor_pass_stem_alt: ap2,
        }
    }
}

#[derive(Debug)]
pub struct VerbStemSet {
    pres_stem: Option<TenseVoiceMoodVariant>,
    fut_stem: Option<TenseVoiceMoodVariant>,
    fut_stem_alt: Option<TenseVoiceMoodVariant>,
    aorist_stem: Option<TenseVoiceMoodVariant>,
    aorist_stem_alt: Option<TenseVoiceMoodVariant>,
    aor_pass_stem: Option<TenseVoiceMoodVariant>,
    aor_pass_stem_alt: Option<TenseVoiceMoodVariant>,
}

impl VerbStemSet {
    fn aug_and_stem<'a>(
        opt_aor: &Option<TenseVoiceMoodVariant>,
        mut stem: &'a str,
    ) -> (&'a str, &'a str) {
        let aug: &str = match stem {
            stm if stm.starts_with("ἀμφι") => {
                stem = stem.splitn(2, "ι").collect::<Vec<&str>>()[1];
                "ἀμφε"
            }
            stm if stm.starts_with("ἀνα") => match opt_aor {
                Some(aor) => {
                    if aor.first_char() == "ἠ" {
                        stem = stem.splitn(2, "ἀ").collect::<Vec<&str>>()[1];
                        "ἠ"
                    } else {
                        stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                        "ἀνε"
                    }
                }
                None => "ἠ",
            },
            stm if stm.starts_with("ἀντι") => {
                stem = stem.splitn(2, "ι").collect::<Vec<&str>>()[1];
                "ἀντε"
            }
            stm if stm.starts_with("ἀπο") => {
                stem = stem.splitn(2, "ο").collect::<Vec<&str>>()[1];
                "ἀπε"
            }
            stm if stm.starts_with("ἀφι") => "",
            stm if stm.starts_with("αἰ") => {
                stem = stem.splitn(2, "ἰ").collect::<Vec<&str>>()[1];
                "ᾐ"
            }
            stm if stm.starts_with("αἱ") => {
                stem = stem.splitn(2, "ἱ").collect::<Vec<&str>>()[1];
                "ᾑ"
            }
            stm if stm.starts_with("ἀ") => {
                stem = stem.splitn(2, "ἀ").collect::<Vec<&str>>()[1];
                "ἠ"
            }
            stm if stm.starts_with("ἁ") => {
                stem = stem.splitn(2, "ἁ").collect::<Vec<&str>>()[1];
                "ἡ"
            }
            stm if stm.starts_with("δια") => {
                stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                "διε"
            }
            stm if stm.starts_with("διε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                "διει"
            }
            stm if stm.starts_with("διο") => {
                stem = stem.splitn(2, "ο").collect::<Vec<&str>>()[1];
                "διω"
            }
            stm if stm.starts_with("εἰσ") => {
                stem = stem.splitn(2, "σ").collect::<Vec<&str>>()[1];
                "εἰσε"
            }
            stm if stm.starts_with("ἐκ") => {
                stem = stem.splitn(2, "κ").collect::<Vec<&str>>()[1];
                "ἐξε"
            }
            stm if stm.starts_with("ἐμ") => {
                stem = stem.splitn(2, "μ").collect::<Vec<&str>>()[1];
                "ἐνε"
            }
            stm if stm.starts_with("ἐγ") => {
                stem = stem.splitn(2, "γ").collect::<Vec<&str>>()[1];
                "ἐνε"
            }
            stm if stm.starts_with("ἐνε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                "ἐνει"
            }
            stm if stm.starts_with("ἐνα") => {
                stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                "ἐνη"
            }
            stm if stm.starts_with("ἐν") => {
                stem = stem.splitn(2, "ν").collect::<Vec<&str>>()[1];
                "ἐνε"
            }
            stm if stm.starts_with("ἐπε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                "ἐπει"
            }
            stm if stm.starts_with("ἐπα") => {
                stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                "ἐπη"
            }
            stm if stm.starts_with("ἐπι") => {
                stem = stem.splitn(2, "ι").collect::<Vec<&str>>()[1];
                "ἐπε"
            }
            stm if stm.starts_with("εὐ") => {
                stem = stem.splitn(2, "ὐ").collect::<Vec<&str>>()[1];
                "ηὐ"
            }
            stm if stm.starts_with("εὑ") => {
                stem = stem.splitn(2, "ὑ").collect::<Vec<&str>>()[1];
                "ηὑ"
            }
            stm if stm.starts_with("ἐ") => {
                stem = stem.splitn(2, "ἐ").collect::<Vec<&str>>()[1];
                match opt_aor {
                    Some(aor) => match &aor {
                        &TenseVoiceMoodVariant::AorActIndWk(_)
                        | &TenseVoiceMoodVariant::AorMidIndWk(_)
                        | &TenseVoiceMoodVariant::AorPassIndWk(_) => {
                            if aor.first_char() == "ἠ" {
                                "ἠ"
                            } else {
                                "εἰ"
                            }
                        }
                        _ => "ἠ",
                    },
                    None => "ἠ",
                }
            }
            stm if stm.starts_with("ἑ") => {
                stem = stem.splitn(2, "ἑ").collect::<Vec<&str>>()[1];
                match opt_aor {
                    Some(aor) => {
                        if aor.first_char() == "ἡ" {
                            "ἡ"
                        } else {
                            "εἱ"
                        }
                    }
                    None => "ἡ",
                }
            }
            stm if stm.starts_with("καταγ") => {
                stem = stem.splitn(2, "γ").collect::<Vec<&str>>()[1];
                "κατηγ"
            }
            stm if stm.starts_with("κατα") => {
                stem = stem.splitn(2, "τα").collect::<Vec<&str>>()[1];
                "κατε"
            }
            stm if stm.starts_with("μεταγ") => {
                stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                "μετη"
            }
            stm if stm.starts_with("μετα") => {
                stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                "μετε"
            }
            stm if stm.starts_with("παραγ") => {
                stem = stem.splitn(2, "ρα").collect::<Vec<&str>>()[1];
                "παρη"
            }
            stm if stm.starts_with("παρα") => {
                stem = stem.splitn(2, "ρα").collect::<Vec<&str>>()[1];
                "παρε"
            }
            stm if stm.starts_with("παρε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                "παρει"
            }
            stm if stm.starts_with("περι") => {
                stem = stem.splitn(2, "ι").collect::<Vec<&str>>()[1];
                "περιε"
            }
            stm if stm.starts_with("περιαγ") => {
                stem = stem.splitn(2, "α").collect::<Vec<&str>>()[1];
                "περιη"
            }
            stm if stm.starts_with("προσ") => {
                stem = stem.splitn(2, "σ").collect::<Vec<&str>>()[1];
                "προσε"
            }
            stm if stm.starts_with("προ") => {
                stem = stem.splitn(2, "ο").collect::<Vec<&str>>()[1];
                "πρου"
            }
            stm if stm.starts_with("συγ") => {
                stem = stem.splitn(2, "γ").collect::<Vec<&str>>()[1];
                "συνε"
            }
            stm if stm.starts_with("συλλ") => {
                stem = stem.splitn(2, "λ").collect::<Vec<&str>>()[1];
                "συνε"
            }
            stm if stm.starts_with("συμ") => {
                stem = stem.splitn(2, "μ").collect::<Vec<&str>>()[1];
                "συνε"
            }
            stm if stm.starts_with("συρ") => {
                stem = stem.splitn(2, "ρ").collect::<Vec<&str>>()[1];
                "συνε"
            }
            stm if stm.starts_with("συσ") => {
                stem = stem.splitn(2, "υσ").collect::<Vec<&str>>()[1];
                "συνε"
            }
            stm if stm.starts_with("οἰ") => {
                stem = stem.splitn(2, "ἰ").collect::<Vec<&str>>()[1];
                "ᾠ"
            }
            stm if stm.starts_with("οἱ") => {
                stem = stem.splitn(2, "ἱ").collect::<Vec<&str>>()[1];
                "ᾡ"
            }
            stm if stm.starts_with("ὀ") => {
                stem = stem.splitn(2, "ὀ").collect::<Vec<&str>>()[1];
                "ὠ"
            }
            stm if stm.starts_with("ὁ") => {
                stem = stem.splitn(2, "ὁ").collect::<Vec<&str>>()[1];
                "ὡ"
            }
            stm if stm.starts_with("ὑπο") => {
                stem = stem.splitn(2, "ο").collect::<Vec<&str>>()[1];
                "ο"
            }
            _ => "ἐ",
        };
        (aug, stem)
    }

    fn remove_aug(opt_pres: &Option<TenseVoiceMoodVariant>, mut stem: &str) -> String {
        match stem {
            stm if stm.starts_with("ἀμφε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἀμφι{}", stem)
            }
            stm if stm.starts_with("ἀνε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἀνα{}", stem)
            }
            stm if stm.starts_with("ἀντε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἀντι{}", stem)
            }
            stm if stm.starts_with("ἀπε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἀπο{}", stem)
            }
            stm if stm.starts_with("δι") => {
                let parts: Vec<&str> = stem.splitn(2, "ε").collect();
                format!("{}α{}", parts[0], parts[1])
            }
            stm if stm.starts_with("εἰσε") => {
                stem = stem.splitn(2, "σε").collect::<Vec<&str>>()[1];
                format!("εἰσ{}", stem)
            }
            stm if stm.starts_with("εἰ") => {
                stem = stem.splitn(2, "εἰ").collect::<Vec<&str>>()[1];
                format!("ἐ{}", stem)
            }
            stm if stm.starts_with("εἱ") => {
                stem = stem.splitn(2, "εἱ").collect::<Vec<&str>>()[1];
                format!("ἑ{}", stem)
            }
            stm if stm.starts_with("ἐξε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐκ{}", stem)
            }
            stm if stm.starts_with("ἐνεπ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐμ{}", stem)
            }
            stm if stm.starts_with("ἐνεβ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐμ{}", stem)
            }
            stm if stm.starts_with("ἐνεφ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐμ{}", stem)
            }
            stm if stm.starts_with("ἐνεψ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐμ{}", stem)
            }
            stm if stm.starts_with("ἐνεκ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐγ{}", stem)
            }
            stm if stm.starts_with("ἐνεγ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐγ{}", stem)
            }
            stm if stm.starts_with("ἐνεχ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐγ{}", stem)
            }
            stm if stm.starts_with("ἐνεξ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐγ{}", stem)
            }
            stm if stm.starts_with("ἐνει") => {
                stem = stem.splitn(2, "ει").collect::<Vec<&str>>()[1];
                format!("ἐνε{}", stem)
            }
            stm if stm.starts_with("ἐνη") => {
                stem = stem.splitn(2, "η").collect::<Vec<&str>>()[1];
                format!("ἐνα{}", stem)
            }
            stm if stm.starts_with("ἐνε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("ἐν{}", stem)
            }
            stm if stm.starts_with("ἐπει") => {
                stem = stem.splitn(2, "ει").collect::<Vec<&str>>()[1];
                format!("ἐπε{}", stem)
            }
            stm if stm.starts_with("ἐπη") => {
                stem = stem.splitn(2, "η").collect::<Vec<&str>>()[1];
                format!("ἐπα{}", stem)
            }
            stm if stm.starts_with("ἐπε") => match opt_pres {
                Some(pres) => {
                    if pres.to_string().starts_with("ἐπι") {
                        stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                        format!("ἐπι{}", stem)
                    } else {
                        stem = stem.splitn(2, "ἐ").collect::<Vec<&str>>()[1];
                        format!("{}", stem)
                    }
                }
                None => "".to_string(),
            },
            stm if stm.starts_with("ηὐ") => {
                stem = stem.splitn(2, "ὐ").collect::<Vec<&str>>()[1];
                format!("εὐ{}", stem)
            }
            stm if stm.starts_with("ηὑ") => {
                stem = stem.splitn(2, "ὑ").collect::<Vec<&str>>()[1];
                format!("εὑ{}", stem)
            }
            stm if stm.starts_with("ἐ") => {
                stem = stem.splitn(2, "ἐ").collect::<Vec<&str>>()[1];
                format!("{}", stem)
            }
            stm if stm.starts_with("ᾐ") => {
                stem = stem.splitn(2, "ᾐ").collect::<Vec<&str>>()[1];
                format!("αἰ{}", stem)
            }
            stm if stm.starts_with("ᾑ") => {
                stem = stem.splitn(2, "ᾑ").collect::<Vec<&str>>()[1];
                format!("αἱ{}", stem)
            }
            stm if stm.starts_with("ἠ") => match opt_pres {
                Some(pres) => {
                    if pres.first_char() == "ἀ" {
                        stem = stem.splitn(2, "ἠ").collect::<Vec<&str>>()[1];
                        format!("ἀ{}", stem)
                    } else {
                        stem = stem.splitn(2, "ἠ").collect::<Vec<&str>>()[1];
                        format!("ἐ{}", stem)
                    }
                }
                None => "".to_string(),
            },
            stm if stm.starts_with("ἡ") => match opt_pres {
                Some(pres) => {
                    if pres.first_char() == "ἁ" {
                        stem = stem.splitn(2, "ἡ").collect::<Vec<&str>>()[1];
                        format!("ἁ{}", stem)
                    } else {
                        stem = stem.splitn(2, "ἠ").collect::<Vec<&str>>()[1];
                        format!("ἑ{}", stem)
                    }
                }
                None => "".to_string(),
            },
            stm if stm.starts_with("ἡ") => {
                stem = stem.splitn(2, "ἡ").collect::<Vec<&str>>()[1];
                format!("ἁ{}", stem)
            }
            stm if stm.starts_with("κατη") => {
                stem = stem.splitn(2, "η").collect::<Vec<&str>>()[1];
                format!("κατα{}", stem)
            }
            stm if stm.starts_with("κατε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("κατα{}", stem)
            }
            stm if stm.starts_with("μετη") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("μετα{}", stem)
            }
            stm if stm.starts_with("μετε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("μετα{}", stem)
            }
            stm if stm.starts_with("παρη") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("παρα{}", stem)
            }
            stm if stm.starts_with("παρε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("παρα{}", stem)
            }
            stm if stm.starts_with("περιε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("περι{}", stem)
            }
            stm if stm.starts_with("περιη") => {
                stem = stem.splitn(2, "η").collect::<Vec<&str>>()[1];
                format!("περια{}", stem)
            }
            stm if stm.starts_with("προσε") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("προσ{}", stem)
            }
            stm if stm.starts_with("πρου") => {
                stem = stem.splitn(2, "υ").collect::<Vec<&str>>()[1];
                format!("προ{}", stem)
            }
            stm if stm.starts_with("συνεγ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συγ{}", stem)
            }
            stm if stm.starts_with("συνεκ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συγ{}", stem)
            }
            stm if stm.starts_with("συνεξ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συγ{}", stem)
            }
            stm if stm.starts_with("συνεχ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συγ{}", stem)
            }
            stm if stm.starts_with("συνελ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συλ{}", stem)
            }
            stm if stm.starts_with("συνεβ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συμ{}", stem)
            }
            stm if stm.starts_with("συνεμ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συμ{}", stem)
            }
            stm if stm.starts_with("συνεπ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συμ{}", stem)
            }
            stm if stm.starts_with("συνεφ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συμ{}", stem)
            }
            stm if stm.starts_with("συνερ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συρ{}", stem)
            }
            stm if stm.starts_with("συνεσ") => {
                stem = stem.splitn(2, "ε").collect::<Vec<&str>>()[1];
                format!("συσ{}", stem)
            }
            stm if stm.starts_with("ᾠ") => {
                stem = stem.splitn(2, "ᾠ").collect::<Vec<&str>>()[1];
                format!("οἰ{}", stem)
            }
            stm if stm.starts_with("ᾡ") => {
                stem = stem.splitn(2, "ᾡ").collect::<Vec<&str>>()[1];
                format!("οἱ{}", stem)
            }
            stm if stm.starts_with("ὠ") => {
                stem = stem.splitn(2, "ὠ").collect::<Vec<&str>>()[1];
                format!("ὀ{}", stem)
            }
            stm if stm.starts_with("ὡ") => {
                stem = stem.splitn(2, "ὡ").collect::<Vec<&str>>()[1];
                format!("ὁ{}", stem)
            }
            stm if stm.starts_with("ὑπ") => {
                let parts: Vec<&str> = stem.splitn(2, "ε").collect();
                format!("{}ο{}", parts[0], parts[1])
            }
            stm if stm.contains("ε") => {
                let parts: Vec<&str> = stem.splitn(2, "ε").collect();
                format!("{}{}", parts[0], parts[1])
            }
            _ => format!("{}", stem),
        }
    }

    pub fn conjugate(&self) -> Verb {
        let pai: Conjugated;
        let ppi: Conjugated;
        let iai: Conjugated;
        let ipi: Conjugated;
        match &self.pres_stem {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::PresActIndAlp(stm) => {
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "ᾳς", "ᾳ", "ωμεν", "ατε", "ωσι"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    pai = Conjugated::Some(active_forms);
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ωμαι", "ῳ", "αται", "ωμεθα", "ασθε", "ωνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    let mut impf_active_forms: Vec<String> = Vec::new();
                    for ending in ["ων", "ας", "α", "ωμεν", "ατε", "ων"].iter() {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_active_forms.push(part);
                    }
                    iai = Conjugated::Some(impf_active_forms);
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ωμην", "ω", "ατο", "ωμεθα", "ασθε", "ωντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresActIndEps(stm) => {
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "εις", "ει", "ουμεν", "ειτε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    pai = Conjugated::Some(active_forms);
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ῃ", "ειται", "ουμεθα", "εισθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    let mut impf_active_forms: Vec<String> = Vec::new();
                    for ending in ["ουν", "εις", "ει", "ουμεν", "ειτε", "ουν"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_active_forms.push(part);
                    }
                    iai = Conjugated::Some(impf_active_forms);
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμην", "ου", "ειτο", "ουμεθα", "εισθε", "ουντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresActIndOmi(stm) => {
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "οις", "οι", "ουμεν", "ουτε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    pai = Conjugated::Some(active_forms);
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ου", "ουται", "ουμεθα", "ουσθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    let mut impf_active_forms: Vec<String> = Vec::new();
                    for ending in ["ουν", "ους", "ου", "ουμεν", "ουτε", "ουν"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_active_forms.push(part);
                    }
                    iai = Conjugated::Some(impf_active_forms);
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμην", "ου", "ουτο", "ουμεθα", "ουσθε", "ουντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresActIndReg(stm) => {
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "εις", "ει", "ομεν", "ετε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    pai = Conjugated::Some(active_forms);
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    let mut impf_active_forms: Vec<String> = Vec::new();
                    for ending in ["ον", "ες", "ε", "ομεν", "ετε", "ον"].iter() {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_active_forms.push(part);
                    }
                    iai = Conjugated::Some(impf_active_forms);
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresPassIndAlp(stm) => {
                    pai = Conjugated::None;
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ωμαι", "ῳ", "αται", "ωμεθα", "ασθε", "ωνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // Imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    iai = Conjugated::None;
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ωμην", "ω", "ατο", "ωμεθα", "ασθε", "ωντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresPassIndEps(stm) => {
                    pai = Conjugated::None;
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ῃ", "ειται", "ουμεθα", "εισθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // Imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    iai = Conjugated::None;
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμην", "ου", "ειτο", "ουμεθα", "εισθε", "ουντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresPassIndOmi(stm) => {
                    pai = Conjugated::None;
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ου", "ουται", "ουμεθα", "ουσθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // Imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    iai = Conjugated::None;
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ουμην", "ου", "ουτο", "ουμεθα", "ουσθε", "ουντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                &TenseVoiceMoodVariant::PresPassIndReg(stm) => {
                    pai = Conjugated::None;
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    ppi = Conjugated::Some(passive_forms);
                    // Imperfects
                    let (aug, stem) = VerbStemSet::aug_and_stem(&self.aorist_stem, &stm);
                    iai = Conjugated::None;
                    let mut impf_passive_forms: Vec<String> = Vec::new();
                    for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter()
                    {
                        let part = format!("{}{}{}", aug, stem, ending);
                        impf_passive_forms.push(part);
                    }
                    ipi = Conjugated::Some(impf_passive_forms);
                }
                _ => {
                    pai = Conjugated::None;
                    ppi = Conjugated::None;
                    iai = Conjugated::None;
                    ipi = Conjugated::None;
                    eprintln!("Unable to conjugate present and imperfect for {}", &stmtype);
                }
            },
            None => {
                pai = Conjugated::None;
                ppi = Conjugated::None;
                iai = Conjugated::None;
                ipi = Conjugated::None;
            }
        }

        let fai: Conjugated;
        let fmi: Conjugated;
        match &self.fut_stem {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::FutActIndEps(stm) => {
                    // Future actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "εις", "ει", "ουμεν", "ειτε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    fai = Conjugated::Some(active_forms);
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ῃ", "ειται", "ουμεθα", "εισθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutActIndReg(stm) => {
                    // Future actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "εις", "ει", "ομεν", "ετε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    fai = Conjugated::Some(active_forms);
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutMidIndEps(stm) => {
                    // Future actives
                    fai = Conjugated::None;
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ῃ", "ειται", "ουμεθα", "εισθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutMidIndReg(stm) => {
                    // Future actives
                    fai = Conjugated::None;
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutPassIndReg(stm) => {
                    // Deponent Future
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in [
                        "θησομαι",
                        "θησῃ",
                        "θησεται",
                        "θησομεθα",
                        "θησεσθε",
                        "θησονται",
                    ]
                    .iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    fai = Conjugated::Some(passive_forms);
                    // Future middles
                    fmi = Conjugated::None;
                }
                _ => {
                    fai = Conjugated::None;
                    fmi = Conjugated::None;
                    eprintln!("Unable to conjugate future voice for {}", &stmtype);
                }
            },
            None => {
                fai = Conjugated::None;
                fmi = Conjugated::None;
            }
        }

        let fai2: Conjugated;
        let fmi2: Conjugated;
        match &self.fut_stem_alt {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::FutActIndEps(stm) => {
                    // Future actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "εις", "ει", "ουμεν", "ειτε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    fai2 = Conjugated::Some(active_forms);
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ῃ", "ειται", "ουμεθα", "εισθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutActIndReg(stm) => {
                    // Future actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ω", "εις", "ει", "ομεν", "ετε", "ουσι"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    fai2 = Conjugated::Some(active_forms);
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutMidIndEps(stm) => {
                    // Future actives
                    fai2 = Conjugated::None;
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ουμαι", "ῃ", "ειται", "ουμεθα", "εισθε", "ουνται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutMidIndReg(stm) => {
                    // Future actives
                    fai2 = Conjugated::None;
                    // Future middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    fmi2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::FutPassIndReg(stm) => {
                    // Deponent Futures
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in [
                        "θησομαι",
                        "θησῃ",
                        "θησεται",
                        "θησομεθα",
                        "θησεσθε",
                        "θησονται",
                    ]
                    .iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    fai2 = Conjugated::Some(passive_forms);
                    // Future middles
                    fmi2 = Conjugated::None;
                }
                _ => {
                    fai2 = Conjugated::None;
                    fmi2 = Conjugated::None;
                    eprintln!("Unable to conjugate second future voice for {}", &stmtype);
                }
            },
            None => {
                fai2 = Conjugated::None;
                fmi2 = Conjugated::None;
            }
        }

        let aai: Conjugated;
        let ami: Conjugated;
        match &self.aorist_stem {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::AorActIndStr(stm) => {
                    // Strong aorist actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ον", "ες", "ε", "ομεν", "ετε", "ον"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    aai = Conjugated::Some(active_forms);
                    // Strong aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorActIndWk(stm) => {
                    // Weak aorist actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["α", "ας", "ε", "αμεν", "ατε", "αν"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    aai = Conjugated::Some(active_forms);
                    // Weak aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorMidIndStr(stm) => {
                    // Strong aorist actives
                    aai = Conjugated::None;
                    // Strong aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorMidIndWk(stm) => {
                    // Weak aorist actives
                    aai = Conjugated::None;
                    // Weak aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorPassIndWk(stm) => {
                    // Passive Deponent aorists
                    // Weak aorist middles
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ην", "ης", "η", "ημεν", "ητε", "ησαν"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    aai = Conjugated::Some(active_forms);
                    ami = Conjugated::None;
                }
                _ => {
                    aai = Conjugated::None;
                    ami = Conjugated::None;
                    eprintln!(
                        "Unable to conjugate aorist active and middle for {}",
                        &stmtype
                    );
                }
            },
            None => {
                aai = Conjugated::None;
                ami = Conjugated::None;
            }
        }

        let aai2: Conjugated;
        let ami2: Conjugated;
        match &self.aorist_stem_alt {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::AorActIndStr(stm) => {
                    // Strong aorist actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ον", "ες", "ε", "ομεν", "ετε", "ον"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    aai2 = Conjugated::Some(active_forms);
                    // Strong aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorActIndWk(stm) => {
                    // Weak aorist actives
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["α", "ας", "ε", "αμεν", "ατε", "αν"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    aai2 = Conjugated::Some(active_forms);
                    // Weak aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorMidIndStr(stm) => {
                    // Strong aorist actives
                    aai2 = Conjugated::None;
                    // Strong aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorMidIndWk(stm) => {
                    // Weak aorist actives
                    aai2 = Conjugated::None;
                    // Weak aorist middles
                    let mut middle_forms: Vec<String> = Vec::new();
                    for ending in ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"].iter()
                    {
                        let part = format!("{}{}", stm, ending);
                        middle_forms.push(part);
                    }
                    ami2 = Conjugated::Some(middle_forms);
                }
                &TenseVoiceMoodVariant::AorPassIndWk(stm) => {
                    // Deponent aorists
                    // Weak aorist middles
                    let mut active_forms: Vec<String> = Vec::new();
                    for ending in ["ην", "ης", "η", "ημεν", "ητε", "ησαν"].iter() {
                        let part = format!("{}{}", stm, ending);
                        active_forms.push(part);
                    }
                    aai2 = Conjugated::Some(active_forms);
                    ami2 = Conjugated::None;
                }
                _ => {
                    aai2 = Conjugated::None;
                    ami2 = Conjugated::None;
                    eprintln!(
                        "Unable to conjugate second aorist active and middle for {}",
                        &stmtype
                    );
                }
            },
            None => {
                aai2 = Conjugated::None;
                ami2 = Conjugated::None;
            }
        }

        let api: Conjugated;
        let fpi: Conjugated;
        match &self.aor_pass_stem {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::AorPassIndWk(stm) => {
                    // aorist passives
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ην", "ης", "η", "ημεν", "ητε", "ησαν"].iter() {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    api = Conjugated::Some(passive_forms);
                    // future passives
                    let mut fut_pass_forms: Vec<String> = Vec::new();
                    // println!("AP stem before: {}", stm);
                    let stem = VerbStemSet::remove_aug(&self.pres_stem, stm);
                    // println!("AP stem after: {}", stem);
                    for ending in ["ησομαι", "ησῃ", "ησεται", "ησομεθα", "ησεσθε", "ησονται"].iter()
                    {
                        let part = format!("{}{}", stem, ending);
                        fut_pass_forms.push(part);
                    }
                    fpi = Conjugated::Some(fut_pass_forms);
                }
                _ => {
                    api = Conjugated::None;
                    fpi = Conjugated::None;
                    eprintln!("Unable to conjugate aorist passive for {}", &stmtype);
                }
            },
            None => {
                api = Conjugated::None;
                fpi = Conjugated::None;
            }
        }

        let api2: Conjugated;
        let fpi2: Conjugated;
        match &self.aor_pass_stem_alt {
            Some(stmtype) => match &stmtype {
                &TenseVoiceMoodVariant::AorPassIndWk(stm) => {
                    // aorist passives
                    let mut passive_forms: Vec<String> = Vec::new();
                    for ending in ["ην", "ης", "η", "ημεν", "ητε", "ησαν"].iter() {
                        let part = format!("{}{}", stm, ending);
                        passive_forms.push(part);
                    }
                    api2 = Conjugated::Some(passive_forms);
                    // future passives
                    let mut fut_pass_forms: Vec<String> = Vec::new();
                    let stem = VerbStemSet::remove_aug(&self.pres_stem, stm);
                    for ending in ["ησομαι", "ησῃ", "ησεται", "ησομεθα", "ησεσθε", "ησονται"].iter()
                    {
                        let part = format!("{}{}", stem, ending);
                        fut_pass_forms.push(part);
                    }
                    fpi2 = Conjugated::Some(fut_pass_forms);
                }
                _ => {
                    api2 = Conjugated::None;
                    fpi2 = Conjugated::None;
                    eprintln!("Unable to conjugate second aorist passive for {}", &stmtype);
                }
            },
            None => {
                api2 = Conjugated::None;
                fpi2 = Conjugated::None;
            }
        }

        Verb {
            pai: pai,
            ppi: ppi,
            iai: iai,
            ipi: ipi,
            fai: fai,
            fai2: fai2,
            fmi: fmi,
            fmi2: fmi2,
            fpi: fpi,
            fpi2: fpi2,
            aai: aai,
            aai2: aai2,
            ami: ami,
            ami2: ami2,
            api: api,
            api2: api2,
        }
    }
}

#[derive(Debug)]
pub struct Verb {
    pub pai: Conjugated,
    pub ppi: Conjugated,
    pub iai: Conjugated,
    pub ipi: Conjugated,
    pub fai: Conjugated,
    pub fai2: Conjugated,
    pub fmi: Conjugated,
    pub fmi2: Conjugated,
    pub fpi: Conjugated,
    pub fpi2: Conjugated,
    pub aai: Conjugated,
    pub aai2: Conjugated,
    pub ami: Conjugated,
    pub ami2: Conjugated,
    pub api: Conjugated,
    pub api2: Conjugated,
}
