use termion::color::Color;

use crate::{position::{Direction, Position}, terminal::Terminal, weapon::Weapon};

pub struct Character {
    position: Position,
    health: u8,
    weapon: Weapon,
    color: Box<dyn Color>,
}

impl ToString for Character {
    fn to_string(&self) -> String {
        format!("x:{} y:{} health:{} {}",
        self.position.getX(), self.position.getY(), self.health, self.weapon.to_string())
    }
}

impl Character {
    pub fn create(position: Position, color: Box<dyn Color>) -> Self {
        Character{position, health: 20, weapon: Weapon::createPistol(), color}
    }

    pub fn moves(&mut self, terminal: &mut Terminal, direction: Direction) {
        terminal.eraseBox(&self.position);
        match direction {
            Direction::Up => {self.position.moveUp();},
            Direction::Down => {self.position.moveDown();},
            Direction::Left => {self.position.moveLeft();},
            Direction::Right => {self.position.moveRight();},
            Direction::None => (),
        }
    }

    pub fn update(&mut self, terminal: &mut Terminal) {
        self.weapon.updateBullets(terminal);
        self.draw(terminal);
    }

    pub fn draw(&self, terminal: &mut Terminal) {
        terminal.drawBox(&self.position, self.getColor())
    }

    pub fn getPosition(&self) -> &Position {
        &self.position
    }

    pub fn getColor(&self) -> &dyn Color {
        self.color.as_ref()
    }

    pub fn getWeapon(&mut self) -> &mut Weapon {
        &mut self.weapon
    }
}