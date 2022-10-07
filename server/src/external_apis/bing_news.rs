use color_eyre::{eyre::Context, Result};
use serde::Deserialize;

/// The configuration for the Bing News API
pub struct Config {
    api_key: String,
    base_url: String,
}

impl Config {
    /// Create a new set of credentials from the environment
    ///
    /// If the required environment variables are not set, this returns an error
    ///
    /// Note: This does NOT validate the API key provided, it only checks that the environment variable is set
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("BING_NEWS_API_KEY").wrap_err("BING_NEWS_API_KEY not set")?;
        let base_url =
            std::env::var("BING_NEWS_BASE_URL").wrap_err("BING_NEWS_BASE_URL not set")?;

        Ok(Self { api_key, base_url })
    }
}

fn generate_client(creds: &Config) -> Result<reqwest::Client, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    let mut header_value = reqwest::header::HeaderValue::from_str(&creds.api_key).unwrap();
    header_value.set_sensitive(true);
    headers.insert("Ocp-Apim-Subscription-Key", header_value);

    reqwest::Client::builder().default_headers(headers).build()
}

#[derive(Deserialize, Debug)]
pub(crate) struct NewsProvider {
    #[serde(rename = "_type")]
    provider_type: String,
    name: String,
    image: Option<NewsImage>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct NewsImageThumbnail {
    #[serde(rename = "contentUrl")]
    content_url: String,
    width: Option<usize>,
    height: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub(crate) struct NewsImage {
    thumbnail: NewsImageThumbnail,
}

#[derive(Deserialize, Debug)]
pub(crate) struct NewsResult {
    name: String,
    url: String,
    description: String,
    #[serde(rename = "datePublished")]
    date_published: chrono::DateTime<chrono::Utc>,
    category: Option<String>,
    image: Option<NewsImage>,
}
#[derive(Deserialize, Debug)]
pub(crate) struct NewsResponse {
    value: Vec<NewsResult>,
}

/// Get the top 10 news articles from Bing News
pub(crate) async fn get_news(config: &Config, query: &str) -> Result<NewsResponse> {
    const NEWS_SEARCH_PATH: &str = "/v7.0/news/search";

    let base_url = &config.base_url;
    let url = format!("{base_url}{NEWS_SEARCH_PATH}");

    let client = generate_client(config).unwrap();
    let res = client
        .get(url)
        .query(&[("q", query), ("count", "10"), ("safeSearch", "strict")])
        .send()
        .await
        .unwrap();

    res.json()
        .await
        .wrap_err("We couldn't talk to the API or were unable to parse the JSON")
}
