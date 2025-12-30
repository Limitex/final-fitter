use crate::cli::config::{GRACEFUL_SHUTDOWN_ATTEMPTS, SHUTDOWN_POLL_INTERVAL};
use crate::cli::Args;
use crate::error::Result;
use crate::infra::process::{Signal, process_exists, read_pid, remove_pid_file, send_signal};

pub fn execute(args: &Args) -> Result<()> {
    let pid = read_pid(&args.pid_file)?;

    send_signal(pid, Signal::Term)?;

    // Wait for graceful shutdown
    for _ in 0..GRACEFUL_SHUTDOWN_ATTEMPTS {
        std::thread::sleep(SHUTDOWN_POLL_INTERVAL);
        if !process_exists(pid) {
            remove_pid_file(&args.pid_file);
            println!("Daemon stopped");
            return Ok(());
        }
    }

    // Force kill if still running
    send_signal(pid, Signal::Kill)?;
    remove_pid_file(&args.pid_file);
    println!("Daemon killed");
    Ok(())
}
