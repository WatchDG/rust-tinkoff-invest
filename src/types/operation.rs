use tinkoff_invest_types as tit;

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
            payment: value.payment.map(|x| x.into()),
            price: value.price.map(|x| x.into()),
            state,
            quantity: value.quantity,
            quantity_rest: value.quantity_rest,
            instrument_type: value.instrument_type.into(),
            operation_type,
            trades: value.trades.iter().map(|x| x.clone().into()).collect(),
            datetime: value.date.map(|x| x.into()),
        }
    }
}
