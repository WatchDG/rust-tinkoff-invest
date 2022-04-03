use std::convert::TryFrom;

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

impl From<tit::HistoricCandle> for Candlestick {
    fn from(value: tit::HistoricCandle) -> Self {
        Candlestick {
            figi: None,
            interval: None,
            open: value.open.map(|x| x.into()),
            high: value.high.map(|x| x.into()),
            low: value.low.map(|x| x.into()),
            close: value.close.map(|x| x.into()),
            volume: value.volume as u64,
            datetime: value.time.map(|x| x.into()),
            is_complete: value.is_complete,
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

impl TryFrom<Candlestick> for StrictCandlestick {
    type Error = TinkoffInvestError;

    fn try_from(value: Candlestick) -> Result<Self, Self::Error> {
        Ok(StrictCandlestick {
            figi: value
                .figi
                .ok_or(TinkoffInvestError::CandlestickFigiNotSet)?,
            interval: value
                .interval
                .ok_or(TinkoffInvestError::CandlestickIntervalNotSet)?,
            open: value
                .open
                .ok_or(TinkoffInvestError::CandlestickPriceOpenNotSet)?,
            high: value
                .high
                .ok_or(TinkoffInvestError::CandlestickPriceHighNotSet)?,
            low: value
                .low
                .ok_or(TinkoffInvestError::CandlestickPriceLowNotSet)?,
            close: value
                .close
                .ok_or(TinkoffInvestError::CandlestickPriceCloseNotSet)?,
            volume: value.volume,
            datetime: value
                .datetime
                .ok_or(TinkoffInvestError::CandlestickDatetimeNotSet)?,
            is_complete: value.is_complete,
        })
    }
}
