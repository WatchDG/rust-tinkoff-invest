use std::collections::HashMap;
use tinkoff_invest_types as tit;

use crate::{enums, types};

#[derive(Debug, Clone)]
pub struct Positions {
    pub money: HashMap<enums::Currency, MoneyPosition>,
    pub securities: HashMap<types::Figi, Position>,
    pub futures: HashMap<types::Figi, Position>,
}

#[derive(Debug, Clone)]
pub struct MoneyPosition {
    pub available: types::MoneyValue,
    pub blocked: types::MoneyValue,
    pub total: types::MoneyValue,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub available: i64,
    pub blocked: i64,
    pub total: i64,
}

impl From<tit::PositionsResponse> for Positions {
    fn from(v: tit::PositionsResponse) -> Self {
        let money_available: Vec<types::Money> = v.money.iter().map(|x| x.into()).collect();
        let money_blocked: Vec<types::Money> = v.blocked.iter().map(|x| x.into()).collect();

        let mut money = HashMap::with_capacity(v.money.len());
        let mut securities = HashMap::with_capacity(v.securities.len());
        let mut futures = HashMap::with_capacity(v.futures.len());

        for money_position in money_available.iter() {
            let position = MoneyPosition {
                available: money_position.value.clone(),
                blocked: types::MoneyValue::from(0),
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
                    available: types::MoneyValue::from(0),
                    blocked: money_position.value.clone(),
                    total: money_position.value.clone(),
                };
                money.insert(money_position.currency.clone(), position);
            }
        }

        for security_position in v.securities.iter() {
            let position = Position {
                available: security_position.balance,
                blocked: security_position.blocked,
                total: security_position.balance + security_position.blocked,
            };
            securities.insert(security_position.figi.clone().into(), position);
        }

        for future_position in v.futures.iter() {
            let position = Position {
                available: future_position.balance,
                blocked: future_position.blocked,
                total: future_position.balance + future_position.blocked,
            };
            futures.insert(future_position.figi.clone().into(), position);
        }

        Self {
            money,
            securities,
            futures,
        }
    }
}
