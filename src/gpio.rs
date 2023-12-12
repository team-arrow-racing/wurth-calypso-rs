use crate::{ConfirmationResult, RequestResult};

/// Pin configuration.
pub enum Configuration {
    Unused,
    Input((State, Pull)),
    Output(State),
    Pwm(Pwm),
}

/// Pin state.
pub enum State {
    Low,
    High,
}

/// Pin pull resistor state.
pub enum Pull {
    Down,
    Up,
}

/// PWM setting
pub struct Pwm {
    period_ms: u16,
    ratio_percent: u8,
}

/// GPIO commands.
pub trait Gpio {
    /// Read the current GPIO configuration for a given pin.
    async fn gpio_get(&mut self, id: u8) -> RequestResult<Configuration>;

    /// Read the default GPIO configuration for a given pin.
    async fn gpio_get_default(
        &mut self,
        id: u8,
    ) -> RequestResult<Configuration>;

    /// Set the value for a given pin.
    async fn gpio_set(
        &mut self,
        id: u8,
        configuration: Configuration,
    ) -> ConfirmationResult;
}
