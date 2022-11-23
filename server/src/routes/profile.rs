pub(crate) mod get {
    use axum::response::IntoResponse;
    use maud::html;

    use crate::{
        auth::CurrentUser,
        templates::{
            base,
            components::buttons::signout_button,
            footer::{Footer, FooterItem},
        },
    };

    pub(crate) async fn router(CurrentUser(current_user): CurrentUser) -> impl IntoResponse {
        base(
            html! {
              section class="pt-16" {
                h1 class="text-xl" { "Hello, " (current_user.name) "!" }

                (signout_button())
              }
            },
            Some(Footer::new(FooterItem::Profile)),
        )
    }
}
