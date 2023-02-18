mod args;
mod urls;

use crate::args::Args;

use crate::urls::{get_urls, normalize_url};

use clap::Parser;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match normalize_url(&args.source) {
        Some(url) => {
            run(url.as_str()).await;
        }
        None => panic!("Invalid URL"),
    }
}

async fn run(source: &str) {
    let client = Client::new();

    match client.get(source).send().await {
        Ok(res) => match res.text().await {
            Ok(html) => match get_urls(source, &html) {
                Ok(urls) => {
                    for url in urls {
                        println!("{}", url);
                    }
                }
                Err(e) => panic!("Error: {}", e),
            },
            Err(e) => panic!("Error: {}", e),
        },
        Err(e) => panic!("Error: {}", e),
    };
}
