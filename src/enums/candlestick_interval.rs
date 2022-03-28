use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum CandlestickInterval {
    Unspecified,
    Min1,
    Min5,
    Min15,
    Hour,
    Day,
}

impl Into<CandlestickInterval> for tit::CandleInterval {
    fn into(self) -> CandlestickInterval {
        match self {
            tit::CandleInterval::Unspecified => CandlestickInterval::Unspecified,
            tit::CandleInterval::CandleInterval1Min => CandlestickInterval::Min1,
            tit::CandleInterval::CandleInterval5Min => CandlestickInterval::Min5,
            tit::CandleInterval::CandleInterval15Min => CandlestickInterval::Min15,
            tit::CandleInterval::Hour => CandlestickInterval::Hour,
            tit::CandleInterval::Day => CandlestickInterval::Day,
        }
    }
}

impl Into<tit::CandleInterval> for CandlestickInterval {
    fn into(self) -> tit::CandleInterval {
        match self {
            CandlestickInterval::Unspecified => tit::CandleInterval::Unspecified,
            CandlestickInterval::Min1 => tit::CandleInterval::CandleInterval1Min,
            CandlestickInterval::Min5 => tit::CandleInterval::CandleInterval5Min,
            CandlestickInterval::Min15 => tit::CandleInterval::CandleInterval15Min,
            CandlestickInterval::Hour => tit::CandleInterval::Hour,
            CandlestickInterval::Day => tit::CandleInterval::Day,
        }
    }
}
