use super::EmptyResponse;
use atat::atat_derive::{AtatCmd, AtatEnum};
use heapless::String;

#[derive(Debug, Clone, Copy, AtatEnum)]
pub enum Mode {
    /// Station mode
    Sta,
    /// Access point mode
    Ap,
    /// Peer-to-peer mode
    P2p,
}

impl Into<String<3>> for Mode {
    fn into(self) -> String<3> {
        String::from(match self {
            Mode::Sta => "STA",
            Mode::Ap => "AP",
            Mode::P2p => "P2P",
        })
    }
}

#[derive(AtatCmd)]
#[at_cmd(
    "+wlanSetMode",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct SetMode {
    #[at_arg(position = 0)]
    pub mode: String<3>,
}

#[derive(AtatCmd)]
#[at_cmd(
    "+wlanScan",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct Scan {
    #[at_arg(position = 0)]
    pub index: u8,
    #[at_arg(position = 1)]
    pub count: u8,
}

#[derive(Debug, Clone, Copy, AtatEnum)]
pub enum SecurityType {
    Open,
    Wep,
    WepShared,
    WpaWpa2,
    Wpa2Plus,
    Wpa3,
    WpaEnt,
    WpsPbc,
    WpsPin,
}

impl Into<String<10>> for SecurityType {
    fn into(self) -> String<10> {
        String::from(match self {
            SecurityType::Open => "OPEN",
            SecurityType::Wep => "WEP",
            SecurityType::WepShared => "WEP_SHARED",
            SecurityType::WpaWpa2 => "WPA_WPA2",
            SecurityType::Wpa2Plus => "WPA2_PLUS",
            SecurityType::Wpa3 => "WPA3",
            SecurityType::WpaEnt => "WPA_ENT",
            SecurityType::WpsPbc => "WPA_PBC",
            SecurityType::WpsPin => "WPA_PIN",
        })
    }
}

#[derive(Debug, Clone, Copy, AtatEnum)]
pub enum SecurityEapType {
    Tls,
    TtlsTls,
    TtlsMschapv2,
    TtlsPsk,
    Peap0Tls,
    Peap0Mschapv2,
    Peap0Psk,
    Peap1Tls,
    Peap1Psk,
}

impl Into<String<14>> for SecurityEapType {
    fn into(self) -> String<14> {
        String::from(match self {
            SecurityEapType::Tls => "TLS",
            SecurityEapType::TtlsTls => "TTLS_TLS",
            SecurityEapType::TtlsMschapv2 => "TTLS_MSCHAPv2",
            SecurityEapType::TtlsPsk => "TTLS_PSK",
            SecurityEapType::Peap0Tls => "PEAP0_TLS",
            SecurityEapType::Peap0Mschapv2 => "PEAP0_MSCHAPv2",
            SecurityEapType::Peap0Psk => "PEAP0_PSK",
            SecurityEapType::Peap1Tls => "PEAP1_TLS",
            SecurityEapType::Peap1Psk => "PEAP1_PSK",
        })
    }
}

#[derive(AtatCmd)]
#[at_cmd(
    "+wlanConnect",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct Connect {
    /// SSID name
    #[at_arg(position = 0)]
    pub ssid: String<32>,
    /// Optional mac address
    #[at_arg(position = 1)]
    pub bssid: Option<String<17>>,
    /// Security type
    pub security_type: String<10>,
    /// Security key
    pub security_key: Option<String<63>>,
    /// Enterprise user name
    pub security_ext_user: Option<String<63>>,
    /// Enterprise anonymous user name
    pub security_ext_anon_user: Option<String<63>>,
    /// Enterprise EAP protocol
    pub security_ext_eap_method: Option<String<14>>,
}

#[derive(AtatCmd)]
#[at_cmd("+wlanDisconnect", EmptyResponse, timeout_ms = 100)]
pub struct Disconnect {}

#[derive(AtatCmd)]
#[at_cmd(
    "+wlanProfileAdd",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct ProfileAdd {
    /// SSID name
    #[at_arg(position = 0)]
    pub ssid: String<32>,
    /// Optional mac address
    #[at_arg(position = 1)]
    pub bssid: Option<String<17>>,
    /// Security type
    pub security_type: String<10>,
    /// Security key
    pub security_key: Option<String<63>>,
    /// Enterprise user name
    pub security_ext_user: Option<String<63>>,
    /// Enterprise anonymous user name
    pub security_ext_anon_user: Option<String<63>>,
    /// Enterprise EAP protocol
    pub security_ext_eap_method: Option<String<14>>,
    /// Profile priority
    pub priority: u8,
}

#[derive(AtatCmd)]
#[at_cmd(
    "+wlanProfileGet",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct ProfileGet {
    /// Index
    index: u8,
}

#[derive(AtatCmd)]
#[at_cmd(
    "+wlanProfileDel",
    EmptyResponse,
    timeout_ms = 100,
    quote_escape_strings = false
)]
pub struct ProfileDelete {
    /// Index
    index: u8,
}
