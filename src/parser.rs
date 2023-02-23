use crate::messages::{Html, NextUrl};
use async_channel::{Receiver, Sender};
use lib_wc::sync::ShutdownListener;

struct Processor {
    /// The channel to receive HTML from.
    receiver: Receiver<Html>,
    /// The channel to send URLs to crawl to.
    sender: Sender<NextUrl>,
    /// The shutdown listener.
    shutdown: ShutdownListener,
}

impl Processor {
    fn new(receiver: Receiver<Html>, sender: Sender<NextUrl>, shutdown: ShutdownListener) -> Self {
        Self {
            receiver,
            sender,
            shutdown,
        }
    }
}
