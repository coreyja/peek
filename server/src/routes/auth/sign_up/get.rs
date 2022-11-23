use axum::response::IntoResponse;
use maud::html;

use crate::templates::{
    base,
    components::{buttons::submit_button, inputs::form_input},
};

pub async fn router() -> impl IntoResponse {
    base(
        html! {
          img src="static/under-logo.png" alt="" class="w-1/2 mx-auto -mt-8";

          form action="/sign-up" method="post" {
            (form_input("name", "Name", "text"));
            (form_input("email", "Email", "email"));
            (form_input("password", "Password", "password"));
            (form_input("passwordConfirmation", "Repeat Password", "password"));

            (submit_button("Sign Up"));
          }
        },
        Some(Default::default()),
    )
}
