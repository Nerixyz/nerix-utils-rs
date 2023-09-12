# nerix-utils-rs

This repository includes common utils I'm using.

_Note_: All the crates use `actix-web` version `4.1` (or higher).

* [`errors`](errors)
    This crate provides common json errors and a redirect error.
    ```toml
    errors = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "errors-v0.4.0" }
    ```

* [`actix-metrics`](actix-metrics) 
    This crate provides a middleware that exposes metrics about the specific route (requests and response-time).
    ```toml
    actix-metrics = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "actix-metrics-v0.4.0" }
    ```
