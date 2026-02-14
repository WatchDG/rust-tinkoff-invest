use crate::{traits, types};

/// Контекст вызова, содержащий информацию для идентификации запроса
#[derive(Debug, Clone)]
pub struct TCallContext {
    /// Идентификатор запроса (x-tracking-id) (опционально)
    pub request_id: Option<String>,
    /// Идентификатор аккаунта (опционально)
    pub account_id: Option<types::AccountId>,
    /// Идентификатор ордера (опционально)
    pub order_id: Option<types::OrderId>,
}

impl TCallContext {
    /// Создает новый контекст
    pub fn new() -> Self {
        Self {
            request_id: None,
            account_id: None,
            order_id: None,
        }
    }

    /// Устанавливает request_id
    pub fn set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Устанавливает account_id из типа, реализующего ToAccountId
    pub fn set_account_id<T>(&mut self, account: Option<T>) -> &mut Self
    where
        T: traits::ToAccountId,
    {
        self.account_id = account.map(|a| a.to_account_id());
        self
    }

    /// Устанавливает order_id из типа, реализующего ToOrderId
    pub fn set_order_id<T>(&mut self, order: Option<T>) -> &mut Self
    where
        T: traits::ToOrderId,
    {
        self.order_id = order.map(|o| o.to_order_id());
        self
    }
}

impl traits::ToAccountId for TCallContext {
    fn to_account_id(&self) -> types::AccountId {
        self.account_id
            .clone()
            .expect("account_id must be set in TCallContext to use ToAccountId")
    }
}

impl traits::ToAccountIdRef for TCallContext {
    fn to_account_id_ref(&self) -> &types::AccountId {
        self.account_id
            .as_ref()
            .expect("account_id must be set in TCallContext to use ToAccountIdRef")
    }
}

impl traits::ToOrderId for TCallContext {
    fn to_order_id(&self) -> types::OrderId {
        self.order_id
            .clone()
            .expect("order_id must be set in TCallContext to use ToOrderId")
    }
}

impl traits::ToOrderIdRef for TCallContext {
    fn to_order_id_ref(&self) -> &types::OrderId {
        self.order_id
            .as_ref()
            .expect("order_id must be set in TCallContext to use ToOrderIdRef")
    }
}

impl Default for TCallContext {
    fn default() -> Self {
        Self::new()
    }
}
