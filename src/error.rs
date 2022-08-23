use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TinkoffInvestError {
    InterceptorNotSet,
    HandlerNotSet,
    AccountNotSet,
    ChannelNotSet,
    UsersServiceClientNotInit,
    InstrumentsServiceClientNotInit,
    MarketDataServiceClientNotInit,
    MarketDataStreamServiceClientNotInit,
    OperationsServiceClientNotInit,
    OperationsStreamServiceClientNotInit,
    OrdersServiceClientNotInit,
    OrdersStreamServiceClientNotInit,
    MarketInstrumentTypeNotCurrency,
    MarketInstrumentTypeNotShare,
    MarketInstrumentTypeNotFuture,
    CandlestickFigiNotSet,
    CandlestickIntervalNotSet,
    CandlestickPriceOpenNotSet,
    CandlestickPriceHighNotSet,
    CandlestickPriceLowNotSet,
    CandlestickPriceCloseNotSet,
    CandlestickDatetimeNotSet,
}

impl Display for TinkoffInvestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TinkoffInvestError::InterceptorNotSet => {
                write!(f, "Interceptor not set.")
            }
            TinkoffInvestError::AccountNotSet => {
                write!(f, "Account not set.")
            }
            TinkoffInvestError::UsersServiceClientNotInit => {
                write!(f, "Users service client not init.")
            }
            TinkoffInvestError::InstrumentsServiceClientNotInit => {
                write!(f, "Instruments service client not init.")
            }
            TinkoffInvestError::MarketDataServiceClientNotInit => {
                write!(f, "Market data service client not init.")
            }
            TinkoffInvestError::MarketDataStreamServiceClientNotInit => {
                write!(f, "Market data stream service client not init.")
            }
            TinkoffInvestError::OperationsServiceClientNotInit => {
                write!(f, "Operations service client not init.")
            }
            TinkoffInvestError::OperationsStreamServiceClientNotInit => {
                write!(f, "Operations stream service client not init.")
            }
            TinkoffInvestError::OrdersServiceClientNotInit => {
                write!(f, "Orders service client not init.")
            }
            TinkoffInvestError::OrdersStreamServiceClientNotInit => {
                write!(f, "Orders stream service client not init.")
            }
            _ => {
                write!(f, "")
            }
        }
    }
}

impl Error for TinkoffInvestError {}
