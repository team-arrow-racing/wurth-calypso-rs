use super::EmptyResponse;
use atat::atat_derive::{AtatCmd, AtatEnum, AtatResp};
use heapless::String;

#[derive(Debug, Clone, Copy, AtatEnum)]
pub enum SocketFamily {
    Inet,
    Inet6,
}

impl Into<String<5>> for SocketFamily {
    fn into(self) -> String<5> {
        String::from(match self {
            SocketFamily::Inet => "INET",
            SocketFamily::Inet6 => "INET6",
        })
    }
}

#[derive(Debug, Clone, Copy, AtatEnum)]
pub enum SocketType {
    Stream,
    Dgram,
}

impl Into<String<6>> for SocketType {
    fn into(self) -> String<6> {
        String::from(match self {
            SocketType::Stream => "STREAM",
            SocketType::Dgram => "DGRAM",
        })
    }
}

#[derive(Debug, Clone, Copy, AtatEnum)]
pub enum SocketProtocol {
    Tcp,
    Udp,
    Sec,
}

impl Into<String<3>> for SocketProtocol {
    fn into(self) -> String<3> {
        String::from(match self {
            SocketProtocol::Tcp => "TCP",
            SocketProtocol::Udp => "UDP",
            SocketProtocol::Sec => "SEC",
        })
    }
}

/// Create a socket
#[derive(AtatCmd)]
#[at_cmd(
    "+socket",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct Socket {
    #[at_arg(position = 0)]
    pub family: String<5>,
    #[at_arg(position = 1)]
    pub type_: String<6>,
    #[at_arg(position = 2)]
    pub protocol: String<3>,
}

/// Close a socket
#[derive(AtatCmd)]
#[at_cmd("+close", EmptyResponse, timeout_ms = 100)]
pub struct Close {
    #[at_arg(position = 0)]
    pub socket_id: u8,
}

/// Bind a socket
#[derive(AtatCmd)]
#[at_cmd("+bind", EmptyResponse, timeout_ms = 100)]
pub struct Bind {
    #[at_arg(position = 0)]
    pub socket_id: u8,
    #[at_arg(position = 1)]
    pub family: String<5>,
    #[at_arg(position = 2)]
    pub local_port: u16,
    #[at_arg(position = 3)]
    pub local_address: String<15>,
}
