mod button;
mod led;

use button::Button;
use led::Led;
use std::error::Error;
use std::sync::mpsc;

const GPIO_LED: u8 = 23;
const GPIO_BUTTON: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let mut light = Led::new(GPIO_LED)?;
    let light_sw = Button::new(GPIO_BUTTON)?;

    let (tx, rx) = mpsc::channel();
    light_sw.run(tx);
    while let Ok(recv) = rx.recv() {
        match recv {
            recv if recv == "off" => light.light_off(),
            recv if recv == "on" => light.light_on(),
            _ => break,
        }
    }
    Ok(())
}
