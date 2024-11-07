use sqlx::PgPool;

use crate::prelude::*;

/// A Postgres-backed implementation of the `DaRepositoryExt` trait.
/// Able to handle a lot of concurrent requests based on the underlying Postgres server.
pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
