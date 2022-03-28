use crate::{types, TinkoffInvestError};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum MarketInstrument {
    Share(types::Share),
    Currency(types::Currency),
}

impl Into<MarketInstrument> for types::Share {
    fn into(self) -> MarketInstrument {
        MarketInstrument::Share(self)
    }
}

impl Into<MarketInstrument> for &types::Share {
    fn into(self) -> MarketInstrument {
        MarketInstrument::Share(self.clone())
    }
}

impl Into<MarketInstrument> for types::Currency {
    fn into(self) -> MarketInstrument {
        MarketInstrument::Currency(self)
    }
}

impl Into<MarketInstrument> for &types::Currency {
    fn into(self) -> MarketInstrument {
        MarketInstrument::Currency(self.clone())
    }
}

impl TryInto<types::Share> for MarketInstrument {
    type Error = TinkoffInvestError;
    fn try_into(self) -> Result<types::Share, Self::Error> {
        match self {
            MarketInstrument::Share(share) => Ok(share),
            MarketInstrument::Currency(_) => Err(Self::Error::CanNotConvertToShare),
        }
    }
}

impl TryInto<types::Currency> for MarketInstrument {
    type Error = TinkoffInvestError;
    fn try_into(self) -> Result<types::Currency, Self::Error> {
        match self {
            MarketInstrument::Share(_) => Err(Self::Error::CanNotConvertToCurrency),
            MarketInstrument::Currency(currency) => Ok(currency),
        }
    }
}
