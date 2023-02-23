use crate::messages::{Request, Response};

use anyhow::{Context, Result};
use async_channel::{Receiver, Sender};
use lib_wc::sync::{MultiRateLimiter, ShutdownListener};
use reqwest::Client;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::select;
use tracing::info;

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

        loop {
            let res: Result<Request> = select! {
                _ = shutdown.recv() => {
                    info!("Shutting down spider {}...", id);
                    break;
                }
                next_url = receiver.recv() => {
                    next_url.context("Spider failed to receive URL")
                }
            };

            if let Ok(url) = res {
                info!("Spider {} received URL: {}", id, url.url);

                let domain = match url.url.domain() {
                    Some(domain) => domain.to_string(),
                    _ => continue,
                };

                let _res = rate_limiter
                    .throttle(domain.to_string(), || Spider::crawl(url))
                    .await;
            }
        }
    }

    async fn crawl(req: Request) -> Result<Response> {
        todo!()
    }
}
