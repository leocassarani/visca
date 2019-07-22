use crate::interface::Interface;
use crate::packet::Packet;
use std::io::Result;

#[repr(u8)]
#[derive(Debug)]
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

pub struct Power<'a> {
    iface: &'a mut Interface,
}

impl<'a> Power<'a> {
    pub fn new(iface: &'a mut Interface) -> Self {
        Power { iface }
    }

    pub fn get(&mut self) -> Result<PowerValue> {
        let packet = Packet::new()
            .address(1)
            .inquiry()
            .camera_1()
            .payload(&[0x00]);

        self.iface.send_packet(&packet)?;

        let mut buf = [0; 4];
        self.iface.recv(&mut buf)?;

        let value = PowerValue::from_u8(buf[2]).expect("invalid power value");
        Ok(value)
    }

    pub fn set(&mut self, value: PowerValue) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .command()
            .camera_1()
            .payload(&[0x00, value as u8]);

        self.iface.send_packet(&packet)?;

        let mut buf = [0; 3];
        self.iface.recv(&mut buf)?;

        let mut buf = [0; 3];
        self.iface.recv(&mut buf)?;

        Ok(())
    }
}
