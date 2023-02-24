# Crawler

A web crawler written in Rust

## Usage

```bash
$ cargo run --release -- --help

A web crawler.

This program crawls the web starting from a given URL.

It uses a pool of spiders to send HTTP requests and a pool of parsers to interpret the HTML and find the next URLs to crawl. The level of concurrency of each pool is configurable.

Beware that a high QPS may get your IP blocked from certain sites.

Usage: crawler [OPTIONS]

Options:
  -t, --target <TARGET>
          The target URL to start crawling from
          
          [default: https://www.wcygan.io]

  -s, --connections <CONNECTIONS>
          The number of connections to use. Connections are background tasks that send network requests to retrieve HTML
          
          [default: 64]

  -p, --processors <PROCESSORS>
          The number of processors to use. Processors are background tasks that interpret HTML and find the next URLs to crawl
          
          [default: 16]

  -i, --interval <INTERVAL>
          The millisecond time interval between requests to a  particular domain. A low interval results in a high QPS which may get your IP blocked from certain sites
          
          [default: 5000]

  -o, --output <OUTPUT>
          The file to write the index to (default: None)

  -h, --help
          Print help (see a summary with '-h')
```

