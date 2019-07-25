const PACKET_MAX_LEN: usize = 16;
const PAYLOAD_MAX_LEN: usize = 12;

#[derive(Debug)]
pub struct Request {
    bytes: [u8; PACKET_MAX_LEN],
    len: usize,
}

impl Request {
    pub fn new() -> Self {
        Request {
            bytes: [0; PACKET_MAX_LEN],
            len: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn address(mut self, addr: u8) -> Request {
        assert!(addr <= 7);
        self.bytes[0] = 0x80 | addr;
        self
    }

    pub fn command(self) -> Request {
        self.msg_type(MessageType::Command)
    }

    pub fn inquiry(self) -> Request {
        self.msg_type(MessageType::Inquiry)
    }

    pub fn camera_1(self) -> Request {
        self.category(Category::Camera1)
    }

    pub fn pan_tilter(self) -> Request {
        self.category(Category::PanTilter)
    }

    pub fn payload(mut self, payload: &[u8]) -> Request {
        let len = payload.len();
        assert!(len <= PAYLOAD_MAX_LEN);

        self.bytes[3..3 + len].copy_from_slice(payload);
        self.len = len + 4;
        self.bytes[self.len - 1] = 0xff;

        self
    }

    fn msg_type(mut self, msg_type: MessageType) -> Request {
        self.bytes[1] = msg_type as u8;
        self
    }

    fn category(mut self, category: Category) -> Request {
        self.bytes[2] = category as u8;
        self
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Category {
    Camera1 = 0x04,
    PanTilter = 0x06,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MessageType {
    Command = 0x01,
    Inquiry = 0x09,
}

const REPLY_MIN_LEN: usize = 3;

#[derive(Debug)]
pub struct Reply {
    bytes: [u8; PACKET_MAX_LEN],
    len: usize,
}

impl Reply {
    pub fn parse(slice: &[u8]) -> Self {
        let len = slice.len();
        assert!(len <= PACKET_MAX_LEN);

        let mut bytes = [0; PACKET_MAX_LEN];
        bytes[..len].copy_from_slice(slice);

        Reply { bytes, len }
    }

    pub fn as_bytes(&self) -> &[u8] {
        assert!(self.len >= REPLY_MIN_LEN);
        &self.bytes[..self.len]
    }

    pub fn address(&self) -> u8 {
        (self.as_bytes()[0] >> 4) - 8
    }

    pub fn socket(&self) -> u8 {
        self.as_bytes()[1] & 0x0f
    }

    pub fn message(&self) -> Message {
        let bytes = self.as_bytes();

        match bytes[1] & 0xf0 {
            0x40 => Message::Ack,
            0x50 => Message::Completion(self.payload()),
            0x60 => Message::Error(ErrorKind::from_u8(bytes[2])),
            _ => unimplemented!(),
        }
    }

    pub fn payload(&self) -> &[u8] {
        let bytes = self.as_bytes();
        &bytes[2..bytes.len() - 1]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message<'a> {
    Ack,
    Completion(&'a [u8]),
    Error(ErrorKind),
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind {
    /// The command had an invalid message length.
    MsgLength = 0x01,
    /// The format or parameters of the command were invalid.
    Syntax = 0x02,
    /// The command could not be accepted because too many commands are being executed already.
    FullBuffer = 0x03,
    /// The command was canceled.
    Canceled = 0x04,
    /// An invalid socket number was specified.
    NoSocket = 0x05,
    /// The command could not be executed in the current state of the camera.
    NotExecutable = 0x41,
    /// Any other error not included in this list.
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

    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::MsgLength => "invalid message length",
            ErrorKind::Syntax => "syntax error",
            ErrorKind::FullBuffer => "command buffer is full",
            ErrorKind::Canceled => "command was canceled",
            ErrorKind::NoSocket => "invalid socket number",
            ErrorKind::NotExecutable => "could not execute command",
            ErrorKind::Other => "other camera error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reply_address() {
        let reply = Reply::parse(&[0x90, 0x41, 0xff]);
        assert_eq!(reply.address(), 0x01);
    }

    #[test]
    fn test_reply_socket() {
        let reply = Reply::parse(&[0x90, 0x52, 0x02, 0xff]);
        assert_eq!(reply.socket(), 0x02);
    }

    #[test]
    fn test_ack_message() {
        let reply = Reply::parse(&[0x90, 0x41, 0xff]);
        assert_eq!(reply.message(), Message::Ack);
    }

    #[test]
    fn test_empty_completion_message() {
        let reply = Reply::parse(&[0x90, 0x51, 0xff]);
        assert_eq!(reply.message(), Message::Completion(&[]));
    }

    #[test]
    fn test_inquiry_reply_message() {
        let bytes = &[0x90, 0x50, 0x00, 0x01, 0x0b, 0x0c, 0xff];
        let reply = Reply::parse(bytes);

        assert_eq!(
            reply.message(),
            Message::Completion(&[0x00, 0x01, 0x0b, 0x0c])
        );
    }

    #[test]
    fn test_error_message() {
        let reply = Reply::parse(&[0x90, 0x60, 0x02, 0xff]);
        assert_eq!(reply.message(), Message::Error(ErrorKind::Syntax));
    }
}
