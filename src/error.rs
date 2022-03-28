use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TinkoffInvestError {
    InterceptorNotSet,
    AccountNotSet,
    UsersServiceClientNotInit,
    InstrumentsServiceClientNotInit,
    MarketDataServiceClientNotInit,
    OperationsServiceClientNotInit,
    OrdersServiceClientNotInit,
    MarketInstrumentKindNotCurrency,
    MarketInstrumentKindNotShare,
}

impl Display for TinkoffInvestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TinkoffInvestError::InterceptorNotSet => {
                write!(f, "{}", "Interceptor not set.")
            }
            TinkoffInvestError::AccountNotSet => {
                write!(f, "{}", "Account not set.")
            }
            TinkoffInvestError::UsersServiceClientNotInit => {
                write!(f, "{}", "Users service client not init.")
            }
            TinkoffInvestError::InstrumentsServiceClientNotInit => {
                write!(f, "{}", "Instruments service client not init.")
            }
            TinkoffInvestError::MarketDataServiceClientNotInit => {
                write!(f, "{}", "Market data service client not init.")
            }
            TinkoffInvestError::OperationsServiceClientNotInit => {
                write!(f, "{}", "Operations service client not init.")
            }
            TinkoffInvestError::OrdersServiceClientNotInit => {
                write!(f, "{}", "Orders service client not init.")
            }
            TinkoffInvestError::MarketInstrumentKindNotCurrency => {
                write!(f, "{}", "")
            }
            TinkoffInvestError::MarketInstrumentKindNotShare => {
                write!(f, "{}", "")
            }
        }
    }
}

impl Error for TinkoffInvestError {}
