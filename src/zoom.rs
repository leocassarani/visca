use crate::interface::Interface;
use crate::packet::Packet;
use std::io::Result;

pub struct Zoom<'a> {
    iface: &'a mut Interface,
}

impl<'a> Zoom<'a> {
    pub fn new(iface: &'a mut Interface) -> Self {
        Zoom { iface }
    }

    pub fn get(&mut self) -> Result<u16> {
        let packet = Packet::new()
            .address(1)
            .inquiry()
            .camera_1()
            .payload(&[0x47]);

        self.iface.send_packet(&packet)?;

        let res = self.iface.recv_packet()?;
        let buf = res.as_bytes();

        let mut val = (buf[2] as u16) << 12;
        val |= (buf[3] as u16) << 8;
        val |= (buf[4] as u16) << 4;
        val |= buf[5] as u16;

        Ok(val)
    }

    pub fn set(&mut self, val: u16) -> Result<()> {
        let payload = &[
            0x47,
            ((val & 0xf000) >> 12) as u8,
            ((val & 0x0f00) >> 8) as u8,
            ((val & 0x00f0) >> 4) as u8,
            (val & 0x000f) as u8,
        ];

        let packet = Packet::new()
            .address(1)
            .command()
            .camera_1()
            .payload(payload);

        self.iface.send_packet_with_reply(&packet)
    }
}
