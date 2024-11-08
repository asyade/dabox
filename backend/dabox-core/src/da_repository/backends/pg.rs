/// This is a placeholder for the Postgres-backed implementation of the `DaRepositoryExt` trait.
use sqlx::PgPool;

use crate::prelude::*;

/// A Postgres-backed implementation of the `DaRepositoryExt` trait.
/// Able to handle a lot of concurrent requests based on the underlying Postgres server.
#[allow(dead_code)]
#[derive(Clone)]
pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl DaRepository for PgRepository {
    async fn create_directory(
        &self,
        _uid: EntityUid,
        _name: &str,
        _parent: Option<DaDirectorySid>,
    ) -> DaResult<DaDirectory> {
        todo!()
    }

    async fn get_directory(&self, _uid: EntityUid, _sid: DaDirectorySid) -> DaResult<DaDirectory> {
        todo!()
    }

    async fn delete_directory(&self, _uid: EntityUid, _sid: DaDirectorySid) -> DaResult<()> {
        todo!()
    }

    async fn rename_directory(
        &self,
        _uid: EntityUid,
        _sid: DaDirectorySid,
        _new_name: &str,
    ) -> DaResult<()> {
        todo!()
    }
}
