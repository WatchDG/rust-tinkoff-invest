//! Список счетов.
//!
//! ```bash
//! TINKOFF_INVEST_TOKEN=... cargo run --example accounts
//! ```

use tinkoff_invest::{TCallContext, TClient, TError};

#[tokio::main]
async fn main() -> Result<(), TError> {
    let token = std::env::var("TINKOFF_INVEST_TOKEN")
        .map_err(|e| TError::InvalidToken(format!("TINKOFF_INVEST_TOKEN is required: {e}")))?;
    let client = TClient::new(token).await?;
    let accounts = client.accounts(&TCallContext::new()).await?;
    for account in accounts {
        println!("{account:?}");
    }
    Ok(())
}
