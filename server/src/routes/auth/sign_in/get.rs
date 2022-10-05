use axum::{extract::Query, response::IntoResponse};
use maud::html;
use serde::Deserialize;

use crate::templates;

#[derive(Deserialize, Debug)]
pub struct SignInQuery {
    flash: Option<String>,
}
pub async fn router(query: Query<SignInQuery>) -> impl IntoResponse {
    dbg!(&query);

    templates::base(html! {
      h1 { "Sign In" }

      @if let Some(flash) =  query.flash.as_ref() {
        @if flash == "incorrect" {
          p { "Incorrect email and/or password" }
        } @else {
            p { "unknown flash" }
        }
      }

      form action="/sign-in" method="post" {
        input type="email" name="email" placeholder="Email";
        input type="password" name="password" placeholder="Password";

        input type="submit" value="Sign Up";
      }
    })
}
