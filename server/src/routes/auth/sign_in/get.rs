use axum::{extract::Query, response::IntoResponse};
use maud::{html, Markup};
use serde::Deserialize;

use crate::templates::{self, components::buttons::submit_button};

#[derive(Deserialize, Debug)]
pub struct SignInQuery {
    flash: Option<String>,
}

fn form_input(name: &str, label: &str, input_type: &str) -> Markup {
    html! {
      label class="block mb-8" {
        div class="pb-2" { (label) }
        input
          type=(input_type)
          name=(name)
          required="required"
          placeholder=(label)
          class="block w-full p-2 border border-[#CADFFF] rounded"
          ;
      }
    }
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
