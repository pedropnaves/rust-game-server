use byte::BytesExt;
use byte::LE;
use byte::ctx::{Str};
use std::net::TcpStream;
use std::io::BufReader;
use std::io::Read;
use crate::network::message::message::Message;

pub struct InputMessage {
    message: Message
}

impl InputMessage {

    pub fn new() -> Self {
        Self {
            message: Message::new()
        }
    }

    pub fn read(&mut self, reader: &mut BufReader<TcpStream>) {
        let (buffer, length) = InputMessage::create_buffer(reader);
        self.message = Message::from(buffer, length);
    }

    pub fn reset(&mut self) {
        self.message = Message::new();
    }

    pub fn length(&self) -> usize {
        return self.message.length;
    }

    fn create_buffer(reader: &mut BufReader<TcpStream>) -> (Vec<u8>, usize) {
        let mut buffer: Vec<u8> = vec![0; Message::MAX_BODY_LENGTH];
            
        let read_result = reader.by_ref()
            .take(Message::HEADER_LENGTH as u64)
            .read(&mut buffer[0..]);

        return match read_result {
            _result => {
                let length = (buffer[0] | buffer[1]) as usize;

                reader.by_ref()
                    .take(length as u64)
                    .read(&mut buffer[Message::HEADER_LENGTH..]).unwrap();

                (buffer, length)
            }
            Err(_) => (buffer, 0),
        };
    }

    pub fn get_byte(&mut self) -> u8 {
        let size = std::mem::size_of::<u8>();
        let mut offset = self.message.position;
        self.message.position += size;
        return self.message.buffer.read_with::<u8>(&mut offset, LE).unwrap()
    }

    pub fn get_u16(&mut self) -> u16 {
        let size = std::mem::size_of::<u16>();
        let mut offset = self.message.position;
        self.message.position += size;
        return self.message.buffer.read_with::<u16>(&mut offset, LE).unwrap()
    }

    #[warn(dead_code)]
    pub fn get_u32(&mut self) -> u32 {
        let size = std::mem::size_of::<u32>();
        let mut offset = self.message.position;
        self.message.position += size;
        return self.message.buffer.read_with::<u32>(&mut offset, LE).unwrap()
    }

    #[warn(dead_code)]
    pub fn get_u64(&mut self) -> u64 {
        let size = std::mem::size_of::<u64>();
        let mut offset = self.message.position;
        self.message.position += size;
        return self.message.buffer.read_with::<u64>(&mut offset, LE).unwrap()
    }

    pub fn get_string(&mut self) -> std::string::String {
        let size = self.get_u16() as usize;
        let mut offset = self.message.position + size;
        self.message.position += size;
        return self.message.buffer.read_with::<&str>(&mut offset, Str::Len(size)).unwrap().to_string();
    }
}