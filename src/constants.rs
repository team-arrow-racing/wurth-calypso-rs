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
