use std::fmt;
use std::str::FromStr;


#[derive(Debug)]
pub enum Error {
    InvalidChainError,
    InvalidSpeciesError,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Chain {
    TRA,
    TRB,
    TRD,
    TRG,
    IGH,
    IGK,
    IGL,
}

impl Chain {
    pub fn enumerated() -> &'static [&'static str] {
        &["TRA", "TRB", "TRD", "TRG", "IGH", "IGK", "IGL"]
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidChainError => write!(
                f,
                "Invalid chain. Possible values are TRA, TRB, TRD, TRG, IGH, IGK, IGL."
            ),
            Error::InvalidSpeciesError => {
                write!(f, "Invalid species. Possible values are human or mouse")
            }
        }
    }
}

impl FromStr for Chain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_uppercase();
        let variant = match s.as_ref() {
            "TRA" => Chain::TRA,
            "TRB" => Chain::TRB,
            "TRD" => Chain::TRD,
            "TRG" => Chain::TRG,
            "IGH" => Chain::IGH,
            "IGK" => Chain::IGK,
            "IGL" => Chain::IGL,
            _ => return Err(Error::InvalidChainError),
        };
        Ok(variant)
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Chain::TRA => "TRA",
            Chain::TRB => "TRB",
            Chain::TRD => "TRD",
            Chain::TRG => "TRG",
            Chain::IGH => "IGH",
            Chain::IGK => "IGK",
            Chain::IGL => "IGL",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Species {
    Human,
    Mouse,
}

impl Species {
    pub fn enumerated() -> &'static [&'static str] {
        &["human", "mouse"]
    }
}

impl FromStr for Species {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_uppercase();
        let variant = match s.as_ref() {
            "HUMAN" | "H" => Species::Human,
            "MOUSE" | "M" => Species::Mouse,
            _ => return Err(Error::InvalidSpeciesError),
        };
        Ok(variant)
    }
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Species::Human => "human",
            Species::Mouse => "mouse"
        };
        write!(f, "{}", s)
    }
}