use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Figi(String);

impl Figi {
    #[inline]
    pub fn new(figi: String) -> Self {
        Self(figi)
    }
}

impl Into<Figi> for String {
    fn into(self) -> Figi {
        Figi(self)
    }
}

impl Into<String> for Figi {
    fn into(self) -> String {
        self.0
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
