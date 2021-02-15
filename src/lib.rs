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

use types::{TinkoffInstrument, TinkoffInstruments, TinkoffResponseData};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

lazy_static! {
    static ref BASE_URI: &'static str = "https://api-invest.tinkoff.ru/openapi";
    static ref GET_STOCKS_URI: Uri = (BASE_URI.to_owned() + "/market/stocks").parse::<Uri>().unwrap();
    static ref GET_BONDS_URI: Uri = (BASE_URI.to_owned() + "/market/bonds").parse::<Uri>().unwrap();
    static ref GET_ETFS_URI: Uri = (BASE_URI.to_owned() + "/market/etfs").parse::<Uri>().unwrap();
    static ref GET_CURRENCIES_URI: Uri = (BASE_URI.to_owned() + "/market/currencies").parse::<Uri>().unwrap();
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
        TinkoffInvest {
            client,
            auth,
        }
    }

    pub async fn get_stocks(&self) -> Result<Vec<TinkoffInstrument>> {
        let response_data = request_get(&self.client, &GET_STOCKS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_bonds(&self) -> Result<Vec<TinkoffInstrument>> {
        let response_data = request_get(&self.client, &GET_BONDS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_etfs(&self) -> Result<Vec<TinkoffInstrument>> {
        let response_data = request_get(&self.client, &GET_ETFS_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_currencies(&self) -> Result<Vec<TinkoffInstrument>> {
        let response_data = request_get(&self.client, &GET_CURRENCIES_URI, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }

    pub async fn get_market_instrument_by_ticker(&self, ticker: &str) -> Result<Vec<TinkoffInstrument>> {
        let uri = (BASE_URI.to_owned() + "/market/search/by-ticker?ticker=" + ticker).parse::<Uri>()?;
        let response_data = request_get(&self.client, &uri, self.auth.as_str()).await?;
        let data = serde_json::from_slice::<TinkoffResponseData<TinkoffInstruments>>(
            response_data.as_ref(),
        )?;
        Ok(data.payload.instruments)
    }
}
