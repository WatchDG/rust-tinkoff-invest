use crate::{enums, types};

use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradingStatus {
    pub instrument_uid: types::Uid,
    pub status: enums::TradingStatus,
    pub datetime: types::DateTime,
}

impl TradingStatus {
    pub fn new(
        instrument_uid: types::Uid,
        status: enums::TradingStatus,
        datetime: Option<types::DateTime>,
    ) -> Self {
        Self {
            instrument_uid,
            status,
            datetime: datetime.unwrap_or_default(),
        }
    }
}

impl From<&tit::TradingStatus> for TradingStatus {
    fn from(value: &tit::TradingStatus) -> Self {
        Self::new(
            value.instrument_uid.clone().as_str().into(),
            value.trading_status().into(),
            value.time.map(|time| time.into()),
        )
    }
}
