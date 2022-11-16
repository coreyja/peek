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
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use maud::html;
use serde::Deserialize;

use crate::{
    auth::Session,
    templates::{self, base},
    Pool,
};

pub async fn router(
    session: Session,
    State(Pool(pool)): State<Pool>,
    form: Form<SignUp>,
) -> Response {
    if form.password != form.password_confirmation {
        return base(
            html! {
              h1 { "Passwords do not match" }
            },
            true,
        )
        .into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let insert_result = sqlx::query!(
        "INSERT INTO Users (name, email, password_hash) VALUES (?, ?, ?) RETURNING id",
        form.name,
        form.email,
        password_hash
    )
    .fetch_one(&pool)
    .await;

    match insert_result {
        Err(sqlx::Error::Database(err)) => {
            // 2067 is the Postgres error code for unique_violation
            assert_eq!(err.code().unwrap(), "2067", "Unexpected error code");

            templates::base(
                html! {
                  h3 { "Email has already been taken" }
                },
                true,
            )
            .into_response()
        }
        Err(err) => {
            panic!("Unexpected error: {:?}", err);
        }
        Ok(user_id) => {
            sqlx::query!(
                "UPDATE Sessions SET user_id = ?, updated_at = datetime() WHERE id = ?",
                user_id.id,
                session.id
            )
            .execute(&pool)
            .await
            .unwrap();

            Redirect::to("/").into_response()
        }
    }
}
