use crate::types;

#[derive(Debug, PartialEq, Eq)]
pub enum MarketDataStreamData {
    Candlestick(types::Candlestick),
    Orderbook(types::OrderBook),
}
