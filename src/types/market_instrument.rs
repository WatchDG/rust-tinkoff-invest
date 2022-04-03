use tinkoff_invest_types as tit;

use crate::{enums, traits, types};

#[derive(Debug, Clone, PartialEq)]
pub struct MarketInstrument {
    pub figi: types::Figi,
    pub isin: types::Isin,
    pub ticker: types::Ticker,
    pub class_code: enums::ClassCode,
    pub kind: enums::MarketInstrumentKind,
    pub name: String,
    pub lot: u64,
    pub currency: enums::Currency,
    pub min_price_increment: Option<types::MoneyValue>,
    pub trading_status: enums::TradingStatus,
    pub is_api_trade_available: bool,
    pub is_buy_available: bool,
    pub is_sell_available: bool,
}

impl From<tit::Currency> for MarketInstrument {
    fn from(value: tit::Currency) -> Self {
        let trading_status = value.trading_status().into();
        MarketInstrument {
            figi: value.figi.into(),
            isin: value.isin.into(),
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            kind: enums::MarketInstrumentKind::Currency,
            name: value.name,
            lot: value.lot as u64,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        }
    }
}

impl From<tit::Share> for MarketInstrument {
    fn from(value: tit::Share) -> Self {
        let trading_status = value.trading_status().into();
        MarketInstrument {
            figi: value.figi.into(),
            isin: value.isin.into(),
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            kind: enums::MarketInstrumentKind::Share,
            name: value.name,
            lot: value.lot as u64,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        }
    }
}

impl traits::ToFigi for &MarketInstrument {
    fn to_figi(&self) -> types::Figi {
        self.figi.clone()
    }
}

impl traits::ToMarketInstrumentKind for &MarketInstrument {
    fn to_market_instrument_kind(&self) -> enums::MarketInstrumentKind {
        self.kind.clone()
    }
}

impl traits::ToClassCode for &MarketInstrument {
    fn to_class_code(&self) -> enums::ClassCode {
        self.class_code.clone()
    }
}
