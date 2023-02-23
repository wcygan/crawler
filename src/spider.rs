use crate::messages::{Request, Response};

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
        loop {
            select! {
                _ = self.shutdown.recv() => {
                    info!("Shutting down spider {}...", self.id);
                    break;
                }
                next_url = self.receiver.recv() => {

                }
            }
        }
    }
}
