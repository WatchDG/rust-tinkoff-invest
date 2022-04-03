#[derive(Debug, Clone, PartialEq)]
pub enum InstrumentType {
    Bond,
    Share,
    Currency,
    Etf,
    Futures,
    InstrumentType(String),
}

impl From<String> for InstrumentType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_ref() {
            "bond" => InstrumentType::Bond,
            "share" => InstrumentType::Share,
            "currency" => InstrumentType::Currency,
            "etf" => InstrumentType::Etf,
            "futures" => InstrumentType::Futures,
            _ => InstrumentType::InstrumentType(value),
        }
    }
}
