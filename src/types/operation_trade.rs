use tinkoff_invest_types as tit;

use crate::types;

#[derive(Debug, Clone, PartialEq)]
pub struct OperationTrade {
    pub id: String,
    pub quantity: i64,
    pub price: Option<types::Money>,
    pub datetime: Option<types::DateTime>,
}

impl From<tit::OperationTrade> for OperationTrade {
    fn from(value: tit::OperationTrade) -> Self {
        OperationTrade {
            id: value.trade_id,
            quantity: value.quantity,
            price: value.price.map(|x| x.into()),
            datetime: value.date_time.map(|x| x.into()),
        }
    }
}
