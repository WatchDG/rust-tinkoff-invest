/// Трейт для получения идентификатора запроса (x-tracking-id)
pub trait RequestId {
    fn request_id(&self) -> Option<&str>;
}
