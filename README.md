# rust-tinkoff-invest

DO NOT use for production

## API reference

### get accounts

```rust
use tinkoff_invest::TinkoffInvest;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

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
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

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
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

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
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

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
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

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
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

    let accounts = tinkoff.accounts().await?;

    let first_account = accounts.get(0).unwrap().clone();

    tinkoff.set_account(Some(first_account));

    let portfolio = tinkoff.portfolio().await?;

    println!("{:?}", portfolio);

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
    
    let mut tinkoff = TinkoffInvest::new(token.into())?;

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