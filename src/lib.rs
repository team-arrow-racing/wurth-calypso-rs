#![no_std]

pub mod client;
pub mod command;
pub mod constants;

pub use constants::*;

pub struct Error<'a> {
    pub description: &'a str,
    pub code: i16,
}

pub type RequestResult<'a, T> = Result<T, Error<'a>>;
pub type ConfirmationResult<'a> = Result<(), Error<'a>>;
