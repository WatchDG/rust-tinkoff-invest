# rust-tinkoff-invest

DO NOT use for production

## get accounts

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