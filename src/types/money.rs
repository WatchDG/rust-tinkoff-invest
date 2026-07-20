use std::ops::Add;
use tinkoff_invest_types as tit;

use crate::TError;
use crate::enums;

/// Денежное значение в формате units + nano (1 unit = 10^9 nano).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyValue {
    pub units: i64,
    pub nano: i32,
}

impl MoneyValue {
    /// Нормализует nano в диапазон `(-1e9, 1e9)` с переносом в units.
    pub fn normalize(mut self) -> Self {
        const NANO: i32 = 1_000_000_000;
        if self.nano >= NANO || self.nano <= -NANO {
            let carry = self.nano / NANO;
            self.units += i64::from(carry);
            self.nano -= carry * NANO;
        }
        if self.units > 0 && self.nano < 0 {
            self.units -= 1;
            self.nano += NANO;
        } else if self.units < 0 && self.nano > 0 {
            self.units += 1;
            self.nano -= NANO;
        }
        self
    }

    #[inline]
    pub fn as_f64(&self) -> f64 {
        (self.units as f64 * 1e9 + self.nano as f64) / 1e9
    }

    /// Преобразует `f64` в `MoneyValue`.
    ///
    /// Возвращает ошибку для NaN и бесконечностей.
    pub fn try_from_f64(value: f64) -> Result<Self, TError> {
        if !value.is_finite() {
            return Err(TError::InvalidMoneyValue(format!(
                "value must be finite, got {value}"
            )));
        }
        let units = value.trunc() as i64;
        let nano = ((value - value.trunc()) * 1e9).round() as i32;
        Ok(Self { units, nano }.normalize())
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

/// Деньги с валютой.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub value: MoneyValue,
    /// Валюта.
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
    use crate::TError;
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

    #[test]
    fn try_from_f64_ok() {
        let v = MoneyValue::try_from_f64(12.345678901).unwrap();
        assert_eq!(v.units, 12);
        assert_eq!(v.nano, 345_678_901);
        assert!((v.as_f64() - 12.345678901).abs() < 1e-9);
    }

    #[test]
    fn try_from_f64_negative() {
        let v = MoneyValue::try_from_f64(-1.5).unwrap();
        assert_eq!(v.units, -1);
        assert_eq!(v.nano, -500_000_000);
    }

    #[test]
    fn try_from_f64_rejects_nan() {
        let err = MoneyValue::try_from_f64(f64::NAN).unwrap_err();
        assert!(matches!(err, TError::InvalidMoneyValue(_)));
    }

    #[test]
    fn normalize_carry() {
        let v = MoneyValue {
            units: 0,
            nano: 1_500_000_000,
        }
        .normalize();
        assert_eq!(v.units, 1);
        assert_eq!(v.nano, 500_000_000);
    }
}
