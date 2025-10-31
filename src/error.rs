//! Top-level error types
#[derive(Debug, thiserror::Error)]
#[error("an application error has occurred")]
pub struct AppError;

/// A suggetion displayed to the user
pub struct Suggestion(pub &'static str);
