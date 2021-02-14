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
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const BASE_URL: &str = "https://api-invest.tinkoff.ru/openapi";

#[derive(Deserialize, Debug)]
pub enum Currency {
    RUB,
    USD,
    EUR,
    GBP,
    HKD,
    CHF,
    JPY,
    CNY,
    TRY,
}

#[derive(Deserialize, Debug)]
pub enum InstrumentType {
    Stock,
    Currency,
    Bond,
    Etf,
}

pub struct TinkoffInvest {
    client: Client<HttpsConnector<HttpConnector>>,
    base_url: &'static str,
    token: &'static str,
}

#[derive(Deserialize, Debug)]
pub struct TinkoffInstrument {
    pub figi: String,
    pub ticker: String,
    pub name: String,
    pub isin: Option<String>,
    #[serde(rename(serialize = "minPriceIncrement", deserialize = "minPriceIncrement"))]
    pub min_price_increment: Option<f64>,
    pub lot: u64,
    pub currency: Option<Currency>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_: InstrumentType,
}

#[derive(Deserialize, Debug)]
struct TinkoffInstruments {
    instruments: Vec<TinkoffInstrument>,
}

#[derive(Deserialize, Debug)]
struct TinkoffResponseData<P> {
    #[serde(rename(serialize = "trackingId", deserialize = "trackingId"))]
    tracking_id: String,
    payload: P,
    status: String,
    error: Option<String>,
}

async fn request_get(
    client: &Client<HttpsConnector<HttpConnector>>,
    uri: &Uri,
    token: &str,
) -> Result<BytesMut> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", "Bearer ".to_owned() + token)
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

impl TinkoffInvest {
    pub fn new(token: &'static str) -> TinkoffInvest {
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        TinkoffInvest {
            client,
            base_url: BASE_URL,
            token,
        }
    }

    pub async fn get_stocks(&self) -> Result<Vec<TinkoffInstrument>> {
        let uri = (self.base_url.to_owned() + "/market/stocks").parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.token).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_bonds(&self) -> Result<Vec<TinkoffInstrument>> {
        let uri = (self.base_url.to_owned() + "/market/bonds").parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.token).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_etfs(&self) -> Result<Vec<TinkoffInstrument>> {
        let uri = (self.base_url.to_owned() + "/market/etfs").parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.token).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_currencies(&self) -> Result<Vec<TinkoffInstrument>> {
        let uri = (self.base_url.to_owned() + "/market/currencies").parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.token).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }
}
