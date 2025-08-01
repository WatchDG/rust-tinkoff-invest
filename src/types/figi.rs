use crate::traits;

/// FIGI (Financial Instrument Global Identifier) - глобальный идентификатор финансового инструмента.
///
/// FIGI - это стандарт идентификации финансовых инструментов, разработанный Bloomberg.
/// Каждый FIGI представляет собой уникальную строку, которая идентифицирует конкретный
/// финансовый инструмент (акции, облигации, фонды и т.д.) на глобальном уровне.
///
/// # Примеры
///
/// ```rust
/// use tinkoff_invest::types::Figi;
///
/// let figi = Figi::new("BBG000B9XRY4".to_string());
/// let figi_from_str: Figi = "BBG000B9XRY4".into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Figi(String);

impl Figi {
    /// Создает новый экземпляр FIGI из строки.
    ///
    /// # Аргументы
    ///
    /// * `figi` - строка, содержащая FIGI идентификатор
    ///
    /// # Примеры
    ///
    /// ```rust
    /// use tinkoff_invest::types::Figi;
    ///
    /// let figi = Figi::new("BBG000B9XRY4".to_string());
    /// ```
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
