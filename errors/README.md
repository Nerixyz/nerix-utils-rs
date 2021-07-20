# errors

```toml
[dependencies]
errors = { git = "https://github.com/Nerixyz/nerix-utils-rs", tag = "errors-v0.1.0" }
```

There are three main features:

* Simple function to create a quick json error
    ```rust
  use actix_web::{Result HttpResponse};
  
  fn my_fn(some_input: String) -> Result<HttpResponse> {
      if some_input.len() < 1337 {
          Err(errors::ErrorBadRequest("input too short"))
      } else {
          Ok(HttpResponse::Ok().json(serde_json::json!({
              "status": "ok"
          })
      }
  }  
  
  ```
* A redirect error that may include an optional reason
    ```rust
  use actix_web::{Result HttpResponse};  
  use errors::redirect_error::RedirectError;
  
  fn my_fn(some_input: String) -> Result<HttpResponse> {
      if some_input.len() > 1337 {
          Err(RedirectError::new("/failed-auth", Some("Username too long")))
      } else {
          Ok(HttpResponse::Ok().json(serde_json::json!({
              "status": "ok"
          })
      }
  }  
  ```
  
* A sql error that will result in a json response