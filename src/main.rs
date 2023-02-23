mod args;
mod index;
mod messages;
mod processor;
mod run;
mod spider;
mod urls;

use crate::args::Args;
use crate::index::Index;
use crate::messages::{Html, NextUrl};
use crate::run::run;
use anyhow::Result;
use async_channel::{Receiver, Sender};
use clap::Parser;
use lib_wc::sync::{MultiRateLimiter, ShutdownController};
use std::sync::Arc;
use std::time::Duration;
use tokio::{select, signal::ctrl_c};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application::new();

    run(&app)?;

    select! {
        _ = ctrl_c() => { return app.shutdown().await; }
    }
}

pub struct Application {
    pub args: Args,
    pub controller: ShutdownController,
    pub rate_limiter: Arc<MultiRateLimiter<String>>,
    pub index: Arc<Index>,
    pub next_url_sender: Sender<NextUrl>,
    pub next_url_receiver: Receiver<NextUrl>,
    pub html_sender: Sender<Html>,
    pub html_receiver: Receiver<Html>,
}

impl Application {
    pub fn new() -> Self {
        let args = Application::initialize();
        let controller = ShutdownController::new();
        let rate_limiter: Arc<MultiRateLimiter<String>> =
            Arc::new(MultiRateLimiter::new(Duration::from_millis(args.interval)));
        let index = Arc::new(Index::new());
        let (next_url_sender, next_url_receiver) = async_channel::bounded::<NextUrl>(1000);
        let (html_sender, html_receiver) = async_channel::bounded::<Html>(1000);
        Self {
            args,
            controller,
            rate_limiter,
            index,
            next_url_sender,
            next_url_receiver,
            html_sender,
            html_receiver,
        }
    }

    fn initialize() -> Args {
        let args = Args::parse();
        tracing_subscriber::fmt::init();
        info!("Starting up...");
        args
    }

    async fn shutdown(self) -> Result<()> {
        self.controller.shutdown().await;
        info!("Shutting down...");
        Ok(())
    }
}
