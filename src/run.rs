use crate::processor::Processor;
use crate::spider::Spider;
use crate::Application;
use anyhow::Result;

pub fn run(app: &Application) -> Result<()> {
    // Start the spiders
    for _ in 0..app.args.connections {
        let mut spider = new_spider(app);
        tokio::spawn(async move {
            spider.run().await;
        });
    }

    // Start the processors
    for _ in 0..app.args.processors {
        let mut processor = new_processor(app);
        tokio::spawn(async move {
            processor.run().await;
        });
    }

    Ok(())
}

fn new_spider(app: &Application) -> Spider {
    Spider::new(
        app.controller.subscribe(),
        app.rate_limiter.clone(),
        app.send_response.clone(),
        app.receive_request.clone(),
    )
}

fn new_processor(app: &Application) -> Processor {
    Processor::new(
        app.controller.subscribe(),
        app.send_request.clone(),
        app.receive_response.clone(),
        app.index.clone(),
    )
}
