//! Выставление лимитной заявки (осторожно: реальная торговля).
//!
//! ```bash
//! TINKOFF_INVEST_TOKEN=... ACCOUNT_ID=... INSTRUMENT_UID=... \
//! ORDER_ID=... PRICE=100.5 QUANTITY=1 cargo run --example limit_order
//! ```

use std::convert::TryFrom;

use tinkoff_invest::{
    TCallContext, TClient, TError,
    enums::OrderDirection,
    types::{AccountId, MoneyValue, OrderId, Uid},
};

#[tokio::main]
async fn main() -> Result<(), TError> {
    let token = std::env::var("TINKOFF_INVEST_TOKEN")
        .map_err(|e| TError::InvalidToken(format!("TINKOFF_INVEST_TOKEN is required: {e}")))?;
    let account_id = std::env::var("ACCOUNT_ID").map_err(|_| TError::AccountNotSet)?;
    let order_id = std::env::var("ORDER_ID")
        .map_err(|_| TError::InvalidMetadata("ORDER_ID is required".to_string()))?;
    let uid = std::env::var("INSTRUMENT_UID")
        .map_err(|_| TError::InvalidUid("INSTRUMENT_UID env var is required".to_string()))?;
    let uid = Uid::try_from(uid.as_str())?;
    let price = std::env::var("PRICE")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| TError::InvalidMoneyValue("PRICE is required".to_string()))?;
    let quantity = std::env::var("QUANTITY")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1);

    let client = TClient::new(token).await?;
    let ctx = TCallContext::new()
        .with_account(AccountId::from(account_id))
        .with_order(OrderId::from(order_id));
    let order = client
        .limit_order(
            &ctx,
            uid,
            OrderDirection::Buy,
            quantity,
            MoneyValue::try_from_f64(price)?,
        )
        .await?;
    println!("{order:?}");
    Ok(())
}
