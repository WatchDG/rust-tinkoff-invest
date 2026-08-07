use std::convert::TryFrom;
use std::str::FromStr;

use crate::TError;

/// Тип рыночного инструмента.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentType {
    Currency,
    Share,
    Future,
}

impl FromStr for InstrumentType {
    type Err = TError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "currency" => Ok(InstrumentType::Currency),
            "share" => Ok(InstrumentType::Share),
            "future" => Ok(InstrumentType::Future),
            other => Err(TError::InvalidInstrumentType(other.to_string())),
        }
    }
}

impl TryFrom<&str> for InstrumentType {
    type Error = TError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for InstrumentType {
    type Error = TError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_types() {
        assert_eq!(
            InstrumentType::try_from("share").unwrap(),
            InstrumentType::Share
        );
        assert_eq!(
            InstrumentType::try_from("CURRENCY").unwrap(),
            InstrumentType::Currency
        );
        assert_eq!(
            InstrumentType::try_from("Future").unwrap(),
            InstrumentType::Future
        );
    }

    #[test]
    fn parse_unknown_type() {
        let err = InstrumentType::try_from("bond").unwrap_err();
        assert!(matches!(err, TError::InvalidInstrumentType(_)));
    }
}
