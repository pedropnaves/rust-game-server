use crate::game::game_world::GameWorld;
use crate::game::player::Player;
use crate::network::connection::MessageSender;
use crate::network::message::input_message::InputMessage;
use crate::protocol::protocol_deserializer::ProtocolDeserializer;
use crate::protocol::protocol_serializer::ProtocolSerializer;

mod message_id {
    pub const LOGIN: u16 = 0x00;
    pub const PLAYER_WALK: u16 = 0x01;
}

pub struct Protocol<T: MessageSender> {
    protocol_deserializer: ProtocolDeserializer,
    protocol_serializer: ProtocolSerializer,
    player: Option<Box<Player>>,
    message_sender: T,
}

impl<T: MessageSender> Protocol<T> {
    pub fn new(message_sender: T) -> Self {
        Self {
            protocol_deserializer: ProtocolDeserializer::new(),
            protocol_serializer: ProtocolSerializer::new(),
            player: None,
            message_sender,
        }
    }

    pub fn handle_message(&mut self, input_message: &mut InputMessage) {
        let message_id = input_message.get_u16();
        match message_id {
            message_id::LOGIN => self.do_login_for_player(input_message),
            message_id::PLAYER_WALK => self.do_walk_for_player(input_message),
            _ => unimplemented!(),
        }
    }

    pub fn handle_disconnect(&mut self) {
        let player = self.player.clone();
        let mut output_message = self
            .protocol_serializer
            .create_notify_player_disconnected(&player.clone().unwrap());
        self.player = None;
        GameWorld::get_instance()
            .lock()
            .unwrap()
            .disconnect_player(&player.unwrap());
        self.message_sender
            .send_message_to_everyone(&mut output_message);
    }

    fn do_login_for_player(&mut self, input_message: &mut InputMessage) {
        let mut game_world = GameWorld::get_instance().lock().unwrap();
        let name = input_message.get_string();
        let new_player = game_world.create_player(name);
        let players = game_world.get_players();
        self.player = Some(Box::new(new_player.clone()));
        let mut output_message = self.protocol_serializer.create_login(&new_player, players);
        self.message_sender.send_message(&mut output_message);
        output_message = self
            .protocol_serializer
            .create_notify_player_connected(&new_player);
        self.message_sender
            .send_message_to_everyone(&mut output_message);
    }

    fn do_walk_for_player(&mut self, input_message: &mut InputMessage) {
        let mut game_world = GameWorld::get_instance().lock().unwrap();
        let direction = self.protocol_deserializer.to_direction(input_message);
        let mut player = self.player.clone().unwrap();
        game_world.do_walk(direction, &mut player);
        let mut output_message = self.protocol_serializer.create_player_walk(&player);
        self.message_sender
            .send_message_to_everyone(&mut output_message);
    }
}
