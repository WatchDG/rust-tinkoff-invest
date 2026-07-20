use tonic::{
    Request, Status,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
};

use crate::TError;

/// Interceptor, добавляющий Bearer-токен в metadata каждого запроса.
#[derive(Debug, Clone)]
pub struct TInterceptor {
    authorization: MetadataValue<Ascii>,
}

impl TInterceptor {
    /// Создаёт interceptor из токена Invest API.
    pub fn new(token: impl AsRef<str>) -> Result<Self, TError> {
        let authorization = format!("bearer {}", token.as_ref())
            .parse()
            .map_err(|e| TError::InvalidToken(format!("{e}")))?;
        Ok(Self { authorization })
    }
}

impl Interceptor for TInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let mut req = request;
        req.metadata_mut()
            .append("authorization", self.authorization.clone());
        Ok(req)
    }
}
