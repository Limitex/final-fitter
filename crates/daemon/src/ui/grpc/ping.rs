use crate::generated::{PingRequest, PingResponse, ping_service_server::PingService};
use crate::usecase::PingUseCase;
use tonic::{Request, Response, Status};
use tracing::{debug, instrument};

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
    #[instrument(skip(self), fields(message = %request.get_ref().message))]
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        debug!("Received ping request");
        let req = request.into_inner();

        let message = self.use_case.ping(req.message).map_err(Status::from)?;

        debug!("Ping request completed");
        Ok(Response::new(PingResponse { message }))
    }
}
