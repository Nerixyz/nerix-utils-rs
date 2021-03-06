use actix_web::{
    dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform},
    error::PayloadError,
    http::header::HeaderValue,
    web::{Bytes, BytesMut},
    Error, HttpMessage,
};
use chrono::{DateTime, Duration, Utc};
use futures::{
    future::{ok, Ready},
    StreamExt,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    borrow::Cow,
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

type Secret = Cow<'static, str>;

/// This middleware verifies an eventsub request
pub struct EventsubVerify {
    secret: Rc<Secret>,
}

impl EventsubVerify {
    pub fn new<T: Into<Secret>>(secret: T) -> Self {
        Self {
            secret: Rc::new(secret.into()),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for EventsubVerify
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = EventsubMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(EventsubMiddleware {
            service: Rc::new(RefCell::new(service)),
            secret: self.secret.clone(),
        })
    }
}

pub struct EventsubMiddleware<S> {
    service: Rc<RefCell<S>>,
    secret: Rc<Secret>,
}

type HmacSha256 = Hmac<Sha256>;

impl<S, B> Service<ServiceRequest> for EventsubMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let secret = self.secret.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();

            let id_header: Option<&HeaderValue> = req.headers().get("Twitch-Eventsub-Message-Id");
            let timestamp_header: Option<&HeaderValue> =
                req.headers().get("Twitch-Eventsub-Message-Timestamp");
            let signature_header: Option<&HeaderValue> =
                req.headers().get("Twitch-Eventsub-Message-Signature");

            let (id_header, timestamp_header, signature_header) =
                match (id_header, timestamp_header, signature_header) {
                    (Some(id), Some(timestamp), Some(signature)) => (id, timestamp, signature),
                    _ => return Err(errors::ErrorUnauthorized("Unauthorized")),
                };
            let (timestamp, signature) = match (
                timestamp_header
                    .to_str()
                    .map(|ts| ts.parse::<DateTime<Utc>>()),
                signature_header.to_str(),
            ) {
                (Ok(Ok(ts)), Ok(sig)) => (ts, sig.to_string()),
                _ => return Err(errors::ErrorUnauthorized("Bad header")),
            };
            if signature.len() <= 7 {
                return Err(errors::ErrorUnauthorized("Bad signature"));
            }
            if Utc::now() - timestamp > Duration::minutes(10) {
                return Err(errors::ErrorUnauthorized("Ancient message LuL"));
            }

            let mut mac =
                HmacSha256::new_from_slice(secret.as_bytes()).expect("should take any key");
            mac.update(id_header.as_bytes());
            mac.update(timestamp_header.as_bytes());

            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                // 10Mb
                if body.len() >= 10_000_000 {
                    return Err(errors::ErrorImATeapot("yeah no, that's too much"));
                }
                body.extend_from_slice(&chunk?);
            }

            mac.update(body.as_ref());
            let bytes = mac.finalize().into_bytes();

            if hex::encode(bytes) != signature[7..] {
                return Err(errors::ErrorUnauthorized("Bad signature"));
            }

            let stream = async_stream::stream! {
                while !body.is_empty() {
                    let out = if body.len() > 8192 {
                        body.split_to(8192)
                    } else {
                        body.split_to(body.len())
                    };
                    yield Ok::<Bytes, PayloadError>(out.freeze());
                }
            };
            req.set_payload(Payload::Stream {
                payload: Box::pin(stream),
            });

            svc.call(req).await
        })
    }
}
