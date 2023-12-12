use crate::{ConfirmationResult, Duration};

/// Device commands.
pub trait Device {
    /// Sends the `AT+test` command to test if the modules is ready to receive
    /// further commands.
    async fn test(&mut self) -> ConfirmationResult;

    /// Sends the `AT+start` command to start the network processor unit (NWP).
    ///
    /// On boot up the network processor is started by default.
    async fn start(&mut self) -> ConfirmationResult;

    /// Sends the `AT+stop` command to put the NWP into hibernation.
    ///
    /// This will result in a loss of all on-going transmissions and
    /// connections.
    async fn stop(&mut self, timeout: Option<Duration>) -> ConfirmationResult;

    fn restart(&mut self, timeout: Option<Duration>) -> ConfirmationResult;

    /// Sends the `AT+reboot` command performing a software reset of the module.
    ///
    /// The module will internally put the NWP to hibernate before restarting
    /// from the reset vector.
    fn reboot(&mut self) -> ConfirmationResult;

    /// Sends the `AT+factoryreset` command restoring the module to the factory
    /// state.
    ///
    /// - All modified files in the file system will be reverted.
    /// - New files that were added will be deleted.
    /// - The network processor settings including the MAC address will be
    /// restored to factory state.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong> Resetting or powering off the module during
    /// this operation can result in permanent damage to the module.
    /// </p>
    fn factory_reset(&mut self) -> ConfirmationResult;

    /// Sends the `AT+sleep` command putting the module into hibernation.
    ///
    /// The module wakes up automatically after the time period specified by the
    /// `time` parameter or a rising edge on the `WAKE_UP` pin.
    ///
    /// On any wakeup trigger, the module starts from the reset vector.
    fn sleep(&mut self, timeout: Duration) -> ConfirmationResult;

    /// Sends `AT+sleep=0` to put the module into a permanent sleep.
    ///
    /// The module will only wake on the a rising edge on the `WAKE_UP` pin.
    ///
    /// On any wakeup trigger, the module starts from the reset vector.
    fn sleep_forever(&mut self) -> ConfirmationResult;

    /// Sends the `AT+powersave` command to put the module into power saving
    /// mode.
    ///
    /// See the product manual for specific details on behaviour whilst in the
    /// power saving state.
    fn power_save(&mut self) -> ConfirmationResult;
}
