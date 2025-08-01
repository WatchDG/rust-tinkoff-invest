use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{enums, traits, types};

/// Потокобезопасный кэш рыночных инструментов с множественными индексами для быстрого поиска.
///
/// Структура поддерживает поиск инструментов по различным критериям:
/// - UID (уникальный идентификатор)
/// - FIGI (финансовый инструмент глобального идентификатора)
/// - Тикер
/// - Комбинация кода класса и тикера
///
/// Все операции являются потокобезопасными благодаря использованию `Arc<RwLock<...>>`.
#[derive(Debug)]
pub struct CachedMarketInstruments {
    hash_map_by_uid: Arc<RwLock<HashMap<types::Uid, types::MarketInstrument>>>,
    hash_map_by_figi: Arc<RwLock<HashMap<Option<types::Figi>, Vec<types::MarketInstrument>>>>,
    hash_map_by_ticker: Arc<RwLock<HashMap<types::Ticker, Vec<types::MarketInstrument>>>>,
    hash_map_by_class_code_ticker:
        Arc<RwLock<HashMap<(enums::ClassCode, types::Ticker), Vec<types::MarketInstrument>>>>,
}

impl CachedMarketInstruments {
    /// Создает новый пустой экземпляр кэша рыночных инструментов.
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        let hash_map_by_uid = HashMap::new();
        let hash_map_by_figi = HashMap::<Option<types::Figi>, Vec<types::MarketInstrument>>::new();
        let hash_map_by_ticker = HashMap::<types::Ticker, Vec<types::MarketInstrument>>::new();
        let hash_map_by_class_code_ticker =
            HashMap::<types::ClassCodeTicker, Vec<types::MarketInstrument>>::new();

        Self {
            hash_map_by_uid: Arc::new(RwLock::new(hash_map_by_uid)),
            hash_map_by_figi: Arc::new(RwLock::new(hash_map_by_figi)),
            hash_map_by_ticker: Arc::new(RwLock::new(hash_map_by_ticker)),
            hash_map_by_class_code_ticker: Arc::new(RwLock::new(hash_map_by_class_code_ticker)),
        }
    }

    /// Получает инструмент по UID.
    ///
    /// # Аргументы
    ///
    /// * `value` - UID инструмента, реализующий трейт `ToUidRef`
    ///
    /// # Возвращает
    ///
    /// `Option<CachedMarketInstrument>` - найденный инструмент или `None`
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let instrument = cache.get_by_uid("some_uid");
    /// ```
    #[inline]
    pub fn get_by_uid<T>(&self, value: T) -> Option<types::MarketInstrument>
    where
        T: traits::ToUidRef,
    {
        self.hash_map_by_uid
            .read()
            .unwrap()
            .get(value.to_uid_ref())
            .cloned()
    }

    /// Получает список инструментов по FIGI.
    ///
    /// # Аргументы
    ///
    /// * `value` - FIGI инструмента (может быть `None`)
    ///
    /// # Возвращает
    ///
    /// `Option<Vec<CachedMarketInstrument>>` - список найденных инструментов или `None`
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let instruments = cache.get_by_figi(Some("BBG000B9XRY4"));
    /// ```
    #[inline]
    pub fn get_by_figi<T>(
        &self,
        value: Option<types::Figi>,
    ) -> Option<Vec<types::MarketInstrument>> {
        self.hash_map_by_figi.read().unwrap().get(&value).cloned()
    }

    /// Получает список инструментов по тикеру.
    ///
    /// # Аргументы
    ///
    /// * `value` - тикер инструмента, реализующий трейт `ToTickerRef`
    ///
    /// # Возвращает
    ///
    /// `Option<Vec<CachedMarketInstrument>>` - список найденных инструментов или `None`
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let instruments = cache.get_by_ticker("AAPL");
    /// ```
    #[inline]
    pub fn get_by_ticker<T>(&self, value: T) -> Option<Vec<types::MarketInstrument>>
    where
        T: traits::ToTickerRef,
    {
        self.hash_map_by_ticker
            .read()
            .unwrap()
            .get(value.to_ticker_ref())
            .cloned()
    }

    /// Получает список инструментов по коду класса и тикеру.
    ///
    /// # Аргументы
    ///
    /// * `value` - объект, реализующий трейты `ToClassCode` и `ToTicker`
    ///
    /// # Возвращает
    ///
    /// `Option<Vec<CachedMarketInstrument>>` - список найденных инструментов или `None`
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let instruments = cache.get_by_class_code_and_ticker(some_instrument);
    /// ```
    #[inline]
    pub fn get_by_class_code_and_ticker<T>(&self, value: T) -> Option<Vec<types::MarketInstrument>>
    where
        T: traits::ToClassCode + traits::ToTicker,
    {
        let class_code_ticker = (value.to_class_code(), value.to_ticker());
        self.hash_map_by_class_code_ticker
            .read()
            .unwrap()
            .get(&class_code_ticker)
            .cloned()
    }

    /// Добавляет новый инструмент в кэш.
    ///
    /// Инструмент добавляется во все индексы для обеспечения быстрого поиска.
    ///
    /// # Аргументы
    ///
    /// * `market_instrument` - рыночный инструмент для добавления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    /// use tinkoff_invest::types::MarketInstrument;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// cache.insert(some_market_instrument);
    /// ```
    #[inline]
    pub fn insert(&self, market_instrument: types::MarketInstrument) {
        let uid = market_instrument.uid.clone();
        let figi = market_instrument.figi.clone();
        let ticker = market_instrument.ticker.clone();
        let class_code_ticker = (
            market_instrument.class_code.clone(),
            market_instrument.ticker.clone(),
        );

        {
            let mut map = self.hash_map_by_uid.write().unwrap();
            map.insert(uid, market_instrument.clone());
        }

        {
            let mut map = self.hash_map_by_figi.write().unwrap();
            if let Some(vec) = map.get_mut(&figi) {
                vec.push(market_instrument.clone());
            } else {
                map.insert(figi, vec![market_instrument.clone()]);
            }
        }

        {
            let mut map = self.hash_map_by_ticker.write().unwrap();
            if let Some(vec) = map.get_mut(&ticker) {
                vec.push(market_instrument.clone());
            } else {
                map.insert(ticker, vec![market_instrument.clone()]);
            }
        }

        {
            let mut map = self.hash_map_by_class_code_ticker.write().unwrap();
            if let Some(vec) = map.get_mut(&class_code_ticker) {
                vec.push(market_instrument);
            } else {
                map.insert(class_code_ticker, vec![market_instrument]);
            }
        }
    }

    /// Обновляет существующий инструмент в кэше.
    ///
    /// Инструмент обновляется во всех индексах. Если инструмент не найден, ничего не происходит.
    ///
    /// # Аргументы
    ///
    /// * `market_instrument` - обновленный рыночный инструмент
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    /// use tinkoff_invest::types::MarketInstrument;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// cache.update(updated_market_instrument);
    /// ```
    #[inline]
    pub fn update(&self, market_instrument: types::MarketInstrument) {
        let uid = market_instrument.uid.clone();
        let figi = market_instrument.figi.clone();
        let ticker = market_instrument.ticker.clone();
        let class_code_ticker = (
            market_instrument.class_code.clone(),
            market_instrument.ticker.clone(),
        );

        {
            let mut map = self.hash_map_by_uid.write().unwrap();
            if let Some(existing) = map.get_mut(&uid) {
                *existing = market_instrument.clone();
            }
        }

        {
            let mut map = self.hash_map_by_figi.write().unwrap();
            if let Some(vec) = map.get_mut(&figi) {
                if let Some(pos) = vec.iter().position(|item| item.uid == uid) {
                    vec[pos] = market_instrument.clone();
                }
            }
        }

        {
            let mut map = self.hash_map_by_ticker.write().unwrap();
            if let Some(vec) = map.get_mut(&ticker) {
                if let Some(pos) = vec.iter().position(|item| item.uid == uid) {
                    vec[pos] = market_instrument.clone();
                }
            }
        }

        {
            let mut map = self.hash_map_by_class_code_ticker.write().unwrap();
            if let Some(vec) = map.get_mut(&class_code_ticker) {
                if let Some(pos) = vec.iter().position(|item| item.uid == uid) {
                    vec[pos] = market_instrument;
                }
            }
        }
    }

    /// Вставляет или обновляет инструмент в кэше.
    ///
    /// Если инструмент с таким UID уже существует, он обновляется.
    /// Если не существует - добавляется новый.
    ///
    /// # Аргументы
    ///
    /// * `market_instrument` - рыночный инструмент для вставки или обновления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    /// use tinkoff_invest::types::MarketInstrument;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// cache.upsert(market_instrument);
    /// ```
    #[inline]
    pub fn upsert(&self, market_instrument: types::MarketInstrument) {
        let uid = market_instrument.uid.clone();

        let exists = {
            let map = self.hash_map_by_uid.read().unwrap();
            map.contains_key(&uid)
        };

        if exists {
            self.update(market_instrument);
        } else {
            self.insert(market_instrument);
        }
    }

    /// Удаляет инструмент по UID из кэша.
    ///
    /// Инструмент удаляется из всех индексов. Возвращает удаленный инструмент.
    ///
    /// # Аргументы
    ///
    /// * `uid` - UID инструмента для удаления, реализующий трейт `ToUidRef`
    ///
    /// # Возвращает
    ///
    /// `Option<CachedMarketInstrument>` - удаленный инструмент или `None`, если не найден
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// if let Some(deleted) = cache.delete_by_uid("some_uid") {
    ///     println!("Удален: {:?}", deleted);
    /// }
    /// ```
    #[inline]
    pub fn delete_by_uid<T>(&self, uid: T) -> Option<types::MarketInstrument>
    where
        T: traits::ToUidRef,
    {
        let uid_ref = uid.to_uid_ref();

        let instrument_to_delete = {
            let map = self.hash_map_by_uid.read().unwrap();
            map.get(uid_ref).cloned()
        };

        if let Some(instrument) = instrument_to_delete {
            let figi = instrument.figi.clone();
            let ticker = instrument.ticker.clone();
            let class_code_ticker = (instrument.class_code.clone(), instrument.ticker.clone());

            {
                let mut map = self.hash_map_by_uid.write().unwrap();
                map.remove(uid_ref);
            }

            {
                let mut map = self.hash_map_by_figi.write().unwrap();
                if let Some(vec) = map.get_mut(&figi) {
                    vec.retain(|item| item.uid != *uid_ref);
                    if vec.is_empty() {
                        map.remove(&figi);
                    }
                }
            }

            {
                let mut map = self.hash_map_by_ticker.write().unwrap();
                if let Some(vec) = map.get_mut(&ticker) {
                    vec.retain(|item| item.uid != *uid_ref);
                    if vec.is_empty() {
                        map.remove(&ticker);
                    }
                }
            }

            {
                let mut map = self.hash_map_by_class_code_ticker.write().unwrap();
                if let Some(vec) = map.get_mut(&class_code_ticker) {
                    vec.retain(|item| item.uid != *uid_ref);
                    if vec.is_empty() {
                        map.remove(&class_code_ticker);
                    }
                }
            }

            Some(instrument)
        } else {
            None
        }
    }

    /// Массово добавляет инструменты в кэш.
    ///
    /// # Аргументы
    ///
    /// * `market_instruments` - вектор рыночных инструментов для добавления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    /// use tinkoff_invest::types::MarketInstrument;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let instruments = vec![instrument1, instrument2, instrument3];
    /// cache.bulk_insert(instruments);
    /// ```
    #[inline]
    pub fn bulk_insert(&self, market_instruments: Vec<types::MarketInstrument>) {
        for market_instrument in market_instruments {
            self.insert(market_instrument);
        }
    }

    /// Массово обновляет инструменты в кэше.
    ///
    /// # Аргументы
    ///
    /// * `market_instruments` - вектор обновленных рыночных инструментов
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    /// use tinkoff_invest::types::MarketInstrument;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let updated_instruments = vec![updated1, updated2, updated3];
    /// cache.bulk_update(updated_instruments);
    /// ```
    #[inline]
    pub fn bulk_update(&self, market_instruments: Vec<types::MarketInstrument>) {
        for market_instrument in market_instruments {
            self.update(market_instrument);
        }
    }

    /// Массово вставляет или обновляет инструменты в кэше.
    ///
    /// # Аргументы
    ///
    /// * `market_instruments` - вектор рыночных инструментов для вставки или обновления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedMarketInstruments;
    /// use tinkoff_invest::types::MarketInstrument;
    ///
    /// let cache = CachedMarketInstruments::new();
    /// let instruments = vec![instrument1, instrument2, instrument3];
    /// cache.bulk_upsert(instruments);
    /// ```
    #[inline]
    pub fn bulk_upsert(&self, market_instruments: Vec<types::MarketInstrument>) {
        for market_instrument in market_instruments {
            self.upsert(market_instrument);
        }
    }
}

impl Default for CachedMarketInstruments {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<types::MarketInstrument>> for CachedMarketInstruments {
    fn from(market_instruments: Vec<types::MarketInstrument>) -> Self {
        let cache = Self::new();
        cache.bulk_insert(market_instruments);
        cache
    }
}
