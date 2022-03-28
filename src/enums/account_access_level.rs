use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum AccountAccessLevel {
    Unspecified,
    FullAccess,
    ReadOnly,
    NoAccess,
}

impl Into<AccountAccessLevel> for tit::AccessLevel {
    fn into(self) -> AccountAccessLevel {
        match self {
            tit::AccessLevel::AccountAccessLevelUnspecified => AccountAccessLevel::Unspecified,
            tit::AccessLevel::AccountAccessLevelFullAccess => AccountAccessLevel::FullAccess,
            tit::AccessLevel::AccountAccessLevelReadOnly => AccountAccessLevel::ReadOnly,
            tit::AccessLevel::AccountAccessLevelNoAccess => AccountAccessLevel::NoAccess,
        }
    }
}
