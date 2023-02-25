use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandlestickInterval {
    Unspecified,
    Min,
    Min2,
    Min3,
    Min5,
    Min10,
    Min15,
    Min30,
    Hour,
    Hour2,
    Hour4,
    Day,
    Week,
    Month,
}

impl From<tit::CandleInterval> for CandlestickInterval {
    fn from(value: tit::CandleInterval) -> Self {
        match value {
            tit::CandleInterval::Unspecified => CandlestickInterval::Unspecified,
            tit::CandleInterval::CandleInterval1Min => CandlestickInterval::Min,
            tit::CandleInterval::CandleInterval2Min => CandlestickInterval::Min2,
            tit::CandleInterval::CandleInterval3Min => CandlestickInterval::Min3,
            tit::CandleInterval::CandleInterval5Min => CandlestickInterval::Min5,
            tit::CandleInterval::CandleInterval10Min => CandlestickInterval::Min10,
            tit::CandleInterval::CandleInterval15Min => CandlestickInterval::Min15,
            tit::CandleInterval::CandleInterval30Min => CandlestickInterval::Min30,
            tit::CandleInterval::Hour => CandlestickInterval::Hour,
            tit::CandleInterval::CandleInterval2Hour => CandlestickInterval::Hour2,
            tit::CandleInterval::CandleInterval4Hour => CandlestickInterval::Hour4,
            tit::CandleInterval::Day => CandlestickInterval::Day,
            tit::CandleInterval::Week => CandlestickInterval::Week,
            tit::CandleInterval::Month => CandlestickInterval::Month
        }
    }
}

impl From<CandlestickInterval> for tit::CandleInterval {
    fn from(value: CandlestickInterval) -> Self {
        match value {
            CandlestickInterval::Unspecified => tit::CandleInterval::Unspecified,
            CandlestickInterval::Min => tit::CandleInterval::CandleInterval1Min,
            CandlestickInterval::Min2 => tit::CandleInterval::CandleInterval2Min,
            CandlestickInterval::Min3 => tit::CandleInterval::CandleInterval3Min,
            CandlestickInterval::Min5 => tit::CandleInterval::CandleInterval5Min,
            CandlestickInterval::Min10 => tit::CandleInterval::CandleInterval10Min,
            CandlestickInterval::Min15 => tit::CandleInterval::CandleInterval15Min,
            CandlestickInterval::Min30 => tit::CandleInterval::CandleInterval30Min,
            CandlestickInterval::Hour => tit::CandleInterval::Hour,
            CandlestickInterval::Hour2 => tit::CandleInterval::CandleInterval2Hour,
            CandlestickInterval::Hour4 => tit::CandleInterval::CandleInterval4Hour,
            CandlestickInterval::Day => tit::CandleInterval::Day,
            CandlestickInterval::Week => tit::CandleInterval::Week,
            CandlestickInterval::Month => tit::CandleInterval::Month
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
