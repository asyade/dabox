use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum DaError {
    #[error("Directory depth limit exceeded (max: {0})")]
    DirectoryDepthLimitExceeded(u32),
    #[error("No directory with id {0:?} found")]
    DirectoryNotFound(DaDirectorySid),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

pub type DaResult<T> = Result<T, DaError>;
