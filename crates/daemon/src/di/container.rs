use std::sync::Arc;

use crate::ui::grpc::PingHandler;
use crate::usecase::PingUseCase;

pub struct Container {
    pub ping_handler: Arc<PingHandler>,
}

impl Container {
    pub fn new() -> Self {
        let ping_use_case = PingUseCase;
        let ping_handler = Arc::new(PingHandler::new(ping_use_case));

        Self { ping_handler }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
