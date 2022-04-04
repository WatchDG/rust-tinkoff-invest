use tinkoff_invest_types as tit;

use crate::types;

/// Книга заявок.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBook {
    /// FIGI инструмента книги заявок.
    pub figi: types::Figi,
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
    pub limit_price_up: Option<types::MoneyValue>,
    pub limit_price_down: Option<types::MoneyValue>,
}

impl From<tit::GetOrderBookResponse> for OrderBook {
    fn from(value: tit::GetOrderBookResponse) -> Self {
        let bid_orders = value.bids.iter().map(|x| x.clone().into()).collect();
        let ask_orders = value.asks.iter().map(|x| x.clone().into()).collect();
        OrderBook {
            figi: value.figi.into(),
            depth: value.depth as u32,
            bid_orders,
            ask_orders,
            last_trade_price: value.last_price.map(|x| x.into()),
            close_trade_price: value.close_price.map(|x| x.into()),
            limit_price_up: value.limit_up.map(|x| x.into()),
            limit_price_down: value.limit_down.map(|x| x.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
