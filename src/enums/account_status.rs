use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountStatus {
    Unspecified,
    New,
    Open,
    Closed,
    All,
}

impl From<tit::AccountStatus> for AccountStatus {
    fn from(value: tit::AccountStatus) -> Self {
        match value {
            tit::AccountStatus::Unspecified => AccountStatus::Unspecified,
            tit::AccountStatus::New => AccountStatus::New,
            tit::AccountStatus::Open => AccountStatus::Open,
            tit::AccountStatus::Closed => AccountStatus::Closed,
            tit::AccountStatus::All => AccountStatus::All,
        }
    }
}
