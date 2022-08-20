use axum::{response::IntoResponse, Extension};
use maud::html;
use sqlx::SqlitePool;

use crate::templates;

pub async fn root(pool: Extension<SqlitePool>) -> impl IntoResponse {
    let session_count = sqlx::query!("SELECT COUNT(*) as count FROM Sessions")
        .fetch_one(&*pool)
        .await
        .unwrap()
        .count;
    templates::base(html! {
      h1 { "Hello, World!" }

      p { "We have " (session_count) " sessions." }
    })
}
