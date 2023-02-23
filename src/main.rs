mod args;
mod messages;
mod processor;
mod run;
mod spider;
mod urls;

use crate::args::Args;
use crate::messages::{Html, NextUrl};
use crate::processor::Processor;
use crate::run::run;
use crate::spider::Spider;
use anyhow::Result;
use async_channel::{Receiver, Sender};
use clap::Parser;
use lib_wc::sync::{MultiRateLimiter, ShutdownController};
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application::new();

    run(&app)?;

    select! {
        _ = tokio::signal::ctrl_c() => {
            app.controller.shutdown().await;
            info!("Shutting down...");
            Ok(())
        }
    }
}

pub struct Application {
    pub args: Args,
    pub controller: ShutdownController,
    pub rate_limiter: Arc<MultiRateLimiter<String>>,
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
        let (next_url_sender, next_url_receiver) = async_channel::bounded::<NextUrl>(1000);
        let (html_sender, html_receiver) = async_channel::bounded::<Html>(1000);
        Self {
            args,
            controller,
            rate_limiter,
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
}
