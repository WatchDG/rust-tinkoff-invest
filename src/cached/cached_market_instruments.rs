use std::collections::{HashMap, HashSet};

use crate::{enums, traits, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedMarketInstruments {
    hash_map_by_figi: HashMap<types::Figi, types::MarketInstrument>,
    hash_map_link_ticker_figi: HashMap<types::Ticker, HashSet<types::Figi>>,
    hash_map_link_class_code_ticker_figi: HashMap<(enums::ClassCode, types::Ticker), types::Figi>,
}

impl CachedMarketInstruments {
    #[inline]
    pub fn new() -> Self {
        Self {
            hash_map_by_figi: HashMap::new(),
            hash_map_link_ticker_figi: HashMap::new(),
            hash_map_link_class_code_ticker_figi: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, market_instrument: types::MarketInstrument) -> &Self {
        let figi = market_instrument.figi.clone();
        let class_code = market_instrument.class_code.clone();
        let ticker = market_instrument.ticker.clone();

        // hash_map_by_figi
        self.hash_map_by_figi
            .insert(figi.clone(), market_instrument);

        // hash_map_link_ticker_figi
        if let Some(hash_set) = self.hash_map_link_ticker_figi.get_mut(&ticker) {
            hash_set.insert(figi.clone());
        } else {
            let mut hash_set = HashSet::new();
            hash_set.insert(figi.clone());
            self.hash_map_link_ticker_figi
                .insert(ticker.clone(), hash_set);
        }

        // hash_map_link_class_code_ticker_figi
        self.hash_map_link_class_code_ticker_figi
            .insert((class_code, ticker), figi);

        self
    }

    #[inline]
    pub fn append(&mut self, market_instruments: Vec<types::MarketInstrument>) -> &Self {
        for market_instrument in market_instruments {
            self.insert(market_instrument);
        }
        self
    }

    #[inline]
    pub fn by_figi<T>(&self, value: T) -> Option<types::MarketInstrument>
    where
        T: traits::ToFigi,
    {
        let figi = value.to_figi();
        self.hash_map_by_figi.get(&figi).cloned()
    }

    #[inline]
    pub fn by_ticker<T>(&self, value: T) -> Option<Vec<types::MarketInstrument>>
    where
        T: traits::ToTicker,
    {
        let ticker = value.to_ticker();
        self.hash_map_link_ticker_figi
            .get(&ticker)
            .map(|x| x.iter().map(|x| self.by_figi(x).unwrap()).collect())
    }

    #[inline]
    pub fn by_class_code_and_ticker<T>(&self, value: T) -> Option<types::MarketInstrument>
    where
        T: traits::ToClassCode + traits::ToTicker,
    {
        let class_code = value.to_class_code();
        let ticker = value.to_ticker();
        let key = (class_code, ticker);
        self.hash_map_link_class_code_ticker_figi
            .get(&key)
            .map(|x| self.by_figi(x).unwrap())
    }
}

impl Default for CachedMarketInstruments {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<types::MarketInstrument>> for CachedMarketInstruments {
    fn from(values: Vec<types::MarketInstrument>) -> Self {
        let mut cached_market_instruments = CachedMarketInstruments::new();
        cached_market_instruments.append(values);
        cached_market_instruments
    }
}
