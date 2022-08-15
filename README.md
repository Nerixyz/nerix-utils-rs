# nerix-utils-rs

This repository includes common utils I'm using.

_Note_: All the crates use `actix-web` version `4.0.0-beta.8`.

* [`errors`](errors)
    This crate provides common json errors, a redirect errors and errors for `sqlx`.
    ```toml
    errors = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "errors-v0.2.2" }
    ```

* [`actix-metrics`](actix-metrics) 
    This crate provides a middleware that exposes metrics about the specific route (requests and response-time).
    ```toml
    actix-metrics = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "actix-metrics-v0.4.0" }
    ```
