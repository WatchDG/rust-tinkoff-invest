use std::convert::TryFrom;

use tinkoff_invest_types as tit;

use crate::{TError, types};

/// Сделка
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    /// Идентификатор сделки.
    pub id: String,
    /// Количество лотов инструмента сделки.
    pub lots: u64,
    /// Цена лота инструмента сделки.
    pub price: Option<types::Money>,
    /// Дата и время совершения сделки.
    pub datetime: Option<types::DateTime>,
}

impl TryFrom<tit::OperationTrade> for Trade {
    type Error = TError;

    fn try_from(value: tit::OperationTrade) -> Result<Self, Self::Error> {
        Ok(Trade {
            id: value.trade_id,
            lots: crate::num::to_u64(value.quantity)?,
            price: value.price.map(|x| x.into()),
            datetime: value.date_time.map(types::DateTime::try_from).transpose()?,
        })
    }
}
