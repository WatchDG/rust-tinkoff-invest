use crate::{enums, traits, types};

pub type ClassCodeTicker = (enums::ClassCode, types::Ticker);

impl traits::ToClassCode for &ClassCodeTicker {
    fn to_class_code(&self) -> enums::ClassCode {
        self.0.clone()
    }
}

impl traits::ToTicker for &ClassCodeTicker {
    fn to_ticker(&self) -> types::Ticker {
        self.1.clone()
    }
}
