use url::Url;

/// The raw HTML which was downloaded from a URL.
pub struct Response {
    pub source: Url,
    pub response: reqwest::Response,
}

/// The URL which should be downloaded next.
pub struct Request {
    pub url: Url,
}
