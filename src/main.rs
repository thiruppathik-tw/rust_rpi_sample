use std::{error::Error, thread, time::Duration};

mod occupancy;

fn main() -> Result<(), Box<dyn Error>> {
    occupancy::device_info();
    loop {
        thread::sleep(Duration::from_millis(1000));
        println!(
            "Current occupancy status: {}",
            occupancy::occupancy_status()?
        );
    }
}
