# nerix-utils-rs

This repository includes common utils I'm using.

_Note_: All the crates use `actix-web` version `4.0.0-beta.8`.

* [`errors`](errors)
    This crate provides common json errors, a redirect errors and errors for `sqlx`.
    ```toml
    errors = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "errors-v0.1.1" }
    ```

* [`actix-metrics`](actix-metrics) 
    This crate provides a middleware that exposes metrics about the specific route (requests and response-time).
    ```toml
    actix-metrics = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "actix-metrics-v0.1.0" }
    ```

* [`eventsub`](eventsub)
    This crate provides a middleware that verifies Twitch's [EventSub](https://dev.twitch.tv/docs/eventsub) requests. 
    ```toml
    eventsub = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "eventsub-v0.1.0" }
    ```
  