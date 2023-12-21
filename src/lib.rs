#![no_std]

pub mod command;
mod constants;

use atat::{asynch::AtatClient, Error};
use command::{wlan::Mode as WlanMode, EmptyResponse};
pub use constants::*;

pub struct Calypso<C: AtatClient> {
    client: C,
}

impl<C: AtatClient> Calypso<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Start the network processor unit (NWP).
    pub async fn start(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::Start {}).await
    }

    /// Stop the network processor unit (NWP).
    pub async fn stop(&mut self) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::Stop { timeout: 0 })
            .await
    }

    /// Test the Calypso is responsive.
    pub async fn test(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::Test {}).await
    }

    /// Reboot the Calypso.
    pub async fn reboot(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::Reboot {}).await
    }

    /// Perform a factory reset.
    ///
    /// Warning: Resetting of powering off the module during this operation can
    /// result in permanent damage to the module.
    pub async fn factory_reset(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::FactoryReset {}).await
    }

    /// Sleep for a given number of seconds.
    ///
    /// Setting the duration to 0ms will cause the Calypso to sleep forever.
    pub async fn sleep(
        &mut self,
        seconds: u32,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::Sleep {
                timeout_secs: seconds,
            })
            .await
    }

    /// Sleep until reset or interrupt.
    pub async fn sleep_forever(&mut self) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::Sleep { timeout_secs: 0 })
            .await
    }

    /// Enter into power saving mode.
    pub async fn powersave(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::PowerSave {}).await
    }

    /// Set WIFI operating mode.
    pub async fn wifi_set_mode(
        &mut self,
        mode: WlanMode,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::SetMode { mode: mode.into() })
            .await
    }
}
