use std::io::{stdin, stdout, Write};
use std::process;
use visca::{Camera, Result};

fn main() -> Result<()> {
    let mut camera = Camera::open("/dev/cu.usbserial-AM00QCCD")?;

    print!("Enter a preset number: ");
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    if let Ok(num) = line.trim().parse() {
        camera.presets().recall(num)
    } else {
        eprintln!("Invalid number!");
        process::exit(1);
    }
}
