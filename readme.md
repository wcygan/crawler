# Crawler

A web crawler written in Rust.

This crawler is used to find all URLs on a given web page.

## Design

The crawler is split into two parts:

1. The connection pool
2. The processor pool

The crawler will spin up as many connections & processors as you specify. 

The connection pool will handle all HTTP requests, while the processor pool will handle all HTML parsing.

Requests to the same domain are rate limited to avoid being blocked by the server.

The URL mapping is written to an index which can be written to disk during shutdown.

## Resources

- [Tokio](https://tokio.rs/) - asynchronous runtime
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/) - HTTP client
- [Dashmap](https://docs.rs/dashmap/5.4.0/dashmap/) - concurrent hash map
- [lib-wc](https://docs.rs/lib-wc/latest/lib_wc/) - concurrent rate limiting & graceful shutdown