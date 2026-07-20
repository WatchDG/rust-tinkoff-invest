//! Свечи по UID инструмента за последние сутки.
//!
//! ```bash
//! TINKOFF_INVEST_TOKEN=... INSTRUMENT_UID=... cargo run --example candles
//! ```

use std::convert::TryFrom;

use tinkoff_invest::{
    TCallContext, TClient, TError,
    enums::CandlestickInterval,
    types::{DateTime, Uid},
};

#[tokio::main]
async fn main() -> Result<(), TError> {
    let token = std::env::var("TINKOFF_INVEST_TOKEN")
        .map_err(|e| TError::InvalidToken(format!("TINKOFF_INVEST_TOKEN is required: {e}")))?;
    let uid = std::env::var("INSTRUMENT_UID")
        .map_err(|_| TError::InvalidUid("INSTRUMENT_UID env var is required".to_string()))?;
    let uid = Uid::try_from(uid.as_str())?;

    let client = TClient::new(token).await?;
    let to = DateTime::now();
    let from = DateTime::from(
        (tinkoff_invest::chrono::Utc::now() - tinkoff_invest::chrono::Duration::days(1))
            .naive_utc(),
    );
    let candles = client
        .candlesticks(
            &TCallContext::new(),
            uid,
            CandlestickInterval::Hour,
            from,
            to,
        )
        .await?;
    for candle in candles.iter().take(5) {
        println!("{candle:?}");
    }
    println!("total: {}", candles.len());
    Ok(())
}
