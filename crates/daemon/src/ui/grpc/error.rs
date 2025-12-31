use tonic::Status;

use crate::error::DaemonError;

impl From<DaemonError> for Status {
    fn from(err: DaemonError) -> Self {
        match &err {
            // Client errors (4xx equivalent)
            DaemonError::EmptyMessage => Status::invalid_argument(err.to_string()),
            DaemonError::InvalidAddress(_) => Status::invalid_argument(err.to_string()),

            // Precondition failures
            DaemonError::ConfigError(_) => Status::failed_precondition(err.to_string()),
            DaemonError::NoListenersConfigured => Status::failed_precondition(err.to_string()),

            // Resource conflicts
            DaemonError::AlreadyRunning => Status::already_exists(err.to_string()),
            DaemonError::LockError(_) => Status::unavailable(err.to_string()),

            // Internal server errors
            DaemonError::DaemonizeError(_) => Status::internal(err.to_string()),
            DaemonError::ReflectionError(_) => Status::internal(err.to_string()),
            DaemonError::IoError(_) => Status::internal(err.to_string()),
        }
    }
}
