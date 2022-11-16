use axum::response::IntoResponse;
use maud::html;

use crate::templates::base;

pub async fn router() -> impl IntoResponse {
    base(
        html! {
          h1 { "Create Account" }


          form action="/sign-up" method="post" {
            input type="text" name="name" placeholder="Name";
            input type="email" name="email" placeholder="Email";
            input type="password" name="password" placeholder="Password";
            input type="password" name="passwordConfirmation" placeholder="Repeat Password";

            input type="submit" value="Sign Up";
          }
        },
        true,
    )
}
