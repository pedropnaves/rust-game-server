mod network;
mod protocol;
mod game;
use crate::network::connection::connection_manager::ConnectionManager;

fn main() {
    ConnectionManager::new().start();
}