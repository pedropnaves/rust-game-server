use std::sync::atomic::AtomicU32;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;
use super::player::Player;
use crate::game::position::Position;
use crate::game::position::Direction;

lazy_static! {
    static ref INSTANCE: Mutex<GameWorld> = Mutex::new(GameWorld::new());
}

pub struct GameWorld {
    current_player_id: AtomicU32,
    players: Arc<RwLock<HashMap<u32, Box<Player>>>>
}

impl GameWorld {

    fn new() -> Self {
        Self {
            current_player_id: AtomicU32::new(0),
            players: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get_instance() -> &'static Mutex<Self> {
        return &INSTANCE;
    }

    pub fn create_player(&mut self, name: std::string::String) -> Player {
        *self.current_player_id.get_mut() += 1;
        let player_id: u32 = *self.current_player_id.get_mut();
        let player = Player {
            id: player_id,
            name: name.to_owned(),
            position: Position {
                x: 0,
                y: 0,
                z: 0,
                direction: Direction::Down
            }
        };

        self.update_player(player_id, player.clone());

        return player;
    }

    pub fn disconnect_player(&mut self, player: &Player) {
        self.players.write().unwrap().remove(&player.id);
    }

    pub fn get_players(&self) -> Vec<Box<Player>> {
        let mut players = Vec::with_capacity(self.players.read().unwrap().len());
        for (_id, player) in self.players.read().unwrap().clone().into_iter(){
            players.push(player);
        }
        return players;
    }

    pub fn do_walk<'a>(&mut self, direction: Direction, player: &'a mut Player) {
        match direction {
            Direction::Up => player.position.y += 1,
            Direction::Down => player.position.y -= 1,
            Direction::Left => player.position.x -= 1,
            Direction::Right => player.position.x += 1
        }
        
        self.update_player(player.id, player.clone());
    }

    fn update_player(&mut self, id: u32, player: Player) {
        self.players.write().unwrap()
        .entry(id)
        .or_insert(Box::new(player));
    }
}