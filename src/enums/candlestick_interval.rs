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
            tit::CandleInterval::Month => CandlestickInterval::Month,
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
            CandlestickInterval::Month => tit::CandleInterval::Month,
        }
    }
}

impl From<&CandlestickInterval> for tit::SubscriptionInterval {
    fn from(v: &CandlestickInterval) -> Self {
        match v {
            CandlestickInterval::Unspecified => tit::SubscriptionInterval::Unspecified,
            CandlestickInterval::Min => tit::SubscriptionInterval::OneMinute,
            CandlestickInterval::Min2 => tit::SubscriptionInterval::SubscriptionInterval2Min,
            CandlestickInterval::Min3 => tit::SubscriptionInterval::SubscriptionInterval3Min,
            CandlestickInterval::Min5 => tit::SubscriptionInterval::FiveMinutes,
            CandlestickInterval::Min10 => tit::SubscriptionInterval::SubscriptionInterval10Min,
            CandlestickInterval::Min15 => tit::SubscriptionInterval::FifteenMinutes,
            CandlestickInterval::Min30 => tit::SubscriptionInterval::SubscriptionInterval30Min,
            CandlestickInterval::Hour => tit::SubscriptionInterval::OneHour,
            CandlestickInterval::Hour2 => tit::SubscriptionInterval::SubscriptionInterval2Hour,
            CandlestickInterval::Hour4 => tit::SubscriptionInterval::SubscriptionInterval4Hour,
            CandlestickInterval::Day => tit::SubscriptionInterval::OneDay,
            CandlestickInterval::Week => tit::SubscriptionInterval::Week,
            CandlestickInterval::Month => tit::SubscriptionInterval::Month,
            _ => panic!("subscription does not support interval: {v:?}"),
        }
    }
}

impl From<tit::SubscriptionInterval> for CandlestickInterval {
    fn from(v: tit::SubscriptionInterval) -> Self {
        match v {
            tit::SubscriptionInterval::Unspecified => CandlestickInterval::Unspecified,
            tit::SubscriptionInterval::OneMinute => CandlestickInterval::Min,
            tit::SubscriptionInterval::SubscriptionInterval2Min => CandlestickInterval::Min2,
            tit::SubscriptionInterval::SubscriptionInterval3Min => CandlestickInterval::Min3,
            tit::SubscriptionInterval::FiveMinutes => CandlestickInterval::Min5,
            tit::SubscriptionInterval::SubscriptionInterval10Min => CandlestickInterval::Min10,
            tit::SubscriptionInterval::FifteenMinutes => CandlestickInterval::Min15,
            tit::SubscriptionInterval::SubscriptionInterval30Min => CandlestickInterval::Min30,
            tit::SubscriptionInterval::OneHour => CandlestickInterval::Hour,
            tit::SubscriptionInterval::SubscriptionInterval2Hour => CandlestickInterval::Hour2,
            tit::SubscriptionInterval::SubscriptionInterval4Hour => CandlestickInterval::Hour4,
            tit::SubscriptionInterval::OneDay => CandlestickInterval::Day,
            tit::SubscriptionInterval::Week => CandlestickInterval::Week,
            tit::SubscriptionInterval::Month => CandlestickInterval::Month,
        }
    }
}
