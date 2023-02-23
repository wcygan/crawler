use crate::messages::{Html, NextUrl};

use async_channel::{Receiver, Sender};
use lib_wc::sync::{RateLimiter, ShutdownListener};
use reqwest::Client;
use std::sync::Arc;

/// The spider which crawls the web.
struct Spider {
    /// The HTTP client.
    client: Client,
    /// The rate limiter.
    rate_limiter: Arc<RateLimiter>,
    /// The shutdown listener.
    shutdown: ShutdownListener,
    /// The channel to send HTML to.
    sender: Sender<Html>,
    /// The channel to receive URLs to crawl.
    receiver: Receiver<NextUrl>,
}

impl Spider {
    pub fn new(
        client: Client,
        rate_limiter: Arc<RateLimiter>,
        shutdown: ShutdownListener,
        sender: Sender<Html>,
        receiver: Receiver<NextUrl>,
    ) -> Self {
        Self {
            client,
            rate_limiter,
            shutdown,
            sender,
            receiver,
        }
    }
}
