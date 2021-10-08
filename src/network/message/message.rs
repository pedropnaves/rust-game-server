pub struct Message {
    pub length: usize,
    pub position: usize,
    pub buffer: Vec<u8>
}

impl Message {
    pub const INITIAL_BUFFER_POSITION: usize = 2;
    pub const HEADER_LENGTH: usize = Message::INITIAL_BUFFER_POSITION;
    pub const MAX_BODY_LENGTH: usize = Message::HEADER_LENGTH + 24590;

    pub fn from(buffer: Vec<u8>, length: usize) -> Self {
        Self {
            length: length,
            position: Message::INITIAL_BUFFER_POSITION,
            buffer: buffer
        }
    }

    pub fn new() -> Self {
        let buffer: Vec<u8> = vec![0; Message::MAX_BODY_LENGTH];
        let length = 0;
        return Message::from(buffer, length);
    }
}