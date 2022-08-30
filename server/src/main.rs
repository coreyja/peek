//! The server for `YANWA`
//!
//! Powered by [`axum`]
#![forbid(unsafe_code, missing_docs)]

use axum::{extract::FromRef, routing::*, Router};
use sqlx::{migrate, SqlitePool};
use std::net::SocketAddr;
use tower_cookies::{CookieManagerLayer, Key};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::*, EnvFilter};
use tracing_tree::HierarchicalLayer;

mod routes;
mod templates;

/// Holder of the [Key] we use for Cookies
#[derive(Clone)]
pub struct CookieKey(Key);

impl FromRef<AppState> for CookieKey {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

/// Wrapper around a SqlitePool
#[derive(Clone)]
pub struct Pool(SqlitePool);

impl FromRef<AppState> for Pool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

struct AppState {
    pool: Pool,
    key: CookieKey,
}

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

    let pool = Pool(pool);

    let key = Key::generate();
    let key = CookieKey(key);

    let state = AppState { pool, key };

    let app = Router::with_state(state)
        .route("/", get(routes::landing))
        .route("/sign-in", get(routes::sign_in_get))
        .route("/sign-in", post(routes::sign_in_post))
        .route("/sign-up", get(routes::sign_up_get))
        .route("/sign-up", post(routes::sign_up_post))
        .route("/sign-out", post(routes::sign_out))
        .route("/team", get(routes::root))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
