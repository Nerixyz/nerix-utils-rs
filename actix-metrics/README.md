# actix-metrics

Exposes metrics about your server through the `metrics` crate.

```toml
[dependencies]
actix-metrics = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "actix-metrics-v0.1.0" }
```

# Example

```rust
use actix_web::{get, App, HttpServer, Responder};
use actic_metrics::Metrics;

#[get("")]
async fn index() -> impl Responder {
    "Hello world"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup metrics ...
    
    HttpServer::new(|| App::new()
        .wrap(Metrics::new())
        .service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```