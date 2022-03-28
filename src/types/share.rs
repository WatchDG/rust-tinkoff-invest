use tinkoff_invest_types;

use crate::{enums, types};

#[derive(Debug, Clone, PartialEq)]
pub struct Share {
    pub figi: types::Figi,
    pub ticker: types::Ticker,
    pub name: String,
    pub lot: u64,
    pub currency: enums::Currency,
    // pub exchange: enums::Exchange,
    // pub issue_size: u64,
    // pub trading_status: enums::TradingStatus,
    // pub is_buy_available: bool,
    // pub is_sell_available: bool,
    // pub is_api_trade_available: bool,
    // pub min_price_increment: Option<types::MoneyValue>,
}

// impl From<&tinkoff_invest_types::Share> for Share {
//     fn from(share: &tinkoff_invest_types::Share) -> Self {
//         Self {
//             figi: share.figi.clone().into(),
//             ticker: share.ticker.clone().into(),
//             lot: share.lot.clone() as u64,
//             currency: share.currency.clone().into(),
//             name: share.name.clone(),
//             exchange: share.exchange.clone().into(),
//             issue_size: share.issue_size.clone() as u64,
//             trading_status: share.trading_status().into(),
//             is_buy_available: share.buy_available_flag.clone(),
//             is_sell_available: share.sell_available_flag.clone(),
//             is_api_trade_available: share.api_trade_available_flag.clone(),
//             min_price_increment: share.min_price_increment.as_ref().map(|x| x.into()),
//         }
//     }
// }

impl Into<Share> for tinkoff_invest_types::Share {
    fn into(self) -> Share {
        Share {
            figi: self.figi.into(),
            ticker: self.ticker.into(),
            name: self.name,
            lot: self.lot as u64,
            currency: self.currency.into(),
        }
    }
}

impl Into<Share> for &tinkoff_invest_types::Share {
    fn into(self) -> Share {
        Share {
            figi: self.figi.clone().into(),
            ticker: self.ticker.clone().into(),
            name: self.name.clone(),
            lot: self.lot as u64,
            currency: self.currency.clone().into(),
        }
    }
}
