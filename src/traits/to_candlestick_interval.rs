use crate::enums;

pub trait ToCandlestickInterval {
    fn to_candlestick_interval(&self) -> enums::CandlestickInterval;
}
