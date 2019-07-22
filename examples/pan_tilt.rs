use std::io::Result;
use std::thread;
use std::time::Duration;
use visca::{Camera, PanTiltValue};

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    let pt = PanTiltValue { pan: 180, tilt: 50 };
    camera.pan_tilt().set(pt)?;
    thread::sleep(Duration::from_secs(3));

    let pan_tilt = camera.pan_tilt().get()?;
    println!("{:?}", pan_tilt);

    Ok(())
}
