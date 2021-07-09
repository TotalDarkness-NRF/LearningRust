use termion::color::Color;
use crate::{position::{Direction, Position}, terminal::Terminal, weapon::Weapon};

pub trait Character {
    fn moves(&mut self, terminal: &mut Terminal, direction: Direction);
    fn update(&mut self, terminal: &mut Terminal);
    fn draw(&self, terminal: &mut Terminal);
    fn attack(&mut self, terminal: &mut Terminal, direction: Direction);
    fn getPosition(&self) -> &Position;
    fn getColor(&self) -> &dyn Color;
    fn getWeapon(&mut self) -> &mut Weapon;
    // TODO add health and damage
}