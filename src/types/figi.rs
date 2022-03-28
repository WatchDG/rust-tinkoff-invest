// pub type Figi = String;
use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Figi(String);

impl From<&str> for Figi {
    fn from(string: &str) -> Self {
        Self(string.into())
    }
}

impl From<String> for Figi {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl From<Figi> for String {
    fn from(figi: Figi) -> Self {
        figi.0
    }
}

impl Into<Figi> for types::Share {
    fn into(self) -> Figi {
        self.figi
    }
}

impl Into<Figi> for &types::Share {
    fn into(self) -> Figi {
        self.figi.clone()
    }
}

impl Into<Figi> for types::Currency {
    fn into(self) -> Figi {
        self.figi
    }
}

impl Into<Figi> for &types::Currency {
    fn into(self) -> Figi {
        self.figi.clone()
    }
}

impl Into<Figi> for enums::MarketInstrument {
    fn into(self) -> Figi {
        match self {
            enums::MarketInstrument::Share(share) => share.into(),
            enums::MarketInstrument::Currency(currency) => currency.into(),
        }
    }
}

impl Into<Figi> for &enums::MarketInstrument {
    fn into(self) -> Figi {
        match self {
            enums::MarketInstrument::Share(share) => share.into(),
            enums::MarketInstrument::Currency(currency) => currency.into(),
        }
    }
}
