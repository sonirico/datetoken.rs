use std::fmt::Display;
use std::fmt::{Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
    Now,

    SnapBegin,
    SnapEnd,

    Add,
    Sub,

    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,

    Monday,
    Tuesday,

    Number(u16),
    Illegal(String),
}

impl Token {
    fn display(&self) -> &str {
        match self {
            Self::Now => "now",
            Self::SnapBegin => "/",
            Self::SnapEnd => "@",
            _ => "ILLEGAL",
        }
    }
}

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "now" => Ok(Self::Now),
            "/" => Ok(Self::SnapBegin),
            "@" => Ok(Self::SnapEnd),
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "s" => Ok(Self::Second),
            "m" => Ok(Self::Minute),
            "h" => Ok(Self::Hour),
            "d" => Ok(Self::Day),
            "w" => Ok(Self::Week),
            "M" => Ok(Self::Month),
            "y" => Ok(Self::Year),
            _ => Err(TokenError),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.display())
    }
}

pub struct TokenError;
