use tonic::{
    Request, Status,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
};

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
        Ok(req)
    }
}
