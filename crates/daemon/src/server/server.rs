use std::net::SocketAddr;

use crate::di::Container;
use crate::ui::GrpcRouter;

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let container = Container::new();
        let router = GrpcRouter::build(container.ping_handler);

        println!("Server listening on {}", self.addr);

        router.serve(self.addr).await?;

        Ok(())
    }
}
