use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
    response::{IntoResponse, Response},
    Form,
};
use maud::html;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{templates, CookieKey, Pool};

pub struct Session {
    id: i64,
}

#[async_trait]
impl<State> FromRequestParts<State> for Session
where
    State: Send + Sync,
    CookieKey: FromRef<State>,
    Pool: FromRef<State>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let Pool(pool) = Pool::from_ref(state);
        let cookies: Cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;
        let CookieKey(key) = CookieKey::from_ref(state);

        let session_id = cookies.private(&key).get("peek-session-id");
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
            .add(Cookie::new("peek-session-id", session_id.to_string()));

        Ok(Session { id: session_id })
    }
}

pub async fn landing() -> impl IntoResponse {
    templates::base(html! {
      h1 { "Hello, World!" }

      a href="/sign-up" { "Sign Up" }
    })
}

pub async fn sign_up_get() -> impl IntoResponse {
    templates::base(html! {
      h1 { "Create Account" }

      form action="/sign-up" method="post" {
        input type="text" name="name" placeholder="Name";
        input type="email" name="email" placeholder="Email";
        input type="password" name="password" placeholder="Password";
        input type="password" name="passwordConfirmation" placeholder="Repeat Password";
        input type="submit" value="Sign Up";
      }
    })
}

#[derive(Deserialize)]
pub struct SignUp {
    name: String,
    email: String,
    password: String,
    #[serde(rename = "passwordConfirmation")]
    password_confirmation: String,
}

pub async fn sign_up_post(form: Form<SignUp>) -> impl IntoResponse {
    let name = &form.name;

    templates::base(html! {
      h1 { "Hello, " (name) "!" }
    })
}

pub async fn root(session: Session, State(Pool(pool)): State<Pool>) -> impl IntoResponse {
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
