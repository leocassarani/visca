use std::io::{stdin, stdout, Error, ErrorKind, Result, Write};
use visca::Camera;

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    print!("Enter a preset number: ");
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    match line.trim().parse() {
        Ok(num) => camera.presets().recall(num),
        Err(e) => Err(Error::new(ErrorKind::Other, e)),
    }
}
