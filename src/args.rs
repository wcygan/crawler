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
    #[clap(short = 's', long = "spiders", default_value_t = 64, value_parser = clap::value_parser ! (u16).range(1..))]
    pub connections: u16,

    /// The number of processors to use. Processors interpret HTML and find the next URLs to crawl.
    #[clap(short = 'p', long = "parsers", default_value_t = 100, value_parser = clap::value_parser ! (u16).range(1..))]
    pub processors: u16,

    /// The millisecond time interval between requests to a  particular domain.
    /// A low interval results in a high QPS which may get your IP blocked from certain sites.
    #[clap(short = 'r', long = "rate", default_value_t = 5000, value_parser = clap::value_parser ! (u64).range(1..))]
    pub interval: u64,
}
