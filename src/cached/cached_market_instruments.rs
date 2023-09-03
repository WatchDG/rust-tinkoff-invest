use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{enums, traits, types};

pub type CachedMarketInstrument = Arc<RwLock<types::MarketInstrument>>;

#[derive(Debug)]
pub struct CachedMarketInstruments {
    hash_map_by_uid: HashMap<types::Uid, CachedMarketInstrument>,
    hash_map_by_figi: HashMap<Option<types::Figi>, Arc<Vec<CachedMarketInstrument>>>,
    hash_map_by_ticker: HashMap<types::Ticker, Arc<Vec<CachedMarketInstrument>>>,
    hash_map_by_class_code_ticker:
        HashMap<(enums::ClassCode, types::Ticker), Arc<Vec<CachedMarketInstrument>>>,
}

impl CachedMarketInstruments {
    #[inline]
    pub fn new(market_instruments: Vec<types::MarketInstrument>) -> Self {
        let mut hash_map_by_uid = HashMap::new();
        let mut hash_map_by_figi =
            HashMap::<Option<types::Figi>, Vec<CachedMarketInstrument>>::new();
        let mut hash_map_by_ticker = HashMap::<types::Ticker, Vec<CachedMarketInstrument>>::new();
        let mut hash_map_by_class_code_ticker =
            HashMap::<types::ClassCodeTicker, Vec<CachedMarketInstrument>>::new();

        for market_instrument in market_instruments {
            let uid = market_instrument.uid.clone();
            let figi = market_instrument.figi.clone();
            let ticker = market_instrument.ticker.clone();
            let class_code_ticker = (
                market_instrument.class_code.clone(),
                market_instrument.ticker.clone(),
            );

            let cached_market_instrument = Arc::new(RwLock::new(market_instrument));

            if let Some(vec) = hash_map_by_figi.get_mut(&figi) {
                vec.push(cached_market_instrument.clone());
            } else {
                let vec = vec![cached_market_instrument.clone()];
                hash_map_by_figi.insert(figi, vec);
            }

            if let Some(vec) = hash_map_by_ticker.get_mut(&ticker) {
                vec.push(cached_market_instrument.clone());
            } else {
                let vec = vec![cached_market_instrument.clone()];
                hash_map_by_ticker.insert(ticker, vec);
            }

            if let Some(vec) = hash_map_by_class_code_ticker.get_mut(&class_code_ticker) {
                vec.push(cached_market_instrument.clone());
            } else {
                let vec = vec![cached_market_instrument.clone()];
                hash_map_by_class_code_ticker.insert(class_code_ticker, vec);
            }

            hash_map_by_uid.insert(uid, cached_market_instrument);
        }

        let mut _hash_map_by_figi = HashMap::new();
        for (k, v) in hash_map_by_figi {
            _hash_map_by_figi.insert(k, Arc::new(v));
        }

        let mut _hash_map_by_ticker = HashMap::new();
        for (k, v) in hash_map_by_ticker {
            _hash_map_by_ticker.insert(k, Arc::new(v));
        }

        let mut _hash_map_by_class_code_ticker = HashMap::new();
        for (k, v) in hash_map_by_class_code_ticker {
            _hash_map_by_class_code_ticker.insert(k, Arc::new(v));
        }

        Self {
            hash_map_by_uid,
            hash_map_by_figi: _hash_map_by_figi,
            hash_map_by_ticker: _hash_map_by_ticker,
            hash_map_by_class_code_ticker: _hash_map_by_class_code_ticker,
        }
    }

    #[inline]
    pub fn get_by_uid<T>(&self, value: T) -> Option<CachedMarketInstrument>
    where
        T: traits::ToUidRef,
    {
        self.hash_map_by_uid.get(value.to_uid_ref()).cloned()
    }

    #[inline]
    pub fn get_by_figi<T>(
        &self,
        value: Option<types::Figi>,
    ) -> Option<Arc<Vec<CachedMarketInstrument>>> {
        self.hash_map_by_figi.get(&value).cloned()
    }

    #[inline]
    pub fn get_by_ticker<T>(&self, value: T) -> Option<Arc<Vec<CachedMarketInstrument>>>
    where
        T: traits::ToTickerRef,
    {
        self.hash_map_by_ticker.get(value.to_ticker_ref()).cloned()
    }

    #[inline]
    pub fn get_by_class_code_and_ticker<T>(
        &self,
        value: T,
    ) -> Option<Arc<Vec<CachedMarketInstrument>>>
    where
        T: traits::ToClassCode + traits::ToTicker,
    {
        let class_code_ticker = (value.to_class_code(), value.to_ticker());
        self.hash_map_by_class_code_ticker
            .get(&class_code_ticker)
            .cloned()
    }
}
