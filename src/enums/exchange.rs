#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exchange {
    SPB,
    MOEX,
    NYSE,
    NASDAQ,
    OTCUS,
    LSE,
    XETRA,
    FX,
    Exchange(String),
}

impl From<String> for Exchange {
    fn from(exchange: String) -> Self {
        match exchange.to_lowercase().as_ref() {
            "spb" => Exchange::SPB,
            "spb_morning" => Exchange::SPB,
            "spb_de" => Exchange::SPB,
            "spb_ru_morning" => Exchange::SPB,
            "moex" => Exchange::MOEX,
            "moex_morning" => Exchange::MOEX,
            "nyse" => Exchange::NYSE,
            "nasdaq" => Exchange::NASDAQ,
            "otcus" => Exchange::OTCUS,
            "lse" => Exchange::LSE,
            "xetra" => Exchange::XETRA,
            "fx" => Exchange::FX,
            _ => Exchange::Exchange(exchange),
        }
    }
}
