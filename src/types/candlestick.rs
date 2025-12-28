use tinkoff_invest_types as tit;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candlestick {
    pub instrument_uid: types::Uid,
    pub interval: enums::CandlestickInterval,
    pub datetime: types::DateTime,
    pub open: Option<types::MoneyValue>,
    pub high: Option<types::MoneyValue>,
    pub low: Option<types::MoneyValue>,
    pub close: Option<types::MoneyValue>,
    pub volume: u64,
    pub is_complete: bool,
}

impl From<tit::Candle> for Candlestick {
    fn from(value: tit::Candle) -> Self {
        let interval = value.interval().into();
        Self {
            instrument_uid: value.instrument_uid.as_str().into(),
            interval,
            open: value.open.map(|x| x.into()),
            high: value.high.map(|x| x.into()),
            low: value.low.map(|x| x.into()),
            close: value.close.map(|x| x.into()),
            volume: value.volume as u64,
            datetime: value
                .time
                .map(|x| x.into())
                .expect("time must be set in Candle"),
            is_complete: false,
        }
    }
}
