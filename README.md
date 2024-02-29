# About

I needed a simple server that captures any post request and logs the output for debugging purposes. This simple rocket project does just that.

## Requirements

Rust
[One line install with RustUp](https://rustup.rs/)

## Startup

```shell
cargo run
```

## How to use

- Point your client to make post requests to this service.
- Send a post request (as of writing this, only post requests with a body are captures);
- Read console for the payload
