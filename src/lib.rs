use std::io::Result;
use std::path::Path;

mod interface;
mod packet;
mod pan_tilt;
mod power;
mod zoom;

use interface::Interface;
use pan_tilt::PanTilt;
use power::Power;
use zoom::Zoom;

pub use pan_tilt::PanTiltValue;
pub use power::PowerValue;

pub struct Camera {
    iface: Interface,
}

impl Camera {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Interface::open(path).map(Camera::new)
    }

    fn new(iface: Interface) -> Self {
        Camera { iface }
    }

    pub fn power(&mut self) -> Power {
        Power::new(&mut self.iface)
    }

    pub fn zoom(&mut self) -> Zoom {
        Zoom::new(&mut self.iface)
    }

    pub fn pan_tilt(&mut self) -> PanTilt {
        PanTilt::new(&mut self.iface)
    }
}
