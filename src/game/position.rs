#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub direction: Direction
}