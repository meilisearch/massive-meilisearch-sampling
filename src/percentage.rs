use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Percentage(pub u8);

impl FromStr for Percentage {
    type Err = InvalidPercentage;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplit_once('%') {
            Some((left, right)) if right.is_empty() => {
                let number = left.parse().map_err(InvalidPercentage::InvalidNumber)?;
                if number > 100 {
                    Err(InvalidPercentage::OutOfBound)
                } else {
                    Ok(Percentage(number))
                }
            }
            _ => Err(InvalidPercentage::MissingModuleSymbol),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidPercentage {
    InvalidNumber(ParseIntError),
    OutOfBound,
    MissingModuleSymbol,
}

impl fmt::Display for InvalidPercentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidPercentage::InvalidNumber(error) => error.fmt(f),
            InvalidPercentage::OutOfBound => {
                f.write_str("The percentage is out of the [0:100] bounds")
            }
            InvalidPercentage::MissingModuleSymbol => {
                f.write_str("The percentage must be followed by the `%` symbol")
            }
        }
    }
}

impl Error for InvalidPercentage {}
