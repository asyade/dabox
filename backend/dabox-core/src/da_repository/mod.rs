/// This module contains extensions trait that allow persistence of `DaDirectory` objects across agnostic backends (e.g. Postgres, SQLite, etc.).
/// In the current implementation, only Postgres is supported but implementing the `DaRepository` trait for other backends should be straightforward.
use crate::prelude::*;
pub(crate) mod backends;

pub use backends::mem::MemRepository;
pub use backends::pg::PgRepository;

pub trait DaRepositoryExt {
    /// Creates a new directory.
    ///
    /// # Arguments
    ///
    /// * `requested_by` - The Uid of the entity that is requesting the creation of the directory.
    /// * `name` - The name of the directory.
    /// * `parent` - The parent directory of the new directory (if any).
    fn create_directory(
        &self,
        requested_by: EntityUid,
        name: &str,
        parent: Option<DaDirectorySid>,
    ) -> impl Future<Output = DaResult<DaDirectory>> + Send;

    /// Retrieves a directory by its ID.
    ///
    /// # Arguments
    ///
    /// * `requested_by` - The Uid of the entity that is requesting the retrieval of the directory.
    /// * `id` - The ID of the directory to retrieve.
    fn get_directory(
        &self,
        requested_by: EntityUid,
        id: DaDirectorySid,
    ) -> impl Future<Output = DaResult<DaDirectory>> + Send;

    /// Deletes a directory by its ID.
    ///
    /// # Arguments
    ///
    /// * `requested_by` - The Uid of the entity that is requesting the deletion of the directory.
    /// * `id` - The ID of the directory to delete.
    fn delete_directory(
        &self,
        requested_by: EntityUid,
        id: DaDirectorySid,
    ) -> impl Future<Output = DaResult<()>> + Send;

    /// Renames a directory by its ID.
    ///
    /// # Arguments
    ///
    /// * `requested_by` - The Uid of the entity that is requesting the renaming of the directory.
    /// * `id` - The ID of the directory to rename.
    /// * `name` - The new name of the directory.
    fn rename_directory(
        &self,
        requested_by: EntityUid,
        id: DaDirectorySid,
        name: &str,
    ) -> impl Future<Output = DaResult<()>> + Send;
}
