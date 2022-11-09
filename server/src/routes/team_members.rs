pub(crate) mod get {
    use axum::response::IntoResponse;
    use maud::{html, Markup};

    use crate::{
        auth::CurrentUser,
        templates::{self, base, with_footer},
    };

    pub async fn router(_: CurrentUser) -> impl IntoResponse {
        base(with_footer(html! {
            h1 { "New Team Member" }

            (form())
        }))
    }

    fn form() -> Markup {
        html! {
            form method="POST" {
                input type="text" name="name" placeholder="Name";
                input type="text" name="zipCode" placeholder="Zip Code";
                input type="text" name="title" placeholder="Title";
                input type="textarea" name="interests" placeholder="Interests";

                input type="submit" value="Create!";
            }
        }
    }
}
pub(crate) mod post {
    use axum::{
        extract::State,
        response::{IntoResponse, Redirect},
        Form,
    };
    use maud::html;
    use serde::Deserialize;

    use crate::{auth::CurrentUser, Pool};

    #[derive(Deserialize, Debug)]
    pub(crate) struct NewTeamMember {
        name: String,
        #[serde(rename = "zipCode")]
        zip_code: String,
        title: String,
        interests: String,
    }

    pub(crate) async fn router(
        user: CurrentUser,
        State(Pool(pool)): State<Pool>,
        Form(new_team_member): Form<NewTeamMember>,
    ) -> impl IntoResponse {
        sqlx::query!(
            "INSERT INTO TeamMembers (user_id, name, zip_code, title, interests) VALUES (?, ?, ?, ?, ?) RETURNING *",
            user.0.id,
            new_team_member.name,
            new_team_member.zip_code,
            new_team_member.title,
            new_team_member.interests
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        Redirect::to("/")
    }
}
