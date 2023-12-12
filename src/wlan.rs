use crate::{ConfirmationResult, RequestResult};

/// WLAN mode.
#[derive(Copy, Clone)]
pub enum WlanMode {
    /// Station mode
    STA,
    /// Access point mode
    AP,
    /// Peer-to-peer mode
    P2P,
}

impl From<WlanMode> for &str {
    fn from(mode: WlanMode) -> Self {
        match mode {
            WlanMode::STA => "STA",
            WlanMode::AP => "AP",
            WlanMode::P2P => "P2P",
        }
    }
}

impl TryFrom<&str> for WlanMode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "STA" => Ok(WlanMode::STA),
            "AP" => Ok(WlanMode::AP),
            "P2P" => Ok(WlanMode::P2P),
            _ => Err("value was not STA, AP, or P2P."),
        }
    }
}

/// Security type.
#[derive(Copy, Clone)]
pub enum SecurityType {
    Open,
    Wep,
    WepShared,
    WpaWpa2,
    Wpa2Plus,
    Wpa3,
    WpaEnt,
    WpsPbs,
    WpsPin,
}

/// WLAN commands.
pub trait Wlan {
    /// Set the WLAN operating mode.
    async fn wlan_set_mode(&mut self, mode: WlanMode) -> ConfirmationResult;

    /// Discover devices on all enabled channels.
    async fn wlan_scan(
        &mut self,
        index: u8,
        count: u8,
    ) -> RequestResult<&[&str]>;

    /// Manually connect to a known access point.
    async fn wlan_connect(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        security_type: SecurityType,
        security_key: Option<&str>,
        security_ext_user: Option<&str>,
        security_ext_anon_user: Option<&str>,
        security_ext_eap_method: Option<&str>,
    ) -> ConfirmationResult;

    /// Disconnect from an existing connection.
    async fn wlan_disconnect(&mut self) -> ConfirmationResult;
}
