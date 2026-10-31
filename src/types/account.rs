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

impl From<&AccountId> for String {
    fn from(value: &AccountId) -> Self {
        value.0.clone()
    }
}

impl traits::ToAccountId for AccountId {
    fn to_account_id(&self) -> AccountId {
        self.clone()
    }
}

impl traits::ToAccountIdRef for AccountId {
    fn to_account_id_ref(&self) -> &AccountId {
        self
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

impl TryFrom<tit::Account> for Account {
    type Error = crate::TError;

    fn try_from(value: tit::Account) -> Result<Self, Self::Error> {
        let account_type = value.r#type().into();
        let status = value.status().into();
        let access_level = value.access_level().into();
        Ok(Account {
            id: AccountId::from(value.id),
            account_type,
            name: value.name,
            status,
            access_level,
            opened_at: value
                .opened_date
                .map(types::DateTime::try_from)
                .transpose()?,
            closed_at: value
                .closed_date
                .map(types::DateTime::try_from)
                .transpose()?,
        })
    }
}

impl traits::ToAccountId for &Account {
    fn to_account_id(&self) -> AccountId {
        self.id.clone()
    }
}

impl traits::ToAccountIdRef for Account {
    fn to_account_id_ref(&self) -> &AccountId {
        &self.id
    }
}

impl traits::ToAccountIdRef for &Account {
    fn to_account_id_ref(&self) -> &AccountId {
        &self.id
    }
}
