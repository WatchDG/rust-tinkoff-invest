use std::collections::{HashMap, VecDeque};

use crate::{types, TinkoffInvestError};

pub struct CachedCandlesticksBucket {
    inner: VecDeque<types::Candlestick>,
    last_datetime: Option<types::DateTime>,
    limit: usize,
}

impl CachedCandlesticksBucket {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            last_datetime: None,
            limit: 0,
        }
    }

    pub fn set_limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    #[inline]
    pub fn push(&mut self, candlestick: types::Candlestick) {
        if candlestick.datetime < self.last_datetime {
            return;
        }

        if candlestick.datetime == self.last_datetime {
            *self.inner.back_mut().unwrap() = candlestick;
            return;
        }

        if self.limit > 0 && self.len() >= self.limit {
            self.inner.pop_front();
        }

        self.last_datetime = candlestick.datetime.clone();
        self.inner.push_back(candlestick);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get_last_n(&mut self, n: usize) -> Option<&[types::Candlestick]> {
        let len = self.len();
        if len < n {
            return None;
        }
        Some(&self.inner.make_contiguous()[(len - n)..len])
    }
}

impl Default for CachedCandlesticksBucket {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CachedCandlesticks {
    inner: HashMap<types::Figi, CachedCandlesticksBucket>,
}

impl CachedCandlesticks {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    #[inline]
    pub fn create_bucket(&mut self, figi: &types::Figi) {
        if !self.inner.contains_key(figi) {
            let bucket = CachedCandlesticksBucket::new();
            self.inner.insert(figi.clone(), bucket);
        }
    }

    #[inline]
    pub fn get_bucket_mut(
        &mut self,
        figi: &types::Figi,
    ) -> Result<&mut CachedCandlesticksBucket, TinkoffInvestError> {
        self.inner
            .get_mut(figi)
            .ok_or(TinkoffInvestError::FigiNotFound)
    }

    #[inline]
    pub fn push(&mut self, candlestick: types::Candlestick) -> Result<(), TinkoffInvestError> {
        let figi = candlestick
            .figi
            .as_ref()
            .ok_or(TinkoffInvestError::FigiNotSet)?;
        let bucket = self.get_bucket_mut(figi)?;
        bucket.push(candlestick);
        Ok(())
    }
}

impl Default for CachedCandlesticks {
    fn default() -> Self {
        Self::new()
    }
}
