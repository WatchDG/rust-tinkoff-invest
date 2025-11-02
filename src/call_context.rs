use crate::{traits, types};

/// Контекст вызова, содержащий информацию для идентификации запроса
#[derive(Debug, Clone)]
pub struct TinkoffInvestCallContext {
    /// Идентификатор запроса (x-tracking-id) (опционально)
    pub request_id: Option<String>,
    /// Идентификатор аккаунта (опционально)
    pub account_id: Option<types::AccountId>,
    /// Идентификатор ордера (опционально)
    pub order_id: Option<types::OrderId>,
}

impl TinkoffInvestCallContext {
    /// Создает новый контекст
    pub fn new() -> Self {
        Self {
            request_id: None,
            account_id: None,
            order_id: None,
        }
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

impl traits::ToAccountId for TinkoffInvestCallContext {
    fn to_account_id(&self) -> types::AccountId {
        self.account_id
            .clone()
            .expect("account_id must be set in TinkoffInvestCallContext to use ToAccountId")
    }
}

impl traits::ToOrderId for TinkoffInvestCallContext {
    fn to_order_id(&self) -> types::OrderId {
        self.order_id
            .clone()
            .expect("order_id must be set in TinkoffInvestCallContext to use ToOrderId")
    }
}
