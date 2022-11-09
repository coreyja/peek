//! The server for `Peek`
//!
//! Powered by [`axum`]
#![forbid(unsafe_code, missing_docs)]

use axum::{
    body::{self, Empty, Full},
    extract::{FromRef, Path},
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::*,
    Router,
};
use opentelemetry_otlp::WithExportConfig;
use reqwest::{header, StatusCode};
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
#[derive(Clone, Debug)]
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

use include_dir::{include_dir, Dir};
static PKG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/pkg");
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

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
        .route("/pkg/*path", get(pkg_path))
        .route("/static/*path", get(static_path))
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
        // Team Members
        .route("/team_members", get(routes::team_members::get::router))
        .route("/team_members", post(routes::team_members::post::router))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn pkg_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match PKG_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
