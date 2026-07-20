use std::sync::Arc;

use tinkoff_invest_types as tit;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candlestick {
    pub instrument_uid: Arc<types::Uid>,
    pub interval: Arc<enums::CandlestickInterval>,
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
        let interval = Arc::new(value.interval().into());
        Self {
            instrument_uid: Arc::new(types::Uid::from_api_str(value.instrument_uid.as_str())),
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
