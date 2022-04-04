#[derive(Debug, Clone, PartialEq)]
pub enum Currency {
    USD,
    RUB,
    EUR,
    CHF,
    CNY,
    GBP,
    JPY,
    HKD,
    SEK,
    Currency(String),
}

impl From<String> for Currency {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_ref() {
            "usd" => Currency::USD,
            "eur" => Currency::EUR,
            "rub" => Currency::RUB,
            "chf" => Currency::CHF,
            "cny" => Currency::CNY,
            "gbp" => Currency::GBP,
            "jpy" => Currency::JPY,
            "hkd" => Currency::HKD,
            "sek" => Currency::SEK,
            _ => Currency::Currency(value),
        }
    }
}
