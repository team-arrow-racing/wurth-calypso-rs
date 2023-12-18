#![no_std]

pub mod command;
mod constants;

use atat::{asynch::AtatClient, Error};
use command::NoResponse;
pub use constants::*;
use fugit::MillisDurationU32;

pub struct Calypso<C: AtatClient> {
    client: C,
}

impl<C: AtatClient> Calypso<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Start the network processor unit (NWP).
    pub async fn start(&mut self) -> Result<NoResponse, Error> {
        self.client.send(&command::device::Start {}).await
    }

    /// Stop the network processor unit (NWP).
    pub async fn stop(&mut self) -> Result<NoResponse, Error> {
        self.client
            .send(&command::device::Stop { timeout: 0 })
            .await
    }

    /// Test the Calypso is responsive.
    pub async fn test(&mut self) -> Result<NoResponse, Error> {
        self.client.send(&command::device::Test {}).await
    }

    /// Reboot the Calypso.
    pub async fn reboot(&mut self) -> Result<NoResponse, Error> {
        self.client.send(&command::device::Reboot {}).await
    }

    /// Perform a factory reset.
    ///
    /// Warning: Resetting of powering off the module during this operation can
    /// result in permanent damage to the module.
    pub async fn factory_reset(&mut self) -> Result<NoResponse, Error> {
        self.client.send(&command::device::FactoryReset {}).await
    }

    /// Sleep for a given number of milliseconds.
    ///
    /// Setting the duration to 0ms will cause the Calypso to sleep forever.
    pub async fn sleep(
        &mut self,
        ms: MillisDurationU32,
    ) -> Result<NoResponse, Error> {
        self.client
            .send(&command::device::Sleep {
                timeout: ms.to_millis(),
            })
            .await
    }

    /// Sleep until reset or interrupt.
    pub async fn sleep_forever(&mut self) -> Result<NoResponse, Error> {
        self.client
            .send(&command::device::Sleep { timeout: 0 })
            .await
    }

    /// Enter into power saving mode.
    pub async fn powersave(&mut self) -> Result<NoResponse, Error> {
        self.client.send(&command::device::PowerSave {}).await
    }
}
