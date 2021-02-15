# rust-tinkoff-invest

## get stocks

```rust
extern crate tinkoff_invest;

use tinkoff_invest::TinkoffInvest;
use std::error::Error;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = "...";
    let tinkoff = TinkoffInvest::new(token);
    let stocks = tinkoff.get_stocks().await?;
    for stock in stocks {
        println!("figi: {} name: {}", stock.figi, stock.name);
    }
    Ok(())
}
```