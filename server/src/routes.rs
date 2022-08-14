use axum::response::IntoResponse;
use maud::html;

use crate::templates;

pub async fn root() -> impl IntoResponse {
    templates::base(html! {
      h1 { "Hello, World!" }
    })
}
