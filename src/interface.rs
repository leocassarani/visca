use serialport::prelude::*;
use std::io::{ErrorKind, Result};
use std::path::Path;
use std::time::Duration;

use crate::packet::Packet;

pub struct Interface {
    port: Box<dyn SerialPort>,
    rbuf: [u8; 16],
    rlen: usize,
}

impl Interface {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let settings = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_secs(1),
        };

        let port = serialport::open_with_settings(path.as_ref(), &settings)?;
        Ok(Interface::new(port))
    }

    fn new(port: Box<dyn SerialPort>) -> Self {
        Interface {
            port,
            rbuf: [0; 16],
            rlen: 0,
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        self.port.write_all(packet.as_bytes())
    }

    pub fn recv_packet(&mut self) -> Result<Packet> {
        if let Some(pos) = memchr::memchr(0xff, &self.rbuf[..self.rlen]) {
            let end = pos + 1;
            let packet = Packet::from_slice(&self.rbuf[..end]);
            self.rlen -= end;

            if self.rlen > 0 {
                let unread = &self.rbuf[end..end + self.rlen].to_vec();
                self.rbuf[..self.rlen].copy_from_slice(unread);
            }

            return Ok(packet);
        }

        loop {
            match self.port.read(&mut self.rbuf[self.rlen..]) {
                Ok(n) => {
                    self.rlen += n;

                    if let Some(pos) = memchr::memchr(0xff, &self.rbuf[..self.rlen]) {
                        let end = pos + 1;
                        let packet = Packet::from_slice(&self.rbuf[..end]);
                        self.rlen -= end;

                        if self.rlen > 0 {
                            let unread = &self.rbuf[end..end + self.rlen].to_vec();
                            self.rbuf[..self.rlen].copy_from_slice(unread);
                        }

                        return Ok(packet);
                    };
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }
}
