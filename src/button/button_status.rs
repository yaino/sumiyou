use rppal::gpio::Level;

#[derive(Clone, PartialEq)]
pub enum ButtonStatus {
    Release,
    Push,
}

impl ButtonStatus {
    pub fn from_level(level: Level) -> ButtonStatus {
        match level {
            Level::Low => ButtonStatus::Push,
            Level::High => ButtonStatus::Release,
        }
    }
}
