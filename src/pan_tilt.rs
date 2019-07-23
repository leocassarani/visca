use crate::interface::Interface;
use crate::packet::Packet;
use std::io::Result;

pub struct PanTilt<'a> {
    iface: &'a mut Interface,
}

impl<'a> PanTilt<'a> {
    pub fn new(iface: &'a mut Interface) -> Self {
        PanTilt { iface }
    }

    pub fn get(&mut self) -> Result<PanTiltValue> {
        let packet = Packet::new()
            .address(1)
            .inquiry()
            .pan_tilter()
            .payload(&[0x12]);

        self.iface.send_packet(&packet)?;

        let packet = self.iface.recv_packet()?;
        let buf = packet.as_bytes();

        let value = PanTiltValue::from_slice(&buf[2..10]);
        Ok(value)
    }

    pub fn set(&mut self, val: PanTiltValue) -> Result<()> {
        let mut payload = [0; 11];
        payload[..2].copy_from_slice(&[0x02, 0x01]);
        payload[3..].copy_from_slice(&val.to_bytes());

        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&payload);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn up(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x03, 0x01]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn down(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x03, 0x02]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn left(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x01, 0x03]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn right(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x02, 0x03]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn up_left(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x01, 0x01]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn up_right(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x02, 0x01]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn down_left(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x01, 0x02]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn down_right(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x02, 0x02]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .pan_tilter()
            .payload(&[0x01, 0x01, 0x01, 0x03, 0x03]);

        self.iface.send_packet(&packet)?;

        self.iface.recv_packet()?;
        self.iface.recv_packet()?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PanTiltValue {
    pub pan: i16,
    pub tilt: i16,
}

impl PanTiltValue {
    fn from_slice(slice: &[u8]) -> Self {
        assert_eq!(slice.len(), 8);

        let mut pan = (slice[0] as i16) << 12;
        pan |= (slice[1] as i16) << 8;
        pan |= (slice[2] as i16) << 4;
        pan |= slice[3] as i16;

        let mut tilt = (slice[4] as i16) << 12;
        tilt |= (slice[5] as i16) << 8;
        tilt |= (slice[6] as i16) << 4;
        tilt |= slice[7] as i16;

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
