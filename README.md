# rust-tinkoff-invest

development stage: BETA

DO NOT use for production

## API reference

### get accounts

```rust
use tinkoff_invest::TinkoffInvest;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let accounts = tinkoff.accounts().await?;

    println!("accounts: {:?}", accounts);

    Ok(())
}
```

### get market instruments

```rust
use tinkoff_invest::{enums::InstrumentType, TinkoffInvest};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let market_instruments = tinkoff
        .market_instruments(InstrumentType::Share)
        .await?;

    println!("{:?}", market_instruments);

    Ok(())
}
```

### get candlesticks

```rust
use tinkoff_invest::{enums::CandlestickInterval, extra::chrono, types::Figi, TinkoffInvest};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let figi = Figi::from("BBG004730N88");
    let from = chrono::NaiveDate::from_ymd(2020, 1, 10);
    let to = chrono::NaiveDate::from_ymd(2020, 1, 11);
    let candlesticks = tinkoff
        .candlesticks(&figi, CandlestickInterval::Min, from.into(), to.into())
        .await?;

    println!("{:?}", candlesticks);

    Ok(())
}
```

### get order book

```rust
use tinkoff_invest::{types::Figi, TinkoffInvest};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let figi = Figi::from("BBG004730N88");
    let order_book = tinkoff.order_book(&figi, 10).await?;

    println!("{:?}", order_book);

    Ok(())
}
```

### get operations

```rust
use tinkoff_invest::{enums::OperationState, extra::chrono, types::Figi, TinkoffInvest};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let accounts = tinkoff.accounts().await?;

    let first_account = accounts.get(0).unwrap().clone();

    tinkoff.set_account(Some(first_account));

    let figi = Figi::from("BBG004730N88");
    let from = chrono::NaiveDate::from_ymd(2020, 1, 1);
    let to = chrono::NaiveDate::from_ymd(2023, 1, 1);
    let operations = tinkoff
        .operations(&figi, OperationState::Unspecified, from.into(), to.into())
        .await?;

    println!("{:?}", operations);

    Ok(())
}
```

### get portfolio

```rust
use tinkoff_invest::TinkoffInvest;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let accounts = tinkoff.accounts().await?;

    let first_account = accounts.get(0).unwrap().clone();

    tinkoff.set_account(Some(first_account));

    let portfolio = tinkoff.portfolio().await?;

    println!("{:?}", portfolio);

    Ok(())
}
```

## Streams

### Market Data Stream

```rust
use tinkoff_invest::enums::{CandlestickInterval, MarketDataStreamData};
use tinkoff_invest::streams::MarketDataStreamBuilder;
use tinkoff_invest::types::Figi;
use tinkoff_invest::TinkoffInvest;

async fn market_data_handler(market_data: MarketDataStreamData) {
    println!("market data: {:?}", market_data);
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let tinkoff = TinkoffInvest::new(token.into()).await?;

    let market_data_stream_builder = MarketDataStreamBuilder::from(&tinkoff);
    let mut market_data_stream = market_data_stream_builder.build().await?;

    market_data_stream
        .subscribe_candlesticks(&[&Figi::from("BBG004730N88")], &CandlestickInterval::Min)
        .await?;

    let mut broadcast_receiver = market_data_stream.subscribe();

    tokio::spawn(async move {
        while let MarketDataStreamData::Candlestick(candlestick) = broadcast_receiver.recv().await.unwrap() {
            println!("{:?}", candlestick);
        }
    });

    market_data_stream.task.await?;

    Ok(())
}
```

## Cached Market Instruments

### example

```rust
use tinkoff_invest::{
    enums::{ClassCode, InstrumentType},
    types::{Figi, Ticker},
    CachedMarketInstruments, TinkoffInvest,
};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let mut tinkoff = TinkoffInvest::new(token.into()).await?;

    let market_instruments = tinkoff
        .market_instruments(InstrumentType::Share)
        .await?;

    let cached_market_instruments = CachedMarketInstruments::from(market_instruments);

    // find by figi
    {
        let figi = Figi::from("BBG004730N88");
        let market_instrument = cached_market_instruments.by_figi(&figi).unwrap();
        println!("{:?}", market_instrument);
    }

    // find by class code and ticker
    {
        let class_code_ticker = (ClassCode::TQBR, Ticker::from("SBER"));
        let market_instrument = cached_market_instruments
            .by_class_code_and_ticker(&class_code_ticker)
            .unwrap();
        println!("{:?}", market_instrument);
    }

    // find by ticker
    {
        let ticker = Ticker::from("SBER");
        let market_instrument = cached_market_instruments.by_ticker(&ticker).unwrap();
        println!("{:?}", market_instrument);
    }

    Ok(())
}
```

## Cached Orderbook

```rust
use std::sync::Arc;
use std::time::Duration;
use tinkoff_invest::cached::CachedOrderbooks;
use tinkoff_invest::enums::MarketDataStreamData;
use tinkoff_invest::streams::MarketDataStreamBuilder;
use tinkoff_invest::types::Uid;
use tinkoff_invest::TinkoffInvest;
use tokio::sync::RwLock;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";

    let tinkoff = TinkoffInvest::new(token.into()).await?;

    let market_data_stream_builder = MarketDataStreamBuilder::from(&tinkoff);
    let mut market_data_stream = market_data_stream_builder.build().await?;

    market_data_stream
        .subscribe_orderbook(&[Uid::from("e6123145-9665-43e0-8413-cd61b8aa9b13")], 10)
        .await?;

    let mut broadcast_receiver = market_data_stream.subscribe();

    let cached_orderbooks = Arc::new(RwLock::new(CachedOrderbooks::new()));

    let write_thread_cached_orderbooks = cached_orderbooks.clone();
    tokio::spawn(async move {
        loop {
            match broadcast_receiver.recv().await {
                Ok(MarketDataStreamData::Orderbook(orderbook)) => {
                    write_thread_cached_orderbooks.write().await.add(orderbook);
                }
                Err(error) => {
                    println!("error: {}", error);
                }
                _ => {}
            }
        }
    });

    let read_thread_cached_orderbooks = cached_orderbooks.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let lock_guard = read_thread_cached_orderbooks.read().await;
            let orderbook = lock_guard.get(&Uid::from("e6123145-9665-43e0-8413-cd61b8aa9b13"));
            println!("orderbook: {:?}", orderbook);
        }
    }).await?;

    Ok(())
}
```