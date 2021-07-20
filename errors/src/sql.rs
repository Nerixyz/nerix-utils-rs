use crate::json_error::JsonError;
use actix_web::http::StatusCode;
use serde::Serialize;
use sqlx::Error;

/// Use this type as the return of an sqlx call
pub type SqlResult<T> = Result<T, JsonError<SqlReason>>;

/// This error is useful when dealing with sqlx responses and not wanting to leak full sql errors.
#[derive(Debug, Serialize, derive_more::Display)]
pub enum SqlReason {
    #[display(fmt = "SQL: NotFound")]
    NotFound,
    #[display(fmt = "SQL: Conflict")]
    Conflict(String),
    #[display(fmt = "SQL: Internal")]
    Internal,
}

impl From<sqlx::Error> for JsonError<SqlReason> {
    fn from(e: Error) -> Self {
        match e {
            Error::RowNotFound | Error::TypeNotFound { .. } | Error::ColumnNotFound(_) => {
                if cfg!(feature = "sql-log") {
                    log::warn!("NotFound sql-error: {}", e);
                }
                Self::new(SqlReason::NotFound, StatusCode::NOT_FOUND)
            }
            Error::Database(db_err) if db_err.constraint().is_some() => {
                let constraint = db_err.constraint().unwrap().to_string();
                if cfg!(feature = "sql-log") {
                    log::warn!("Conflict sql-error: {}", e);
                }
                Self::new(SqlReason::Conflict(constraint), StatusCode::CONFLICT)
            }
            _ => {
                if cfg!(feature = "sql-log") {
                    log::warn!("Internal sql-error: {}", e);
                }
                Self::new(SqlReason::Internal, StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
