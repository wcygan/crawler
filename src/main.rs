mod args;

use crate::args::Args;

use anyhow::Context;
use clap::Parser;
use reqwest::Client;
use url::Url;
use anyhow::Result;
use scraper::{Html, Selector};


#[tokio::main]
async fn main() {
    let args = Args::parse();
    run(&args.source).await;
}

async fn run(source: &str) {
    let client = Client::new();

    match client.get(source).send().await {
        Ok(res) => {
            match res.text().await {
                Ok(html) => {
                    match get_urls(source, &html) {
                        Ok(urls) => {
                            for url in urls {
                                println!("{}", url);
                            }
                        }
                        Err(e) => panic!("Error: {}", e),
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        Err(e) => panic!("Error: {}", e),
    };
}

fn get_urls(url: &str, html: &str) -> Result<Vec<String>> {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse("a")
        .map_err(|e| anyhow::anyhow!("Failed to parse selector: {}", e))?;
    let mut urls = Vec::new();

    for element in fragment.select(&selector) {
        if let Some(path) = element.value().attr("href") {
            if path.starts_with('/') {
                match get_base_url(url) {
                    Some(base_url) => {
                        let url = join_urls(&base_url, path)
                            .context("Failed to join URLs")?;
                        urls.push(url.to_string());
                    }
                    None => {
                        urls.push(path.to_string());
                    }
                }
            } else {
                urls.push(path.to_string());
            }
        }
    }

    Ok(urls)
}

fn get_base_url(url_string: &str) -> Option<String> {
    match Url::parse(url_string) {
        Ok(url) => {
            let base_url = url.join("/").unwrap();
            Some(base_url.to_string())
        },
        Err(_) => None
    }
}

fn join_urls(base_url_string: &str, relative_url: &str) -> Option<String> {
    let base_url: Url = match Url::parse(base_url_string) {
        Ok(url) => url,
        Err(_) => return None
    };

    let joined_url = base_url.join(relative_url).unwrap();

    Some(joined_url.to_string())
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_get_urls() {
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

        let urls = get_urls(url, html).unwrap();
        assert_eq!(urls.len(), 3);
        assert_eq!(urls[0], "https://www.rust-lang.org/foo");
        assert_eq!(urls[1], "https://www.rust-lang.org/bar");
        assert_eq!(urls[2], "https://www.rust-lang.org");
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
}