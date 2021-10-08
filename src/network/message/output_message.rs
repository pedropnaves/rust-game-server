use byte::BytesExt;
use crate::network::message::message::Message;

pub struct OutputMessage {
    message: super::message::Message
}

impl OutputMessage {

    pub fn new() -> Self {
        Self {
            message: Message::new()
        }
    }

    pub fn write_message_size(&mut self) {
        let length = self.message.length as u16;
        self.message.buffer.write::<u16>(&mut 0, length).unwrap();
    }

    pub fn get_buffer(&self) -> &[u8] {
        return &self.message.buffer[..];
    }

    #[warn(dead_code)]
    pub fn add_byte(&mut self, value: u8) {
        let size = std::mem::size_of::<u8>();
        let mut offset = self.message.position;
        self.message.position += size;
        self.message.length += size;
        self.message.buffer.write::<u8>(&mut offset, value).unwrap();
    }

    pub fn add_u16(&mut self, value: u16) {
        let size = std::mem::size_of::<u16>();
        let mut offset = self.message.position;
        self.message.position += size;
        self.message.length += size;
        self.message.buffer.write::<u16>(&mut offset, value).unwrap();
    }

    pub fn add_u32(&mut self, value: u32) {
        let size = std::mem::size_of::<u32>();
        let mut offset = self.message.position;
        self.message.position += size;
        self.message.length += size;
        self.message.buffer.write::<u32>(&mut offset, value).unwrap();
    }

    #[warn(dead_code)]
    pub fn add_u64(&mut self, value: u64) {
        let size = std::mem::size_of::<u64>();
        let mut offset = self.message.position;
        self.message.position += size;
        self.message.length += size;
        self.message.buffer.write::<u64>(&mut offset, value).unwrap();
    }

    #[warn(dead_code)]
    pub fn add_string(&mut self, value: &str) {
        let string_len = value.len();
        self.add_u16(string_len as u16);
        let mut offset = self.message.position;
        self.message.position += string_len;
        self.message.length += string_len;
        self.message.buffer.write::<&str>(&mut offset, value).unwrap();
    }
}