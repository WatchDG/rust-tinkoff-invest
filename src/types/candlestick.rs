use tinkoff_invest_types as tit;

use crate::types;

#[derive(Debug, Clone, PartialEq)]
pub struct Candlestick {
    pub open: Option<types::MoneyValue>,
    pub high: Option<types::MoneyValue>,
    pub low: Option<types::MoneyValue>,
    pub close: Option<types::MoneyValue>,
    pub volume: u64,
    pub datetime: Option<types::DateTime>,
    pub is_complete: bool,
}

impl Into<Candlestick> for &tit::HistoricCandle {
    fn into(self) -> Candlestick {
        Candlestick {
            open: self.open.as_ref().map(|x| x.into()),
            high: self.high.as_ref().map(|x| x.into()),
            low: self.low.as_ref().map(|x| x.into()),
            close: self.close.as_ref().map(|x| x.into()),
            volume: self.volume as u64,
            datetime: self.time.as_ref().map(|x| x.into()),
            is_complete: self.is_complete,
        }
    }
}
