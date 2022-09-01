use crate::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketDataStreamData {
    Candlestick(types::Candlestick),
    Orderbook(types::OrderBook),
}
