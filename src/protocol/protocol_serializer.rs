use super::protocol_utils::direction_id;
use crate::game::player::Player;
use crate::game::position::Direction;
use crate::network::message::output_message::OutputMessage;

mod message_id {
    pub const LOGIN: u16 = 0x00;
    pub const PLAYER_CONNECTED: u16 = 0x01;
    pub const PLAYER_DISCONNECTED: u16 = 0x02;
    pub const PLAYER_WALK: u16 = 0x03;
}

pub struct ProtocolSerializer;

impl ProtocolSerializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_login(&self, connected_player: &Player, players: Vec<Player>) -> OutputMessage {
        let mut output_message = OutputMessage::new();
        output_message.add_u16(message_id::LOGIN);
        self.add_player(connected_player, &mut output_message);
        let players_size = players.len() - 1;
        output_message.add_u16(players_size as u16);

        for player in players {
            if player.id != connected_player.id {
                self.add_player(&player, &mut output_message);
            }
        }
        output_message
    }

    pub fn create_notify_player_connected(&self, player: &Player) -> OutputMessage {
        let mut output_message = OutputMessage::new();
        output_message.add_u16(message_id::PLAYER_CONNECTED);
        self.add_player(player, &mut output_message);
        output_message
    }

    pub fn create_notify_player_disconnected(&self, player: &Player) -> OutputMessage {
        let mut output_message = OutputMessage::new();
        output_message.add_u16(message_id::PLAYER_DISCONNECTED);
        output_message.add_u32(player.id);
        output_message
    }

    pub fn create_player_walk(&self, player: &Player) -> OutputMessage {
        let mut output_message = OutputMessage::new();
        output_message.add_u16(message_id::PLAYER_WALK);

        self.add_player(player, &mut output_message);

        output_message
    }

    fn add_player(&self, player: &Player, output_message: &mut OutputMessage) {
        output_message.add_u32(player.id);
        output_message.add_string(player.name.as_str());
        output_message.add_u16(player.position.x);
        output_message.add_u16(player.position.y);
        output_message.add_u16(player.position.z);
        self.from_direction(output_message, player.position.direction);
    }

    pub fn from_direction(&self, output_message: &mut OutputMessage, player_direction: Direction) {
        match player_direction {
            Direction::Up => output_message.add_byte(direction_id::UP),
            Direction::Down => output_message.add_byte(direction_id::DOWN),
            Direction::Left => output_message.add_byte(direction_id::LEFT),
            Direction::Right => output_message.add_byte(direction_id::RIGHT),
        }
    }
}
