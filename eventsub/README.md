# eventsub

This crate provides a middleware for actix-web 4 that checks incoming eventsub requests.

_Note_: It doesn't do the callback verification it only checks the signature and timestamp.

```toml
[dependencies]
eventsub = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "eventsub-v0.1.0" }
```

# Example 

```rust
use actix_web::web;

use eventsub::EventsubVerify;

mod eventsub_routes;

fn register_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/eventsub")
            .wrap(Metrics::new("eventsub"))
            .wrap(EventsubVerify::new(MY_EVENTSUB_SECRET))
            .configure(eventsub_routes::init)
    );
}

```