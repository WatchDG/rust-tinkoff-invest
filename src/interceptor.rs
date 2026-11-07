use std::fmt;

use tonic::{
    Request, Status,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
};

use crate::TError;

/// Interceptor, добавляющий Bearer-токен в metadata каждого запроса.
#[derive(Clone)]
pub struct TInterceptor {
    authorization: MetadataValue<Ascii>,
}

impl fmt::Debug for TInterceptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TInterceptor")
            .field("authorization", &"bearer ***")
            .finish()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_redacts_token() {
        let interceptor = TInterceptor::new("secret-token").unwrap();
        let rendered = format!("{interceptor:?}");
        assert!(!rendered.contains("secret-token"));
        assert!(rendered.contains("bearer ***"));
    }
}
