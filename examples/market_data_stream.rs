//! Bidirectional market-data stream (свечи + last price).
//!
//! ```bash
//! TINKOFF_INVEST_TOKEN=... INSTRUMENT_UID=... cargo run --example market_data_stream --features streams
//! ```

use std::convert::TryFrom;

use tinkoff_invest::{
    TCallContext, TClient, TError,
    enums::CandlestickInterval,
    streams::{MarketDataEvent, MarketDataSubscription},
    types::Uid,
};

#[tokio::main]
async fn main() -> Result<(), TError> {
    let token = std::env::var("TINKOFF_INVEST_TOKEN")
        .map_err(|e| TError::InvalidToken(format!("TINKOFF_INVEST_TOKEN is required: {e}")))?;
    let uid = std::env::var("INSTRUMENT_UID")
        .map_err(|_| TError::InvalidUid("INSTRUMENT_UID env var is required".to_string()))?;
    let uid = Uid::try_from(uid.as_str())?;

    let client = TClient::new(token).await?;
    let mut session = client.market_data_stream(&TCallContext::new()).await?;

    session
        .subscribe(vec![
            MarketDataSubscription::Candles {
                instrument_uid: uid.clone(),
                interval: CandlestickInterval::Min,
                waiting_close: false,
            },
            MarketDataSubscription::LastPrice {
                instrument_uid: uid,
            },
        ])
        .await?;

    let mut remaining = 10usize;
    while let Some(event) = session.recv().await {
        match event? {
            MarketDataEvent::CandlesSubscriptionAck { tracking_id }
            | MarketDataEvent::LastPriceSubscriptionAck { tracking_id } => {
                println!("subscribed: {tracking_id}");
            }
            MarketDataEvent::Candle(c) => {
                println!("candle: {c:?}");
                remaining = remaining.saturating_sub(1);
            }
            MarketDataEvent::LastPrice {
                instrument_uid,
                price,
                datetime,
            } => {
                println!("last_price uid={instrument_uid:?} price={price:?} at={datetime:?}");
                remaining = remaining.saturating_sub(1);
            }
            other => println!("event: {other:?}"),
        }
        if remaining == 0 {
            break;
        }
    }
    Ok(())
}
