use std::error::Error;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering.
// BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

// BCM GPIO 16 is tied to physical pin 36.
const GPIO_PIR: u8 = 16;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking LED based on PIR input in a {}.", DeviceInfo::new()?.model());

    // Set pin 23 as output pin
    let mut pin_led = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Set pin 16 as input pin
    let pin_pir = Gpio::new()?.get(GPIO_PIR)?.into_input();

    pin_led.set_low();

    loop {
        // Read PIR data and toggle the LED based on the input
        if pin_pir.is_high() {
            pin_led.set_high();
            // println!("Motion detected");
        } else {
            pin_led.set_low();
        }
    }
}