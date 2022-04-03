use crate::types::MarketInstrument;
use crate::{traits, types};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticker(String);

impl Ticker {
    #[inline]
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Ticker {
    fn from(value: &str) -> Self {
        Ticker::new(value.into())
    }
}

impl From<String> for Ticker {
    fn from(value: String) -> Self {
        Ticker::new(value)
    }
}

impl From<Ticker> for String {
    fn from(value: Ticker) -> Self {
        value.0
    }
}

impl From<types::MarketInstrument> for Ticker {
    fn from(value: MarketInstrument) -> Self {
        value.ticker
    }
}

impl traits::ToTicker for &Ticker {
    fn to_ticker(&self) -> Ticker {
        Ticker::new(self.0.clone())
    }
}
