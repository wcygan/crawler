use crate::messages::{Html, NextUrl};
use anyhow::Result;
use async_channel::{Receiver, Sender};
use lib_wc::sync::RateLimiter;
use reqwest::Client;
use std::sync::Arc;

/// The spider which crawls the web.
struct Spider {
    /// The HTTP client.
    client: Client,
    /// The rate limiter.
    rate_limiter: Arc<RateLimiter>,
    /// The channel to send HTML to.
    sender: Sender<Html>,
    /// The channel to receive URLs to crawl.
    receiver: Receiver<NextUrl>,
}

impl Spider {
    fn new(
        rate_limiter: Arc<RateLimiter>,
        sender: Sender<Html>,
        receiver: Receiver<NextUrl>,
    ) -> Self {
        Self {
            client: Client::new(),
            rate_limiter,
            sender,
            receiver,
        }
    }

    async fn run(&self) -> Result<()> {
        println!("Spider started");
        Ok(())
    }
}

pub fn spawn_spiders(
    amount: usize,
    rate_limiter: Arc<RateLimiter>,
    sender: Sender<Html>,
    receiver: Receiver<NextUrl>,
) {
    for _ in 0..amount {
        let rate_limiter = rate_limiter.clone();
        let sender = sender.clone();
        let receiver = receiver.clone();
        tokio::spawn(async move {
            let spider = Spider::new(rate_limiter, sender, receiver);
            spider.run().await;
        });
    }
}
