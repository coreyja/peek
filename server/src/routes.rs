use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use maud::{html, Markup};
use tracing::{info, instrument};

use crate::templates::base;
use crate::{templates, Pool};

use crate::auth::{CurrentUser, OptionalCurrentUser, Session};

pub(crate) mod auth;
pub(crate) mod news;
pub(crate) mod team_members;

#[instrument]
pub async fn landing(
    OptionalCurrentUser(current_user): OptionalCurrentUser,
    State(pool): State<Pool>,
) -> Response {
    if current_user.is_some() {
        return Redirect::to("/home").into_response();
    }

    templates::base(html! {
        h1 class="text-center my-8 font-serif text-2xl text-[#001571] font-bold" { "Weather & News Updates" }
        img src="static/hero.png" alt="Peek Hero" class="my-8";

        p class="font-sans my-4 px-8 text-center leading-relaxed text-[#000620] text-2xl" {
            "Taking a peek at local weather and news, keeps you connected with your long distance coworkers."
        }


        a href="/sign-up" class="text-xl block mx-8 my-2 py-2 px-8 font-bold bg-[#CADFFF] text-center font-sans text-[#001571] rounded-lg" { "Sign Up" }
        a href="/sign-in" class="text-xl block mx-8 my-2 py-2 px-8 font-bold bg-[#001571] text-center font-sans text-[#CADFFF] rounded-lg" { "Sign In" }
    }, false).into_response()
}

#[instrument]
pub async fn home(
    session: Session,
    CurrentUser(current_user): CurrentUser,
    State(Pool(pool)): State<Pool>,
) -> impl IntoResponse {
    let team_members = sqlx::query!(
        "SELECT * FROM TeamMembers WHERE user_id = ?",
        current_user.id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    base(
        html! {
            img src="static/under-logo.png" alt="" class="w-1/2 mx-auto -mt-8";

            h1 class="text-center font-serif text-2xl text-[#001571] font-bold" { "Welcome to Peek!" }

            p class="font-sans mt-4 px-12 text-center leading-relaxed text-[#000620] text-xl font-light" {
                "Local weather and news small talk starters to help connect with far away team members"
            }

            @if team_members.is_empty() {
                img
                    src="static/home-page-empty.png"
                    alt="Image of lady on computer, her team mate is chatting in a chat bubble above her laptop"
                    ;

                h1 class="text-center font-serif text-2xl text-[#001571] font-bold" { "Add your first team member!" }
            } @else {
                ul {
                    @for team_member in team_members {
                        li { (team_member.name)  "  ("  (team_member.zip_code)  ")" }
                    }
                }
            }

            form action="/sign-out" method="post" {
                input type="submit" value="Sign Out";
            }
        },
        true,
    )
}
