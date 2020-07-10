use std::fmt;

pub enum TenseVoiceMoodVariant {
    PresActIndReg(String),
    PresActIndAlp(String),
    PresActIndEps(String),
    PresActIndOmi(String),
    PresPassIndReg(String),
    PresPassIndAlp(String),
    PresPassIndEps(String),
    PresPassIndOmi(String),
    FutActIndReg(String),
    FutActIndEps(String),
    FutMidIndReg(String),
    FutMidIndEps(String),
    FutPassIndReg(String),
    AorActIndWk(String),
    AorActIndStr(String),
    AorMidIndWk(String),
    AorMidIndStr(String),
    AorPassIndWk(String),
}

impl fmt::Display for TenseVoiceMoodVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TenseVoiceMoodVariant::PresActIndReg(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresActIndAlp(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresActIndEps(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresActIndOmi(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresPassIndReg(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresPassIndAlp(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresPassIndEps(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::PresPassIndOmi(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::FutActIndReg(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::FutActIndEps(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::FutMidIndReg(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::FutMidIndEps(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::FutPassIndReg(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::AorActIndWk(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::AorActIndStr(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::AorMidIndWk(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::AorMidIndStr(val) => write!(f, "{}", val),
            TenseVoiceMoodVariant::AorPassIndWk(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Debug for TenseVoiceMoodVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TenseVoiceMoodVariant::PresActIndReg(val) => write!(f, "PresActIndReg: \"{}\"", val),
            TenseVoiceMoodVariant::PresActIndAlp(val) => write!(f, "PresActIndAlp: \"{}\"", val),
            TenseVoiceMoodVariant::PresActIndEps(val) => write!(f, "PresActIndEps: \"{}\"", val),
            TenseVoiceMoodVariant::PresActIndOmi(val) => write!(f, "PresActIndOmi: \"{}\"", val),
            TenseVoiceMoodVariant::PresPassIndReg(val) => write!(f, "PresPassIndReg: \"{}\"", val),
            TenseVoiceMoodVariant::PresPassIndAlp(val) => write!(f, "PresPassIndAlp: \"{}\"", val),
            TenseVoiceMoodVariant::PresPassIndEps(val) => write!(f, "PresPassIndEps: \"{}\"", val),
            TenseVoiceMoodVariant::PresPassIndOmi(val) => write!(f, "PresPassIndOmi: \"{}\"", val),
            TenseVoiceMoodVariant::FutActIndReg(val) => write!(f, "FutActIndReg: \"{}\"", val),
            TenseVoiceMoodVariant::FutActIndEps(val) => write!(f, "FutActIndEps: \"{}\"", val),
            TenseVoiceMoodVariant::FutMidIndReg(val) => write!(f, "FutMidIndReg: \"{}\"", val),
            TenseVoiceMoodVariant::FutMidIndEps(val) => write!(f, "FutMidIndEps: \"{}\"", val),
            TenseVoiceMoodVariant::FutPassIndReg(val) => write!(f, "FutPassIndReg: \"{}\"", val),
            TenseVoiceMoodVariant::AorActIndWk(val) => write!(f, "AorActIndWk: \"{}\"", val),
            TenseVoiceMoodVariant::AorActIndStr(val) => write!(f, "AorActIndStr: \"{}\"", val),
            TenseVoiceMoodVariant::AorMidIndWk(val) => write!(f, "AorMidIndWk: \"{}\"", val),
            TenseVoiceMoodVariant::AorMidIndStr(val) => write!(f, "AorMidIndStr: \"{}\"", val),
            TenseVoiceMoodVariant::AorPassIndWk(val) => write!(f, "AorPassIndWk: \"{}\"", val),
        }
    }
}

impl TenseVoiceMoodVariant {
    pub fn first_char(&self) -> String {
        let first: String = match &self {
            &TenseVoiceMoodVariant::PresActIndReg(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresActIndAlp(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresActIndEps(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresActIndOmi(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresPassIndReg(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresPassIndAlp(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresPassIndEps(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::PresPassIndOmi(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::FutActIndReg(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::FutActIndEps(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::FutMidIndReg(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::FutMidIndEps(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::FutPassIndReg(val) => {
                format!("{}", val.chars().next().unwrap())
            }
            &TenseVoiceMoodVariant::AorActIndWk(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::AorActIndStr(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::AorMidIndWk(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::AorMidIndStr(val) => format!("{}", val.chars().next().unwrap()),
            &TenseVoiceMoodVariant::AorPassIndWk(val) => format!("{}", val.chars().next().unwrap()),
        };
        first
    }
}

#[derive(Debug)]
pub enum Conjugated {
    Some(Vec<String>),
    None,
}

impl Conjugated {
    pub fn print(&self) {
        match self {
            Conjugated::Some(v) => {
                let mut s = String::new();
                for part in v {
                    s.push_str(format!(", {}", part).as_ref());
                }
                println!("{}", &s[2..]);
            }
            Conjugated::None => {}
        }
    }

    pub fn to_vec(&self) -> Option<&Vec<String>> {
        match self {
            Conjugated::Some(v) => Some(&v),
            _ => None,
        }
    }
}
