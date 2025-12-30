use crate::cli::Args;
use crate::error::Result;
use crate::infra::process::{process_exists, read_pid};

pub fn execute(args: &Args) -> Result<()> {
    match read_pid(&args.pid_file) {
        Ok(pid) if process_exists(pid) => {
            println!("Running (PID: {})", pid);
        }
        _ => {
            println!("Not running");
        }
    }
    Ok(())
}
