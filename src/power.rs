use crate::interface::Interface;
use crate::packet::{Message, Request};
use std::io::{Error, ErrorKind, Result};

pub struct Power<'a> {
    iface: &'a mut Interface,
}

impl<'a> Power<'a> {
    pub fn new(iface: &'a mut Interface) -> Self {
        Power { iface }
    }

    pub fn get(&mut self) -> Result<PowerValue> {
        let req = Request::new()
            .address(1)
            .inquiry()
            .camera_1()
            .payload(&[0x00]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(|reply| match reply.message() {
                Message::Completion(&[byte]) => PowerValue::from_u8(byte)
                    .ok_or(Error::new(ErrorKind::Other, "invalid power value")),
                _ => Err(Error::new(ErrorKind::Other, "unexpected message")),
            })
    }

    pub fn set(&mut self, value: PowerValue) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .camera_1()
            .payload(&[0x00, value as u8]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(|reply| match reply.message() {
                Message::Completion(&[]) => Ok(()),
                _ => Err(Error::new(ErrorKind::Other, "expected an empty reply")),
            })
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PowerValue {
    On = 0x02,
    Off = 0x03,
}

impl PowerValue {
    fn from_u8(b: u8) -> Option<Self> {
        match b {
            0x02 => Some(PowerValue::On),
            0x03 => Some(PowerValue::Off),
            _ => None,
        }
    }
}
