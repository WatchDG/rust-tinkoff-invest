use std::convert::TryFrom;

use tinkoff_invest_types as tit;

use crate::{TError, enums, traits, types};

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
    pub risk_rate_long: Option<types::MoneyValue>,
    pub risk_rate_short: Option<types::MoneyValue>,
    pub future_asset: Option<String>,
    pub future_asset_size: Option<types::MoneyValue>,
    pub future_expiration_date: Option<types::DateTime>,
    pub option_strike_price: Option<types::MoneyValue>,
    pub option_expiration_date: Option<types::DateTime>,
    pub is_api_trade_available: bool,
    pub is_buy_available: bool,
    pub is_sell_available: bool,
}

impl TryFrom<tit::Currency> for MarketInstrument {
    type Error = TError;

    fn try_from(value: tit::Currency) -> Result<Self, Self::Error> {
        let trading_status = value.trading_status().into();
        Ok(Self {
            uid: types::Uid::from_api_str(value.uid.as_str())?,
            figi: Some(value.figi.into()),
            isin: Some(value.isin.into()),
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Currency,
            name: value.name,
            lot_size: crate::num::to_u64(value.lot)?,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            risk_rate_long: None,
            risk_rate_short: None,
            future_asset: None,
            future_asset_size: None,
            future_expiration_date: None,
            option_strike_price: None,
            option_expiration_date: None,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        })
    }
}

impl TryFrom<tit::Share> for MarketInstrument {
    type Error = TError;

    fn try_from(value: tit::Share) -> Result<Self, Self::Error> {
        let trading_status = value.trading_status().into();
        Ok(Self {
            uid: types::Uid::from_api_str(value.uid.as_str())?,
            figi: Some(value.figi.into()),
            isin: Some(value.isin.into()),
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Share,
            name: value.name,
            lot_size: crate::num::to_u64(value.lot)?,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            risk_rate_long: None,
            risk_rate_short: None,
            future_asset: None,
            future_asset_size: None,
            future_expiration_date: None,
            option_strike_price: None,
            option_expiration_date: None,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        })
    }
}

impl TryFrom<tit::Future> for MarketInstrument {
    type Error = TError;

    fn try_from(value: tit::Future) -> Result<Self, Self::Error> {
        let trading_status = value.trading_status().into();
        Ok(Self {
            uid: types::Uid::from_api_str(value.uid.as_str())?,
            figi: Some(value.figi.into()),
            isin: None,
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Future,
            name: value.name,
            lot_size: crate::num::to_u64(value.lot)?,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            risk_rate_long: None,
            risk_rate_short: None,
            future_asset: Some(value.basic_asset),
            future_asset_size: value.basic_asset_size.map(|x| x.into()),
            future_expiration_date: value
                .expiration_date
                .map(types::DateTime::try_from)
                .transpose()?,
            option_strike_price: None,
            option_expiration_date: None,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        })
    }
}

impl TryFrom<tit::Option> for MarketInstrument {
    type Error = TError;

    fn try_from(value: tit::Option) -> Result<Self, Self::Error> {
        let trading_status = value.trading_status().into();
        Ok(Self {
            uid: types::Uid::from_api_str(value.uid.as_str())?,
            figi: None,
            isin: None,
            ticker: value.ticker.into(),
            class_code: value.class_code.into(),
            instrument_type: enums::InstrumentType::Option,
            name: value.name,
            lot_size: crate::num::to_u64(value.lot)?,
            currency: value.currency.into(),
            min_price_increment: value.min_price_increment.map(|x| x.into()),
            trading_status,
            risk_rate_long: None,
            risk_rate_short: None,
            future_asset: None,
            future_asset_size: None,
            future_expiration_date: None,
            option_strike_price: value.strike_price.map(|x| x.into()),
            option_expiration_date: value
                .expiration_date
                .map(types::DateTime::try_from)
                .transpose()?,
            is_api_trade_available: value.api_trade_available_flag,
            is_buy_available: value.buy_available_flag,
            is_sell_available: value.sell_available_flag,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_is_not_future() {
        let option = tit::Option {
            uid: "550e8400-e29b-41d4-a716-446655440000".into(),
            lot: 1,
            ..Default::default()
        };
        let instrument = MarketInstrument::try_from(option).unwrap();
        assert_eq!(instrument.instrument_type, enums::InstrumentType::Option);
    }
}
