use tinkoff_invest_types;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    /// Идентификатор.
    pub id: String,
    /// Глобальный идентификатор финансового инструмента.
    pub figi: types::Figi,
    /// Тип.
    pub kind: enums::OrderKind,
    pub direction: enums::OrderDirection,
    /// Кол-во запрощенных лотов.
    pub lots_requested: u64,
    /// Кол-во исполненных лотов.
    pub lots_executed: u64,
    /// Начальная цена.
    pub initial_price: Option<types::Money>,
    /// Начальная цена в пунктах (для фьючерсов).
    pub initial_price_pt: Option<types::MoneyValue>,
    /// Начальная сумма.
    pub initial_amount: Option<types::Money>,
    pub executed_amount: Option<types::Money>,
    pub initial_commission: Option<types::Money>,
    pub executed_commission: Option<types::Money>,
    /// Статус исполнения заявки
    pub status: enums::OrderStatus,
}

impl Into<Order> for tinkoff_invest_types::PostOrderResponse {
    fn into(self) -> Order {
        Order {
            id: self.order_id.clone(),
            figi: self.figi.clone().into(),
            kind: self.order_type().into(),
            direction: self.direction().into(),
            lots_requested: self.lots_requested as u64,
            lots_executed: self.lots_executed as u64,
            initial_price: self.initial_security_price.as_ref().map(|x| x.into()),
            initial_price_pt: self.initial_order_price_pt.as_ref().map(|x| x.into()),
            initial_amount: self.initial_order_price.as_ref().map(|x| x.into()),
            executed_amount: self.executed_order_price.as_ref().map(|x| x.into()),
            initial_commission: self.initial_commission.as_ref().map(|x| x.into()),
            executed_commission: self.executed_commission.as_ref().map(|x| x.into()),
            status: self.execution_report_status().into(),
        }
    }
}

impl Into<Order> for tinkoff_invest_types::OrderState {
    fn into(self) -> Order {
        Order {
            id: self.order_id.clone(),
            kind: self.order_type().into(),
            figi: self.figi.clone().into(),
            direction: self.direction().into(),
            lots_requested: self.lots_requested as u64,
            lots_executed: self.lots_executed as u64,
            initial_price: self.initial_security_price.as_ref().map(|x| x.into()),
            initial_price_pt: None,
            initial_amount: self.initial_order_price.as_ref().map(|x| x.into()),
            executed_amount: self.executed_order_price.as_ref().map(|x| x.into()),
            initial_commission: self.initial_commission.as_ref().map(|x| x.into()),
            executed_commission: self.executed_commission.as_ref().map(|x| x.into()),
            status: self.execution_report_status().into(),
        }
    }
}
