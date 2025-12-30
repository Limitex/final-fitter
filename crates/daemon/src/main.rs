use daemon::{Server, server::ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::new()
        .with_tcp("[::1]:50051".parse()?)
        .with_uds("/tmp/ffit-daemon.sock");

    Server::new(config).run().await
}
