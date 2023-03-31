# Crawler

A web crawler written in Rust.

This crawler creates a web graph by exploring all URLs that it finds.

## Design

The crawler is split into two parts:

1. The connection pool
2. The processor pool

The crawler will spin up as many connections & processors as you specify. 

The connection pool will handle all HTTP requests, while the processor pool will handle all HTML parsing.

Requests to the same domain are rate limited to avoid being blocked by the server.

The URL mapping is written to an index which can be written to disk during shutdown.

## Resources

- [Tokio](https://crates.io/crates/tokio) - asynchronous runtime
- [Tokio-utils](https://crates.io/crates/tokio-utils) - rate limiter, graceful shutdown
- [Reqwest](https://crates.io/crates/reqwest/) - HTTP client
- [Dashmap](https://crates.io/crates/dashmap/) - concurrent hash map