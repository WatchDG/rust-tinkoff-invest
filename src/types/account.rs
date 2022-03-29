use tinkoff_invest_types;

use crate::{enums, traits, types};

#[derive(Debug, Clone)]
pub struct Account {
    /// Идентификатор.
    pub id: String,
    /// Тип.
    pub kind: enums::AccountKind,
    /// Наименование.
    pub name: String,
    /// Статус.
    pub status: enums::AccountStatus,
    /// Уровень доступа.
    pub access_level: enums::AccountAccessLevel,
    /// Дата открытия.
    pub opened_datetime: Option<types::DateTime>,
    /// Дата закрытия.
    pub closed_datetime: Option<types::DateTime>,
}

impl Into<Account> for tinkoff_invest_types::Account {
    fn into(self) -> Account {
        let kind = self.r#type().into();
        let status = self.status().into();
        let access_level = self.access_level().into();
        Account {
            id: self.id,
            kind,
            name: self.name,
            status,
            access_level,
            opened_datetime: self.opened_date.map(|x| x.into()),
            closed_datetime: self.closed_date.map(|x| x.into()),
        }
    }
}

impl Into<Account> for &tinkoff_invest_types::Account {
    fn into(self) -> Account {
        Account {
            id: self.id.clone(),
            kind: self.r#type().into(),
            name: self.name.clone(),
            access_level: self.access_level().into(),
            status: self.status().into(),
            opened_datetime: self.opened_date.as_ref().map(|x| x.into()),
            closed_datetime: self.closed_date.as_ref().map(|x| x.into()),
        }
    }
}

impl traits::ToAccountId for &Account {
    fn to_account_id(&self) -> String {
        self.id.clone()
    }
}
