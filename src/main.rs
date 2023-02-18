mod args;
mod messages;
mod parser;
mod spider;
mod urls;

use crate::args::Args;
use std::sync::Arc;

use crate::messages::{Html, NextUrl};
use crate::spider::spawn_spiders;
use crate::urls::{get_urls, normalize_url};
use anyhow::Result;
use clap::Parser;
use lib_wc::sync::RateLimiter;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let rate_limiter = Arc::new(RateLimiter::new(args.max_qps)?);
    let (html_rx, html_tx) = async_channel::bounded::<Html>(100);
    let (url_rx, url_tx) = async_channel::bounded::<NextUrl>(100);

    Ok(())
}
