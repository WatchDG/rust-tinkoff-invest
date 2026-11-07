use crate::{traits, types};

/// Контекст вызова с опциональным `request_id` (x-tracking-id).
///
/// Для запросов с `account_id` используйте [`TAccountContext`].
/// Для запросов с `account_id` и `order_id` используйте [`TOrderContext`].
///
/// Компилятор гарантирует наличие обязательных полей через типы контекста.
#[derive(Debug, Clone)]
pub struct TCallContext {
    /// Идентификатор запроса (x-tracking-id), опционально.
    pub request_id: Option<String>,
}

impl TCallContext {
    /// Создаёт новый контекст без `request_id`.
    pub fn new() -> Self {
        Self { request_id: None }
    }

    /// Задаёт `request_id` (builder-style).
    pub fn with_request_id(mut self, request_id: Option<String>) -> Self {
        self.request_id = request_id;
        self
    }

    /// Устанавливает `request_id`.
    pub fn set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Создаёт [`TAccountContext`] с заданным `account_id`.
    pub fn with_account(self, account_id: impl traits::ToAccountId) -> TAccountContext {
        TAccountContext {
            request_id: self.request_id,
            account_id: account_id.to_account_id(),
        }
    }
}

impl Default for TCallContext {
    fn default() -> Self {
        Self::new()
    }
}

impl traits::RequestId for TCallContext {
    fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

/// Контекст для запросов, требующих идентификатор аккаунта
/// (portfolio, positions, operations).
#[derive(Debug, Clone)]
pub struct TAccountContext {
    /// Идентификатор запроса (x-tracking-id), опционально.
    pub request_id: Option<String>,
    /// Идентификатор аккаунта.
    pub account_id: types::AccountId,
}

impl TAccountContext {
    /// Создаёт контекст с обязательным `account_id`.
    pub fn new(account_id: impl traits::ToAccountId) -> Self {
        Self {
            request_id: None,
            account_id: account_id.to_account_id(),
        }
    }

    /// Задаёт `request_id` (builder-style).
    pub fn with_request_id(mut self, request_id: Option<String>) -> Self {
        self.request_id = request_id;
        self
    }

    /// Создаёт [`TOrderContext`] с заданным `order_id`.
    pub fn with_order(self, order_id: impl traits::ToOrderId) -> TOrderContext {
        TOrderContext {
            request_id: self.request_id,
            account_id: self.account_id,
            order_id: order_id.to_order_id(),
        }
    }
}

impl traits::RequestId for TAccountContext {
    fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl traits::ToAccountIdRef for TAccountContext {
    fn to_account_id_ref(&self) -> &types::AccountId {
        &self.account_id
    }
}

/// Контекст для запросов, требующих идентификатор аккаунта и ордера
/// (order, limit_order, cancel_order).
#[derive(Debug, Clone)]
pub struct TOrderContext {
    /// Идентификатор запроса (x-tracking-id), опционально.
    pub request_id: Option<String>,
    /// Идентификатор аккаунта.
    pub account_id: types::AccountId,
    /// Идентификатор ордера.
    pub order_id: types::OrderId,
}

impl TOrderContext {
    /// Создаёт контекст с обязательными `account_id` и `order_id`.
    pub fn new(account_id: impl traits::ToAccountId, order_id: impl traits::ToOrderId) -> Self {
        Self {
            request_id: None,
            account_id: account_id.to_account_id(),
            order_id: order_id.to_order_id(),
        }
    }

    /// Задаёт `request_id` (builder-style).
    pub fn with_request_id(mut self, request_id: Option<String>) -> Self {
        self.request_id = request_id;
        self
    }
}

impl traits::RequestId for TOrderContext {
    fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl traits::ToAccountIdRef for TOrderContext {
    fn to_account_id_ref(&self) -> &types::AccountId {
        &self.account_id
    }
}

impl traits::ToOrderIdRef for TOrderContext {
    fn to_order_id_ref(&self) -> &types::OrderId {
        &self.order_id
    }
}
