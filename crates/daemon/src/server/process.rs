use crate::cli::Args;

#[cfg(unix)]
pub fn daemonize(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    use daemonize::Daemonize;
    use std::fs::OpenOptions;

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&args.log_file)?;
    let err_file = log_file.try_clone()?;

    Daemonize::new()
        .pid_file(&args.pid_file)
        .chown_pid_file(true)
        .working_directory(&args.workdir)
        .umask(0o027)
        .stdout(log_file)
        .stderr(err_file)
        .start()
        .map_err(|e| format!("Failed to daemonize: {}", e))?;

    Ok(())
}

#[cfg(not(unix))]
pub fn daemonize(_args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    Err("Daemon mode not supported on this platform".into())
}

pub const fn is_daemon_supported() -> bool {
    cfg!(unix)
}
