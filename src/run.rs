use crate::spider::Spider;
use crate::Application;
use anyhow::Result;

pub fn run(_application: &Application) -> Result<()> {
    for _ in 0.._application.args.connections {
        let mut spider = new_spider(&_application);
        tokio::spawn(async move {
            spider.run().await;
        });
    }

    Ok(())
}

fn new_spider(app: &Application) -> Spider {
    Spider::new(
        app.rate_limiter.clone(),
        app.controller.subscribe(),
        app.html_sender.clone(),
        app.next_url_receiver.clone(),
    )
}
