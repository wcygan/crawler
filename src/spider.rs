use crate::messages::{Request, Response};

use anyhow::Result;
use async_channel::{Receiver, Sender};
use reqwest::Client;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::select;
use tokio_utils::{MultiRateLimiter, ShutdownMonitor};
use tracing::{debug, info};

/// Spiders are responsible for sending HTTP requests to retrieve the HTML of a URL.
/// They pass HTML to the Parsers.
pub struct Spider {
    /// The ID of the spider.
    _id: usize,
    /// The HTTP client.
    client: Client,
    /// The rate limiter.
    rate_limiter: Arc<MultiRateLimiter<String>>,
    /// The shutdown listener.
    shutdown: ShutdownMonitor,
    /// The channel to send HTML to.
    sender: Sender<Response>,
    /// The channel to receive URLs to crawl.
    receiver: Receiver<Request>,
}

impl Spider {
    pub fn new(
        shutdown: ShutdownMonitor,
        rate_limiter: Arc<MultiRateLimiter<String>>,
        sender: Sender<Response>,
        receiver: Receiver<Request>,
    ) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self {
            _id: COUNTER.fetch_add(1, SeqCst),
            client: Client::new(),
            rate_limiter,
            shutdown,
            sender,
            receiver,
        }
    }

    pub async fn run(&mut self) {
        let Spider {
            _id: _,
            client,
            rate_limiter,
            shutdown,
            sender,
            receiver,
        } = self;

        // Run the spider until the shutdown signal is received.
        select! {
            _ = shutdown.recv() => { }
            _ = do_work(client, rate_limiter, sender, receiver) => { }
        }
    }
}

async fn do_work(
    client: &Client,
    rate_limiter: &MultiRateLimiter<String>,
    sender: &Sender<Response>,
    receiver: &Receiver<Request>,
) {
    loop {
        // Wait for a URL to crawl.
        let url = match receiver.recv().await {
            Ok(url) => url,
            Err(e) => {
                debug!("Spider failed to receive URL: {}", e);
                continue;
            }
        };

        // Determine the domain of the URL to throttle the crawler
        let domain = match url.url.domain() {
            Some(domain) => domain.to_string(),
            _ => continue,
        };

        // Throttle the crawler.
        let res = rate_limiter
            .throttle(domain.to_string(), || crawl(client, url))
            .await;

        // Send the response to the parsers.
        if let Ok(response) = res {
            if let Err(e) = sender.send(response).await {
                debug!("Spider failed to send response: {}", e);
            }
        }
    }
}

async fn crawl(client: &Client, req: Request) -> Result<Response> {
    info!("Crawling {}", req.url);
    let res = client.get(req.url.as_str()).send().await?;
    Ok(Response::new(req.url, res))
}
