use crate::messages::{Request, Response};

use anyhow::{Context, Result};
use async_channel::{Receiver, Sender};
use lib_wc::sync::{MultiRateLimiter, ShutdownListener};
use reqwest::Client;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::select;
use tracing::{debug, info};

/// The spider which crawls the web.
pub struct Spider {
    /// The ID of the spider.
    id: usize,
    /// The HTTP client.
    client: Client,
    /// The rate limiter.
    rate_limiter: Arc<MultiRateLimiter<String>>,
    /// The shutdown listener.
    shutdown: ShutdownListener,
    /// The channel to send HTML to.
    sender: Sender<Response>,
    /// The channel to receive URLs to crawl.
    receiver: Receiver<Request>,
}

impl Spider {
    pub fn new(
        shutdown: ShutdownListener,
        rate_limiter: Arc<MultiRateLimiter<String>>,
        sender: Sender<Response>,
        receiver: Receiver<Request>,
    ) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: COUNTER.fetch_add(1, SeqCst),
            client: Client::new(),
            rate_limiter,
            shutdown,
            sender,
            receiver,
        }
    }

    pub async fn run(&mut self) {
        let Spider {
            id,
            client,
            rate_limiter,
            shutdown,
            sender,
            receiver,
        } = self;

        // TODO: split this up so that the program can terminate immediately (or use shutdown on every await...)
        // select! {
        //    _ = self.shutdown.recv() => { }
        //    _ = do_work => { }
        // }

        loop {
            info!("Spider {} is waiting for URL...", id);

            // Get the next URL to crawl
            let res: Result<Request> = select! {
                _ = shutdown.recv() => {
                    info!("Shutting down spider {}...", id);
                    return;
                }
                next_url = receiver.recv() => {
                    next_url.context("Spider failed to receive URL")
                }
            };

            let url = match res {
                Ok(url) => url,
                Err(e) => {
                    debug!("Spider failed to receive URL: {}", e);
                    continue;
                }
            };

            let domain = match url.url.domain() {
                Some(domain) => domain.to_string(),
                _ => continue,
            };

            let res = select! {
                res = rate_limiter.throttle(domain.to_string(), || Spider::crawl(client, url)) => {
                    // Type hint so that the IDE doesn't complain :)
                    let res: Result<Response> = res;
                    res
                },
                _ = shutdown.recv() => {
                    info!("Shutting down spider {}...", id);
                    return;
                }
            };

            if let Ok(response) = res {
                if let Err(e) = sender.send(response).await {
                    debug!("Spider failed to send response: {}", e);
                }
            }
        }
    }

    async fn crawl(mut client: &Client, req: Request) -> Result<Response> {
        info!("Crawling {}", req.url);
        let res = client.get(req.url.as_str()).send().await?;
        Ok(Response::new(req.url, res))
    }
}
