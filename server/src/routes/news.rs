pub(crate) mod get {
    use axum::response::IntoResponse;
    use maud::html;

    use crate::templates::base;

    pub(crate) async fn router() -> impl IntoResponse {
        base(
            html! {
                h1 { "News Search" }

                form action="/news" method="post" {
                    input type="text" name="q" placeholder="Search query";
                    input type="submit" value="Search";
                }
            },
            Some(Default::default()),
        )
    }
}

pub(crate) mod post {
    use axum::{response::IntoResponse, Form};
    use maud::html;
    use serde::Deserialize;

    use crate::{
        external_apis::bing_news::{get_news, Config},
        templates::base,
    };

    #[derive(Deserialize, Debug)]
    pub(crate) struct NewsQuery {
        q: String,
    }

    pub(crate) async fn router(Form(query): Form<NewsQuery>) -> impl IntoResponse {
        let creds = Config::from_env().unwrap();

        let results = get_news(&creds, &query.q).await;

        base(
            html! {
                h2 { "News Search" }

                form action="/news" method="post" {
                    input type="text" name="q" placeholder="Jersy City, NJ" value=(query.q);
                    input type="submit" value="Search";
                }

                p { "You searched for " (query.q) }

                h3 { "Results" }

                ul {
                    li { "Result 1" }
                    li { "Result 2" }
                    li { "Result 3" }
                }

                pre { (format!("{:#?}", results)) }
            },
            Some(Default::default()),
        )
    }
}
