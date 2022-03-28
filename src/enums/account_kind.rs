use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum AccountKind {
    Unspecified,
    Tinkoff,
    TinkoffIis,
    InvestBox,
}

impl Into<AccountKind> for tit::AccountType {
    fn into(self) -> AccountKind {
        match self {
            tit::AccountType::Unspecified => AccountKind::Unspecified,
            tit::AccountType::Tinkoff => AccountKind::Tinkoff,
            tit::AccountType::TinkoffIis => AccountKind::TinkoffIis,
            tit::AccountType::InvestBox => AccountKind::InvestBox,
        }
    }
}
