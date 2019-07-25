use std::thread;
use std::time::Duration;
use visca::{Camera, PanTiltValue, Result};

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    let pt = PanTiltValue { pan: 80, tilt: 50 };
    camera.pan_tilt().set_absolute(pt)?;

    thread::sleep(Duration::from_secs(2));

    camera.pan_tilt().get().and_then(|pan_tilt| {
        println!("{:?}", pan_tilt);
        Ok(())
    })
}
