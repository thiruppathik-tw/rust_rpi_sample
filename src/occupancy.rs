use std::error::Error;

use rppal::gpio::{Gpio};
use rppal::gpio::Error as OccupancyError;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering.
// BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

// BCM GPIO 16 is tied to physical pin 36.
const GPIO_PIR: u8 = 16;

pub fn device_info() -> String {
    let d = DeviceInfo::new();
    if d.is_ok() {
        let device = d.unwrap().model().to_string();
        println!(
            "Blinking LED based on PIR input in a {}.",
            device
        );
        return device;
    } else {
        println!("Error: {}", d.unwrap_err());
        return "UnknownDevice".to_string();
    }
}

pub fn occupancy_status() -> Result<bool, OccupancyError>{
    // Set pin 23 as output pin
    let mut pin_led = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Set pin 16 as input pin
    let pin_pir = Gpio::new()?.get(GPIO_PIR)?.into_input();

    pin_led.set_low();
    let mut status = false;

    // Read PIR data and toggle the LED based on the input
    if pin_pir.is_high() {
        pin_led.set_high();
        status = true;
        // println!("Motion detected");
    } else {
        pin_led.set_low();
        status = false;
    }
    Ok(status)
}

pub fn occupancy_manager() -> Result<(), Box<dyn Error>> {
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
