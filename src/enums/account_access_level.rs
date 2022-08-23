use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountAccessLevel {
    Unspecified,
    FullAccess,
    ReadOnly,
    NoAccess,
}

impl From<tit::AccessLevel> for AccountAccessLevel {
    fn from(value: tit::AccessLevel) -> Self {
        match value {
            tit::AccessLevel::AccountAccessLevelUnspecified => AccountAccessLevel::Unspecified,
            tit::AccessLevel::AccountAccessLevelFullAccess => AccountAccessLevel::FullAccess,
            tit::AccessLevel::AccountAccessLevelReadOnly => AccountAccessLevel::ReadOnly,
            tit::AccessLevel::AccountAccessLevelNoAccess => AccountAccessLevel::NoAccess,
        }
    }
}
