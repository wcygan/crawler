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
use anyhow::{Context, Result};
use async_channel::{Receiver, Sender};
use clap::Parser;
use dashmap::DashSet;
use lib_wc::sync::{MultiRateLimiter, ShutdownController};
use std::sync::Arc;
use std::time::Duration;
use tokio::{select, signal::ctrl_c};
use tracing::info;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application::new().await?;

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
    pub async fn new() -> Result<Self> {
        let mut args = Application::initialize();
        let controller = ShutdownController::new();
        let rate_limiter: Arc<MultiRateLimiter<String>> =
            Arc::new(MultiRateLimiter::new(Duration::from_millis(args.interval)));
        let index = Arc::new(Index::new(args.output.take()));
        let (send_request, receive_request) = async_channel::unbounded();
        let (send_response, receive_response) = async_channel::unbounded();

        // Seed the initial request
        send_request
            .send(Request::new(Url::parse(&args.target)?))
            .await
            .context("Failed to send initial request")?;

        // Set the initial URL as visited
        index
            .inner
            .entry(args.target.clone())
            .or_insert_with(DashSet::new)
            .insert(args.target.clone());

        Ok(Self {
            args,
            controller,
            rate_limiter,
            index,
            send_request,
            receive_request,
            send_response,
            receive_response,
        })
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
