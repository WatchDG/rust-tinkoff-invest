use std::ops::Add;
use tinkoff_invest_types as tit;

use crate::enums;

#[derive(Debug, Clone, PartialEq)]
pub struct MoneyValue {
    pub units: i64,
    pub nano: i32,
}

impl MoneyValue {
    #[inline]
    pub fn as_f64(&self) -> f64 {
        (self.units as f64 * 1e9 + self.nano as f64) / 1e9
    }
}

impl Add for MoneyValue {
    type Output = MoneyValue;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            units: self.units + rhs.units + ((self.nano + rhs.nano) / 1_000_000_000) as i64,
            nano: (self.nano + rhs.nano) % 1_000_000_000,
        }
    }
}

impl From<i64> for MoneyValue {
    fn from(v: i64) -> Self {
        Self { units: v, nano: 0 }
    }
}

impl From<f64> for MoneyValue {
    fn from(v: f64) -> Self {
        MoneyValue {
            units: v.trunc() as i64,
            nano: ((v * 1e10 - v.trunc() * 1e10) / 10f64) as i32,
        }
    }
}

impl From<tit::Quotation> for MoneyValue {
    fn from(value: tit::Quotation) -> Self {
        MoneyValue {
            units: value.units,
            nano: value.nano,
        }
    }
}

impl From<MoneyValue> for tit::Quotation {
    fn from(value: MoneyValue) -> Self {
        tit::Quotation {
            units: value.units,
            nano: value.nano,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    pub value: MoneyValue,
    /// Валюта
    pub currency: enums::Currency,
}

impl Money {
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.value.as_f64()
    }
}

impl From<tit::MoneyValue> for Money {
    fn from(value: tit::MoneyValue) -> Self {
        Money {
            value: MoneyValue {
                units: value.units,
                nano: value.nano,
            },
            currency: value.currency.into(),
        }
    }
}

impl From<&tit::MoneyValue> for Money {
    fn from(value: &tit::MoneyValue) -> Self {
        Money {
            value: MoneyValue {
                units: value.units,
                nano: value.nano,
            },
            currency: value.currency.clone().into(),
        }
    }
}
