pub mod json_error;
pub mod redirect_error;

macro_rules! error_helper {
    ($name:ident, $status:ident) => {
        paste::paste! {
            #[doc = "Helper function that wraps any error and generates a `" $status "` response."]
            #[allow(non_snake_case)]
            pub fn $name<T>(err: T) -> actix_web::Error
            where
            T: std::fmt::Debug + std::fmt::Display + serde::Serialize + 'static,
            {
                json_error::JsonError::new(err, actix_web::http::StatusCode::$status).into()
            }
        }
    };
}

error_helper!(ErrorBadRequest, BAD_REQUEST);
error_helper!(ErrorUnauthorized, UNAUTHORIZED);
error_helper!(ErrorPaymentRequired, PAYMENT_REQUIRED);
error_helper!(ErrorForbidden, FORBIDDEN);
error_helper!(ErrorNotFound, NOT_FOUND);
error_helper!(ErrorMethodNotAllowed, METHOD_NOT_ALLOWED);
error_helper!(ErrorNotAcceptable, NOT_ACCEPTABLE);
error_helper!(
    ErrorProxyAuthenticationRequired,
    PROXY_AUTHENTICATION_REQUIRED
);
error_helper!(ErrorRequestTimeout, REQUEST_TIMEOUT);
error_helper!(ErrorConflict, CONFLICT);
error_helper!(ErrorGone, GONE);
error_helper!(ErrorLengthRequired, LENGTH_REQUIRED);
error_helper!(ErrorPayloadTooLarge, PAYLOAD_TOO_LARGE);
error_helper!(ErrorUriTooLong, URI_TOO_LONG);
error_helper!(ErrorUnsupportedMediaType, UNSUPPORTED_MEDIA_TYPE);
error_helper!(ErrorRangeNotSatisfiable, RANGE_NOT_SATISFIABLE);
error_helper!(ErrorImATeapot, IM_A_TEAPOT);
error_helper!(ErrorMisdirectedRequest, MISDIRECTED_REQUEST);
error_helper!(ErrorUnprocessableEntity, UNPROCESSABLE_ENTITY);
error_helper!(ErrorLocked, LOCKED);
error_helper!(ErrorFailedDependency, FAILED_DEPENDENCY);
error_helper!(ErrorUpgradeRequired, UPGRADE_REQUIRED);
error_helper!(ErrorPreconditionFailed, PRECONDITION_FAILED);
error_helper!(ErrorPreconditionRequired, PRECONDITION_REQUIRED);
error_helper!(ErrorTooManyRequests, TOO_MANY_REQUESTS);
error_helper!(
    ErrorRequestHeaderFieldsTooLarge,
    REQUEST_HEADER_FIELDS_TOO_LARGE
);
error_helper!(
    ErrorUnavailableForLegalReasons,
    UNAVAILABLE_FOR_LEGAL_REASONS
);
error_helper!(ErrorExpectationFailed, EXPECTATION_FAILED);
error_helper!(ErrorInternalServerError, INTERNAL_SERVER_ERROR);
error_helper!(ErrorNotImplemented, NOT_IMPLEMENTED);
error_helper!(ErrorBadGateway, BAD_GATEWAY);
error_helper!(ErrorServiceUnavailable, SERVICE_UNAVAILABLE);
error_helper!(ErrorGatewayTimeout, GATEWAY_TIMEOUT);
error_helper!(ErrorHttpVersionNotSupported, HTTP_VERSION_NOT_SUPPORTED);
error_helper!(ErrorVariantAlsoNegotiates, VARIANT_ALSO_NEGOTIATES);
error_helper!(ErrorInsufficientStorage, INSUFFICIENT_STORAGE);
error_helper!(ErrorLoopDetected, LOOP_DETECTED);
error_helper!(ErrorNotExtended, NOT_EXTENDED);
error_helper!(
    ErrorNetworkAuthenticationRequired,
    NETWORK_AUTHENTICATION_REQUIRED
);
