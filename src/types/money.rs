use std::ops::Add;
use tinkoff_invest_types as tit;

use crate::enums;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        let mut units = self.units + rhs.units;
        let mut nano = self.nano + rhs.nano;

        if nano >= 1_000_000_000 {
            units += 1;
            nano -= 1_000_000_000;
        } else if nano <= -1_000_000_000 {
            units -= 1;
            nano += 1_000_000_000;
        }

        if units >= 1 && nano < 0 {
            units -= 1;
            nano += 1_000_000_000;
        } else if units <= -1 && nano > 0 {
            units += 1;
            nano -= 1_000_000_000;
        }

        Self { units, nano }
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

impl From<tit::MoneyValue> for MoneyValue {
    fn from(value: tit::MoneyValue) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use crate::types::MoneyValue;

    #[test]
    fn test_1() {
        let a = MoneyValue {
            units: 1,
            nano: 100_000_000,
        };
        let b = MoneyValue {
            units: 0,
            nano: 900_000_000,
        };
        let c = a + b;
        assert_eq!(c.units, 2);
        assert_eq!(c.nano, 0)
    }

    #[test]
    fn test_2() {
        let a = MoneyValue {
            units: 1,
            nano: 100_000_000,
        };
        let b = MoneyValue {
            units: -0,
            nano: -900_000_000,
        };
        let c = a + b;
        assert_eq!(c.units, 0);
        assert_eq!(c.nano, 200_000_000);
    }

    #[test]
    fn test_3() {
        let a = MoneyValue {
            units: 0,
            nano: 100_000_000,
        };
        let b = MoneyValue {
            units: -0,
            nano: -900_000_000,
        };
        let c = a + b;
        assert_eq!(c.units, 0);
        assert_eq!(c.nano, -800_000_000);
    }

    #[test]
    fn test_4() {
        let a = MoneyValue {
            units: -0,
            nano: -100_000_000,
        };
        let b = MoneyValue {
            units: -0,
            nano: -900_000_000,
        };
        let c = a + b;
        assert_eq!(c.units, -1);
        assert_eq!(c.nano, 0);
    }

    #[test]
    fn test_5() {
        let a = MoneyValue {
            units: 0,
            nano: 900_000_000,
        };
        let b = MoneyValue {
            units: -1,
            nano: -700_000_000,
        };
        let c = a + b;
        assert_eq!(c.units, 0);
        assert_eq!(c.nano, -800_000_000);
    }
}
