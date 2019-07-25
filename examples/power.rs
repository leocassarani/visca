use std::thread;
use std::time::Duration;
use visca::{Camera, PowerValue, Result};

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    camera.power().set(PowerValue::Off)?;

    thread::sleep(Duration::from_secs(2));

    camera.power().set(PowerValue::On)
}
