use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
    Extension,
};
use maud::html;
use sqlx::SqlitePool;
use tower_cookies::{Cookie, Cookies, Key};

use crate::templates;

pub struct Session {
    id: i64,
}

#[async_trait]
impl<Body: Send> FromRequest<Body> for Session {
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let Extension(pool): Extension<SqlitePool> = Extension::from_request(req)
            .await
            .map_err(|err| err.into_response())?;
        let cookies: Cookies = Cookies::from_request(req)
            .await
            .map_err(|err| err.into_response())?;
        let Extension(key): Extension<Key> = Extension::from_request(req)
            .await
            .map_err(|err| err.into_response())?;

        let session_id = cookies.private(&key).get("yanwa-session-id");
        let existing_session_id: Option<i64> = if let Some(session_id) = session_id {
            let session_id = session_id.value();
            sqlx::query!("SELECT * FROM Sessions WHERE id = ?", session_id)
                .fetch_optional(&pool)
                .await
                .unwrap()
                .map(|session| session.id)
        } else {
            None
        };
        let session_id = if let Some(session_id) = existing_session_id {
            session_id
        } else {
            let session_id = sqlx::query!("INSERT INTO Sessions DEFAULT VALUES RETURNING id")
                .fetch_one(&pool)
                .await
                .unwrap()
                .id;
            session_id
        };
        cookies
            .private(&key)
            .add(Cookie::new("yanwa-session-id", session_id.to_string()));

        Ok(Session { id: session_id })
    }
}

pub async fn root(session: Session, Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let session_count = sqlx::query!("SELECT COUNT(*) as count FROM Sessions")
        .fetch_one(&pool)
        .await
        .unwrap()
        .count;
    templates::base(html! {
      h1 { "Hello, World!" }

      p { "We have " (session_count) " sessions." }
      p { "Your session_id is " (session.id) }
    })
}
