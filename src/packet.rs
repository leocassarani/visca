const PACKET_MAX_LEN: usize = 16;
const PAYLOAD_MAX_LEN: usize = 12;

#[derive(Debug)]
pub struct Packet {
    bytes: [u8; PACKET_MAX_LEN],
    len: usize,
}

#[repr(u8)]
enum Category {
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
            bytes: [0; PACKET_MAX_LEN],
            len: 0,
        }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let len = slice.len();
        assert!(len <= PACKET_MAX_LEN);

        let mut bytes = [0; PACKET_MAX_LEN];
        bytes[..len].copy_from_slice(slice);

        Packet { bytes, len }
    }

    pub fn is_ack(&self) -> bool {
        self.as_bytes()[1] == 0x41
    }

    pub fn is_completion(&self) -> bool {
        self.as_bytes()[1] == 0x51
    }

    pub fn address(mut self, addr: u8) -> Packet {
        assert!(addr <= 7);
        self.bytes[0] = 0x80 | addr;
        self
    }

    pub fn command(self) -> Packet {
        self.msg_type(MessageType::Command)
    }

    pub fn inquiry(self) -> Packet {
        self.msg_type(MessageType::Inquiry)
    }

    pub fn camera_1(self) -> Packet {
        self.category(Category::Camera1)
    }

    pub fn pan_tilter(self) -> Packet {
        self.category(Category::PanTilter)
    }

    pub fn payload(mut self, payload: &[u8]) -> Packet {
        let len = payload.len();
        assert!(len <= PAYLOAD_MAX_LEN);

        self.bytes[3..3 + len].copy_from_slice(payload);
        self.len = len + 4;
        self.bytes[self.len - 1] = 0xff;

        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    fn msg_type(mut self, msg_type: MessageType) -> Packet {
        self.bytes[1] = msg_type as u8;
        self
    }

    fn category(mut self, category: Category) -> Packet {
        self.bytes[2] = category as u8;
        self
    }
}

const REPLY_MIN_LEN: usize = 3;

pub struct Reply {
    buf: [u8; PACKET_MAX_LEN],
    len: usize,
}

impl Reply {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let len = bytes.len();
        assert!(len <= PACKET_MAX_LEN);

        let mut buf = [0; PACKET_MAX_LEN];
        buf[..len].copy_from_slice(bytes);

        Reply { buf, len }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn address(&self) -> u8 {
        assert!(self.len >= REPLY_MIN_LEN);
        (self.as_bytes()[0] >> 4) - 8
    }

    pub fn socket(&self) -> u8 {
        assert!(self.len >= REPLY_MIN_LEN);
        self.as_bytes()[1] & 0x0f
    }

    pub fn message(&self) -> Message {
        assert!(self.len >= REPLY_MIN_LEN);

        let bytes = self.as_bytes();
        match bytes[1] & 0xf0 {
            0x40 => Message::Ack,
            0x50 => Message::Completion(self.payload()),
            0x60 => Message::Error(ErrorKind::from_u8(bytes[2])),
            _ => unimplemented!(),
        }
    }

    fn payload(&self) -> &[u8] {
        let bytes = self.as_bytes();
        &bytes[2..bytes.len() - 1]
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind {
    MsgLength = 0x01,
    Syntax = 0x02,
    FullBuffer = 0x03,
    Canceled = 0x04,
    NoSocket = 0x05,
    NotExecutable = 0x41,
    Other,
}

impl ErrorKind {
    pub fn from_u8(b: u8) -> Self {
        match b {
            0x01 => ErrorKind::MsgLength,
            0x02 => ErrorKind::Syntax,
            0x03 => ErrorKind::FullBuffer,
            0x04 => ErrorKind::Canceled,
            0x05 => ErrorKind::NoSocket,
            0x41 => ErrorKind::NotExecutable,
            _ => ErrorKind::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message<'a> {
    Ack,
    Completion(&'a [u8]),
    Error(ErrorKind),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reply_as_bytes() {
        let bytes = &[0x90, 0x41, 0xff];
        let reply = Reply::from_bytes(bytes);
        assert_eq!(reply.as_bytes(), bytes);
    }

    #[test]
    fn test_reply_address() {
        let bytes = &[0x90, 0x41, 0xff];
        let reply = Reply::from_bytes(bytes);
        assert_eq!(reply.address(), 0x01);
    }

    #[test]
    fn test_reply_socket() {
        let bytes = &[0x90, 0x52, 0x02, 0xff];
        let reply = Reply::from_bytes(bytes);
        assert_eq!(reply.socket(), 0x02);
    }

    #[test]
    fn test_ack_message() {
        let reply = Reply::from_bytes(&[0x90, 0x41, 0xff]);
        assert_eq!(reply.message(), Message::Ack);
    }

    #[test]
    fn test_cmd_completion_message() {
        let reply = Reply::from_bytes(&[0x90, 0x51, 0xff]);
        assert_eq!(reply.message(), Message::Completion(&[]));
    }

    #[test]
    fn test_inquiry_reply_message() {
        let bytes = &[0x90, 0x50, 0x00, 0x01, 0x0b, 0x0c, 0xff];
        let reply = Reply::from_bytes(bytes);

        assert_eq!(
            reply.message(),
            Message::Completion(&[0x00, 0x01, 0x0b, 0x0c])
        );
    }

    #[test]
    fn test_error_message() {
        let reply = Reply::from_bytes(&[0x90, 0x60, 0x02, 0xff]);
        assert_eq!(reply.message(), Message::Error(ErrorKind::Syntax));
    }
}
