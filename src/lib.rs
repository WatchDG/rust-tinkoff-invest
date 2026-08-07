//! Клиентская библиотека для [Tinkoff Invest API](https://developer.tbank.ru/invest/intro/intro).
//!
//! # Пример
//!
//! ```no_run
//! use tinkoff_invest::{TCallContext, TClient};
//!
//! # async fn run() -> Result<(), tinkoff_invest::TError> {
//! let client = TClient::new(std::env::var("TINKOFF_INVEST_TOKEN").unwrap()).await?;
//! let accounts = client.accounts(&TCallContext::new()).await?;
//! # Ok(())
//! # }
//! ```

mod call_context;
mod client;
pub mod enums;
mod error;
pub mod interceptor;
#[cfg(feature = "streams")]
pub mod streams;
pub mod tls;
pub mod traits;
pub mod types;

pub use call_context::{TAccountContext, TCallContext, TOrderContext};
pub use client::{TClient, TClientBuilder, TClientBuilderFlags};
pub use error::TError;
pub use tls::{russian_trusted_ca_certificates, russian_trusted_tls_config};

// re-export
pub use chrono;
pub use tonic::transport::{Certificate, ClientTlsConfig};
