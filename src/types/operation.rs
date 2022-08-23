use tinkoff_invest_types as tit;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    /// Идентификатор операции.
    pub id: String,
    /// Идентификатор родительской операции.
    pub parent_id: Option<String>,
    /// FIGI инструмента операции.
    pub figi: types::Figi,
    /// Количество лотов инструмента операции.
    pub lots: u64,
    /// Неисполненное количество лотов инструмента операции.
    pub lots_rest: u64,
    /// Цена лота инструмента операции.
    pub price: Option<types::Money>,
    /// Суммарная стоимость операции.
    pub total: Option<types::Money>,
    /// Валюта операции.
    pub currency: enums::Currency,
    /// Состояние операции.
    pub state: enums::OperationState,
    /// Тип операции.
    pub operation_type: enums::OperationType,
    /// Сделки операции.
    pub trades: Vec<types::Trade>,
    /// Дата и время совершения операции.
    pub datetime: Option<types::DateTime>,
}

impl From<tit::Operation> for Operation {
    fn from(value: tit::Operation) -> Self {
        let parent_id = if !value.parent_operation_id.is_empty() {
            Some(value.parent_operation_id.clone())
        } else {
            None
        };
        let state = value.state().into();
        let operation_type = value.operation_type().into();
        Operation {
            id: value.id,
            parent_id,
            figi: value.figi.into(),
            currency: value.currency.into(),
            total: value.payment.map(|x| x.into()),
            price: value.price.map(|x| x.into()),
            state,
            lots: value.quantity as u64,
            lots_rest: value.quantity_rest as u64,
            operation_type,
            trades: value.trades.iter().map(|x| x.clone().into()).collect(),
            datetime: value.date.map(|x| x.into()),
        }
    }
}
