use reqwest::Response;
use url::Url;

/// The raw HTML which was downloaded from a URL.
pub struct Html {
    response: Response,
}

/// The URL which should be downloaded next.
pub struct NextUrl {
    url: Url,
}
