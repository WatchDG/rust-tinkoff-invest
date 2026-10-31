use std::convert::TryFrom;

use tinkoff_invest_types as tit;

use crate::{TError, types};

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

impl TryFrom<tit::GetOrderBookResponse> for OrderBook {
    type Error = TError;

    fn try_from(value: tit::GetOrderBookResponse) -> Result<Self, Self::Error> {
        let mut bid_orders = Vec::with_capacity(value.bids.len());
        for x in &value.bids {
            bid_orders.push((*x).try_into()?);
        }
        let mut ask_orders = Vec::with_capacity(value.asks.len());
        for x in &value.asks {
            ask_orders.push((*x).try_into()?);
        }
        Ok(Self {
            instrument_uid: types::Uid::from_api_str(value.instrument_uid.as_str())?,
            depth: crate::num::to_u32(value.depth)?,
            bid_orders,
            ask_orders,
            last_trade_price: value.last_price.map(|x| x.into()),
            close_trade_price: value.close_price.map(|x| x.into()),
            limit_price_up: value.limit_up.map(|x| x.into()),
            limit_price_down: value.limit_down.map(|x| x.into()),
            datetime: value
                .orderbook_ts
                .map(types::DateTime::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<tit::OrderBook> for OrderBook {
    type Error = TError;

    fn try_from(value: tit::OrderBook) -> Result<Self, Self::Error> {
        let mut bid_orders = Vec::with_capacity(value.bids.len());
        for x in &value.bids {
            bid_orders.push((*x).try_into()?);
        }
        let mut ask_orders = Vec::with_capacity(value.asks.len());
        for x in &value.asks {
            ask_orders.push((*x).try_into()?);
        }
        Ok(Self {
            instrument_uid: types::Uid::from_api_str(value.instrument_uid.as_str())?,
            depth: crate::num::to_u32(value.depth)?,
            bid_orders,
            ask_orders,
            last_trade_price: None,
            close_trade_price: None,
            limit_price_up: value.limit_up.map(|x| x.into()),
            limit_price_down: value.limit_down.map(|x| x.into()),
            datetime: value.time.map(types::DateTime::try_from).transpose()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBookOrder {
    /// Цена лота инструмента.
    pub price: Option<types::MoneyValue>,
    /// Количество лотов инструмента.
    pub lots: u64,
}

impl TryFrom<tit::Order> for OrderBookOrder {
    type Error = TError;

    fn try_from(value: tit::Order) -> Result<Self, Self::Error> {
        Ok(OrderBookOrder {
            price: value.price.map(|x| x.into()),
            lots: crate::num::to_u64(value.quantity)?,
        })
    }
}
