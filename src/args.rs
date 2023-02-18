use clap::Parser;

/// A web crawler.
///
/// This program crawls the web starting from a given URL.
///
/// It uses a pool of spiders to send HTTP requests and a pool of parsers to interpret the HTML
/// and find the next URLs to crawl. The level of concurrency of each pool is configurable.
///
/// Beware that a high QPS may get your IP blocked from certain sites.
#[derive(Parser)]
pub struct Args {
    /// The target URL to start crawling from.
    #[clap(short = 't', long = "target", default_value = "https://www.wcygan.io")]
    pub target: String,

    /// The number of connections to use. These send network requests to retrieve HTML.
    #[clap(short = 's', long = "spiders", default_value = "4")]
    pub connections: usize,

    /// The number of parsers to use. These interpret HTML and find the next URLs to crawl.
    #[clap(short = 'p', long = "parsers", default_value = "8")]
    pub parsers: usize,

    /// The maximum number of requests per second. A high QPS may get your IP blocked from certain sites.
    #[clap(short = 'q', long = "max-qps", default_value = "0.5")]
    pub max_qps: f64,
}
