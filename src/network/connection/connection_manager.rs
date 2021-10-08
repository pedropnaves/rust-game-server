use std::sync::Mutex;
use std::sync::Arc;
use std::sync::RwLock;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufWriter;
use std::io::Write;
use std::collections::HashMap;
use crate::network::message::output_message::OutputMessage;
use threadpool::ThreadPool;
use super::connection::Connection;

#[derive(Clone)]
pub struct ConnectionManager {
    connections_count: Arc<Mutex<u32>>,
    streams: Arc<RwLock<HashMap<u32, TcpStream>>>
}

impl ConnectionManager {

    pub fn new() -> Self {
        Self {
            connections_count: Arc::new(Mutex::new(0)),
            streams: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn start(self) {
        let tcp_listener = TcpListener::bind("127.0.0.1:7777").unwrap();
        let connection_pool = ThreadPool::new(20);
        for stream_result in tcp_listener.incoming() {
            let stream = stream_result.unwrap();
            let mut connections_count = self.connections_count.lock().unwrap();
            let mut connection = Connection::new(
                connections_count.clone(), 
                Box::new(stream.try_clone().unwrap()), 
                self.clone()
            );
            self.streams.write().unwrap().entry(*connections_count).or_insert(stream);
            *connections_count += 1;
            connection_pool.execute( move || {
                connection.listen_to_messages();
            });
        }
    }

    pub fn disconnect_by_id(&self, id: u32) {
        self.streams.write().unwrap().remove(&id);
    }

    pub fn send_message(&self, stream: &TcpStream, output_message: &mut OutputMessage) {
        self.send(stream, output_message);
    }

    pub fn send_for_everyone_except(&self, connection_id: u32, output_message: &mut OutputMessage) {
        for (id, stream) in self.streams.read().unwrap().iter() {
            if id != &connection_id {
                self.send(stream, output_message);
            }
        }
    }

    fn send(&self, stream: &TcpStream, output_message: &mut OutputMessage) {
        let mut writer = BufWriter::new(stream);
        output_message.write_message_size();
        let buffer = output_message.get_buffer();
        writer.write(buffer).unwrap();
        writer.flush().unwrap();
    }
}