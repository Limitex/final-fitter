use crate::generated::FILE_DESCRIPTOR_SET;
use crate::generated::ping_service_server::PingServiceServer;
use crate::ui::grpc::PingHandler;
use tonic::transport::server::Router;

pub struct GrpcRouter;

impl GrpcRouter {
    pub fn build(ping_handler: PingHandler) -> Router {
        let reflection = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1()
            .unwrap();

        tonic::transport::Server::builder()
            .add_service(reflection)
            .add_service(PingServiceServer::new(ping_handler))
    }
}
