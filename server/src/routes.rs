use axum::extract::State;
use axum::response::IntoResponse;
use maud::{html, Markup};
use tracing::{info, instrument};

use crate::{templates, Pool};

use crate::auth::{CurrentUser, OptionalCurrentUser, Session};

pub(crate) mod auth;
pub(crate) mod news;
pub(crate) mod team_members;

#[instrument]
pub async fn landing(
    current_user: OptionalCurrentUser,
    State(pool): State<Pool>,
) -> impl IntoResponse {
    async fn user_content(current_user: &CurrentUser, pool: Pool) -> Markup {
        let team_members = sqlx::query!(
            "SELECT * FROM TeamMembers WHERE user_id = ?",
            current_user.0.id
        )
        .fetch_all(&pool.0)
        .await
        .unwrap();

        html! {
            div {
                p { "You are logged in!" }

                ul {
                    @for team_member in team_members {
                        li { (team_member.name)  "  ("  (team_member.zip_code)  ")" }
                    }
                }
            }
        }
    }

    let user_markup = match current_user.0 {
        Some(ref user) => user_content(user, pool).await,
        None => html! {},
    };

    let name = current_user
        .0
        .map(|user| user.0.name)
        .unwrap_or_else(|| "stranger".into());

    info!("Landing page for {}", name);

    templates::base(html! {
      h1 { "Hello, " (name) "!" }
      (user_markup)

      a href="/sign-up" { "Sign Up" }
      a href="/sign-in" { "Sign In" }

      a href="/team_members" { "Add Team Member" }

      form action="/sign-out" method="post" {
        input type="submit" value="Sign Out";
      }

    })
}

#[instrument(skip(pool))]
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
