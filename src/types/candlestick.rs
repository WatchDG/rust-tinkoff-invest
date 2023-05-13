use tinkoff_invest_types as tit;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candlestick {
    pub uid: Option<types::Uid>,
    pub figi: Option<types::Figi>,
    pub interval: Option<enums::CandlestickInterval>,
    pub datetime: Option<types::DateTime>,
    pub open: Option<types::MoneyValue>,
    pub high: Option<types::MoneyValue>,
    pub low: Option<types::MoneyValue>,
    pub close: Option<types::MoneyValue>,
    pub volume: u64,
    pub is_complete: bool,
}

impl From<tit::HistoricCandle> for Candlestick {
    fn from(value: tit::HistoricCandle) -> Self {
        Candlestick {
            uid: None,
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

impl From<tit::Candle> for Candlestick {
    fn from(value: tit::Candle) -> Self {
        let interval = value.interval().into();
        Self {
            uid: None,
            figi: Some(value.figi.into()),
            interval: Some(interval),
            open: value.open.map(|x| x.into()),
            high: value.high.map(|x| x.into()),
            low: value.low.map(|x| x.into()),
            close: value.close.map(|x| x.into()),
            volume: value.volume as u64,
            datetime: value.time.map(|x| x.into()),
            is_complete: false,
        }
    }
}
