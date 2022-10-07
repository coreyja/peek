use serde::Deserialize;

/// The credentials for the Bing News API
pub struct Creds {
    api_key: String,
}

impl Creds {
    /// Create a new set of credentials from the environment
    ///
    /// If the required environment variables are not set, this returns None
    ///
    /// Note: This does NOT validate the API key provided, it only checks that the environment variable is set
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("BING_NEWS_API_KEY");

        api_key.ok().map(|api_key| Self { api_key })
    }
}

const BING_NEWS_API_URL: &str = "https://api.bing.microsoft.com/v7.0/news/search";

fn generate_client(creds: &Creds) -> Result<reqwest::Client, reqwest::Error> {
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
    image: NewsImage,
}
#[derive(Deserialize, Debug)]
pub(crate) struct NewsResponse {
    value: Vec<NewsResult>,
}

/// Get the top 10 news articles from Bing News
pub(crate) async fn get_news(creds: &Creds, query: &str) -> NewsResponse {
    let client = generate_client(creds).unwrap();
    let res = client
        .get(BING_NEWS_API_URL)
        .query(&[("q", query), ("count", "10")])
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}
