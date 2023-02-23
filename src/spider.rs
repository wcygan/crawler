use crate::messages::{Html, NextUrl};
use std::hash::Hash;

use async_channel::{Receiver, Sender};
use lib_wc::sync::{MultiRateLimiter, ShutdownListener};
use reqwest::Client;
use std::sync::Arc;
use tokio::select;

/// The spider which crawls the web.
pub struct Spider {
    /// The HTTP client.
    client: Client,
    /// The rate limiter.
    rate_limiter: Arc<MultiRateLimiter<String>>,
    /// The shutdown listener.
    shutdown: ShutdownListener,
    /// The channel to send HTML to.
    sender: Sender<Html>,
    /// The channel to receive URLs to crawl.
    receiver: Receiver<NextUrl>,
}

impl Spider {
    pub fn new(
        rate_limiter: Arc<MultiRateLimiter<String>>,
        shutdown: ShutdownListener,
        sender: Sender<Html>,
        receiver: Receiver<NextUrl>,
    ) -> Self {
        Self {
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
                    break;
                }
                next_url = self.receiver.recv() => {

                }
            }
        }
    }
}
