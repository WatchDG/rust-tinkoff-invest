use tinkoff_invest_types as tit;

use crate::{enums, traits, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountId(String);

impl From<String> for AccountId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<AccountId> for String {
    fn from(value: AccountId) -> Self {
        value.0
    }
}

impl traits::ToAccountId for AccountId {
    fn to_account_id(&self) -> AccountId {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    /// Идентификатор аккаунта.
    pub id: AccountId,
    /// Тип аккаунта.
    pub account_type: enums::AccountType,
    /// Наименование аккаунта.
    pub name: String,
    /// Статус аккаунта.
    pub status: enums::AccountStatus,
    /// Уровень доступа аккаунта.
    pub access_level: enums::AccountAccessLevel,
    /// Дата открытия аккаунта.
    pub opened_at: Option<types::DateTime>,
    /// Дата закрытия аккаунта.
    pub closed_at: Option<types::DateTime>,
}

impl From<tit::Account> for Account {
    fn from(value: tit::Account) -> Self {
        let account_type = value.r#type().into();
        let status = value.status().into();
        let access_level = value.access_level().into();
        Account {
            id: AccountId::from(value.id),
            account_type,
            name: value.name,
            status,
            access_level,
            opened_at: value.opened_date.map(|x| x.into()),
            closed_at: value.closed_date.map(|x| x.into()),
        }
    }
}

impl traits::ToAccountId for &Account {
    fn to_account_id(&self) -> AccountId {
        self.id.clone()
    }
}
