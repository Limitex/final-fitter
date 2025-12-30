use tonic::{Request, Response, Status};

use crate::generated::{PingRequest, PingResponse, ping_service_server::PingService};
use crate::usecase::PingUseCase;

pub struct PingHandler {
    use_case: PingUseCase,
}

impl PingHandler {
    pub fn new(use_case: PingUseCase) -> Self {
        Self { use_case }
    }
}

#[tonic::async_trait]
impl PingService for PingHandler {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let req = request.into_inner();
        let message = self.use_case.ping(req.message);
        Ok(Response::new(PingResponse { message }))
    }
}
