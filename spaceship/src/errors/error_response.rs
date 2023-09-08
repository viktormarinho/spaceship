pub use spaceship_derive::ErrorResponse;

/// Struct used by the `spaceship::errors::ErrorResponse` derive macro
/// 
/// Ideally should not be called explicitly in outside code, only by macros
#[derive(Debug, serde::Serialize)]
pub struct ErrorResponseBody {
    pub error: String
}

/// Derive macro that automatically implements `axum::IntoResponse`
/// for a enum containing possible handler errors.
/// 
/// Example:
/// 
/// ```
/// #[derive(ErrorResponse)]
/// enum RegisterError {
///     #[err(status = 400, message = "This name is already taken")]
///     NameAlreadyTaken,
///     #[err(status = 400, message = "This username is too short. Use at least 8 characters.")]
///     UsernameTooShort,
/// }
/// 
/// async fn handler() -> Result<(), RegisterError> {
///     if name_is_taken() {
///         return Err(RegisterError::NameAlreadyTaken); // Now you can return these enum variants in any handler!
///     }
/// 
///     ... 
/// }
/// ```
/// 
/// All attribute fields:
/// - status - Determines the response status (Default if not specified: 500);
/// - message - This message is going to be shown in the error response body (Default if not specified: "Internal server error");
pub trait ErrorResponse {}