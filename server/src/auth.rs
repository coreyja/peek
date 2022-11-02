use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use tracing::instrument;

use crate::{CookieKey, Pool};

#[derive(Deserialize, Serialize, Debug)]
pub struct CurrentUser(pub(crate) User);

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionalCurrentUser(pub(crate) Option<CurrentUser>);

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub id: i64,
    pub user_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}

const SESSION_COOKIE_KEY: &str = "peek-session-id";

#[async_trait]
impl<State> FromRequestParts<State> for Session
where
    State: Send + Sync,
    CookieKey: FromRef<State>,
    Pool: FromRef<State>,
{
    type Rejection = Response;

    #[instrument(skip_all, ret)]
    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let Pool(pool) = Pool::from_ref(state);
        let cookies: Cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;
        let CookieKey(key) = CookieKey::from_ref(state);

        let session_id = cookies.private(&key).get(SESSION_COOKIE_KEY);
        let existing_session: Option<Session> = if let Some(session_id) = session_id {
            let session_id = session_id.value();
            sqlx::query_as!(
                Session,
                "SELECT id, user_id FROM Sessions WHERE id = ?",
                session_id
            )
            .fetch_optional(&pool)
            .await
            .unwrap()
        } else {
            None
        };

        let session = if let Some(session) = existing_session {
            session
        } else {
            let session_id = sqlx::query!("INSERT INTO Sessions DEFAULT VALUES RETURNING id")
                .fetch_one(&pool)
                .await
                .unwrap()
                .id;
            Session {
                id: session_id,
                user_id: None,
            }
        };
        cookies
            .private(&key)
            .add(Cookie::new(SESSION_COOKIE_KEY, session.id.to_string()));

        Ok(session)
    }
}

#[async_trait]
impl<State> FromRequestParts<State> for CurrentUser
where
    State: Send + Sync,
    CookieKey: FromRef<State>,
    Pool: FromRef<State>,
{
    type Rejection = Response;

    #[instrument(skip_all, ret)]
    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let optional_current_user = OptionalCurrentUser::from_request_parts(parts, state).await?;

        if let Some(current_user) = optional_current_user.0 {
            Ok(current_user)
        } else {
            Err(Redirect::to("/sign-in").into_response())
        }
    }
}

#[async_trait]
impl<State> FromRequestParts<State> for OptionalCurrentUser
where
    State: Send + Sync,
    CookieKey: FromRef<State>,
    Pool: FromRef<State>,
{
    type Rejection = Response;

    #[instrument(skip_all, ret)]
    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        let user: Option<User> = if let Some(user_id) = session.user_id {
            let Pool(pool) = Pool::from_ref(state);

            sqlx::query_as!(User, "SELECT id, name FROM Users WHERE id = ?", user_id)
                .fetch_optional(&pool)
                .await
                .unwrap()
        } else {
            None
        };

        let current_user = user.map(CurrentUser);

        Ok(OptionalCurrentUser(current_user))
    }
}
