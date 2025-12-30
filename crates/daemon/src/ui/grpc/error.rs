use tonic::Status;

use crate::domain::DomainError;

impl From<DomainError> for Status {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::EmptyMessage => Status::invalid_argument(err.to_string()),
            DomainError::InvalidMessage(_) => Status::invalid_argument(err.to_string()),
        }
    }
}
