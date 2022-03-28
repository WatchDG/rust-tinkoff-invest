#[derive(Debug, Clone, PartialEq)]
pub struct Isin(String);

impl Isin {
    #[inline]
    pub fn new(figi: String) -> Self {
        Self(figi)
    }
}

impl Into<Isin> for String {
    fn into(self) -> Isin {
        Isin::new(self)
    }
}

impl Into<String> for Isin {
    fn into(self) -> String {
        self.0
    }
}
