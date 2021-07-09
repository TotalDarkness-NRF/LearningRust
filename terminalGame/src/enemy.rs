use termion::color::Color;

use crate::{character::Character, position::Position, terminal::Terminal, weapon::Weapon};

pub struct Enemy {
    position: Position,
    health: u8,
    weapon: Weapon,
    color: Box<dyn Color>,
}

impl Character for Enemy {
    fn moves(&mut self, terminal: &mut crate::terminal::Terminal, direction: crate::position::Direction) {
        todo!()
    }

    fn update(&mut self, terminal: &mut crate::terminal::Terminal) {
        todo!()
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

    fn attack(&mut self, terminal: &mut Terminal, direction: crate::position::Direction) {
        todo!()
    }
}