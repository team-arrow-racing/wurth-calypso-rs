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
use heapless::Vec;

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
}
