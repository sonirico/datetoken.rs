use std::fmt::Display;
use std::fmt::{Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Weekdays {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl FromStr for Weekdays {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mon" => Ok(Self::Mon),
            "tue" => Ok(Self::Tue),
            "wed" => Ok(Self::Wed),
            "thu" => Ok(Self::Thu),
            "fri" => Ok(Self::Fri),
            "sat" => Ok(Self::Sat),
            "sun" => Ok(Self::Sun),
            _ => Err(TokenError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TimeUnits {
    Sec,
    Min,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl FromStr for TimeUnits {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(Self::Sec),
            "m" => Ok(Self::Min),
            "h" => Ok(Self::Hour),
            "d" => Ok(Self::Day),
            "w" => Ok(Self::Week),
            "M" => Ok(Self::Month),
            "Y" => Ok(Self::Year),
            _ => Err(TokenError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Now,

    SnapBegin,
    SnapEnd,

    Add,
    Sub,

    Unit(TimeUnits),

    Weekday(Weekdays),

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
            _ => {
                if let Ok(tok) = s.parse::<TimeUnits>() {
                    Ok(Self::Unit(tok))
                } else if let Ok(tok) = s.parse::<Weekdays>() {
                    Ok(Self::Weekday(tok))
                } else {
                    Err(TokenError)
                }
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.display())
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenError;
