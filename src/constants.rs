/// Maximum recommended payload size.
pub const MAX_PAYLOAD_SIZE: u16 = 1460;

/// Default receive buffer size.
pub const DEFAULT_RECEIVE_BUFFER_SIZE: u16 = 2048;

/// Maximum length of sent command sand responses.
pub const MAX_LINE_SIZE: u16 = 2048;

/// Maximum length of IP address strings.
pub const MAX_IP_ADDRESS_LENGTH: u8 = 44;

/// Maximum host name length (e.g. URLs or IP addresses).
pub const MAX_HOST_NAME_LENGTH: u8 = 128;

/// Maximum length of response text.
pub const MAX_RESPONSE_TEXT_LENGTH: u16 = MAX_LINE_SIZE;

/// Version information.
// WIP: static lifetime is likely wrong, but this is just so it compiles.
pub struct Version {
    chip_id: &'static str,
    mac: &'static str,
    phy: &'static str,
    nwp: &'static str,
    rom: &'static str,
    firmware: &'static str,
}

/// System time.
pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
    day: u8,
    month: u8,
    year: u16,
}

/// Unique device identifier.
pub type UDID = [u8; 16];

/// Parity setting.
#[derive(Copy, Clone)]
pub enum Parity {
    None = 0,
    Even = 1,
    Odd = 2,
}

/// Socket type.
#[derive(Copy, Clone)]
pub enum SocketType {
    UDP,
    TCPServer,
    TCPClient,
}
