use tinkoff_invest_types;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    pub figi: types::Figi,
    pub ticker: types::Ticker,
    pub name: String,
    pub class_code: enums::ClassCode,
    pub lot: u64,
    pub currency: enums::Currency,
    pub exchange: enums::Exchange,
    pub trading_status: enums::TradingStatus,
    pub is_buy_available: bool,
    pub is_sell_available: bool,
    pub is_api_trade_available: bool,
    pub min_price_increment: Option<types::MoneyValue>,
}

impl From<&tinkoff_invest_types::Currency> for Currency {
    fn from(currency: &tinkoff_invest_types::Currency) -> Self {
        Self {
            figi: currency.figi.clone().into(),
            ticker: currency.ticker.clone().into(),
            class_code: currency.class_code.clone().into(),
            lot: currency.lot.clone() as u64,
            currency: currency.currency.clone().into(),
            name: currency.name.clone(),
            exchange: currency.exchange.clone().into(),
            trading_status: currency.trading_status().into(),
            is_buy_available: currency.buy_available_flag.clone(),
            is_sell_available: currency.sell_available_flag.clone(),
            is_api_trade_available: currency.api_trade_available_flag.clone(),
            min_price_increment: currency.min_price_increment.as_ref().map(|x| x.into()),
        }
    }
}
