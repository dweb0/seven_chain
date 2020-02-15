use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum Error {
    InvalidChainError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid chain. Possible values are TRA, TRB, TRD, TRG, IGH, IGK, IGL."
        )
    }
}

impl FromStr for Chain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Chain, Self::Err> {
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
