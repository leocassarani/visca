use crate::interface::Interface;
use crate::packet::{Message, Reply, Request};
use std::io::{Error, ErrorKind, Result};

pub struct PanTilt<'a> {
    iface: &'a mut Interface,
}

impl<'a> PanTilt<'a> {
    pub fn new(iface: &'a mut Interface) -> Self {
        PanTilt { iface }
    }

    pub fn get(&mut self) -> Result<PanTiltValue> {
        let req = Request::new()
            .address(1)
            .inquiry()
            .pan_tilter()
            .payload(&[0x12]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(|reply| match reply.message() {
                Message::Completion(payload) if payload.len() == PAN_TILT_VALUE_LEN => {
                    Ok(PanTiltValue::from_bytes(payload))
                }
                _ => Err(Error::new(ErrorKind::Other, "unexpected message")),
            })
    }

    pub fn set(&mut self, val: PanTiltValue) -> Result<()> {
        let mut payload = [0; 11];
        payload[..2].copy_from_slice(&[0x02, 0x01]);
        payload[3..].copy_from_slice(&val.to_bytes());

        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&payload);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn up(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x03, 0x01]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn down(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x03, 0x02]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn left(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x01, 0x03]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn right(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x02, 0x03]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn up_left(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x01, 0x01]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn up_right(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x02, 0x01]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn down_left(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x01, 0x02]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn down_right(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x02, 0x02]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }

    pub fn stop(&mut self) -> Result<()> {
        let req = Request::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x03, 0x03]);

        self.iface
            .send_request_with_reply(&req)
            .and_then(empty_reply)
    }
}

fn empty_reply(reply: Reply) -> Result<()> {
    match reply.message() {
        Message::Completion(&[]) => Ok(()),
        _ => Err(Error::new(ErrorKind::Other, "expected an empty reply")),
    }
}

const PAN_TILT_VALUE_LEN: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PanTiltValue {
    pub pan: i16,
    pub tilt: i16,
}

impl PanTiltValue {
    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), PAN_TILT_VALUE_LEN);

        let mut pan = (bytes[0] as i16) << 12;
        pan |= (bytes[1] as i16) << 8;
        pan |= (bytes[2] as i16) << 4;
        pan |= bytes[3] as i16;

        let mut tilt = (bytes[4] as i16) << 12;
        tilt |= (bytes[5] as i16) << 8;
        tilt |= (bytes[6] as i16) << 4;
        tilt |= bytes[7] as i16;

        PanTiltValue { pan, tilt }
    }

    fn to_bytes(&self) -> [u8; 8] {
        let pan = self.pan as u16;
        let tilt = self.tilt as u16;

        [
            ((pan & 0xf000) >> 12) as u8,
            ((pan & 0x0f00) >> 8) as u8,
            ((pan & 0x00f0) >> 4) as u8,
            (pan & 0x000f) as u8,
            ((tilt & 0xf000) >> 12) as u8,
            ((tilt & 0x0f00) >> 8) as u8,
            ((tilt & 0x00f0) >> 4) as u8,
            (tilt & 0x000f) as u8,
        ]
    }
}
