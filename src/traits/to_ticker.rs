use crate::types;

pub trait ToTicker {
    fn to_ticker(&self) -> types::Ticker;
}

pub trait ToTickerRef {
    fn to_ticker_ref(&self) -> &types::Ticker;
}
