use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use sqlx::query;

use crate::{auth::Session, Pool};

pub(crate) mod sign_in;

pub(crate) mod sign_up;

pub async fn sign_out(session: Session, State(Pool(pool)): State<Pool>) -> impl IntoResponse {
    query!(
        "UPDATE Sessions SET user_id = NULL, updated_at = datetime() WHERE id = ?",
        session.id
    )
    .execute(&pool)
    .await
    .unwrap();

    Redirect::to("/")
}
