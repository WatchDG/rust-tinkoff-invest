use std::convert::TryFrom;
use std::sync::Arc;

use tinkoff_invest_types as tit;

use crate::{TError, enums, types};

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

impl TryFrom<tit::Candle> for Candlestick {
    type Error = TError;

    fn try_from(value: tit::Candle) -> Result<Self, Self::Error> {
        let interval = Arc::new(value.interval().into());
        let datetime = value
            .time
            .ok_or_else(|| TError::InvalidMetadata("candle time is not set".to_string()))?;
        Ok(Self {
            instrument_uid: Arc::new(types::Uid::from_api_str(value.instrument_uid.as_str())?),
            interval,
            open: value.open.map(|x| x.into()),
            high: value.high.map(|x| x.into()),
            low: value.low.map(|x| x.into()),
            close: value.close.map(|x| x.into()),
            volume: crate::num::to_u64(value.volume)?,
            datetime: datetime.try_into()?,
            // Stream `Candle` has no completeness flag. `waiting_close`
            // subscriptions are marked complete in the stream reader.
            is_complete: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candle(volume: i64) -> tit::Candle {
        tit::Candle {
            instrument_uid: "550e8400-e29b-41d4-a716-446655440000".into(),
            volume,
            time: Some(tit::prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            ..Default::default()
        }
    }

    #[test]
    fn negative_volume_rejected() {
        let err = Candlestick::try_from(candle(-1)).unwrap_err();
        assert!(matches!(err, TError::InvalidQuantity(_)));
    }
}
