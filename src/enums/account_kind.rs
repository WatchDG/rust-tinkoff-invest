use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum AccountKind {
    Unspecified,
    Tinkoff,
    TinkoffIis,
    InvestBox,
}

impl From<tit::AccountType> for AccountKind {
    fn from(value: tit::AccountType) -> Self {
        match value {
            tit::AccountType::Unspecified => AccountKind::Unspecified,
            tit::AccountType::Tinkoff => AccountKind::Tinkoff,
            tit::AccountType::TinkoffIis => AccountKind::TinkoffIis,
            tit::AccountType::InvestBox => AccountKind::InvestBox,
        }
    }
}
