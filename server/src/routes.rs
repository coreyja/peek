use axum::extract::State;
use axum::response::IntoResponse;
use maud::html;

use crate::{templates, Pool};

use crate::auth::{CurrentUser, Session};

pub(crate) mod auth;
pub(crate) mod news;

pub async fn landing(current_user: CurrentUser) -> impl IntoResponse {
    let name = current_user
        .0
        .map(|user| user.name)
        .unwrap_or_else(|| "stranger".into());

    templates::base(html! {
      h1 { "Hello, " (name) "!" }

      a href="/sign-up" { "Sign Up" }
      a href="/sign-in" { "Sign In" }

      form action="/sign-out" method="post" {
        input type="submit" value="Sign Out";
      }
    })
}

pub async fn team(session: Session, State(Pool(pool)): State<Pool>) -> impl IntoResponse {
    let session_count = sqlx::query!("SELECT COUNT(*) as count FROM Sessions")
        .fetch_one(&pool)
        .await
        .unwrap()
        .count;
    templates::base(html! {
        h1 { "Hello, World!" }

        p { "We have " (session_count) " sessions." }
        p { "Your session_id is " (session.id) }

        @match session.user_id {
            Some(user_id) => {
                p { "You are signed in as user " (user_id) }
            }
            None => {
                p { "You are not signed in" }
            }
       }
    })
}
