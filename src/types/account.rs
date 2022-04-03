use tinkoff_invest_types as tit;

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

impl From<tit::Account> for Account {
    fn from(value: tit::Account) -> Self {
        let kind = value.r#type().into();
        let status = value.status().into();
        let access_level = value.access_level().into();
        Account {
            id: value.id,
            kind,
            name: value.name,
            status,
            access_level,
            opened_datetime: value.opened_date.map(|x| x.into()),
            closed_datetime: value.closed_date.map(|x| x.into()),
        }
    }
}

impl traits::ToAccountId for &Account {
    fn to_account_id(&self) -> String {
        self.id.clone()
    }
}
