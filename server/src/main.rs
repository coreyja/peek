//! The server for `YANWA`
//!
//! Powered by [`axum`]
#![forbid(unsafe_code, missing_docs)]

use axum::{routing::get, Extension, Router};
use sqlx::{migrate, SqlitePool};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::*, EnvFilter};
use tracing_tree::HierarchicalLayer;

mod routes;
mod templates;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "yanwa=debug,tower_http=debug".into()),
        ))
        .with(HierarchicalLayer::new(3))
        .init();

    let pool = SqlitePool::connect(
        &std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".into()),
    )
    .await
    .unwrap();

    migrate!("./migrations/").run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(routes::root))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
