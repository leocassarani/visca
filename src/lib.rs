use std::error;
use std::fmt;
use std::io;
use std::path::Path;
use std::result;

mod commands;
mod interface;
mod packet;

use commands::{PanTilt, Presets, Zoom};
use interface::Interface;

pub use commands::PanTiltValue;
pub use packet::ErrorKind;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Camera(ErrorKind),
    InvalidReply,
    ReadBufferFull,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::Camera(kind) => write!(f, "{}", kind.as_str()),
            Error::InvalidReply => write!(f, "invalid reply"),
            Error::ReadBufferFull => write!(f, "read buffer is full"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

pub type Result<T> = result::Result<T, Error>;

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

    pub fn presets(&mut self) -> Presets {
        Presets::new(&mut self.iface)
    }

    pub fn zoom(&mut self) -> Zoom {
        Zoom::new(&mut self.iface)
    }
}
