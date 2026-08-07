use crate::{enums, types};

/// Типизированное событие bidirectional market-data стрима.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketDataEvent {
    CandlesSubscriptionAck {
        tracking_id: String,
    },
    OrderBookSubscriptionAck {
        tracking_id: String,
    },
    TradesSubscriptionAck {
        tracking_id: String,
    },
    InfoSubscriptionAck {
        tracking_id: String,
    },
    LastPriceSubscriptionAck {
        tracking_id: String,
    },
    Candle(types::Candlestick),
    OrderBook(types::OrderBook),
    Trade(MarketTrade),
    TradingStatus {
        instrument_uid: types::Uid,
        status: enums::TradingStatus,
    },
    LastPrice {
        instrument_uid: types::Uid,
        price: Option<types::MoneyValue>,
        datetime: Option<types::DateTime>,
    },
    /// Keep-alive от сервера (по умолчанию не пробрасывается наружу).
    Ping,
}

/// Обезличенная сделка из стрима.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketTrade {
    pub instrument_uid: types::Uid,
    pub price: Option<types::MoneyValue>,
    pub lots: u64,
    pub datetime: Option<types::DateTime>,
}

/// Описание подписки.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketDataSubscription {
    Candles {
        instrument_uid: types::Uid,
        interval: enums::CandlestickInterval,
        waiting_close: bool,
    },
    OrderBook {
        instrument_uid: types::Uid,
        depth: i32,
    },
    Trades {
        instrument_uid: types::Uid,
    },
    Info {
        instrument_uid: types::Uid,
    },
    LastPrice {
        instrument_uid: types::Uid,
    },
}

/// Исходящая команда сессии (мапится в `MarketDataRequest`).
#[derive(Debug, Clone)]
pub enum MarketDataCommand {
    Subscribe(Vec<MarketDataSubscription>),
    Unsubscribe(Vec<MarketDataSubscription>),
    GetMySubscriptions,
    Ping,
    PingSettings { ping_delay_ms: i32 },
}
