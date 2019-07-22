use std::io::Result;
use std::thread;
use std::time::Duration;
use visca::{Camera, PowerValue};

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    let power = camera.power().get()?;
    println!("{:?}", power);

    camera.power().set(PowerValue::On)?;

    thread::sleep(Duration::from_secs(3));

    let power = camera.power().get()?;
    println!("{:?}", power);

    Ok(())
}
