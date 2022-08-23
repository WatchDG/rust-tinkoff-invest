use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandlestickInterval {
    Unspecified,
    Min,
    Min5,
    Min15,
    Hour,
    Day,
}

impl From<tit::CandleInterval> for CandlestickInterval {
    fn from(value: tit::CandleInterval) -> Self {
        match value {
            tit::CandleInterval::Unspecified => CandlestickInterval::Unspecified,
            tit::CandleInterval::CandleInterval1Min => CandlestickInterval::Min,
            tit::CandleInterval::CandleInterval5Min => CandlestickInterval::Min5,
            tit::CandleInterval::CandleInterval15Min => CandlestickInterval::Min15,
            tit::CandleInterval::Hour => CandlestickInterval::Hour,
            tit::CandleInterval::Day => CandlestickInterval::Day,
        }
    }
}

impl From<CandlestickInterval> for tit::CandleInterval {
    fn from(value: CandlestickInterval) -> Self {
        match value {
            CandlestickInterval::Unspecified => tit::CandleInterval::Unspecified,
            CandlestickInterval::Min => tit::CandleInterval::CandleInterval1Min,
            CandlestickInterval::Min5 => tit::CandleInterval::CandleInterval5Min,
            CandlestickInterval::Min15 => tit::CandleInterval::CandleInterval15Min,
            CandlestickInterval::Hour => tit::CandleInterval::Hour,
            CandlestickInterval::Day => tit::CandleInterval::Day,
        }
    }
}

impl From<&CandlestickInterval> for tit::SubscriptionInterval {
    fn from(v: &CandlestickInterval) -> Self {
        match v {
            CandlestickInterval::Unspecified => tit::SubscriptionInterval::Unspecified,
            CandlestickInterval::Min => tit::SubscriptionInterval::OneMinute,
            CandlestickInterval::Min5 => tit::SubscriptionInterval::FiveMinutes,
            _ => panic!("subscription does not support interval: {:?}", v),
        }
    }
}

impl From<tit::SubscriptionInterval> for CandlestickInterval {
    fn from(v: tit::SubscriptionInterval) -> Self {
        match v {
            tit::SubscriptionInterval::Unspecified => CandlestickInterval::Unspecified,
            tit::SubscriptionInterval::OneMinute => CandlestickInterval::Min,
            tit::SubscriptionInterval::FiveMinutes => CandlestickInterval::Min5,
        }
    }
}
