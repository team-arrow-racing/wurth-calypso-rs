//! AT Commands for the Wurth Calypso Module
//!
//! See section 8 in the user manual.

pub mod device;
pub mod wlan;

use atat::atat_derive::{AtatResp, AtatUrc};

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse;

/// Unsolicited result codes
#[derive(Debug, PartialEq, Clone, AtatUrc)]
pub enum Urc {
    /// Startup message
    #[at_urc("+eventstartup")]
    StartUp,
    /// General events
    #[at_urc("+eventgeneral")]
    General,
    /// WLAN events
    #[at_urc("+eventwlan")]
    Wlan,
    /// Socket events
    #[at_urc("+eventsocket")]
    Socket,
    /// NetApp events
    #[at_urc("+eventnetapp")]
    NetApp,
    /// MQTT events
    #[at_urc("+eventmqtt")]
    Mqtt,
    /// Fatal error events
    #[at_urc("+eventfatalerror")]
    Fatal,
    /// Custom events
    #[at_urc("+eventcustom")]
    Custom,
}
