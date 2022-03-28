use tinkoff_invest_types;

use crate::enums;

#[derive(Debug, Clone, PartialEq)]
pub struct MoneyValue {
    pub units: i64,
    pub nano: i32,
}

impl MoneyValue {
    #[inline]
    pub fn value(&self) -> f64 {
        self.units as f64 + (self.nano as f64) / 1e9
    }
}

impl Into<MoneyValue> for tinkoff_invest_types::Quotation {
    fn into(self) -> MoneyValue {
        MoneyValue {
            units: self.units,
            nano: self.nano,
        }
    }
}

impl Into<MoneyValue> for &tinkoff_invest_types::Quotation {
    fn into(self) -> MoneyValue {
        MoneyValue {
            units: self.units,
            nano: self.nano,
        }
    }
}

impl Into<tinkoff_invest_types::Quotation> for MoneyValue {
    fn into(self) -> tinkoff_invest_types::Quotation {
        tinkoff_invest_types::Quotation {
            units: self.units,
            nano: self.nano,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    pub value: MoneyValue,
    pub currency: enums::Currency,
}

impl Money {
    #[inline]
    pub fn value(&self) -> f64 {
        self.value.value()
    }
}

impl Into<Money> for tinkoff_invest_types::MoneyValue {
    fn into(self) -> Money {
        Money {
            value: MoneyValue {
                units: self.units,
                nano: self.nano,
            },
            currency: self.currency.into(),
        }
    }
}

impl Into<Money> for &tinkoff_invest_types::MoneyValue {
    fn into(self) -> Money {
        Money {
            value: MoneyValue {
                units: self.units,
                nano: self.nano,
            },
            currency: self.currency.clone().into(),
        }
    }
}
