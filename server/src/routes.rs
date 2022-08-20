use axum::{response::IntoResponse, Extension};
use maud::html;
use sqlx::SqlitePool;
use tower_cookies::{Cookies, Key};

use crate::templates;

pub async fn root(pool: Extension<SqlitePool>, cookies: Cookies) -> impl IntoResponse {
    let session_count = sqlx::query!("SELECT COUNT(*) as count FROM Sessions")
        .fetch_one(&*pool)
        .await
        .unwrap()
        .count;

    let key = Key::generate();
    let session_id = cookies.private(&key).get("yanwa-session-id");
    let existing_session_id: Option<i64> = if let Some(session_id) = session_id {
        let session_id = session_id.value();
        sqlx::query!("SELECT * FROM Sessions WHERE id = ?", session_id)
            .fetch_optional(&*pool)
            .await
            .unwrap()
            .map(|session| session.id)
    } else {
        None
    };
    let session_id = existing_session_id.unwrap_or_else(|| {
        let session_id = sqlx::query!("INSERT INTO Sessions (created_at) VALUES (?)", Utc::now())
            .fetch_one(&*pool)
            .await
            .unwrap()
            .id;
        session_id
    });
    cookies
        .private(&key)
        .set("yanwa-session-id", session_id.to_string());
    templates::base(html! {
      h1 { "Hello, World!" }

      p { "We have " (session_count) " sessions." }
    })
}
