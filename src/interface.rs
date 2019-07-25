use crate::packet::{Message, Reply, Request};
use crate::{Error, Result};
use serialport::prelude::*;
use std::io;
use std::path::Path;
use std::time::Duration;

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

        serialport::open_with_settings(path.as_ref(), &settings)
            .map(|port| Interface::new(port))
            .map_err(|err| Error::Io(err.into()))
    }

    fn new(port: Box<dyn SerialPort>) -> Self {
        Interface {
            port,
            rbuf: [0; 16],
            rlen: 0,
        }
    }

    pub fn send_request_with_reply(&mut self, req: &Request) -> Result<Reply> {
        self.send_request(req)?;

        self.recv_reply().and_then(|reply| match reply.message() {
            Message::Ack => self.recv_reply().and_then(|reply| match reply.message() {
                Message::Completion(_) => Ok(reply),
                Message::Error(err) => Err(Error::Camera(err)),
                _ => Err(Error::InvalidReply),
            }),
            Message::Completion(_) => Ok(reply),
            Message::Error(err) => Err(Error::Camera(err)),
        })
    }

    pub fn send_request(&mut self, req: &Request) -> Result<()> {
        self.port
            .write_all(req.as_bytes())
            .map_err(|err| err.into())
    }

    pub fn recv_reply(&mut self) -> Result<Reply> {
        if let Some(reply) = self.extract_reply() {
            return Ok(reply);
        }

        loop {
            match self.port.read(&mut self.rbuf[self.rlen..]) {
                Ok(n) => {
                    self.rlen += n;

                    if let Some(reply) = self.extract_reply() {
                        return Ok(reply);
                    } else if self.rbuf_full() {
                        return Err(Error::ReadBufferFull);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(Error::Io(err)),
            }
        }
    }

    fn extract_reply(&mut self) -> Option<Reply> {
        memchr::memchr(0xff, &self.rbuf[..self.rlen]).map(|pos| {
            let end = pos + 1;
            let packet = Reply::parse(&self.rbuf[..end]);
            self.rlen -= end;

            if self.rlen > 0 {
                let mut swap = [0; 16];
                let unread = &self.rbuf[end..end + self.rlen];
                swap[..self.rlen].copy_from_slice(unread);
                self.rbuf[..self.rlen].copy_from_slice(&swap[..self.rlen]);
            }

            packet
        })
    }

    fn rbuf_full(&self) -> bool {
        self.rlen == self.rbuf.len()
    }
}
