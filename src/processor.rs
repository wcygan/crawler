use crate::index::Index;
use crate::messages::{Request, Response};
use crate::urls::get_urls;
use anyhow::{Context, Result};
use async_channel::{Receiver, Sender};
use dashmap::mapref::entry::Entry::{Occupied, Vacant};
use dashmap::DashSet;
use lib_wc::sync::ShutdownListener;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::select;
use tracing::{debug, info};
use url::Url;

pub struct Processor {
    /// The ID of the processor.
    id: usize,
    /// The channel to receive HTML from.
    receiver: Receiver<Response>,
    /// The channel to send URLs to crawl to.
    sender: Sender<Request>,
    /// The shutdown listener.
    shutdown: ShutdownListener,
    /// The index.
    index: Arc<Index>,
}

impl Processor {
    pub fn new(
        shutdown: ShutdownListener,
        sender: Sender<Request>,
        receiver: Receiver<Response>,
        index: Arc<Index>,
    ) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: COUNTER.fetch_add(1, SeqCst),
            receiver,
            sender,
            shutdown,
            index,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let res: Result<Response> = select! {
                _ = self.shutdown.recv() => {
                    info!("Shutting down processor {}...", self.id);
                    return;
                }
                res = self.receiver.recv() => {
                    res.context("Failed to receive HTML")
                }
            };

            let response = match res {
                Ok(response) => response,
                Err(err) => {
                    debug!("Failure: {}", err);
                    continue;
                }
            };

            let text = match response.response.text().await {
                Ok(text) => text,
                Err(err) => {
                    debug!("Failed to get text from response: {}", err);
                    continue;
                }
            };

            let key = response.source.to_string();

            // Check if we've already processed this URL
            match self.index.inner.entry(key.clone()) {
                Occupied(_entry) => continue,
                Vacant(entry) => {
                    entry.insert(DashSet::new());
                }
            }

            let urls = match get_urls(response.source, &text) {
                Ok(urls) => urls,
                Err(err) => {
                    debug!("Failed to get URLs from HTML: {}", err);
                    continue;
                }
            };

            match self.index.inner.entry(key.clone()) {
                Occupied(entry) => {
                    for url in urls {
                        match Url::parse(url.as_str()) {
                            Ok(url) => {
                                if !entry.get().contains(url.as_str()) {
                                    let _ = self.sender.send(Request { url }).await;
                                }
                            }
                            Err(err) => {
                                debug!("Failed to parse URL {}: {}", url, err);
                            }
                        }

                        entry.get().insert(url);
                    }
                }
                Vacant(_entry) => unreachable!(),
            }
        }
    }
}
