use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use maud::html;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    templates::{self, base},
    CookieKey, Pool,
};

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

pub async fn landing() -> impl IntoResponse {
    templates::base(html! {
      h1 { "Hello, World!" }

      a href="/sign-up" { "Sign Up" }
      a href="/sign-in" { "Sign In" }
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

pub async fn sign_in_get() -> impl IntoResponse {
    templates::base(html! {
      h1 { "Sign In" }

      form action="/sign-in" method="post" {
        input type="email" name="email" placeholder="Email";
        input type="password" name="password" placeholder="Password";

        input type="submit" value="Sign Up";
      }
    })
}

#[derive(Deserialize)]
pub struct SignInForm {
    email: String,
    password: String,
}

pub async fn sign_in_post(
    State(Pool(pool)): State<Pool>,
    Form(form): Form<SignInForm>,
) -> impl IntoResponse {
    let user = sqlx::query!("SELECT * FROM Users WHERE email = ?", form.email)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .unwrap();

    templates::base(html! {
      h1 { "Hello, " (user.name) "!" }
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

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub async fn sign_up_post(State(Pool(pool)): State<Pool>, form: Form<SignUp>) -> impl IntoResponse {
    if form.password != form.password_confirmation {
        return base(html! {
          h1 { "Passwords do not match" }
        });
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let insert_result = sqlx::query!(
        "INSERT INTO Users (name, email, password_hash) VALUES (?, ?, ?)",
        form.name,
        form.email,
        password_hash
    )
    .fetch_optional(&pool)
    .await;

    match insert_result {
        Err(sqlx::Error::Database(err)) => {
            assert_eq!(err.code().unwrap(), "2067", "Unexpected error code");

            templates::base(html! {
              h3 { "Email has already been taken" }
            })
        }
        Err(err) => {
            panic!("Unexpected error: {:?}", err);
        }
        Ok(_) => templates::base(html! {
          h1 { "Hello, " (form.name) "!" }

          form action="/sign-out" method="post" {
            input type="submit" value="Sign Out";
          }
        }),
    }
}

pub async fn sign_out() -> impl IntoResponse {
    Redirect::to("/").into_response()
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
