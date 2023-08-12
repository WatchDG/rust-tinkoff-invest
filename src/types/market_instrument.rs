use tinkoff_invest_types as tit;

use crate::{enums, traits, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketInstrument {
    pub uid: types::Uid,
    pub figi: Option<types::Figi>,
    pub isin: Option<types::Isin>,
    pub ticker: types::Ticker,
    pub class_code: enums::ClassCode,
    pub instrument_type: enums::InstrumentType,
    pub name: String,
    pub lot_size: u64,
    pub currency: enums::Currency,
    pub min_price_increment: Option<types::MoneyValue>,
    pub trading_status: enums::TradingStatus,
    pub option_strike_price: Option<types::MoneyValue>,
    pub option_expiration_date: Option<types::DateTime>,
    pub is_api_trade_available: bool,
    pub is_buy_available: bool,
    pub is_sell_available: bool,
}

impl From<tit::Currency> for MarketInstrument {
    fn from(value: tit::Currency) -> Self {
        let trading_status = value.trading_status().into();
        Self {
            uid: value.uid.as_str().into(),
            figi: Some(value.figi.into()),
            isin: Some(value.isin.into()),
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Currency,
            name: value.name,
            lot_size: value.lot as u64,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            option_strike_price: None,
            option_expiration_date: None,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        }
    }
}

impl From<tit::Share> for MarketInstrument {
    fn from(value: tit::Share) -> Self {
        let trading_status = value.trading_status().into();
        Self {
            uid: value.uid.as_str().into(),
            figi: Some(value.figi.into()),
            isin: Some(value.isin.into()),
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Share,
            name: value.name,
            lot_size: value.lot as u64,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            option_strike_price: None,
            option_expiration_date: None,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        }
    }
}

impl From<tit::Future> for MarketInstrument {
    fn from(value: tit::Future) -> Self {
        let trading_status = value.trading_status().into();
        Self {
            uid: value.uid.as_str().into(),
            figi: Some(value.figi.into()),
            isin: None,
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Future,
            name: value.name,
            lot_size: value.lot as u64,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            option_strike_price: None,
            option_expiration_date: None,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        }
    }
}

impl From<tit::Option> for MarketInstrument {
    fn from(value: tit::Option) -> Self {
        let trading_status = value.trading_status().into();
        Self {
            uid: value.uid.as_str().into(),
            figi: None,
            isin: None,
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Future,
            name: value.name,
            lot_size: value.lot as u64,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            option_strike_price: value.strike_price.map(|x| x.into()),
            option_expiration_date: value.expiration_date.map(|x| x.into()),
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        }
    }
}

impl traits::ToUid for &MarketInstrument {
    fn to_uid(&self) -> types::Uid {
        self.uid.clone()
    }
}

impl traits::ToInstrumentType for &MarketInstrument {
    fn to_instrument_type(&self) -> enums::InstrumentType {
        self.instrument_type.clone()
    }
}

impl traits::ToClassCode for &MarketInstrument {
    fn to_class_code(&self) -> enums::ClassCode {
        self.class_code.clone()
    }
}
