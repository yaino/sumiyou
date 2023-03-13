mod button_status;
use self::button_status::ButtonStatus;
use rppal::gpio::{Gpio, InputPin, Level};
use std::{error::Error, sync::mpsc::Sender, thread, time::Duration};

/// ボタン
pub struct Button {
    pin: InputPin,
    count: u8,
    status: ButtonStatus,
}

impl Button {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: Gpio::new()?.get(pin)?.into_input(),
            count: 0,
            status: ButtonStatus::Release,
        })
    }

    pub fn run(mut self, tx: Sender<String>) {
        thread::spawn(move || loop {
            if self.observe_button_change() {
                match self.status() {
                    ButtonStatus::Push => tx.send("on".to_string()).unwrap(),
                    ButtonStatus::Release => tx.send("off".to_string()).unwrap(),
                }
            }
            thread::sleep(Duration::from_millis(10));
        });
    }

    fn observe_button_change(&mut self) -> bool {
        match self.pin.read() {
            Level::Low => {
                // ボタンの押下状態に変化があった
                if self.status == ButtonStatus::from_level(Level::High) {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = ButtonStatus::from_level(Level::Low);
                        self.count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    self.count = 0;
                    false
                }
            }
            Level::High => {
                // ボタンの押下状態に変化があった
                if self.status == ButtonStatus::from_level(Level::Low) {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = ButtonStatus::from_level(Level::High);
                        self.count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    self.count = 0;
                    false
                }
            }
        }
    }

    pub fn status(&self) -> ButtonStatus {
        self.status.clone()
    }
}
