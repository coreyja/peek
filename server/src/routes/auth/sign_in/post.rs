use argon2::{PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use sqlx::query;

use crate::{auth::Session, Pool};

#[derive(Deserialize)]
pub struct SignInForm {
    email: String,
    #[allow(unused)]
    password: String,
}

pub enum SignInError {
    InvalidCredentials,
    UserNotFound,
}

impl IntoResponse for SignInError {
    fn into_response(self) -> Response {
        Redirect::to("/sign-in?flash=incorrect").into_response()
    }
}

pub async fn router(
    session: Session,
    State(Pool(pool)): State<Pool>,
    Form(form): Form<SignInForm>,
) -> Result<Response, SignInError> {
    let user = sqlx::query!("SELECT * FROM Users WHERE email = ?", form.email)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .ok_or(SignInError::UserNotFound)?;

    let hash = PasswordHash::new(&user.password_hash).unwrap();
    let argon2 = argon2::Argon2::default();
    if argon2
        .verify_password(form.password.as_bytes(), &hash)
        .is_ok()
    {
        query!(
            "UPDATE Sessions SET user_id = ? WHERE id = ?",
            user.id,
            session.id
        )
        .execute(&pool)
        .await
        .unwrap();

        Ok(Redirect::to("/").into_response())
    } else {
        Err(SignInError::InvalidCredentials)
    }
}
