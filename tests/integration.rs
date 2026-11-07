//! Интеграционные тесты (нужен `TINKOFF_INVEST_TOKEN`).
//!
//! ```bash
//! TINKOFF_INVEST_TOKEN=... cargo test --features integration-tests -- --ignored
//! ```

#![cfg(feature = "integration-tests")]

use tinkoff_invest::{TCallContext, TClient};

#[tokio::test]
#[ignore = "requires TINKOFF_INVEST_TOKEN and network"]
async fn fetch_accounts() {
    let token = std::env::var("TINKOFF_INVEST_TOKEN").expect("TINKOFF_INVEST_TOKEN");
    let client = TClient::new(token).await.expect("client");
    let accounts = client
        .accounts(&TCallContext::new())
        .await
        .expect("accounts");
    assert!(!accounts.is_empty());
}
