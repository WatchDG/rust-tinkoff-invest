use uuid::Uuid;

use crate::{traits, types};

/// Контекст вызова, содержащий информацию для идентификации запроса
#[derive(Debug, Clone)]
pub struct TinkoffInvestCallContext {
    /// Идентификатор запроса (x-tracking-id)
    pub request_id: String,
    /// Идентификатор аккаунта (опционально)
    pub account_id: Option<types::AccountId>,
}

impl TinkoffInvestCallContext {
    /// Создает новый контекст с заданным request_id или автоматически генерирует его
    pub fn new(request_id: Option<String>, account_id: Option<types::AccountId>) -> Self {
        Self {
            request_id: request_id.unwrap_or_else(|| Uuid::now_v7().to_string()),
            account_id,
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
}

impl traits::ToAccountId for TinkoffInvestCallContext {
    fn to_account_id(&self) -> types::AccountId {
        self.account_id
            .clone()
            .expect("account_id must be set in TinkoffInvestCallContext to use ToAccountId")
    }
}
