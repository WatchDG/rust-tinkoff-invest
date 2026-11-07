use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Ошибка библиотеки `tinkoff-invest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TError {
    InterceptorNotSet,
    UsersServiceClientNotInit,
    InstrumentsServiceClientNotInit,
    MarketDataServiceClientNotInit,
    MarketDataStreamServiceClientNotInit,
    OperationsServiceClientNotInit,
    OrdersServiceClientNotInit,
    MarketInstrumentTypeNotCurrency,
    MarketInstrumentTypeNotShare,
    MarketInstrumentTypeNotFuture,
    /// gRPC-статус от API.
    Status {
        code: String,
        message: String,
    },
    /// Ошибка транспортного уровня (connect/TLS).
    Transport(String),
    /// Некорректный токен авторизации.
    InvalidToken(String),
    /// Некорректный UUID инструмента.
    InvalidUid(String),
    /// Неизвестный тип инструмента.
    InvalidInstrumentType(String),
    /// Некорректное значение metadata (например x-tracking-id).
    InvalidMetadata(String),
    /// Некорректное денежное значение.
    InvalidMoneyValue(String),
    /// Отрицательное или не помещающееся количество, объём или глубина.
    InvalidQuantity(String),
    /// Ошибка конфигурации TLS.
    TlsConfig(String),
    /// Стрим закрыт (канал команд/событий или gRPC-соединение).
    StreamClosed,
}

impl Display for TError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TError::InterceptorNotSet => write!(f, "Interceptor not set"),
            TError::UsersServiceClientNotInit => {
                write!(f, "Users service client is not initialized")
            }
            TError::InstrumentsServiceClientNotInit => {
                write!(f, "Instruments service client is not initialized")
            }
            TError::MarketDataServiceClientNotInit => {
                write!(f, "Market data service client is not initialized")
            }
            TError::MarketDataStreamServiceClientNotInit => {
                write!(f, "Market data stream service client is not initialized")
            }
            TError::OperationsServiceClientNotInit => {
                write!(f, "Operations service client is not initialized")
            }
            TError::OrdersServiceClientNotInit => {
                write!(f, "Orders service client is not initialized")
            }
            TError::MarketInstrumentTypeNotCurrency => {
                write!(f, "Market instrument type is not currency")
            }
            TError::MarketInstrumentTypeNotShare => {
                write!(f, "Market instrument type is not share")
            }
            TError::MarketInstrumentTypeNotFuture => {
                write!(f, "Market instrument type is not future")
            }
            TError::Status { code, message } => {
                write!(f, "gRPC status {code}: {message}")
            }
            TError::Transport(message) => write!(f, "Transport error: {message}"),
            TError::InvalidToken(message) => write!(f, "Invalid token: {message}"),
            TError::InvalidUid(message) => write!(f, "Invalid UID: {message}"),
            TError::InvalidInstrumentType(message) => {
                write!(f, "Invalid instrument type: {message}")
            }
            TError::InvalidMetadata(message) => write!(f, "Invalid metadata: {message}"),
            TError::InvalidMoneyValue(message) => write!(f, "Invalid money value: {message}"),
            TError::InvalidQuantity(message) => write!(f, "Invalid quantity: {message}"),
            TError::TlsConfig(message) => write!(f, "TLS config error: {message}"),
            TError::StreamClosed => write!(f, "Stream closed"),
        }
    }
}

impl Error for TError {}

impl From<tonic::Status> for TError {
    fn from(status: tonic::Status) -> Self {
        TError::Status {
            code: status.code().to_string(),
            message: status.message().to_string(),
        }
    }
}

impl From<tonic::transport::Error> for TError {
    fn from(error: tonic::transport::Error) -> Self {
        TError::Transport(error.to_string())
    }
}
