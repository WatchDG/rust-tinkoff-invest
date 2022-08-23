use tinkoff_invest_types as tit;

use crate::types;

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

impl From<tit::OperationTrade> for Trade {
    fn from(value: tit::OperationTrade) -> Self {
        Trade {
            id: value.trade_id,
            lots: value.quantity as u64,
            price: value.price.map(|x| x.into()),
            datetime: value.date_time.map(|x| x.into()),
        }
    }
}
