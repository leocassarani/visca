const PACKET_MIN_LEN: usize = 4;
const PACKET_MAX_LEN: usize = 16;

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
            len: PACKET_MIN_LEN,
        }
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
        assert!(len <= 12);

        self.bytes[3..3 + len].copy_from_slice(payload);
        self.len = PACKET_MIN_LEN + len;
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
