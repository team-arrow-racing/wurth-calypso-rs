#![no_std]

mod constants;
mod device;
mod dhcp;
mod dns;
mod fs;
mod gpio;
mod http;
mod mdns;
mod mqtt;
mod ping;
mod sntp;
mod socket;
mod wlan;

pub use constants::*;
use core::fmt::Write;
use embedded_hal::serial::Read;
use heapless::{String, Vec};

pub struct Error<'a> {
    pub description: &'a str,
    pub code: i16,
}

pub type RequestResult<'a, T> = Result<T, Error<'a>>;
pub type ConfirmationResult<'a> = Result<(), Error<'a>>;

pub type Duration = fugit::SecsDurationU32;

/// Interface to a Calypso Wi-Fi module.
pub struct Calypso<S> {
    serial: S,
    buffer: Vec<u8, 128>,
}

impl<S> Calypso<S>
where
    S: Read<u8> + Write,
{
    /// Creates a new Calypso instance
    pub fn new(serial: S) -> Self {
        Calypso {
            serial,
            buffer: Vec::new(),
        }
    }

    /// Sends the `AT+start` command to start the network processor unit (NWP).
    ///
    /// On boot up the network processor is started by default.
    pub fn start(&mut self) -> ConfirmationResult {
        self.command_with_ack("start")
    }

    /// Sends the `AT+stop` command to put the NWP into hibernation.
    ///
    /// This will result in a loss of all on-going transmissions and
    /// connections.
    pub fn stop(&mut self) -> Confirmation {
        self.command_with_ack("stop")
    }

    /// Sends the `AT+test` command to test if the modules is ready to receive
    /// further commands.
    pub fn test(&mut self) -> Confirmation {
        self.command_with_ack("test")
    }

    /// Sends the `AT+reboot` command performing a software reset of the module.
    ///
    /// The module will internally put the NWP to hibernate before restarting
    /// from the reset vector.
    pub fn reboot(&mut self) -> Confirmation {
        self.command_with_ack("reboot")
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
        self.command_with_ack("factoryreset")
    }

    /// Sends the `AT+sleep` command putting the module into hibernation.
    ///
    /// The module wakes up automatically after the time period specified by the
    /// `time` parameter or a rising edge on the `WAKE_UP` pin.
    ///
    /// On any wakeup trigger, the module starts from the reset vector.
    pub fn sleep(&mut self, time: Duration) -> Confirmation {
        assert!(time <= Duration::secs(86400));

        let mut cmd: String<16> = String::new();

        let _ = write!(cmd, "sleep={}", time);

        self.command_with_ack(cmd.as_mut_str())
    }

    /// Sends `AT+sleep=0` to put the module into a permanent sleep.
    ///
    /// The module will only wake on the a rising edge on the `WAKE_UP` pin.
    ///
    /// On any wakeup trigger, the module starts from the reset vector.
    pub fn sleep_forever(&mut self, time: Duration) -> Confirmation {
        self.command_with_ack("sleep=0")
    }

    /// Sends the `AT+powersave` command to put the module into power saving
    /// mode.
    ///
    /// See the product manual for specific details on behaviour whilst in the
    /// power saving state.
    pub fn power_save(&mut self) -> Confirmation {
        self.command_with_ack("powersave")
    }

    // General configuration

    /// Retrives the module component version information.
    pub fn get_version(&mut self) -> Result<Version, &'static str> {
        todo!()
    }

    /// Gets the current module system time.
    pub fn get_time(&mut self) -> Result<Time, &'static str> {
        todo!()
    }

    /// Set the module system time.
    pub fn set_time(&mut self, time: Time) -> Confirmation {
        todo!()
    }

    /// If `true`, module settings are retained after reset.
    pub fn get_persistance(&mut self) -> Result<bool, &'static str> {
        todo!()
    }

    /// Configure if module settings are retained after reset.
    pub fn set_persistance(&mut self, persistance: bool) -> Confirmation {
        let mut cmd: String<32> = String::new();

        // bool cast to u8 to send '0' or '1' as per the spec.
        let _ = write!(cmd, "set=general,persistent,{}", persistance as u8);

        self.command_with_ack(cmd.as_mut_str())
    }

    /// Gets the unique device identifier.
    pub fn get_udid(&mut self) -> Result<UDID, &'static str> {
        todo!()
    }

    // UART configuration

    /// Gets the current UART baud rate setting.
    // TODO: use embedded_time library for baud rate type.
    pub fn get_uart_baud_rate(&mut self) -> Result<u32, &'static str> {
        todo!()
    }

    /// Configure the UART baud rate setting.
    // TODO: use embedded_time library for baud rate type.
    pub fn set_uart_baud_rate(&mut self, baud: u32) -> Confirmation {
        assert!(baud >= 115200);
        assert!(baud <= 3000000);

        let mut cmd: String<32> = String::new();

        let _ = write!(cmd, "set=UART,baudrate,{}", baud);

        self.command_with_ack(cmd.as_mut_str())
    }

    /// Gets the current UART parity setting.
    pub fn get_uart_parity(&mut self) -> Result<Parity, &'static str> {
        todo!()
    }

    /// Configure the UART parity setting.
    pub fn set_uart_parity(&mut self, parity: Parity) -> Confirmation {
        let mut cmd: String<32> = String::new();

        let _ = write!(cmd, "set=UART,parity,{}", parity as u8);

        self.command_with_ack(cmd.as_mut_str())
    }

    /// Gets the current UART flow control setting.
    pub fn get_uart_flow_control(&mut self) -> Result<bool, &'static str> {
        todo!()
    }

    /// Configure the UART flow control setting.
    pub fn set_uart_flow_control(&mut self, enabled: bool) -> Confirmation {
        let mut cmd: String<32> = String::new();

        let _ = write!(cmd, "set=UART,flowcontrol,{}", enabled);

        self.command_with_ack(cmd.as_mut_str())
    }

    // Transparent mode configuration
    // TODO: this is not currently part of our required feature set.

    /// GPIO configuration

    /// Get the remote GPIO lock state.
    pub fn get_gpio_remote_lock(&mut self) -> Result<bool, &'static str> {
        todo!()
    }

    /// Configure the remote GPIO lock state.
    pub fn set_gpio_remote_lock(&mut self, locked: bool) -> Confirmation {
        let mut cmd: String<32> = String::new();

        let _ = write!(cmd, "set=GPIO,remote_lock,{}", locked);

        self.command_with_ack(cmd.as_mut_str())
    }

    // WLAN commands

    /// Set the WLAN mode.
    ///
    /// The AP mode is primarily intended for device provisioning and can
    /// support up to 4 stations.
    ///
    /// The AP mode consumes more power and is therefore not suitable for
    /// low-power applications.
    pub fn set_wlan_mode(&mut self, mode: WLANMode) -> Confirmation {
        let _mode: &str = mode.into();

        let mut cmd: String<32> = String::new();

        let m: &str = mode.into();

        let _ = write!(cmd, "wlanSetMode={}", m);

        self.command_with_ack(cmd.as_mut_str())
    }

    /// Sends a given command and awaits a response.
    ///
    /// This method is exposed to allow send arbitrary commands if they are not
    /// yet implemented.
    pub fn command(
        &mut self,
        command: &str,
    ) -> Result<&'static str, &'static str> {
        self.serial.write_str("AT+").unwrap();
        self.serial.write_str(command).unwrap();
        self.serial.write_str("\r\n").unwrap();

        Ok("")
    }

    /// Sends a command returning `Ok` with no value if the responsse is ok and
    /// passing through the error otherwise.
    pub fn command_with_ack(&mut self, command: &str) -> Confirmation {
        match self.command(command) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

}
