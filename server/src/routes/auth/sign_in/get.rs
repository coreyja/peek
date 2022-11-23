use axum::{extract::Query, response::IntoResponse};
use maud::html;
use serde::Deserialize;

use crate::templates::{
    self,
    components::{
        buttons::submit_button,
        inputs::{FormInput, FormInputOptions},
    },
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
            (FormInput {
                name: "email",
                label: "Email",
                options: FormInputOptions {
                    input_type: "email",
                    required: true,
                },
            });
            (FormInput {
                name: "password",
                label: "Password",
                options: FormInputOptions {
                    input_type: "password",
                    required: true,
                },
            });

            (submit_button("Sign In"));
          }
        },
        None,
    )
}
