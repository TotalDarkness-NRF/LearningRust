use termion::color::Color;
use crate::{character::Character, position::{Direction, Position}, terminal::Terminal, weapon::Weapon};

pub struct Player {
    position: Position,
    health: u8,
    weapon: Weapon,
    color: Box<dyn Color>,
}

impl ToString for Player {
    fn to_string(&self) -> String {
        format!("x:{} y:{} health:{} {}",
        self.position.getX(), self.position.getY(), self.health, self.weapon.to_string())
    }
}

impl Character for Player {
    fn moves(&mut self, terminal: &mut Terminal, direction: Direction) {
        terminal.eraseBox(&self.position); // TODO see if we can have this somewhere else
        match direction {
            Direction::Up => {self.position.moveUp();},
            Direction::Down => {self.position.moveDown();},
            Direction::Left => {self.position.moveLeft();},
            Direction::Right => {self.position.moveRight();},
            Direction::None => (),
        }
    }

    fn update(&mut self, terminal: &mut Terminal) {
        self.weapon.updateBullets(terminal);
        self.draw(terminal);
    }

    fn draw(&self, terminal: &mut Terminal) {
        terminal.drawBox(&self.position, self.getColor())
    }

    fn getPosition(&self) -> &Position {
        &self.position
    }

    fn getColor(&self) -> &dyn Color {
        self.color.as_ref()
    }

    fn getWeapon(&mut self) -> &mut Weapon {
        &mut self.weapon
    }

    fn attack(&mut self, direction: Direction) {
        let position: Position = self.position.clone();
        self.getWeapon().shoot(direction, position)
    }
}

impl Player {
    pub fn create(position: Position, color: Box<dyn Color>) -> Self {
        Player{position, health: 20, weapon: Weapon::createPistol(), color}
    }
}