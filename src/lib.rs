#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;

use bytes::{BufMut, BytesMut};
use hyper::{
    body::HttpBody,
    client::{Client, HttpConnector},
    Body, Method, Request, StatusCode, Uri,
};
use hyper_tls::HttpsConnector;

mod types;

use crate::types::{TinkoffUserAccount, TinkoffUserAccounts, Stock};
use types::{TinkoffMarketInstrument, TinkoffInstruments, TinkoffResponseData};

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
}

async fn request_get(
    client: &Client<HttpsConnector<HttpConnector>>,
    uri: &Uri,
    auth: &str,
) -> Result<BytesMut> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", auth)
        .body(Body::empty())?;
    let mut response = client.request(request).await?;
    let status = response.status();
    if status == StatusCode::UNAUTHORIZED {
        let error = std::io::Error::new(std::io::ErrorKind::Other, "UNAUTHORIZED");
        return Err(Box::new(error));
    }
    let mut body = BytesMut::with_capacity(1024);
    while let Some(chunk) = response.body_mut().data().await {
        body.put(chunk?);
    }
    Ok(body)
}

pub struct TinkoffInvest {
    client: Client<HttpsConnector<HttpConnector>>,
    auth: String,
}

impl TinkoffInvest {
    pub fn new(token: &'static str) -> TinkoffInvest {
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        let auth = "Bearer ".to_owned() + token;
        TinkoffInvest { client, auth }
    }

    pub async fn get_stock_market_instruments(&self) -> Result<Vec<TinkoffMarketInstrument>> {
        let response_data = request_get(&self.client, &GET_STOCKS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_bond_market_instruments(&self) -> Result<Vec<TinkoffMarketInstrument>> {
        let response_data = request_get(&self.client, &GET_BONDS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_etf_market_instruments(&self) -> Result<Vec<TinkoffMarketInstrument>> {
        let response_data = request_get(&self.client, &GET_ETFS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_currency_market_instruments(&self) -> Result<Vec<TinkoffMarketInstrument>> {
        let response_data =
            request_get(&self.client, &GET_CURRENCIES_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_market_instrument_by_ticker(
        &self,
        ticker: &str,
    ) -> Result<Vec<TinkoffMarketInstrument>> {
        let uri =
            (BASE_URI.to_owned() + "/market/search/by-ticker?ticker=" + ticker).parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_market_instrument_by_figi(
        &self,
        figi: &str,
    ) -> Result<Vec<TinkoffMarketInstrument>> {
        let uri = (BASE_URI.to_owned() + "/market/search/by-figi?figi=" + figi).parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_accounts(&self) -> Result<Vec<TinkoffUserAccount>> {
        let response_data =
            request_get(&self.client, &GET_ACCOUNTS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffUserAccounts>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.accounts)
    }

    /// get stocks
    pub async fn get_stocks(&self) -> Result<Vec<Stock>> {
        let market_instruments = self.get_stock_market_instruments().await?;
        let stocks = market_instruments.into_iter().filter_map(|tmi| {
            if tmi.isin.is_none() || tmi.min_price_increment.is_none() || tmi.currency.is_none() {
                return Option::None;
            }
            Option::Some(
                Stock {
                    figi: tmi.figi,
                    ticker: tmi.ticker,
                    name: tmi.name,
                    isin: tmi.isin?,
                    min_price_increment: tmi.min_price_increment?,
                    lot: tmi.lot,
                    currency: tmi.currency?,
                }
            )
        }).collect();
        Ok(stocks)
    }
}
