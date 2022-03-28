use crate::types::{Figi, Ticker};
use crate::{enums, types};
use std::collections::{HashMap, HashSet};

pub struct CachedMarketInstruments {
    hash_map_by_figi: HashMap<types::Figi, enums::MarketInstrument>,
    hash_map_link_ticker_figi: HashMap<types::Ticker, HashSet<types::Figi>>,
}

impl CachedMarketInstruments {
    #[inline]
    pub fn new() -> Self {
        Self {
            hash_map_by_figi: HashMap::new(),
            hash_map_link_ticker_figi: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, market_instrument: &enums::MarketInstrument) -> &Self {
        let figi: Figi = market_instrument.into();
        let ticker: Ticker = market_instrument.into();
        self.hash_map_by_figi
            .insert(figi.clone(), market_instrument.clone());
        if let Some(hash_set) = self.hash_map_link_ticker_figi.get_mut(&ticker) {
            hash_set.insert(figi.clone());
        } else {
            let mut hash_set = HashSet::new();
            hash_set.insert(figi.clone());
            self.hash_map_link_ticker_figi.insert(ticker, hash_set);
        }
        self
    }

    #[inline]
    pub fn append(&mut self, market_instruments: Vec<enums::MarketInstrument>) -> &Self {
        market_instruments.iter().for_each(|x| {
            self.insert(x);
        });
        self
    }

    #[inline]
    pub fn by_figi<T>(&self, value: T) -> Option<enums::MarketInstrument>
    where
        T: Into<types::Figi>,
    {
        let figi = value.into();
        self.hash_map_by_figi.get(&figi).map(|x| x.clone())
    }

    #[inline]
    pub fn by_ticker<T>(&self, value: T) -> Option<Vec<enums::MarketInstrument>>
    where
        T: Into<types::Ticker>,
    {
        let ticker = value.into();
        self.hash_map_link_ticker_figi.get(&ticker).map(|x| {
            x.into_iter()
                .map(|x| self.by_figi(x.clone()).unwrap())
                .collect()
        })
    }
}