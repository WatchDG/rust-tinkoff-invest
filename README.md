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
use tinkoff_invest::{enums::MarketInstrumentKind, TinkoffInvest};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";
    let mut tinkoff = TinkoffInvest::new(token.into())?;

    let market_instruments = tinkoff
        .market_instruments(MarketInstrumentKind::Share)
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
        .candlesticks(&figi, CandlestickInterval::Min1, from.into(), to.into())
        .await?;

    println!("{:?}", candlesticks);

    Ok(())
}
```

## Cached Market Instruments

### example

```rust
use tinkoff_invest::{
    enums::{ClassCode, MarketInstrumentKind},
    types::{Figi, Ticker},
    CachedMarketInstruments, TinkoffInvest,
};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "...";
    let mut tinkoff = TinkoffInvest::new(token.into())?;

    let market_instruments = tinkoff
        .market_instruments(MarketInstrumentKind::Share)
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