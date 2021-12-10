use super::player::Player;
use crate::game::position::Direction;
use crate::game::position::Position;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

lazy_static! {
    static ref INSTANCE: Mutex<GameWorld> = Mutex::new(GameWorld::new());
}

pub struct GameWorld {
    current_player_id: AtomicU32,
    players: Arc<RwLock<HashMap<u32, Box<Player>>>>,
}

impl GameWorld {
    fn new() -> Self {
        Self {
            current_player_id: AtomicU32::new(0),
            players: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_instance() -> &'static Mutex<Self> {
        &INSTANCE
    }

    pub fn create_player(&mut self, name: std::string::String) -> Player {
        *self.current_player_id.get_mut() += 1;
        let player_id: u32 = *self.current_player_id.get_mut();
        let player = Player {
            id: player_id,
            name,
            position: Position {
                x: 0,
                y: 0,
                z: 0,
                direction: Direction::Down,
            },
        };

        self.update_player(player_id, player.clone());

        player
    }

    pub fn disconnect_player(&mut self, player: &Player) {
        self.players.write().unwrap().remove(&player.id);
    }

    pub fn get_players(&self) -> Vec<Player> {
        self.players
            .read()
            .unwrap()
            .clone()
            .into_iter()
            .map(|(_, p)| *p)
            .collect()
    }

    pub fn do_walk(&mut self, direction: Direction, player: &mut Player) {
        match direction {
            Direction::Up => player.position.y += 1,
            Direction::Down => player.position.y -= 1,
            Direction::Left => player.position.x -= 1,
            Direction::Right => player.position.x += 1,
        }

        self.update_player(player.id, player.clone());
    }

    fn update_player(&mut self, id: u32, player: Player) {
        self.players
            .write()
            .unwrap()
            .entry(id)
            .or_insert_with(|| Box::new(player));
    }
}
