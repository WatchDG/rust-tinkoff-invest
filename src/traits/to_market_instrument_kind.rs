use crate::enums;

pub trait ToMarketInstrumentKind {
    fn to_market_instrument_kind(&self) -> enums::MarketInstrumentKind;
}
