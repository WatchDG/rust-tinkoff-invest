use std::collections::HashMap;

use crate::types;

#[derive(Debug)]
pub struct CachedOrderbook {
    inner: HashMap<types::Figi, types::OrderBook>,
}

impl CachedOrderbook {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, orderbook: types::OrderBook) {
        self.inner.insert(orderbook.figi.clone(), orderbook);
    }

    pub fn remove(&mut self, figi: &types::Figi) {
        self.inner.remove(figi);
    }

    pub fn get(&self, figi: &types::Figi) -> Option<&types::OrderBook> {
        self.inner.get(figi)
    }
}

impl Default for CachedOrderbook {
    fn default() -> Self {
        Self::new()
    }
}
