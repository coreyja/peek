use axum::{extract::Query, response::IntoResponse};
use maud::{html, Markup};
use serde::Deserialize;

use crate::templates::{
    self,
    components::{buttons::submit_button, inputs::form_input},
};

#[derive(Deserialize, Debug)]
pub struct SignInQuery {
    flash: Option<String>,
}

pub async fn router(query: Query<SignInQuery>) -> impl IntoResponse {
    dbg!(&query);

    templates::base(
        html! {
          img src="static/under-logo.png" alt="" class="w-1/2 mx-auto -mt-8";

          @if let Some(flash) =  query.flash.as_ref() {
            @if flash == "incorrect" {
              p { "Incorrect email and/or password" }
            } @else {
                p { "unknown flash" }
            }
          }

          form action="/sign-in" method="post" {
            (form_input("email", "Email", "email"));
            (form_input("password", "Password", "password"));

            (submit_button("Sign In"));
          }
        },
        false,
    )
}
