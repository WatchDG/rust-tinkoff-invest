use std::convert::TryFrom;

use tinkoff_invest_types as tit;

use crate::{TError, enums, types};

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

impl TryFrom<tit::Operation> for Operation {
    type Error = TError;

    fn try_from(value: tit::Operation) -> Result<Self, Self::Error> {
        let parent_id = if !value.parent_operation_id.is_empty() {
            Some(value.parent_operation_id.clone())
        } else {
            None
        };
        let state = value.state().into();
        let operation_type = value.operation_type().into();
        Ok(Operation {
            id: value.id,
            parent_id,
            figi: value.figi.into(),
            currency: value.currency.into(),
            total: value.payment.map(|x| x.into()),
            price: value.price.map(|x| x.into()),
            state,
            lots: crate::num::to_u64(value.quantity)?,
            lots_rest: crate::num::to_u64(value.quantity_rest)?,
            operation_type,
            trades: {
                let mut trades = Vec::with_capacity(value.trades.len());
                for x in &value.trades {
                    trades.push(x.clone().try_into()?);
                }
                trades
            },
            datetime: value.date.map(types::DateTime::try_from).transpose()?,
        })
    }
}
