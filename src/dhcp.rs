use crate::ConfirmationResult;

/// DHCP commands.
pub trait Dhcp {
    /// Start the DHCP application.
    async fn dhcp_start(&mut self) -> ConfirmationResult;

    /// Stop the DHCP application.
    async fn dhpc_stop(&mut self) -> ConfirmationResult;
}
