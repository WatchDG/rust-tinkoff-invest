use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountType {
    Unspecified,
    Tinkoff,
    TinkoffIis,
    InvestBox,
    InvestFund,
    Debit,
    Saving,
    Dfa,
    SharedSaving,
    Deposit,
    Oms,
    OfpDeposit,
}

impl From<tit::AccountType> for AccountType {
    fn from(value: tit::AccountType) -> Self {
        match value {
            tit::AccountType::Unspecified => AccountType::Unspecified,
            tit::AccountType::Tinkoff => AccountType::Tinkoff,
            tit::AccountType::TinkoffIis => AccountType::TinkoffIis,
            tit::AccountType::InvestBox => AccountType::InvestBox,
            tit::AccountType::InvestFund => AccountType::InvestFund,
            tit::AccountType::Debit => AccountType::Debit,
            tit::AccountType::Saving => AccountType::Saving,
            tit::AccountType::Dfa => AccountType::Dfa,
            tit::AccountType::SharedSaving => AccountType::SharedSaving,
            tit::AccountType::Deposit => AccountType::Deposit,
            tit::AccountType::Oms => AccountType::Oms,
            tit::AccountType::OfpDeposit => AccountType::OfpDeposit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_proto_variants() {
        assert_eq!(
            AccountType::from(tit::AccountType::Tinkoff),
            AccountType::Tinkoff
        );
        assert_eq!(AccountType::from(tit::AccountType::Dfa), AccountType::Dfa);
        assert_eq!(
            AccountType::from(tit::AccountType::Saving),
            AccountType::Saving
        );
        assert_eq!(
            AccountType::from(tit::AccountType::SharedSaving),
            AccountType::SharedSaving
        );
        assert_eq!(
            AccountType::from(tit::AccountType::OfpDeposit),
            AccountType::OfpDeposit
        );
    }
}
