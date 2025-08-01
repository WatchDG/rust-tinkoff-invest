/// ISIN (International Securities Identification Number) - международный номер идентификации ценных бумаг.
///
/// ISIN - это 12-значный буквенно-цифровой код, который уникально идентифицирует ценные бумаги
/// (акции, облигации, опционы, фьючерсы и т.д.) на международном уровне. Формат ISIN состоит из:
/// - 2 буквы кода страны (ISO 3166-1 alpha-2)
/// - 9 символов национального идентификатора
/// - 1 контрольная цифра
///
/// ISIN используется для стандартизации идентификации ценных бумаг в международной торговле
/// и является обязательным требованием для многих финансовых операций.
///
/// # Примеры
///
/// ```rust
/// use tinkoff_invest::types::Isin;
///
/// let isin = Isin::new("US0378331005".to_string()); // Apple Inc.
/// let isin_from_str: Isin = "RU000A0JX0J2".into(); // ОФЗ
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Isin(String);

impl Isin {
    /// Создает новый экземпляр ISIN из строки.
    ///
    /// # Аргументы
    ///
    /// * `figi` - строка, содержащая ISIN идентификатор (12 символов)
    ///
    /// # Примеры
    ///
    /// ```rust
    /// use tinkoff_invest::types::Isin;
    ///
    /// let isin = Isin::new("US0378331005".to_string());
    /// ```
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
