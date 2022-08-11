use std::collections::HashMap;
use tinkoff_invest_types as tit;

use crate::types::MoneyValue;
use crate::{enums, types};

#[derive(Debug, Clone)]
pub struct Positions {
    money: HashMap<enums::Currency, MoneyPosition>,
}

#[derive(Debug, Clone)]
pub struct MoneyPosition {
    pub available: types::MoneyValue,
    pub blocked: types::MoneyValue,
    pub total: types::MoneyValue,
}

impl From<tit::PositionsResponse> for Positions {
    fn from(v: tit::PositionsResponse) -> Self {
        let money_available: Vec<types::Money> = v.money.iter().map(|x| x.into()).collect();
        let money_blocked: Vec<types::Money> = v.blocked.iter().map(|x| x.into()).collect();

        let mut money = HashMap::with_capacity(v.money.len());

        for money_position in money_available.iter() {
            let position = MoneyPosition {
                available: money_position.value.clone(),
                blocked: MoneyValue::from(0f64),
                total: money_position.value.clone(),
            };
            money.insert(money_position.currency.clone(), position);
        }

        for money_position in money_blocked.iter() {
            if let Some(position) = money.get_mut(&money_position.currency) {
                position.blocked = money_position.value.clone();
                position.total = position.total.clone() + position.blocked.clone();
            } else {
                let position = MoneyPosition {
                    available: MoneyValue::from(0f64),
                    blocked: money_position.value.clone(),
                    total: money_position.value.clone(),
                };
                money.insert(money_position.currency.clone(), position);
            }
        }

        Self { money }
    }
}
