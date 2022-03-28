use crate::types;

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

impl Into<Ticker> for types::MarketInstrument {
    fn into(self) -> Ticker {
        self.ticker
    }
}
