use super::NoResponse;
use atat::atat_derive::AtatCmd;
use heapless::String;

#[derive(AtatCmd)]
#[at_cmd("+start", NoResponse, timeout_ms = 100)]
pub struct Start {}

#[derive(AtatCmd)]
#[at_cmd("+stop", NoResponse, timeout_ms = 100)]
pub struct Stop {
    #[at_arg(position = 0)]
    pub timeout: u16,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+test", NoResponse, timeout_ms = 100)]
pub struct Test {}

#[derive(AtatCmd)]
#[at_cmd("+reboot", NoResponse, timeout_ms = 100)]
pub struct Reboot {}

#[derive(AtatCmd)]
#[at_cmd("+factoryreset", NoResponse)]
pub struct FactoryReset {}

#[derive(AtatCmd)]
#[at_cmd("+sleep", NoResponse, timeout_ms = 100)]
pub struct Sleep {
    #[at_arg(position = 0)]
    pub timeout: u32,
}

#[derive(AtatCmd)]
#[at_cmd("+powersave", NoResponse, timeout_ms = 100)]
pub struct PowerSave {}

#[derive(AtatCmd)]
#[at_cmd("+get", NoResponse, timeout_ms = 100)]
pub struct Get {
    #[at_arg(position = 0)]
    pub id: String<16>,
    #[at_arg(position = 1)]
    pub option: String<24>,
}

#[derive(AtatCmd)]
#[at_cmd("+set", NoResponse, timeout_ms = 100)]
pub struct Set {
    #[at_arg(position = 0)]
    pub id: String<16>,
    #[at_arg(position = 1)]
    pub option: String<24>,
}
