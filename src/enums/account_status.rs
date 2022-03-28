use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    Unspecified,
    New,
    Open,
    Closed,
}

impl Into<AccountStatus> for tit::AccountStatus {
    fn into(self) -> AccountStatus {
        match self {
            tit::AccountStatus::Unspecified => AccountStatus::Unspecified,
            tit::AccountStatus::New => AccountStatus::New,
            tit::AccountStatus::Open => AccountStatus::Open,
            tit::AccountStatus::Closed => AccountStatus::Closed,
        }
    }
}
