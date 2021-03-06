#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate percent_encoding;
extern crate tinkoff_invest_types;

use hyper::{
    client::{Client, HttpConnector},
    StatusCode, Uri,
};
use hyper_tls::HttpsConnector;

pub use tinkoff_invest_types::CandlestickResolution;
use tinkoff_invest_types::{
    Candlestick, CandlesticksPayload, CurrencyPortfolioPayload, CurrencyPortfolioPosition,
    ErrorPayload, MarketInstrument, MarketInstrumentsPayload, Operation, OperationType,
    OperationsPayload, Order, Orderbook, PlacedOrder, PortfolioPayload, PortfolioPosition,
    ResponseData, UserAccount, UserAccountsPayload,
};

use std::fmt;

mod request;
mod types;

pub use crate::request::{request_get, request_post, Payload};
pub use crate::types::{Stock, StocksInfo};
use chrono::{DateTime, SecondsFormat, TimeZone};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

lazy_static! {
    static ref STOCKS_URI: Uri = Uri::builder()
        .scheme("https")
        .authority("api-invest.tinkoff.ru")
        .path_and_query("/openapi/market/stocks")
        .build()
        .unwrap();
    static ref BONDS_URI: Uri = Uri::builder()
        .scheme("https")
        .authority("api-invest.tinkoff.ru")
        .path_and_query("/openapi/market/bonds")
        .build()
        .unwrap();
    static ref ETFS_URI: Uri = Uri::builder()
        .scheme("https")
        .authority("api-invest.tinkoff.ru")
        .path_and_query("/openapi/market/etfs")
        .build()
        .unwrap();
    static ref CURRENCIES_URI: Uri = Uri::builder()
        .scheme("https")
        .authority("api-invest.tinkoff.ru")
        .path_and_query("/openapi/market/currencies")
        .build()
        .unwrap();
    static ref ACCOUNTS_URI: Uri = Uri::builder()
        .scheme("https")
        .authority("api-invest.tinkoff.ru")
        .path_and_query("/openapi/user/accounts")
        .build()
        .unwrap();
}

pub struct TinkoffInvest {
    client: Client<HttpsConnector<HttpConnector>>,
    auth: String,
}

impl TinkoffInvest {
    pub fn new(token: &str) -> TinkoffInvest {
        let https_connector = HttpsConnector::new();
        let client = Client::builder().build(https_connector);
        let auth = "Bearer ".to_owned() + token;
        TinkoffInvest { client, auth }
    }

    /// Get accounts
    pub async fn accounts(&self) -> Result<Vec<UserAccount>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &ACCOUNTS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<UserAccountsPayload>>(body.as_ref())?;
        Ok(data.payload.accounts)
    }

    /// Get stocks as market instruments
    pub async fn stock_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &STOCKS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get bonds as market instruments
    pub async fn bond_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &BONDS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get etf as market instruments
    pub async fn etf_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &ETFS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get currencies as market instruments
    pub async fn currency_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &CURRENCIES_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get stocks
    pub async fn stocks(&self) -> Result<Vec<Stock>> {
        let market_instruments = self.stock_market_instruments().await?;
        let stocks = market_instruments
            .into_iter()
            .filter_map(|mi| {
                if mi.isin.is_none() || mi.min_price_increment.is_none() || mi.currency.is_none() {
                    return Option::None;
                }
                Option::Some(Stock {
                    figi: mi.figi,
                    ticker: mi.ticker,
                    name: mi.name,
                    isin: mi.isin?,
                    min_price_increment: mi.min_price_increment?,
                    lot: mi.lot,
                    currency: mi.currency?,
                })
            })
            .collect();
        Ok(stocks)
    }

    /// Get stocks info
    pub fn stocks_info(stocks: Vec<Stock>) -> StocksInfo {
        StocksInfo::new(stocks)
    }

    /// Get active orders
    pub async fn orders(&self, account_id: Option<&str>) -> Result<Vec<Order>> {
        let mut path = "/openapi/orders".to_string();
        if let Some(account_id) = account_id {
            path += ("?brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()
            .unwrap();
        let (_status_code, _headers, body) =
            request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<Vec<Order>>>(body.as_ref())?;
        Ok(data.payload)
    }

    /// Place limit order
    pub async fn limit_order(
        &self,
        figi: &str,
        operation: OperationType,
        lots: u64,
        price: f64,
        account_id: Option<&str>,
    ) -> Result<PlacedOrder> {
        let mut path = "/openapi/orders/limit-order?figi=".to_owned() + figi;
        if let Some(account_id) = account_id {
            path += ("&brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()?;
        let payload = json!({
            "operation": operation,
            "lots": lots,
            "price": price
        })
        .to_string();
        let (status_code, _headers, body) = request_post(
            &self.client,
            &uri,
            &self.auth,
            Payload::Payload(payload.as_ref()),
        )
        .await?;
        if status_code != StatusCode::OK {
            let data = serde_json::from_slice::<ResponseData<ErrorPayload>>(body.as_ref())?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                data.payload.message,
            )));
        }
        let data = serde_json::from_slice::<ResponseData<PlacedOrder>>(body.as_ref())?;
        Ok(data.payload)
    }

    /// Place market order
    pub async fn market_order(
        &self,
        figi: &str,
        operation: OperationType,
        lots: u64,
        account_id: Option<&str>,
    ) -> Result<PlacedOrder> {
        let mut path = "/openapi/orders/market-order?figi=".to_owned() + figi;
        if let Some(account_id) = account_id {
            path += ("&brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()?;
        let payload = json!({
            "operation": operation,
            "lots": lots
        })
        .to_string();
        let (status_code, _headers, body) = request_post(
            &self.client,
            &uri,
            &self.auth,
            Payload::Payload(payload.as_ref()),
        )
        .await?;
        if status_code != StatusCode::OK {
            let data = serde_json::from_slice::<ResponseData<ErrorPayload>>(body.as_ref())?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                data.payload.message,
            )));
        }
        let data = serde_json::from_slice::<ResponseData<PlacedOrder>>(body.as_ref())?;
        Ok(data.payload)
    }

    /// Cancel order
    pub async fn cancel_order(&self, order_id: &str, account_id: Option<&str>) -> Result<()> {
        let mut path = "/openapi/orders/cancel?orderId=".to_owned() + order_id;
        if let Some(account_id) = account_id {
            path += ("&brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()?;
        let (status_code, _headers, body) =
            request_post(&self.client, &uri, &self.auth, Payload::None).await?;
        if status_code != StatusCode::OK {
            let data = serde_json::from_slice::<ResponseData<ErrorPayload>>(body.as_ref())?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                data.payload.message,
            )));
        }
        Ok(())
    }

    /// Get portfolio
    pub async fn portfolio(&self, account_id: Option<&str>) -> Result<Vec<PortfolioPosition>> {
        let mut path = "/openapi/portfolio".to_string();
        if let Some(account_id) = account_id {
            path += ("?brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()
            .unwrap();
        let (_status_code, _headers, body) =
            request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<PortfolioPayload>>(body.as_ref())?;
        Ok(data.payload.positions)
    }

    /// Get currency portfolio
    pub async fn currency_portfolio(
        &self,
        account_id: Option<&str>,
    ) -> Result<Vec<CurrencyPortfolioPosition>> {
        let mut path = "/openapi/portfolio/currencies".to_string();
        if let Some(account_id) = account_id {
            path += ("?brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()
            .unwrap();
        let (_status_code, _headers, body) =
            request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<CurrencyPortfolioPayload>>(body.as_ref())?;
        Ok(data.payload.currencies)
    }

    /// Get operations
    pub async fn operations<Tz: TimeZone>(
        &self,
        from: DateTime<Tz>,
        to: DateTime<Tz>,
        figi: Option<&str>,
        account_id: Option<&str>,
    ) -> Result<Vec<Operation>>
    where
        Tz::Offset: fmt::Display,
    {
        let from_datetime = percent_encode(
            from.to_rfc3339_opts(SecondsFormat::Micros, false)
                .as_bytes(),
            NON_ALPHANUMERIC,
        )
        .to_string();
        let to_datetime = percent_encode(
            to.to_rfc3339_opts(SecondsFormat::Micros, false).as_bytes(),
            NON_ALPHANUMERIC,
        )
        .to_string();
        let mut path = "/openapi/operations?from=".to_owned()
            + from_datetime.as_str()
            + "&to="
            + to_datetime.as_str();
        if let Some(figi) = figi {
            path += ("&figi=".to_owned() + figi).as_str();
        }
        if let Some(account_id) = account_id {
            path += ("&brokerAccountId=".to_owned() + account_id).as_str();
        }
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()
            .unwrap();
        let (_status_code, _headers, body) =
            request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<OperationsPayload>>(body.as_ref())?;
        Ok(data.payload.operations)
    }

    /// Get orderbook
    pub async fn orderbook(&self, figi: &str, depth: usize) -> Result<Orderbook> {
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(
                "/openapi/market/orderbook?figi=".to_owned()
                    + figi
                    + "&depth="
                    + depth.to_string().as_str(),
            )
            .build()?;
        let (status_code, _headers, body) = request_get(&self.client, &uri, &self.auth).await?;
        if status_code != StatusCode::OK {
            let data = serde_json::from_slice::<ResponseData<ErrorPayload>>(body.as_ref())?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                data.payload.message,
            )));
        }
        let data = serde_json::from_slice::<ResponseData<Orderbook>>(body.as_ref())?;
        Ok(data.payload)
    }

    /// Get candlesticks
    pub async fn candlesticks<Tz: TimeZone>(
        &self,
        from: DateTime<Tz>,
        to: DateTime<Tz>,
        figi: &str,
        interval: CandlestickResolution,
    ) -> Result<Vec<Candlestick>>
    where
        Tz::Offset: fmt::Display,
    {
        let from_datetime = percent_encode(
            from.to_rfc3339_opts(SecondsFormat::Micros, false)
                .as_bytes(),
            NON_ALPHANUMERIC,
        )
        .to_string();
        let to_datetime = percent_encode(
            to.to_rfc3339_opts(SecondsFormat::Micros, false).as_bytes(),
            NON_ALPHANUMERIC,
        )
        .to_string();
        let path = "/openapi/market/candles?from=".to_owned()
            + from_datetime.as_str()
            + "&to="
            + to_datetime.as_str()
            + "&figi="
            + figi
            + "&interval="
            + interval.to_string().as_str();
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query(path)
            .build()
            .unwrap();
        let (_status_code, _headers, body) =
            request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<CandlesticksPayload>>(body.as_ref())?;
        Ok(data.payload.candles)
    }
}
