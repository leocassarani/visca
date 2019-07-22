use std::io::Result;
use std::thread;
use std::time::{Duration, Instant};
use visca::Camera;

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;
    let want = 444;

    let now = Instant::now();
    camera.zoom().set(want)?;

    loop {
        let zoom = camera.zoom().get()?;
        if zoom == want {
            println!("{}ms", now.elapsed().as_millis());
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
