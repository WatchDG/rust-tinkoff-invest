use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TError {
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
    FigiNotFound,
    FigiNotSet,
}

impl Display for TError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TError::InterceptorNotSet => {
                write!(f, "Interceptor not set.")
            }
            TError::AccountNotSet => {
                write!(f, "Account not set.")
            }
            TError::UsersServiceClientNotInit => {
                write!(f, "Users service client not init.")
            }
            TError::InstrumentsServiceClientNotInit => {
                write!(f, "Instruments service client not init.")
            }
            TError::MarketDataServiceClientNotInit => {
                write!(f, "Market data service client not init.")
            }
            TError::MarketDataStreamServiceClientNotInit => {
                write!(f, "Market data stream service client not init.")
            }
            TError::OperationsServiceClientNotInit => {
                write!(f, "Operations service client not init.")
            }
            TError::OperationsStreamServiceClientNotInit => {
                write!(f, "Operations stream service client not init.")
            }
            TError::OrdersServiceClientNotInit => {
                write!(f, "Orders service client not init.")
            }
            TError::OrdersStreamServiceClientNotInit => {
                write!(f, "Orders stream service client not init.")
            }
            _ => {
                write!(f, "")
            }
        }
    }
}

impl Error for TError {}
