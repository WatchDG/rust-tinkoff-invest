use crate::types;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Figi(String);

impl Figi {
    #[inline]
    pub fn new(figi: String) -> Self {
        Self(figi)
    }
}

impl Into<Figi> for String {
    fn into(self) -> Figi {
        Figi(self)
    }
}

impl Into<String> for Figi {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Figi> for types::MarketInstrument {
    fn into(self) -> Figi {
        self.figi
    }
}
