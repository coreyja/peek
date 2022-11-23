use axum::response::IntoResponse;
use maud::html;

use crate::templates::{
    base,
    components::{
        buttons::submit_button,
        inputs::{FormInput, FormInputOptions},
    },
};

pub async fn router() -> impl IntoResponse {
    base(
        html! {
          img src="static/under-logo.png" alt="" class="w-1/2 mx-auto -mt-8";

          form action="/sign-up" method="post" {
            (FormInput {
                name: "name",
                label: "Name",
                options: FormInputOptions {
                    required: true,
                    ..Default::default()
                },
            });
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
            (FormInput {
                name: "passwordConfirmation",
                label: "Repeat Password",
                options: FormInputOptions {
                    input_type: "password",
                    required: true,
                },
            });

            (submit_button("Sign Up"));
          }
        },
        Some(Default::default()),
    )
}
