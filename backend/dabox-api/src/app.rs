use crate::prelude::*;
use crate::routes::directory::*;

use axum::routing::{delete, get, post, put};
use axum::Router;
use tokio::net::ToSocketAddrs;
use tower_http::trace::TraceLayer;

pub trait AppExt {
    fn serve<A: ToSocketAddrs>(self, addr: A) -> impl Future<Output = std::io::Result<()>>;
}

pub fn create_app<R: DaRepository + 'static>(repository: Arc<R>) -> Router {
    Router::new()
        .route("/directory/:id", get(get_directory::<R>))
        .route("/directory", post(post_directory::<R>))
        .route("/directory/:id", put(put_directory::<R>))
        .route("/directory/:id", delete(delete_directory::<R>))
        .with_state(repository)
        // Add tracing layer to generate spans for each http request regardless of the potential handler
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &extract::Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<extract::MatchedPath>()
                    .map(extract::MatchedPath::as_str);
                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
}

impl AppExt for Router {
    async fn serve<A: ToSocketAddrs>(self, addr: A) -> std::io::Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, self.into_make_service()).await?;
        Ok(())
    }
}