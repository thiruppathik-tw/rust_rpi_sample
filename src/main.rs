use std::error::Error;

mod occupancy;

fn main() -> Result<(), Box<dyn Error>> {
    occupancy::device_info();
    occupancy::occupancy_manager()
}