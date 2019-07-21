use serialport::prelude::*;
use std::io::Result;
use std::path::Path;

pub struct Camera {
    iface: Interface,
}

impl Camera {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let iface = Interface::open(path.as_ref())?;
        Ok(Camera { iface })
    }

    pub fn power(&mut self) -> Power {
        Power {
            iface: &mut self.iface,
        }
    }

    pub fn zoom(&mut self) -> Zoom {
        Zoom {
            iface: &mut self.iface,
        }
    }

    pub fn pan_tilt(&mut self) -> PanTilt {
        PanTilt {
            iface: &mut self.iface,
        }
    }
}

struct Interface {
    port: Box<dyn SerialPort>,
}

impl Interface {
    fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let port = serialport::open(path.as_ref())?;
        Ok(Interface { port })
    }

    fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        println!("{:#x?}", packet.as_bytes());
        self.port.write_all(packet.as_bytes())
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<()> {
        self.port.read_exact(buf)
    }
}

struct Packet {
    bytes: [u8; 16],
    len: usize,
}

const PACKET_MIN_LEN: usize = 4;

#[repr(u8)]
enum Category {
    Interface = 0x00,
    Camera1 = 0x04,
    PanTilter = 0x06,
}

#[repr(u8)]
enum MessageType {
    Command = 0x01,
    Inquiry = 0x09,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            bytes: [0; 16],
            len: PACKET_MIN_LEN,
        }
    }

    pub fn address(mut self, addr: u8) -> Packet {
        self.bytes[0] = 0x80 | (0x0f & addr);
        self
    }

    pub fn msg_type(mut self, msg_type: MessageType) -> Packet {
        self.bytes[1] = msg_type as u8;
        self
    }

    pub fn category(mut self, category: Category) -> Packet {
        self.bytes[2] = category as u8;
        self
    }

    pub fn payload(mut self, payload: &[u8]) -> Packet {
        let len = payload.len();
        assert!(len <= 12);

        self.bytes[3..3 + len].copy_from_slice(payload);
        self.len = PACKET_MIN_LEN + len;
        self.bytes[self.len - 1] = 0xff;

        self
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

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
    pub fn get(&mut self) -> Result<PowerValue> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Inquiry)
            .category(Category::Camera1)
            .payload(&[0x00]);

        self.iface.send_packet(&packet)?;

        let mut buf = [0; 4];
        self.iface.recv(&mut buf)?;

        println!("{:#x?}", buf);

        let value = PowerValue::from_u8(buf[2]).expect("invalid power value");
        Ok(value)
    }

    pub fn set(&mut self, value: PowerValue) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Command)
            .category(Category::Camera1)
            .payload(&[0x00, value as u8]);

        self.iface.send_packet(&packet)
    }
}

pub struct Zoom<'a> {
    iface: &'a mut Interface,
}

impl<'a> Zoom<'a> {
    pub fn get(&mut self) -> Result<u16> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Inquiry)
            .category(Category::Camera1)
            .payload(&[47]);

        self.iface.send_packet(&packet)?;

        let mut buf = [0; 7];
        self.iface.recv(&mut buf)?;

        let mut val = (buf[2] as u16) << 12;
        val += (buf[3] as u16) << 8;
        val += (buf[4] as u16) << 4;
        val += buf[5] as u16;

        Ok(val)
    }

    pub fn set(&mut self, val: u16) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Command)
            .category(Category::Camera1)
            .payload(&[
                0x47,
                (val & 0xf000 >> 12) as u8,
                (val & 0x0f00 >> 8) as u8,
                (val & 0x00f0 >> 4) as u8,
                (val & 0x000f) as u8,
            ]);

        self.iface.send_packet(&packet)
    }

    pub fn stop(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Command)
            .category(Category::Camera1)
            .payload(&[0x07, 0x00]);

        self.iface.send_packet(&packet)
    }

    pub fn set_tele(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Command)
            .category(Category::Camera1)
            .payload(&[0x07, 0x02]);

        self.iface.send_packet(&packet)
    }

    pub fn set_wide(&mut self) -> Result<()> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Command)
            .category(Category::Camera1)
            .payload(&[0x07, 0x03]);

        self.iface.send_packet(&packet)
    }
}

pub struct PanTiltValue {
    pub pan: u16,
    pub tilt: u16,
}

impl PanTiltValue {
    fn from_slice(slice: &[u8]) -> Self {
        let mut pan = (slice[0] as u16) << 12;
        pan += (slice[1] as u16) << 8;
        pan += (slice[2] as u16) << 4;
        pan += slice[3] as u16;

        let mut tilt = (slice[4] as u16) << 12;
        tilt += (slice[5] as u16) << 8;
        tilt += (slice[6] as u16) << 4;
        tilt += slice[8] as u16;

        PanTiltValue { pan, tilt }
    }
}

pub struct PanTilt<'a> {
    iface: &'a mut Interface,
}

impl<'a> PanTilt<'a> {
    pub fn get(&mut self) -> Result<PanTiltValue> {
        let packet = Packet::new()
            .address(1)
            .msg_type(MessageType::Inquiry)
            .category(Category::PanTilter)
            .payload(&[0x12]);

        self.iface.send_packet(&packet)?;

        let mut buf = [0; 11];
        self.iface.recv(&mut buf)?;

        let value = PanTiltValue::from_slice(&buf[2..10]);
        Ok(value)
    }
}
