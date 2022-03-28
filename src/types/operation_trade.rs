use tinkoff_invest_types;

use crate::types;

#[derive(Debug, Clone, PartialEq)]
pub struct OperationTrade {
    pub id: String,
    pub quantity: i64,
    pub price: Option<types::Money>,
    pub datetime: Option<types::DateTime>,
}

impl Into<OperationTrade> for tinkoff_invest_types::OperationTrade {
    fn into(self) -> OperationTrade {
        OperationTrade {
            id: self.trade_id.clone(),
            quantity: self.quantity,
            price: self.price.map(|x| x.into()),
            datetime: self.date_time.map(|x| x.into()),
        }
    }
}

impl Into<OperationTrade> for &tinkoff_invest_types::OperationTrade {
    fn into(self) -> OperationTrade {
        OperationTrade {
            id: self.trade_id.clone(),
            quantity: self.quantity,
            price: self.price.as_ref().map(|x| x.into()),
            datetime: self.date_time.as_ref().map(|x| x.into()),
        }
    }
}
