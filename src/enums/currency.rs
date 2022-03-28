#[derive(Debug, Clone, PartialEq)]
pub enum Currency {
    USD,
    RUB,
    EUR,
    CHF,
    CNY,
    Currency(String),
}

impl Into<Currency> for String {
    fn into(self) -> Currency {
        match self.to_lowercase().as_ref() {
            "usd" => Currency::USD,
            "eur" => Currency::EUR,
            "rub" => Currency::RUB,
            "chf" => Currency::CHF,
            "cny" => Currency::CNY,
            _ => Currency::Currency(self),
        }
    }
}
