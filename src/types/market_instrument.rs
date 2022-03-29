use tinkoff_invest_types as tit;

use crate::enums::MarketInstrumentKind;
use crate::types::Figi;
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

impl Into<MarketInstrument> for tit::Currency {
    fn into(self) -> MarketInstrument {
        let trading_status = self.trading_status().into();
        MarketInstrument {
            figi: self.figi.into(),
            isin: self.isin.into(),
            ticker: self.ticker.into(),
            class_code: self.class_code.into(),
            kind: enums::MarketInstrumentKind::Currency,
            name: self.name,
            lot: self.lot as u64,
            currency: self.currency.into(),
            min_price_increment: self.min_price_increment.map(|x| x.into()),
            trading_status,
            is_api_trade_available: self.api_trade_available_flag,
            is_buy_available: self.buy_available_flag,
            is_sell_available: self.sell_available_flag,
        }
    }
}

impl Into<MarketInstrument> for tit::Share {
    fn into(self) -> MarketInstrument {
        let trading_status = self.trading_status().into();
        MarketInstrument {
            figi: self.figi.into(),
            isin: self.isin.into(),
            ticker: self.ticker.into(),
            class_code: self.class_code.into(),
            kind: enums::MarketInstrumentKind::Share,
            name: self.name,
            lot: self.lot as u64,
            currency: self.currency.into(),
            min_price_increment: self.min_price_increment.map(|x| x.into()),
            trading_status,
            is_api_trade_available: self.api_trade_available_flag,
            is_buy_available: self.buy_available_flag,
            is_sell_available: self.sell_available_flag,
        }
    }
}

impl traits::ToFigi for &MarketInstrument {
    fn to_figi(&self) -> Figi {
        self.figi.clone()
    }
}

impl traits::ToMarketInstrumentKind for &MarketInstrument {
    fn to_market_instrument_kind(&self) -> MarketInstrumentKind {
        self.kind.clone()
    }
}
