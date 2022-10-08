//! The server for `Peek`
//!
//! Powered by [`axum`]
#![forbid(unsafe_code, missing_docs)]

use axum::{extract::FromRef, routing::*, Router};
use opentelemetry_otlp::WithExportConfig;
use sqlx::{migrate, SqlitePool};
use std::{collections::HashMap, fs::OpenOptions, net::SocketAddr, time::Duration};

use tower_cookies::{CookieManagerLayer, Key};
use tower_http::trace::TraceLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{prelude::*, EnvFilter};
use tracing_tree::HierarchicalLayer;

mod auth;
mod routes;
mod templates;

/// The External APIs for Peek
pub mod external_apis;

use color_eyre::eyre::Result;

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
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opentelemetry_layer = if let Ok(honeycomb_key) = std::env::var("HONEYCOMB_KEY") {
        let mut map = HashMap::<String, String>::new();
        map.insert("x-honeycomb-team".to_string(), honeycomb_key);
        map.insert("x-honeycomb-dataset".to_string(), "peek".to_string());

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .http()
                    .with_endpoint("https://api.honeycomb.io/v1/traces")
                    .with_timeout(Duration::from_secs(3))
                    .with_headers(map),
            )
            .install_batch(opentelemetry::runtime::Tokio)?;

        let opentelemetry_layer = OpenTelemetryLayer::new(tracer);

        Some(opentelemetry_layer)
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "peek=debug,tower_http=debug".into()),
        ))
        .with(HierarchicalLayer::new(3))
        .with(opentelemetry_layer)
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let path = std::env::var("DATABASE_PATH");

        if let Ok(p) = &path {
            OpenOptions::new().write(true).create(true).open(p).unwrap();

            format!("sqlite:{}", p)
        } else {
            "sqlite::memory:".to_string()
        }
    });

    let pool = SqlitePool::connect(&database_url).await?;

    migrate!("./migrations/").run(&pool).await?;

    let pool = Pool(pool);

    let key = Key::generate();
    let key = CookieKey(key);

    let state = AppState { pool, key };

    let app = Router::with_state(state)
        // Root Route
        .route("/", get(routes::landing))
        // Auth Routes
        .route("/sign-in", get(routes::auth::sign_in::get::router))
        .route("/sign-in", post(routes::auth::sign_in::post::router))
        .route("/sign-up", get(routes::auth::sign_up::get::router))
        .route("/sign-up", post(routes::auth::sign_up::post::router))
        .route("/sign-out", post(routes::auth::sign_out))
        // Bing News Search
        .route("/news", get(routes::news::get::router))
        .route("/news", post(routes::news::post::router))
        // Old Route, basically a legacy page at this point
        .route("/team", get(routes::team))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
