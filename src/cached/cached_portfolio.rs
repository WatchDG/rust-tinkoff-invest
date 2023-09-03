use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::types;

pub struct CachedPortfolio {
    inner: HashMap<types::Uid, Arc<RwLock<types::PortfolioPosition>>>,
}

impl CachedPortfolio {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    #[inline]
    pub fn get(&self, instrument_id: &types::Uid) -> Option<Arc<RwLock<types::PortfolioPosition>>> {
        self.inner.get(instrument_id).map(Arc::clone)
    }

    #[inline]
    pub fn insert(&mut self, portfolio_position: types::PortfolioPosition) {
        let key = portfolio_position.instrument_uid.clone();
        let value = Arc::new(RwLock::new(portfolio_position));
        self.inner.insert(key, value);
    }

    #[inline]
    pub fn update(&mut self, portfolio_position: types::PortfolioPosition) {
        let key = portfolio_position.instrument_uid.clone();
        if let Some(lock) = self.inner.get(&key) {
            let mut value = lock.write().unwrap();
            value.quantity = portfolio_position.quantity;
        }
    }

    #[inline]
    pub fn upsert(&mut self, portfolio_position: types::PortfolioPosition) {
        let key = portfolio_position.instrument_uid.clone();
        if self.inner.get(&key).is_some() {
            self.update(portfolio_position);
        } else {
            self.insert(portfolio_position)
        }
    }

    pub fn bulk_upsert(&mut self, portfolio_positions: Vec<types::PortfolioPosition>) {
        for portfolio_position in portfolio_positions {
            self.upsert(portfolio_position)
        }
    }
}

impl Default for CachedPortfolio {
    fn default() -> Self {
        CachedPortfolio::new()
    }
}
