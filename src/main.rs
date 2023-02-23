mod args;
mod messages;
mod parser;
mod run;
mod spider;
mod urls;

use crate::args::Args;
use crate::messages::{Html, NextUrl};
use crate::run::run;
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

pub struct Application<'a> {
    controller: ShutdownController,
    rate_limiter: Arc<MultiRateLimiter<&'a str>>,
    next_url_sender: Sender<NextUrl>,
    next_url_receiver: Receiver<NextUrl>,
    html_sender: Sender<Html>,
    html_receiver: Receiver<Html>,
}

impl Application<'_> {
    pub fn new() -> Self {
        let args = Application::initialize();
        let controller = ShutdownController::new();
        let rate_limiter: Arc<MultiRateLimiter<&str>> =
            Arc::new(MultiRateLimiter::new(Duration::from_millis(args.interval)));
        let (next_url_sender, next_url_receiver) = async_channel::bounded::<NextUrl>(1000);
        let (html_sender, html_receiver) = async_channel::bounded::<Html>(1000);
        Self {
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
