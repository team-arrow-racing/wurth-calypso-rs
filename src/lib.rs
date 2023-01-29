#![no_main]
#![no_std]

use core::fmt::Write;
use embedded_hal::serial::Read;

/// Interface to a Calypso Wi-Fi module.
pub struct Calypso<S> {
    serial: S,
}

type Confirmation = Result<(), &'static str>;

type Duration = fugit::SecsDurationU32;

impl<S> Calypso<S>
where
    S: Read<u8> + Write,
{
    /// Creates a new Calypso instance
    pub fn new(serial: S) -> Self {
        Calypso { serial }
    }

    /// Sends the `AT+start` command to start the network processor unit (NWP).
    ///
    /// On boot up the network processor is started by default.
    pub fn start(&mut self) -> Confirmation {
        match self.command("start") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sends the `AT+stop` command to put the NWP into hibernation.
    ///
    /// This will result in a loss of all on-going transmissions and
    /// connections.
    pub fn stop(&mut self) -> Confirmation {
        match self.command("stop") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sends the `AT+test` command to test if the modules is ready to receive
    /// further commands.
    pub fn test(&mut self) -> Confirmation {
        match self.command("test") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sends the `AT+reboot` command performing a software reset of the module.
    ///
    /// The module will internally put the NWP to hibernate before restarting
    /// from the reset vector.
    pub fn reboot(&mut self) -> Confirmation {
        match self.command("reboot") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

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
    pub fn factory_reset(&mut self) -> Confirmation {
        match self.command("factoryreset") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sends the `AT+sleep` command putting the module into hibernation.
    ///
    /// The module wakes up automatically after the time period specified by the
    /// `time` parameter.
    pub fn sleep(&mut self, time: Duration) -> Confirmation {
        assert!(time <= Duration::secs(86400));

        // TODO: let cmd = format!("sleep={}", time); implement sleep time

        match self.command("sleep=0") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn command(
        &mut self,
        command: &str,
    ) -> Result<&'static str, &'static str> {
        self.serial.write_str("AT+").unwrap();
        self.serial.write_str(command).unwrap();
        self.serial.write_str("\r\n").unwrap();

        Ok("")
    }
}