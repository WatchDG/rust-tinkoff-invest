# rust-tinkoff-invest

Клиентская библиотека для [Tinkoff Invest API](https://developer.tbank.ru/invest/intro/intro).

**Стадия:** BETA — API может меняться.

## Документация

- [docs.rs/tinkoff-invest](https://docs.rs/tinkoff-invest)
- [Tinkoff Invest API](https://developer.tbank.ru/invest/intro/intro)

## Быстрый старт

```toml
[dependencies]
tinkoff-invest = "3"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust,no_run
use tinkoff_invest::{TCallContext, TClient};

#[tokio::main]
async fn main() -> Result<(), tinkoff_invest::TError> {
    let token = std::env::var("TINKOFF_INVEST_TOKEN").expect("TINKOFF_INVEST_TOKEN");
    let client = TClient::new(token).await?;
    let accounts = client.accounts(&TCallContext::new()).await?;
    println!("{accounts:?}");
    Ok(())
}
```

Примеры в каталоге `examples/` (нужен `TINKOFF_INVEST_TOKEN`):

```bash
cargo run --example accounts
cargo run --example candles
cargo run --example limit_order
cargo run --example market_data_stream --features streams
```

## Features

| Feature | По умолчанию | Назначение |
|---------|--------------|------------|
| `users` | да | UsersService |
| `instruments` | да | InstrumentsService |
| `market-data` | да | MarketDataService |
| `operations` | да | OperationsService |
| `orders` | да | OrdersService |
| `streams` | нет | MarketDataStream (bidirectional session) |
| `integration-tests` | нет | интеграционные тесты |

## License

MIT OR Apache-2.0
