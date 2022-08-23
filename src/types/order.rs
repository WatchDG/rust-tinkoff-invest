use tinkoff_invest_types as tit;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl From<tit::PostOrderResponse> for Order {
    fn from(value: tit::PostOrderResponse) -> Self {
        let figi = value.figi.clone().into();
        let kind = value.order_type().into();
        let direction = value.direction().into();
        let status = value.execution_report_status().into();
        Order {
            id: value.order_id,
            figi,
            kind,
            direction,
            lots_requested: value.lots_requested as u64,
            lots_executed: value.lots_executed as u64,
            initial_price: value.initial_security_price.as_ref().map(|x| x.into()),
            initial_price_pt: value
                .initial_order_price_pt
                .as_ref()
                .map(|x| x.clone().into()),
            initial_amount: value.initial_order_price.as_ref().map(|x| x.into()),
            executed_amount: value.executed_order_price.as_ref().map(|x| x.into()),
            initial_commission: value.initial_commission.as_ref().map(|x| x.into()),
            executed_commission: value.executed_commission.as_ref().map(|x| x.into()),
            status,
        }
    }
}

impl From<tit::OrderState> for Order {
    fn from(value: tit::OrderState) -> Self {
        let figi = value.figi.clone().into();
        let kind = value.order_type().into();
        let direction = value.direction().into();
        let status = value.execution_report_status().into();
        Order {
            id: value.order_id,
            figi,
            kind,
            direction,
            lots_requested: value.lots_requested as u64,
            lots_executed: value.lots_executed as u64,
            initial_price: value.initial_security_price.as_ref().map(|x| x.into()),
            initial_price_pt: None,
            initial_amount: value.initial_order_price.as_ref().map(|x| x.into()),
            executed_amount: value.executed_order_price.as_ref().map(|x| x.into()),
            initial_commission: value.initial_commission.as_ref().map(|x| x.into()),
            executed_commission: value.executed_commission.as_ref().map(|x| x.into()),
            status,
        }
    }
}
