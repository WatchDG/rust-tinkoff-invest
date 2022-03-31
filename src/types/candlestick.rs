use std::convert::TryInto;
use tinkoff_invest_types as tit;

use crate::{enums, types, TinkoffInvestError};

#[derive(Debug, Clone, PartialEq)]
pub struct Candlestick {
    pub figi: Option<types::Figi>,
    pub interval: Option<enums::CandlestickInterval>,
    pub open: Option<types::MoneyValue>,
    pub high: Option<types::MoneyValue>,
    pub low: Option<types::MoneyValue>,
    pub close: Option<types::MoneyValue>,
    pub volume: u64,
    pub datetime: Option<types::DateTime>,
    pub is_complete: bool,
}

impl Into<Candlestick> for tit::HistoricCandle {
    fn into(self) -> Candlestick {
        Candlestick {
            figi: None,
            interval: None,
            open: self.open.map(|x| x.into()),
            high: self.high.map(|x| x.into()),
            low: self.low.map(|x| x.into()),
            close: self.close.map(|x| x.into()),
            volume: self.volume as u64,
            datetime: self.time.map(|x| x.into()),
            is_complete: self.is_complete,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrictCandlestick {
    pub figi: types::Figi,
    pub interval: enums::CandlestickInterval,
    pub open: types::MoneyValue,
    pub high: types::MoneyValue,
    pub low: types::MoneyValue,
    pub close: types::MoneyValue,
    pub volume: u64,
    pub datetime: types::DateTime,
    pub is_complete: bool,
}

impl TryInto<StrictCandlestick> for Candlestick {
    type Error = TinkoffInvestError;

    fn try_into(self) -> Result<StrictCandlestick, Self::Error> {
        Ok(StrictCandlestick {
            figi: self.figi.ok_or(TinkoffInvestError::CandlestickFigiNotSet)?,
            interval: self
                .interval
                .ok_or(TinkoffInvestError::CandlestickIntervalNotSet)?,
            open: self
                .open
                .ok_or(TinkoffInvestError::CandlestickPriceOpenNotSet)?,
            high: self
                .high
                .ok_or(TinkoffInvestError::CandlestickPriceHighNotSet)?,
            low: self
                .low
                .ok_or(TinkoffInvestError::CandlestickPriceLowNotSet)?,
            close: self
                .close
                .ok_or(TinkoffInvestError::CandlestickPriceCloseNotSet)?,
            volume: self.volume,
            datetime: self
                .datetime
                .ok_or(TinkoffInvestError::CandlestickDatetimeNotSet)?,
            is_complete: self.is_complete,
        })
    }
}
