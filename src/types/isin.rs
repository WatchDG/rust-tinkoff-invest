#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Isin(String);

impl Isin {
    #[inline]
    pub fn new(figi: String) -> Self {
        Self(figi)
    }
}

impl From<&str> for Isin {
    fn from(value: &str) -> Self {
        Isin::new(value.into())
    }
}

impl From<String> for Isin {
    fn from(value: String) -> Self {
        Isin::new(value)
    }
}

impl From<Isin> for String {
    fn from(value: Isin) -> Self {
        value.0
    }
}
