pub(crate) mod get {
    use axum::response::IntoResponse;
    use maud::{html, Markup};

    use crate::{
        auth::CurrentUser,
        templates::{
            base,
            components::{buttons::submit_button, inputs::form_input},
            footer::{Footer, FooterItem},
        },
    };

    pub async fn router(_: CurrentUser) -> impl IntoResponse {
        base(
            html! {
                (form())
            },
            Some(Footer::new(FooterItem::Add)),
        )
    }

    fn form() -> Markup {
        html! {
            form method="POST" class="pt-16" {
                (form_input("name", "Name", "text", true))
                (form_input("zipCode", "Zip Code", "text", true))
                (form_input("title", "Title", "text", false))
                (form_input("interests", "Interests", "textarea", false))

                (submit_button("Create!"))
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
