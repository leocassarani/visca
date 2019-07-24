use crate::interface::Interface;
use crate::packet::{Message, Request};
use std::io::{Error, ErrorKind, Result};

pub struct Zoom<'a> {
    iface: &'a mut Interface,
}

impl<'a> Zoom<'a> {
    pub fn new(iface: &'a mut Interface) -> Self {
        Zoom { iface }
    }

    pub fn get(&mut self) -> Result<u16> {
        let req = Request::new()
            .address(1)
            .inquiry()
            .camera_1()
            .payload(&[0x47]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(|reply| match reply.message() {
                Message::Completion(payload) if payload.len() == 4 => {
                    let mut val = (payload[0] as u16) << 12;
                    val |= (payload[1] as u16) << 8;
                    val |= (payload[2] as u16) << 4;
                    val |= payload[3] as u16;
                    Ok(val)
                }
                _ => Err(Error::new(ErrorKind::Other, "unexpected message")),
            })
    }

    pub fn set(&mut self, val: u16) -> Result<()> {
        let payload = &[
            0x47,
            ((val & 0xf000) >> 12) as u8,
            ((val & 0x0f00) >> 8) as u8,
            ((val & 0x00f0) >> 4) as u8,
            (val & 0x000f) as u8,
        ];

        let req = Request::new()
            .address(1)
            .command()
            .camera_1()
            .payload(payload);

        self.iface
            .send_request_with_reply(&req)
            .and_then(|reply| match reply.message() {
                Message::Completion(&[]) => Ok(()),
                _ => Err(Error::new(ErrorKind::Other, "expected an empty reply")),
            })
    }
}
