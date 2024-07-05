use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountType {
    Unspecified,
    Tinkoff,
    TinkoffIis,
    InvestBox,
    InvestFund,
}

impl From<tit::AccountType> for AccountType {
    fn from(value: tit::AccountType) -> Self {
        match value {
            tit::AccountType::Unspecified => AccountType::Unspecified,
            tit::AccountType::Tinkoff => AccountType::Tinkoff,
            tit::AccountType::TinkoffIis => AccountType::TinkoffIis,
            tit::AccountType::InvestBox => AccountType::InvestBox,
            tit::AccountType::InvestFund => AccountType::InvestFund,
        }
    }
}
