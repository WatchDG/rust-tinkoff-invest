#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentType {
    Currency,
    Share,
    Future,
    // Option,
}

impl From<String> for InstrumentType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_ref() {
            "currency" => InstrumentType::Currency,
            "share" => InstrumentType::Share,
            "future" => InstrumentType::Future,
            _ => panic!("{value:?}"),
        }
    }
}
