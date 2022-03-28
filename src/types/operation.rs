use tinkoff_invest_types;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub id: String,
    pub parent_id: Option<String>,
    pub figi: types::Figi,
    pub currency: enums::Currency,
    pub payment: Option<types::Money>,
    pub price: Option<types::Money>,
    pub state: enums::OperationState,
    pub quantity: i64,
    pub quantity_rest: i64,
    pub instrument_type: enums::InstrumentType,
    pub operation_type: enums::OperationType,
    pub trades: Vec<types::OperationTrade>,
    pub datetime: Option<types::DateTime>,
}

impl Into<Operation> for &tinkoff_invest_types::Operation {
    fn into(self) -> Operation {
        let parent_id = if self.parent_operation_id.len() > 0 {
            Some(self.parent_operation_id.clone())
        } else {
            None
        };
        let state = self.state().into();
        Operation {
            id: self.id.clone(),
            parent_id,
            figi: self.figi.clone().into(),
            currency: self.currency.clone().into(),
            payment: self.payment.as_ref().map(|x| x.into()),
            price: self.price.as_ref().map(|x| x.into()),
            state,
            quantity: self.quantity,
            quantity_rest: self.quantity_rest,
            instrument_type: self.instrument_type.clone().into(),
            operation_type: self.operation_type().into(),
            trades: self.trades.iter().map(|x| x.into()).collect(),
            datetime: self.date.as_ref().map(|x| x.into()),
        }
    }
}
