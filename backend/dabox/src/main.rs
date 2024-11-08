use std::sync::Arc;

use dabox_api::app::{create_app, AppExt};
use dabox_core::da_repository::MemRepository;
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter};

const DEFAULT_LISTEN_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let listen_addres =
        std::env::var("LISTEN_ADDRESS").unwrap_or(DEFAULT_LISTEN_ADDRESS.to_string());

    let repository = Arc::new(MemRepository::new());

    #[cfg(feature = "default-dataset")]
    populate_repository(&repository).await;

    info!(listen_address = listen_addres, "Starting Dabox API");
    create_app(repository)
        .enable_cors()
        .serve(listen_addres)
        .await
        .unwrap();
}

/// Populate the repository with a default dataset
#[cfg(feature = "default-dataset")]
async fn populate_repository(repository: &MemRepository) {
    use dabox_core::{
        entity::EntityUid,
        prelude::{DaDirectorySid, DaRepository},
    };

    enum PlaceHolder {
        Empty,
        WithChildren(Vec<PlaceHolder>),
    }

    #[async_recursion::async_recursion]
    async fn populate_bucket(
        uid: EntityUid,
        repository: &MemRepository,
        place_holders: PlaceHolder,
        parent_sid: Option<DaDirectorySid>,
    ) {
        let dir = repository
            .create_directory(uid, "Empty", parent_sid)
            .await
            .unwrap();
        match place_holders {
            PlaceHolder::Empty => {}
            PlaceHolder::WithChildren(children) => {
                for child in children {
                    populate_bucket(uid, repository, child, Some(dir.sid)).await;
                }
            }
        }
    }

    populate_bucket(
        0,
        repository,
        PlaceHolder::WithChildren(vec![
            PlaceHolder::Empty,
            PlaceHolder::WithChildren((0..5).map(|_| PlaceHolder::Empty).collect()),
            PlaceHolder::Empty,
            PlaceHolder::WithChildren((0..1000).map(|_| PlaceHolder::Empty).collect()),
            PlaceHolder::WithChildren((0..1000).map(|_| PlaceHolder::Empty).collect()),
        ]),
        None,
    )
    .await;

    populate_bucket(
        42,
        repository,
        PlaceHolder::WithChildren(vec![
            PlaceHolder::Empty,
            PlaceHolder::WithChildren((0..100000).map(|_| PlaceHolder::Empty).collect()),
            PlaceHolder::WithChildren((0..100000).map(|_| PlaceHolder::Empty).collect()),
            PlaceHolder::WithChildren((0..100000).map(|_| PlaceHolder::Empty).collect()),
            PlaceHolder::WithChildren((0..100000).map(|_| PlaceHolder::Empty).collect()),
            PlaceHolder::WithChildren((0..100000).map(|_| PlaceHolder::Empty).collect()),
        ]),
        None,
    )
    .await;
}
