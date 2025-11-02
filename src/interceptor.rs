use tonic::{
    Request, Status,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TinkoffInvestInterceptor {
    authorization: MetadataValue<Ascii>,
}

impl TinkoffInvestInterceptor {
    #[inline]
    pub fn new(token: String) -> Self {
        let authorization = format!("bearer {}", token).parse().unwrap();
        Self { authorization }
    }
}

impl Interceptor for TinkoffInvestInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let mut req = request;
        req.metadata_mut()
            .append("authorization", self.authorization.clone());
        req.metadata_mut()
            .append("x-tracking-id", Uuid::new_v4().to_string().parse().unwrap());
        Ok(req)
    }
}
