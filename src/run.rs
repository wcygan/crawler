use crate::parser::Parser;
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

    // Start the parsers
    for _ in 0..app.args.parsers {
        let mut parser = new_parser(app);
        tokio::spawn(async move {
            parser.run().await;
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

fn new_parser(app: &Application) -> Parser {
    Parser::new(
        app.controller.subscribe(),
        app.send_request.clone(),
        app.receive_response.clone(),
        app.index.clone(),
    )
}
