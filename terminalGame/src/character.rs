use crate::{position::{Direction, Position}, terminal::Terminal, weapon::Weapon};

pub struct Character {
    position: Position,
    health: u8,
    weapon: Weapon,
}

impl Character {
    pub fn create(position: Position) -> Self {
        Character{position, health: 20, weapon: Weapon::createPistol()}
    }

    pub fn moves(&mut self, terminal: &mut Terminal, direction: Direction) {
        terminal.eraseBox(&self.position);
        match direction {
            Direction::Up => self.position.moveUp(),
            Direction::Down => self.position.moveDown(),
            Direction::Left => self.position.moveLeft(),
            Direction::Right => self.position.moveRight(),
            Direction::None => (),
        }
    }

    pub fn getPosition(&self) -> &Position {
        &self.position
    }
}