use super::connection_manager::ConnectionManager;
use crate::network::message::input_message::InputMessage;
use crate::network::message::output_message::OutputMessage;
use crate::protocol::protocol_impl::Protocol;
use std::io::BufReader;
use std::net::TcpStream;

pub trait MessageSender {
    fn send_message(&self, output_message: &mut OutputMessage);
    fn send_message_to_everyone(&self, output_message: &mut OutputMessage);
}

pub struct Connection {
    id: u32,
    stream: Box<TcpStream>,
    connection_manager: ConnectionManager,
}

impl Connection {
    pub fn new(id: u32, stream: Box<TcpStream>, connection_manager: ConnectionManager) -> Self {
        Self {
            id,
            stream,
            connection_manager,
        }
    }

    pub fn listen_to_messages(&mut self) {
        let mut input_message = InputMessage::new();
        let stream_clone = self.stream.try_clone().unwrap();
        let connection_manager = self.connection_manager.clone();
        let connection_id = self.id;
        let mut protocol = Protocol::new(self);
        let mut reader = BufReader::new(stream_clone);
        loop {
            input_message.read(&mut reader);

            if input_message.length() > 0 {
                protocol.handle_message(&mut input_message);
                input_message.reset();
            } else {
                connection_manager.disconnect_by_id(connection_id);
                protocol.handle_disconnect();
                break;
            }
        }
    }
}

impl MessageSender for &mut Connection {
    fn send_message(&self, output_message: &mut OutputMessage) {
        self.connection_manager
            .send_message(&self.stream, output_message);
    }

    fn send_message_to_everyone(&self, output_message: &mut OutputMessage) {
        self.connection_manager
            .send_for_everyone_except(self.id, output_message);
    }
}
