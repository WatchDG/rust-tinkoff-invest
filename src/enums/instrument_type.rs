#[derive(Debug, Clone, PartialEq)]
pub enum InstrumentType {
    Bond,
    Share,
    Currency,
    Etf,
    Futures,
    InstrumentType(String),
}

impl Into<InstrumentType> for String {
    fn into(self) -> InstrumentType {
        match self.to_lowercase().as_ref() {
            "bond" => InstrumentType::Bond,
            "share" => InstrumentType::Share,
            "currency" => InstrumentType::Currency,
            "etf" => InstrumentType::Etf,
            "futures" => InstrumentType::Futures,
            _ => InstrumentType::InstrumentType(self),
        }
    }
}
