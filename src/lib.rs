#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tinkoff_invest_types;

use hyper::{
    client::{Client, HttpConnector},
    StatusCode, Uri,
};
use hyper_tls::HttpsConnector;

use tinkoff_invest_types::{
    ErrorPayload, MarketInstrument, MarketInstrumentsPayload, OperationType, Order, PlacedOrder,
    ResponseData,
};

mod request;
mod types;

pub use crate::request::{request_get, request_post, Payload};
pub use crate::types::{Stock, StocksInfo};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

lazy_static! {
    static ref BASE_URI: &'static str = "https://api-invest.tinkoff.ru/openapi";
    static ref GET_STOCKS_URI: Uri = (BASE_URI.to_owned() + "/market/stocks")
        .parse::<Uri>()
        .unwrap();
    static ref GET_BONDS_URI: Uri = (BASE_URI.to_owned() + "/market/bonds")
        .parse::<Uri>()
        .unwrap();
    static ref GET_ETFS_URI: Uri = (BASE_URI.to_owned() + "/market/etfs")
        .parse::<Uri>()
        .unwrap();
    static ref GET_CURRENCIES_URI: Uri = (BASE_URI.to_owned() + "/market/currencies")
        .parse::<Uri>()
        .unwrap();
    static ref GET_ACCOUNTS_URI: Uri = (BASE_URI.to_owned() + "/user/accounts")
        .parse::<Uri>()
        .unwrap();
    static ref GET_ORDERS_URI: Uri = (BASE_URI.to_owned() + "/orders").parse::<Uri>().unwrap();
}

pub struct TinkoffInvest {
    client: Client<HttpsConnector<HttpConnector>>,
    auth: String,
}

impl TinkoffInvest {
    pub fn new(token: &str) -> TinkoffInvest {
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        let auth = "Bearer ".to_owned() + token;
        TinkoffInvest { client, auth }
    }

    /// Get stocks as market instruments
    pub async fn stock_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &GET_STOCKS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get bonds as market instruments
    pub async fn bond_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &GET_BONDS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get etf as market instruments
    pub async fn etf_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &GET_ETFS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<ResponseData<MarketInstrumentsPayload>>(body.as_ref())?;
        Ok(data.payload.instruments)
    }

    /// Get currencies as market instruments
    pub async fn currency_market_instruments(&self) -> Result<Vec<MarketInstrument>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &GET_CURRENCIES_URI, self.auth.as_str()).await?;
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
    pub async fn orders(&self) -> Result<Vec<Order>> {
        let (_status_code, _headers, body) =
            request_get(&self.client, &GET_ORDERS_URI, self.auth.as_str()).await?;
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
    ) -> Result<PlacedOrder> {
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query("/openapi/orders/limit-order?figi=".to_owned() + figi)
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
    ) -> Result<PlacedOrder> {
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query("/openapi/orders/market-order?figi=".to_owned() + figi)
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
    pub async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let uri = Uri::builder()
            .scheme("https")
            .authority("api-invest.tinkoff.ru")
            .path_and_query("/openapi/orders/cancel?orderId=".to_owned() + order_id)
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
}
