use super::position::Position;

#[derive(Clone)]
pub struct Player {
    pub id: u32,
    pub name: std::string::String,
    pub position: Position,
}
