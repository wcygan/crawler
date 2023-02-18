use crate::messages::{Html, NextUrl};
use async_channel::{Receiver, Sender};

struct Parser {
    /// The channel to receive HTML from.
    receiver: Receiver<Html>,
    /// The channel to send URLs to crawl to.
    sender: Sender<NextUrl>,
}

impl Parser {
    fn new(receiver: Receiver<Html>, sender: Sender<NextUrl>) -> Self {
        Self { receiver, sender }
    }
}
