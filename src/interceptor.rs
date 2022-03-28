use tonic::{service::Interceptor, Request, Status};
use uuid::Uuid;

#[derive(Clone)]
pub struct TinkoffInvestInterceptor {
    token: String,
}

impl TinkoffInvestInterceptor {
    #[inline]
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Interceptor for TinkoffInvestInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let mut req = request;
        req.metadata_mut().append(
            "authorization",
            format!("bearer {}", self.token).parse().unwrap(),
        );
        req.metadata_mut()
            .append("x-tracking-id", Uuid::new_v4().to_string().parse().unwrap());
        Ok(req)
    }
}
