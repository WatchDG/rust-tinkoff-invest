use crate::Result;
use bytes::{BufMut, BytesMut};
use hyper::{
    body::HttpBody,
    client::{Client, HttpConnector},
    Body, HeaderMap, Method, Request, StatusCode, Uri,
};
use hyper_tls::HttpsConnector;

pub async fn request_get(
    client: &Client<HttpsConnector<HttpConnector>>,
    uri: &Uri,
    auth: &str,
) -> Result<(StatusCode, HeaderMap, BytesMut)> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", auth)
        .body(Body::empty())?;
    let mut response = client.request(request).await?;
    let status = response.status();
    let headers = response.headers().to_owned();
    let mut body = BytesMut::with_capacity(1024);
    if !headers.contains_key("content-length") || !headers.contains_key("content-type") {
        return Ok((status, headers, body));
    }
    while let Some(chunk) = response.body_mut().data().await {
        body.put(chunk?);
    }
    Ok((status, headers, body))
}

pub async fn request_post(
    client: &Client<HttpsConnector<HttpConnector>>,
    uri: &Uri,
    auth: &str,
    payload: &str,
) -> Result<(StatusCode, HeaderMap, BytesMut)> {
    let request = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Authorization", auth)
        .body(Body::from(payload.to_owned()))?;
    let mut response = client.request(request).await?;
    let status = response.status();
    let headers = response.headers().to_owned();
    let mut body = BytesMut::with_capacity(1024);
    if !headers.contains_key("content-length") || !headers.contains_key("content-type") {
        return Ok((status, headers, body));
    }
    while let Some(chunk) = response.body_mut().data().await {
        body.put(chunk?);
    }
    Ok((status, headers, body))
}
