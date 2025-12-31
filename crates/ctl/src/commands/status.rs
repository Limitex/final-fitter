use crate::config::CtlConfig;
use crate::error::Result;
use crate::infra::process::{process_exists, read_pid};
use crate::{log_dim, log_info};

pub fn execute(config: &CtlConfig) -> Result<()> {
    match read_pid(&config.pid_file) {
        Ok(pid) if process_exists(pid) => {
            log_info!("Running (PID: {})", pid);
        }
        _ => {
            log_dim!("Not running");
        }
    }
    Ok(())
}
