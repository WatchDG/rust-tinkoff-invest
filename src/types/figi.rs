use crate::traits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Figi(String);

impl Figi {
    #[inline]
    pub fn new(figi: String) -> Self {
        Self(figi)
    }
}

impl From<&str> for Figi {
    fn from(value: &str) -> Self {
        Figi::new(value.into())
    }
}

impl From<String> for Figi {
    fn from(value: String) -> Self {
        Figi::new(value)
    }
}

impl From<Figi> for String {
    fn from(value: Figi) -> Self {
        value.0
    }
}

impl traits::ToFigi for &Figi {
    fn to_figi(&self) -> Figi {
        Figi::new(self.0.clone())
    }
}

impl traits::ToFigiRef for &Figi {
    fn to_figi_ref(&self) -> &Figi {
        self
    }
}
