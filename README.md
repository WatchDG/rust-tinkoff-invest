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

    let stocks_info = TinkoffInvest::stocks_info(stocks);

    let stock_wfc = stocks_info.by_ticker("WFC").unwrap();
    let stock_sber = stocks_info.by_ticker("SBER").unwrap();
    println!("[{}] {}", &stock_wfc.figi, &stock_wfc.name);
    println!("[{}] {}", &stock_sber.figi, &stock_sber.name);

    Ok(())
}
```