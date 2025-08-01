use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{traits, types};

/// Потокобезопасный кэш торговых статусов с индексом по UID для быстрого поиска.
///
/// Структура поддерживает поиск торговых статусов по UID инструмента.
/// Все операции являются потокобезопасными благодаря использованию `Arc<RwLock<...>>`.
#[derive(Debug)]
pub struct CachedTradingStatuses {
    hash_map_by_uid: Arc<RwLock<HashMap<types::Uid, types::TradingStatus>>>,
}

impl CachedTradingStatuses {
    /// Создает новый пустой экземпляр кэша торговых статусов.
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        let hash_map_by_uid = HashMap::new();

        Self {
            hash_map_by_uid: Arc::new(RwLock::new(hash_map_by_uid)),
        }
    }

    /// Получает торговый статус по UID инструмента.
    ///
    /// # Аргументы
    ///
    /// * `value` - UID инструмента, реализующий трейт `ToUidRef`
    ///
    /// # Возвращает
    ///
    /// `Option<TradingStatus>` - найденный торговый статус или `None`
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// let status = cache.get_by_uid("some_uid");
    /// ```
    #[inline]
    pub fn get_by_uid<T>(&self, value: T) -> Option<types::TradingStatus>
    where
        T: traits::ToUidRef,
    {
        self.hash_map_by_uid
            .read()
            .unwrap()
            .get(value.to_uid_ref())
            .cloned()
    }

    /// Добавляет новый торговый статус в кэш.
    ///
    /// Торговый статус добавляется в индекс по UID для обеспечения быстрого поиска.
    ///
    /// # Аргументы
    ///
    /// * `trading_status` - торговый статус для добавления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    /// use tinkoff_invest::types::TradingStatus;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// cache.insert(some_trading_status);
    /// ```
    #[inline]
    pub fn insert(&self, trading_status: types::TradingStatus) {
        let uid = trading_status.instrument_uid.clone();

        {
            let mut map = self.hash_map_by_uid.write().unwrap();
            map.insert(uid, trading_status);
        }
    }

    /// Обновляет существующий торговый статус в кэше.
    ///
    /// Торговый статус обновляется в индексе по UID. Если торговый статус не найден, ничего не происходит.
    ///
    /// # Аргументы
    ///
    /// * `trading_status` - обновленный торговый статус
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    /// use tinkoff_invest::types::TradingStatus;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// cache.update(updated_trading_status);
    /// ```
    #[inline]
    pub fn update(&self, trading_status: types::TradingStatus) {
        let uid = trading_status.instrument_uid.clone();

        {
            let mut map = self.hash_map_by_uid.write().unwrap();
            if let Some(existing) = map.get_mut(&uid) {
                *existing = trading_status;
            }
        }
    }

    /// Вставляет или обновляет торговый статус в кэше.
    ///
    /// Если торговый статус с таким UID уже существует, он обновляется.
    /// Если не существует - добавляется новый.
    ///
    /// # Аргументы
    ///
    /// * `trading_status` - торговый статус для вставки или обновления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    /// use tinkoff_invest::types::TradingStatus;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// cache.upsert(trading_status);
    /// ```
    #[inline]
    pub fn upsert(&self, trading_status: types::TradingStatus) {
        let uid = trading_status.instrument_uid.clone();

        let exists = {
            let map = self.hash_map_by_uid.read().unwrap();
            map.contains_key(&uid)
        };

        if exists {
            self.update(trading_status);
        } else {
            self.insert(trading_status);
        }
    }

    /// Удаляет торговый статус по UID инструмента из кэша.
    ///
    /// Торговый статус удаляется из индекса по UID. Возвращает удаленный торговый статус.
    ///
    /// # Аргументы
    ///
    /// * `uid` - UID инструмента для удаления, реализующий трейт `ToUidRef`
    ///
    /// # Возвращает
    ///
    /// `Option<TradingStatus>` - удаленный торговый статус или `None`, если не найден
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// if let Some(deleted) = cache.delete_by_uid("some_uid") {
    ///     println!("Удален: {:?}", deleted);
    /// }
    /// ```
    #[inline]
    pub fn delete_by_uid<T>(&self, uid: T) -> Option<types::TradingStatus>
    where
        T: traits::ToUidRef,
    {
        let uid_ref = uid.to_uid_ref();

        let status_to_delete = {
            let map = self.hash_map_by_uid.read().unwrap();
            map.get(uid_ref).cloned()
        };

        if let Some(status) = status_to_delete {
            {
                let mut map = self.hash_map_by_uid.write().unwrap();
                map.remove(uid_ref);
            }

            Some(status)
        } else {
            None
        }
    }

    /// Массово добавляет торговые статусы в кэш.
    ///
    /// # Аргументы
    ///
    /// * `trading_statuses` - вектор торговых статусов для добавления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    /// use tinkoff_invest::types::TradingStatus;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// let statuses = vec![status1, status2, status3];
    /// cache.bulk_insert(statuses);
    /// ```
    #[inline]
    pub fn bulk_insert(&self, trading_statuses: Vec<types::TradingStatus>) {
        for trading_status in trading_statuses {
            self.insert(trading_status);
        }
    }

    /// Массово обновляет торговые статусы в кэше.
    ///
    /// # Аргументы
    ///
    /// * `trading_statuses` - вектор обновленных торговых статусов
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    /// use tinkoff_invest::types::TradingStatus;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// let updated_statuses = vec![updated1, updated2, updated3];
    /// cache.bulk_update(updated_statuses);
    /// ```
    #[inline]
    pub fn bulk_update(&self, trading_statuses: Vec<types::TradingStatus>) {
        for trading_status in trading_statuses {
            self.update(trading_status);
        }
    }

    /// Массово вставляет или обновляет торговые статусы в кэше.
    ///
    /// # Аргументы
    ///
    /// * `trading_statuses` - вектор торговых статусов для вставки или обновления
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::cached::CachedTradingStatuses;
    /// use tinkoff_invest::types::TradingStatus;
    ///
    /// let cache = CachedTradingStatuses::new();
    /// let statuses = vec![status1, status2, status3];
    /// cache.bulk_upsert(statuses);
    /// ```
    #[inline]
    pub fn bulk_upsert(&self, trading_statuses: Vec<types::TradingStatus>) {
        for trading_status in trading_statuses {
            self.upsert(trading_status);
        }
    }
}

impl Default for CachedTradingStatuses {
    fn default() -> Self {
        Self::new()
    }
}
