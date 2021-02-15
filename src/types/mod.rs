use serde::Deserialize;

use tinkoff_invest_types::Currency;

#[derive(Debug)]
pub struct Stock {
    pub figi: String,
    pub ticker: String,
    pub name: String,
    pub isin: String,
    pub min_price_increment: f64,
    pub lot: u64,
    pub currency: Currency,
}
