use crate::config::CtlConfig;
use crate::error::Result;
use crate::infra::process::{process_exists, read_pid};

pub fn execute(config: &CtlConfig) -> Result<()> {
    match read_pid(&config.pid_file) {
        Ok(pid) if process_exists(pid) => {
            println!("Running (PID: {})", pid);
        }
        _ => {
            println!("Not running");
        }
    }
    Ok(())
}
