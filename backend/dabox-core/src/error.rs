use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum DaError {
    #[error(
        "Access denied: {requested_by} try to access a resource that is owned by {resource_owner}"
    )]
    AccessDenied {
        requested_by: EntityUid,
        resource_owner: EntityUid,
    },
    #[error("Directory depth limit exceeded (max: {0})")]
    DirectoryDepthLimitExceeded(u32),
    #[error("No directory with id {0:?} found")]
    DirectoryNotFound(DaDirectorySid),
    #[error(transparent)]
    #[cfg(feature = "database")]
    SqlxError(#[from] sqlx::Error),
    #[error("Failed to apply migrations")]
    #[cfg(feature = "database")]
    SqlxMigration(#[from] sqlx::migrate::MigrateError),
}

pub type DaResult<T> = Result<T, DaError>;
