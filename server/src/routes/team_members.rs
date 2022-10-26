pub(crate) mod get {
    use axum::response::IntoResponse;
    use maud::html;

    use crate::{auth::CurrentUser, templates};

    pub async fn router(user: CurrentUser) -> impl IntoResponse {
        templates::base(html! {
            h1 { "New Team Member" }

            form method="POST" {
                input type="text" name="name" placeholder="Name";
                input type="text" name="zipCode" placeholder="Zip Code";
                input type="text" name="title" placeholder="Title";
                input type="textarea" name="interests" placeholder="Interests";

                input type="submit" value="Create!";
            }
        })
    }
}
