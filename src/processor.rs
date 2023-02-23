use crate::index::Index;
use crate::messages::{Request, Response};
use anyhow::{Context, Result};
use async_channel::{Receiver, Sender};
use lib_wc::sync::ShutdownListener;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::select;
use tracing::info;

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
        }
    }
}
