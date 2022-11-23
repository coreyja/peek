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
            (form_input("name", "Name", "text", true));
            (form_input("email", "Email", "email", true));
            (form_input("password", "Password", "password", true));
            (form_input("passwordConfirmation", "Repeat Password", "password", true));

            (submit_button("Sign Up"));
          }
        },
        Some(Default::default()),
    )
}
