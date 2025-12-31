use crate::config::{CtlConfig, GRACEFUL_SHUTDOWN_ATTEMPTS, SHUTDOWN_POLL_INTERVAL};
use crate::error::Result;
use crate::infra::process::{Signal, process_exists, read_pid, remove_pid_file, send_signal};

pub fn execute(config: &CtlConfig) -> Result<()> {
    let pid = read_pid(&config.pid_file)?;

    // Graceful shutdown with SIGTERM
    send_signal(pid, Signal::Term)?;

    for _ in 0..GRACEFUL_SHUTDOWN_ATTEMPTS {
        std::thread::sleep(SHUTDOWN_POLL_INTERVAL);
        if !process_exists(pid) {
            remove_pid_file(&config.pid_file);
            println!("Daemon stopped");
            return Ok(());
        }
    }

    // Force kill if graceful shutdown timed out
    send_signal(pid, Signal::Kill)?;
    remove_pid_file(&config.pid_file);
    println!("Daemon killed");
    Ok(())
}
