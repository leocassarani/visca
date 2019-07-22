use serialport::prelude::*;
use std::io::Result;
use std::path::Path;
use std::time::Duration;

use crate::packet::Packet;

pub struct Interface {
    port: Box<dyn SerialPort>,
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
        Interface { port }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        self.port.write_all(packet.as_bytes())
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<()> {
        self.port.read_exact(buf)
    }
}
