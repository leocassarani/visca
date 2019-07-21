use std::io::Result;
use visca::{Camera, PowerValue};

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    let power = camera.power().get()?;
    println!("{:?}", power);

    Ok(())
}
