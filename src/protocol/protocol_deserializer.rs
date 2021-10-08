use crate::network::message::input_message::InputMessage;
use crate::game::position::Direction;
use super::protocol_utils::direction_id;

pub struct ProtocolDeserializer;

impl ProtocolDeserializer {

    pub fn new() -> Self {
        Self {}
    }

    pub fn to_direction(&self, input_message: &mut InputMessage) -> Direction {
        let direction_id = input_message.get_byte();
        return match direction_id {
            direction_id::UP => Direction::Up,
            direction_id::DOWN => Direction::Down,
            direction_id::LEFT => Direction::Left,
            direction_id::RIGHT => Direction::Right,
            _ => unimplemented!()
        };
    }
}