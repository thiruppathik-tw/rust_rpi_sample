#[allow(unused)]
#[derive(Debug)]
pub enum OccupancyError {
    /// Unknown model.
    ///
    /// The Raspberry Pi model or SoC can't be identified. Support for
    /// new models is usually added shortly after they are officially
    /// announced and available to the public. Make sure you're using
    /// the latest release of RPPAL.
    ///
    /// You may also encounter this error if your Linux distribution
    /// doesn't provide any of the common user-accessible system files
    /// that are used to identify the model and SoC.
    UnknownModel,
    /// Pin is already in use.
    ///
    /// The pin is already in use elsewhere in your application. If the pin is currently in
    /// use, you may retrieve it again after the [`Pin`] (or a derived [`InputPin`],
    /// [`OutputPin`] or [`IoPin`]) instance goes out of scope.
    ///
    /// [`Pin`]: struct.Pin.html
    /// [`InputPin`]: struct.InputPin.html
    /// [`OutputPin`]: struct.OutputPin.html
    /// [`IoPin`]: struct.IoPin.html
    PinUsed(u8),
    /// Pin is not available.
    ///
    /// The GPIO peripheral doesn't expose a GPIO pin with the specified number. Pins are
    /// addressed by their BCM GPIO numbers, rather than their physical location on the GPIO
    /// header.
    PinNotAvailable(u8),
    /// Permission denied when opening `/dev/gpiomem`, `/dev/mem` or `/dev/gpiochipN` for
    /// read/write access.
    ///
    /// More information on possible causes for this error can be found [here].
    ///
    /// [here]: index.html#permission-denied
    PermissionDenied(String),
    /// I/O error.
    IoError,
    /// Thread panicked.
    ThreadPanic,
}

#[allow(unused)]
pub fn device_info() -> String {
    String::from("Raspberry Pi")
}

#[allow(unused)]
pub fn occupancy_status() -> Result<bool, OccupancyError>{
    Ok(false)
}
