use anyhow::anyhow;
use axum::{response::IntoResponse, http::StatusCode};

// this module is nice
pub mod error_response;
pub use error_response::{ErrorResponse, ErrorResponseBody};

// everything below here is fully experimental

/// Constructs a `spaceship::errors::Failure` wrapped in an Error,
/// internally using the `anyhow!` macro.
/// 
/// ```
/// async fn handler() -> HandlerResult<()> {
///    let user = get_user();
///
///    if user.is_err() {
///        return failure!("User not authenticated").status(StatusCode::UNAUTHORIZED);
///    }
///
///   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! failure {
    ($msg:literal $(,)?) => {
        Err(Failure::new(anyhow!($msg)))
    };
    ($err:expr $(,)?) => {
        Err(Failure::new(anyhow!($err)))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(Failure::new(anyhow!($fmt, $($arg)*)))
    };
}

/// Expands to an early return with a Failure from a Handler function
/// 
/// Usage:
/// 
/// ```
/// pub fn handler() -> HandlerResult<()> {
///     internal_error!("Something went very wrong!") 
/// }
/// ```
#[macro_export]
macro_rules! internal_error {
    ($msg:literal $(,)?) => {
        return failure!($msg);
    };
    ($err:expr $(,)?) => {
        return failure!($err);
    };
    ($fmt:expr, $($arg:tt)*) => {
        return failure!($fmt, $($arg)*);
    };
}

/// Represents a handler function failure, wraps `anyhow::Error` with a type
/// that already implements IntoResponse
pub struct Failure(anyhow::Error, StatusCode);

impl Failure {
    pub fn new(err: anyhow::Error) -> Self {
        Failure(err, StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub type HandlerResult<T> = std::result::Result<T, Failure>;

impl IntoResponse for Failure {
    fn into_response(self) -> axum::response::Response {
        (
            self.1,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Failure
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub trait LazyChangeStatus<T> {
    fn status(self, status: StatusCode) -> HandlerResult<T>;
}

impl<T> LazyChangeStatus<T> for HandlerResult<T> {
    fn status(self, status: StatusCode) -> HandlerResult<T> {
        match self {
            Ok(_) => panic!("The status method should be used to change a inner Failure status"),
            Err(failure) => return Err(Failure(failure.0, status))
        }
    }
}

async fn handler() -> HandlerResult<()> {
    let user = get_user();

    if user.is_err() {
        return failure!("User not authenticated").status(StatusCode::UNAUTHORIZED);
    }

    Ok(())
}

fn get_user() -> Result<(), ()> {
    Ok(())
}