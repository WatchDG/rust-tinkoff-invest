use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticker(String);

impl Ticker {
    #[inline]
    pub fn new(ticker: String) -> Self {
        Self(ticker)
    }
}

impl Into<Ticker> for String {
    fn into(self) -> Ticker {
        Ticker(self)
    }
}

impl Into<String> for Ticker {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Ticker> for types::Share {
    fn into(self) -> Ticker {
        self.ticker
    }
}

impl Into<Ticker> for &types::Share {
    fn into(self) -> Ticker {
        self.ticker.clone()
    }
}

impl Into<Ticker> for types::Currency {
    fn into(self) -> Ticker {
        self.ticker
    }
}

impl Into<Ticker> for &types::Currency {
    fn into(self) -> Ticker {
        self.ticker.clone()
    }
}

impl Into<Ticker> for enums::MarketInstrument {
    fn into(self) -> Ticker {
        match self {
            enums::MarketInstrument::Share(share) => share.into(),
            enums::MarketInstrument::Currency(currency) => currency.into(),
        }
    }
}

impl Into<Ticker> for &enums::MarketInstrument {
    fn into(self) -> Ticker {
        match self {
            enums::MarketInstrument::Share(share) => share.into(),
            enums::MarketInstrument::Currency(currency) => currency.into(),
        }
    }
}
