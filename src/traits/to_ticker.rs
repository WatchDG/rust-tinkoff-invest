use crate::types;

pub trait ToTicker {
    fn to_ticker(&self) -> types::Ticker;
}
