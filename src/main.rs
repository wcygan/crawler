mod args;
mod index;
mod messages;
mod processor;
mod run;
mod spider;
mod urls;

use crate::args::Args;
use crate::index::Index;
use crate::messages::{Request, Response};
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
    pub send_request: Sender<Request>,
    pub receive_request: Receiver<Request>,
    pub send_response: Sender<Response>,
    pub receive_response: Receiver<Response>,
}

impl Application {
    pub fn new() -> Self {
        let args = Application::initialize();
        let controller = ShutdownController::new();
        let rate_limiter: Arc<MultiRateLimiter<String>> =
            Arc::new(MultiRateLimiter::new(Duration::from_millis(args.interval)));
        let index = Arc::new(Index::new());
        let (send_request, receive_request) = async_channel::bounded::<Request>(1000);
        let (send_response, receive_response) = async_channel::bounded::<Response>(1000);
        Self {
            args,
            controller,
            rate_limiter,
            index,
            send_request,
            receive_request,
            send_response,
            receive_response,
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
