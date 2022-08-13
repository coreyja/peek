//! The server for `YANWA`
//!
//! Powered by [`axum`]
#![forbid(unsafe_code, missing_docs)]

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use maud::html;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    html! {
      h1 { "Hello, World!" }
    }
}
