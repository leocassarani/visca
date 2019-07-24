use std::io::Result;
use std::path::Path;

mod commands;
mod interface;
mod packet;

use commands::{PanTilt, Power, Zoom};
use interface::Interface;

pub use commands::{PanTiltValue, PowerValue};

pub struct Camera {
    iface: Interface,
}

impl Camera {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let iface = Interface::open(path)?;
        Ok(Camera::new(iface))
    }

    fn new(iface: Interface) -> Self {
        Camera { iface }
    }

    pub fn pan_tilt(&mut self) -> PanTilt {
        PanTilt::new(&mut self.iface)
    }

    pub fn power(&mut self) -> Power {
        Power::new(&mut self.iface)
    }

    pub fn zoom(&mut self) -> Zoom {
        Zoom::new(&mut self.iface)
    }
}
