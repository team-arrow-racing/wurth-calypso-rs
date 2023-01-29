#![no_main]
#![no_std]

use core::fmt::Write;
use embedded_hal::serial::Read;

pub struct Calypso<S> {
    serial: S,
}

type Confirmation = Result<(), &'static str>;

type Duration = fugit::SecsDurationU32;

impl<S> Calypso<S>
where
    S: Read<u8> + Write,
{
    pub fn new(serial: S) -> Self {
        Calypso { serial }
    }

    pub fn start(&mut self) -> Confirmation {
        match self.command("start") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn stop(&mut self) -> Confirmation {
        match self.command("stop") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn test(&mut self) -> Confirmation {
        match self.command("test") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn reboot(&mut self) -> Confirmation {
        match self.command("reboot") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn factory_reset(&mut self) -> Confirmation {
        match self.command("factoryreset") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

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
