use tinkoff_invest_types as tit;

use crate::types;

/// Книга заявок.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBook {
    pub instrument_uid: types::Uid,
    /// Глубина стакана.
    pub depth: u32,
    /// Заказы на покупку книги заявок.
    pub bid_orders: Vec<types::OrderBookOrder>,
    /// Заказы на продажу книги заявок.
    pub ask_orders: Vec<types::OrderBookOrder>,
    /// Цена последней сделки.
    pub last_trade_price: Option<types::MoneyValue>,
    /// Цена закрытия.
    pub close_trade_price: Option<types::MoneyValue>,
    /// Верхний лимит цены.
    pub limit_price_up: Option<types::MoneyValue>,
    /// Нижний лимит цены.
    pub limit_price_down: Option<types::MoneyValue>,
    /// Дата и время формирования стакана на бирже.
    pub datetime: Option<types::DateTime>,
}

impl From<tit::GetOrderBookResponse> for OrderBook {
    fn from(value: tit::GetOrderBookResponse) -> Self {
        let bid_orders = value.bids.iter().map(|x| x.clone().into()).collect();
        let ask_orders = value.asks.iter().map(|x| x.clone().into()).collect();
        Self {
            // figi: value.figi.into(),
            instrument_uid: types::Uid::from(value.instrument_uid.as_str()),
            depth: value.depth as u32,
            bid_orders,
            ask_orders,
            last_trade_price: value.last_price.map(|x| x.into()),
            close_trade_price: value.close_price.map(|x| x.into()),
            limit_price_up: value.limit_up.map(|x| x.into()),
            limit_price_down: value.limit_down.map(|x| x.into()),
            datetime: value.orderbook_ts.map(|x| x.into()),
        }
    }
}

impl From<tit::OrderBook> for OrderBook {
    fn from(value: tit::OrderBook) -> Self {
        let bid_orders = value.bids.iter().map(|x| x.clone().into()).collect();
        let ask_orders = value.asks.iter().map(|x| x.clone().into()).collect();
        Self {
            instrument_uid: types::Uid::from(value.instrument_uid.as_str()),
            depth: value.depth as u32,
            bid_orders,
            ask_orders,
            last_trade_price: None,
            close_trade_price: None,
            limit_price_up: value.limit_up.map(|x| x.into()),
            limit_price_down: value.limit_down.map(|x| x.into()),
            datetime: value.time.map(|x| x.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBookOrder {
    /// Цена лота инструмента.
    pub price: Option<types::MoneyValue>,
    /// Количество лотов инструмента.
    pub lots: u64,
}

impl From<tit::Order> for OrderBookOrder {
    fn from(value: tit::Order) -> Self {
        OrderBookOrder {
            price: value.price.map(|x| x.into()),
            lots: value.quantity as u64,
        }
    }
}
