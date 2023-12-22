#![no_std]

pub mod command;
mod constants;

use atat::{asynch::AtatClient, Error};
use command::wlan::SecurityEapType as WlanEnterpriseEapType;
use command::{
    wlan::{Mode as WlanMode, SecurityType},
    EmptyResponse,
};
pub use constants::*;
use heapless::String;

pub struct Calypso<C: AtatClient> {
    client: C,
}

impl<C: AtatClient> Calypso<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Start the network processor unit (NWP).
    pub async fn start(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::Start {}).await
    }

    /// Stop the network processor unit (NWP).
    pub async fn stop(&mut self) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::Stop { timeout: 0 })
            .await
    }

    /// Test the Calypso is responsive.
    pub async fn test(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::Test {}).await
    }

    /// Reboot the Calypso.
    pub async fn reboot(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::Reboot {}).await
    }

    /// Perform a factory reset.
    ///
    /// Warning: Resetting of powering off the module during this operation can
    /// result in permanent damage to the module.
    pub async fn factory_reset(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::FactoryReset {}).await
    }

    /// Sleep for a given number of seconds.
    ///
    /// Setting the duration to 0ms will cause the Calypso to sleep forever.
    pub async fn sleep(
        &mut self,
        seconds: u32,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::Sleep {
                timeout_secs: seconds,
            })
            .await
    }

    /// Sleep until reset or interrupt.
    pub async fn sleep_forever(&mut self) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::Sleep { timeout_secs: 0 })
            .await
    }

    /// Enter into power saving mode.
    pub async fn powersave(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::device::PowerSave {}).await
    }

    /// Enter into provisioning mode.
    pub async fn provisioning_start(&mut self) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::ProvisioningStart {})
            .await
    }

    /// Exit provisioning mode.
    pub async fn provisioning_stop(&mut self) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::device::ProvisioningStop {})
            .await
    }

    /// Set WIFI operating mode.
    pub async fn wlan_set_mode(
        &mut self,
        mode: WlanMode,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::SetMode { mode: mode.into() })
            .await
    }

    /// Scan for WIFI access points.
    pub async fn wlan_scan(
        &mut self,
        index: u8,
        count: u8,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Scan { index, count })
            .await
    }

    /// Connect to a WIFI access point with "Open" security.
    pub async fn wlan_connect_open(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::Open.into(),
                security_key: None,
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_wep(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        password: &str,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::Wep.into(),
                security_key: Some(password.into()),
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_wep_shared(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        password: &str,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::WepShared.into(),
                security_key: Some(password.into()),
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_wpa_wpa2(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        password: &str,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::WpaWpa2.into(),
                security_key: Some(password.into()),
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_wpa2_plus(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        password: &str,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::Wpa2Plus.into(),
                security_key: Some(password.into()),
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_wpa3(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        password: &str,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::Wpa3.into(),
                security_key: Some(password.into()),
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_enterprise(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
        user: Option<&str>,
        anon_user: Option<&str>,
        password: &str,
        eap_method: WlanEnterpriseEapType,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::WpaEnt.into(),
                security_key: Some(password.into()),
                security_ext_user: user.map(|inner| inner.into()),
                security_ext_anon_user: anon_user.map(|inner| inner.into()),
                security_ext_eap_method: Some(eap_method.into()),
            })
            .await
    }

    pub async fn wlan_connect_wps_pbc(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::WpsPbc.into(),
                security_key: None,
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    pub async fn wlan_connect_wps_pin(
        &mut self,
        ssid: &str,
        bssid: Option<&str>,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .send(&command::wlan::Connect {
                ssid: ssid.into(),
                bssid: bssid.map(|inner| inner.into()),
                security_type: SecurityType::WpsPin.into(),
                security_key: None,
                security_ext_user: None,
                security_ext_anon_user: None,
                security_ext_eap_method: None,
            })
            .await
    }

    /// Manually disconnect from an existing WIFI connection.
    pub async fn wifi_disconnect(&mut self) -> Result<EmptyResponse, Error> {
        self.client.send(&command::wlan::Disconnect {}).await
    }
}
