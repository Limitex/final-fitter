use daemon::generated::{PingRequest, ping_service_client::PingServiceClient};

use crate::config::CtlConfig;
use crate::error::{CtlError, Result};
use crate::infra::grpc::connect;
use crate::infra::process::is_running;
use crate::log_success;

pub async fn execute(config: &CtlConfig, message: &str) -> Result<()> {
    require_running(config)?;

    let channel = connect(config).await?;

    let mut client = PingServiceClient::new(channel);
    let response = client
        .ping(tonic::Request::new(PingRequest {
            message: message.to_string(),
        }))
        .await?;

    log_success!("{}", response.into_inner().message);
    Ok(())
}

fn require_running(config: &CtlConfig) -> Result<()> {
    if !is_running(&config.pid_file) {
        return Err(CtlError::DaemonNotRunning);
    }
    Ok(())
}
