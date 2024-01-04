use std::collections::HashMap;

use crate::traits;
use crate::types;

#[derive(Debug)]
pub struct CachedOrderbooks {
    inner: HashMap<types::Uid, types::OrderBook>,
}

impl CachedOrderbooks {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, orderbook: types::OrderBook) {
        self.inner
            .insert(orderbook.instrument_uid.clone(), orderbook);
    }

    pub fn remove<T>(&mut self, instrument_id: T)
    where
        T: traits::ToUidRef,
    {
        self.inner.remove(instrument_id.to_uid_ref());
    }

    pub fn get<T>(&self, instrument_id: T) -> Option<&types::OrderBook>
    where
        T: traits::ToUidRef,
    {
        self.inner.get(instrument_id.to_uid_ref())
    }
}

impl Default for CachedOrderbooks {
    fn default() -> Self {
        Self::new()
    }
}
