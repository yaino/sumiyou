use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

/// LED(プルアップ接続)
pub struct Led(OutputPin);

impl Led {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self(Gpio::new()?.get(pin)?.into_output()))
    }

    pub fn light_on(&mut self) {
        self.0.set_high()
    }

    pub fn light_off(&mut self) {
        self.0.set_low()
    }
}
