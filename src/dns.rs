use crate::ConfirmationResult;

/// DNS server commands.
pub trait Dns {
    /// Start the DNS application.
    async fn dns_start(&mut self) -> ConfirmationResult;

    /// Stop the DNS application.
    async fn dns_stop(&mut self) -> ConfirmationResult;
}
