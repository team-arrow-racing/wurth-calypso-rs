use crate::ConfirmationResult;

/// HTTP commands.
pub trait Http {
    /// Start the HTTP application.
    async fn http_start(&mut self) -> ConfirmationResult;

    /// Stop the HTTP application.
    async fn http_stop(&mut self) -> ConfirmationResult;
}
