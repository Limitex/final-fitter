use tonic::Status;

use crate::error::DaemonError;

impl From<DaemonError> for Status {
    fn from(err: DaemonError) -> Self {
        match err {
            DaemonError::EmptyMessage => Status::invalid_argument(err.to_string()),
            _ => Status::internal(err.to_string()),
        }
    }
}
