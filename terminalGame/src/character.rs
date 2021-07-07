use crate::position::Position;

pub struct Character {
    pub position: Position,
    pub health: u8,
}

impl Character {
    pub fn create(position: Position) -> Self {
        Character{position, health: 20}
    }

    pub fn moveUp(&mut self) {
        self.position.setY(self.position.getY() - 1);
    }
    pub fn moveDown(&mut self) {
        self.position.setY(self.position.getY() + 1);
    }
    pub fn moveRight(&mut self) {
        self.position.setX(self.position.getX() - 1);
    }
    pub fn moveLeft(&mut self) {
        self.position.setX(self.position.getX() + 1);
    }
}