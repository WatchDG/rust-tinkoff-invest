use std::collections::HashMap;
use tinkoff_invest_types::Currency;

#[derive(Debug, Clone)]
pub struct Stock {
    pub figi: String,
    pub ticker: String,
    pub name: String,
    pub isin: String,
    pub min_price_increment: f64,
    pub lot: u64,
    pub currency: Currency,
}

#[derive(Debug, Clone)]
pub struct StocksInfo {
    stocks: Vec<Stock>,
    hash_map_by_ticker: HashMap<String, Stock>,
    hash_map_by_figi: HashMap<String, Stock>,
    hash_map_by_isin: HashMap<String, Stock>,
}

impl StocksInfo {
    pub fn new(stocks: Vec<Stock>) -> StocksInfo {
        let mut hash_map_by_ticker = HashMap::<String, Stock>::with_capacity(stocks.len());
        let mut hash_map_by_figi = HashMap::<String, Stock>::with_capacity(stocks.len());
        let mut hash_map_by_isin = HashMap::<String, Stock>::with_capacity(stocks.len());

        stocks.iter().for_each(|stock| {
            hash_map_by_ticker.insert(stock.clone().ticker, stock.clone());
            hash_map_by_figi.insert(stock.clone().figi, stock.clone());
            hash_map_by_isin.insert(stock.clone().isin, stock.clone());
        });

        StocksInfo {
            stocks,
            hash_map_by_ticker,
            hash_map_by_figi,
            hash_map_by_isin,
        }
    }

    pub fn by_ticker(&self, ticker: &str) -> Option<&Stock> {
        self.hash_map_by_ticker.get(ticker)
    }

    pub fn by_figi(&self, figi: &str) -> Option<&Stock> {
        self.hash_map_by_figi.get(figi)
    }

    pub fn by_isin(&self, isin: &str) -> Option<&Stock> {
        self.hash_map_by_isin.get(isin)
    }
}
