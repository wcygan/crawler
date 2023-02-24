use anyhow::Context;
use anyhow::Result;
use clap::Parser;

use scraper::{Html, Selector};
use url::Url;

/// This is a mess and it works great :)
pub fn get_urls(url: Url, html: &str) -> Result<Vec<String>> {
    let fragment = Html::parse_document(html);
    let selector =
        Selector::parse("a").map_err(|e| anyhow::anyhow!("Failed to parse selector: {}", e))?;
    let mut urls = Vec::new();
    for element in fragment.select(&selector) {
        if let Some(path) = element.value().attr("href") {
            if path.starts_with('/') || path.starts_with("./") {
                match get_base_url(&url) {
                    Some(base_url) => {
                        let url = join_urls(&base_url, path).context("Failed to join URLs")?;
                        urls.push(url.to_string());
                    }
                    None => {}
                }
            } else {
                match Url::parse(path) {
                    Ok(_) => {
                        urls.push(path.to_string());
                    }
                    Err(_) => match Url::parse(&url.to_string()) {
                        Ok(url) => {
                            let url = match join_urls(&url.to_string(), path) {
                                Some(url) => url,
                                None => continue,
                            };
                            urls.push(url.to_string());
                        }
                        Err(_) => {}
                    },
                }
            }
        }
    }

    Ok(urls)
}

fn get_base_url(url: &Url) -> Option<String> {
    let base_url = url
        .domain()
        .map(|domain| format!("{}://{}", url.scheme(), domain));

    base_url
}

fn join_urls(base_url_string: &str, relative_url: &str) -> Option<String> {
    let base_url: Url = match Url::parse(base_url_string) {
        Ok(url) => url,
        Err(_) => return None,
    };

    base_url
        .join(relative_url)
        .ok()
        .map(|url| url.as_str().to_string())
}

pub fn normalize_url(url_str: &str) -> Option<String> {
    let url_with_scheme = if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
        format!("https://{}", url_str)
    } else {
        url_str.to_string()
    };

    // Add a `/` to the end of the URL if it doesn't have one
    let url_with_slash = if !url_with_scheme.ends_with('/') {
        format!("{}/", url_with_scheme)
    } else {
        url_with_scheme
    };

    Some(url_with_slash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_urls() -> Result<()> {
        let url = "https://www.rust-lang.org";
        let html = r#"
            <html>
                <body>
                    <a href="/foo">Foo</a>
                    <a href="/bar">Bar</a>
                    <a href="https://www.rust-lang.org">Rust</a>
                </body>
            </html>
        "#;

        let urls = get_urls(Url::parse(url)?, html).unwrap();
        assert_eq!(urls.len(), 3);
        assert_eq!(urls[0], "https://www.rust-lang.org/foo");
        assert_eq!(urls[1], "https://www.rust-lang.org/bar");
        assert_eq!(urls[2], "https://www.rust-lang.org");
        Ok(())
    }

    #[test]
    fn test_get_urls2() -> Result<()> {
        let url = "https://www.rust-lang.org";
        let html = r#"
            <html>
                <body>
                    <a href="./foo">Foo</a>
                    <a href="/bar">Bar</a>
                    <a href="https://www.rust-lang.org">Rust</a>
                    <a href="https://www.rust-lang.org/foo">Rust</a>
                </body>
            </html>
        "#;

        let urls = get_urls(Url::parse(url)?, html).unwrap();
        assert_eq!(urls.len(), 4);
        assert_eq!(urls[0], "https://www.rust-lang.org/foo");
        assert_eq!(urls[1], "https://www.rust-lang.org/bar");
        assert_eq!(urls[2], "https://www.rust-lang.org");
        assert_eq!(urls[3], "https://www.rust-lang.org/foo");
        Ok(())
    }

    #[test]
    fn test_combine_relative_url() {
        let a = "https://en.wikipedia.org/wiki/Vienna";
        let b = "/wiki/Category:States_of_Austria";

        let url = join_urls(a, b).unwrap();
        let expected = "https://en.wikipedia.org/wiki/Category:States_of_Austria";
        assert_eq!(url, expected);
    }

    #[test]
    fn test_combine_relative_url2() {
        let a = "https://en.wikipedia.org/wiki/Vienna/";
        let b = "/wiki/Category:States_of_Austria";

        let url = join_urls(a, b).unwrap();
        let expected = "https://en.wikipedia.org/wiki/Category:States_of_Austria";
        assert_eq!(url, expected);
    }

    #[test]
    fn test_combine_relative_url3() {
        let a = "https://www.netlify.com/";
        let b = "./blog/2020/08/17/integrate-next.js-and-contentful/";

        let url = join_urls(a, b).unwrap();
        let expected = "https://www.netlify.com/blog/2020/08/17/integrate-next.js-and-contentful/";
        assert_eq!(url, expected);
    }

    #[test]
    fn test_normalize_url() {
        let url1 = "https://farooq.dev";
        let url2 = "farooq.dev";
        let expected = "https://farooq.dev/";

        let normalized_url1 = normalize_url(url1).unwrap();
        assert_eq!(normalized_url1, expected);
        let normalized_url2 = normalize_url(url2).unwrap();
        assert_eq!(normalized_url2, expected);
    }
}
