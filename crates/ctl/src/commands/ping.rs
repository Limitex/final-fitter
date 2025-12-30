use daemon::generated::{PingRequest, ping_service_client::PingServiceClient};

use crate::cli::Args;
use crate::error::{CtlError, Result};
use crate::infra::grpc::connect;
use crate::infra::process::is_running;

pub async fn execute(args: &Args, message: &str) -> Result<()> {
    require_running(args)?;

    let channel = connect(args).await?;

    let mut client = PingServiceClient::new(channel);
    let response = client
        .ping(tonic::Request::new(PingRequest {
            message: message.to_string(),
        }))
        .await?;

    println!("{}", response.into_inner().message);
    Ok(())
}

fn require_running(args: &Args) -> Result<()> {
    if !is_running(&args.pid_file) {
        return Err(CtlError::DaemonNotRunning);
    }
    Ok(())
}
