use crate::{character::Character, position::{Direction, Position}, terminal::Terminal, weapon::Weapon};
use rand::{Rng, thread_rng};
use termion::color::{self, Color};

pub struct Enemy {
    health: u8,
    position: Position,
    weapon: Weapon,
    color: Box<dyn Color>,
}

impl Character for Enemy {
    fn moves(&mut self, terminal: &mut Terminal, direction: Direction) {
        todo!()
    }

    fn update(&mut self, terminal: &mut Terminal) {
        
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
        todo!()
    }
}

impl Enemy {
    pub fn new() -> Self {
        let rng = thread_rng().gen_range(0.0..=100.0);
        if rng < 50.0 { EnemyType::human() } 
        else if rng < 75.0 { EnemyType::elf() }
        else if rng < 95.0 { EnemyType::machine() }
        else { EnemyType::alien() }
    }
}

enum EnemyType {
    Alien,
    Elf,
    Human,
    Machine,
}

impl EnemyType {
    pub fn human() -> Enemy {
        Enemy {
            health: 20,
            position: Position::newRandom(),
            weapon: Weapon::createShotgun(),
            color: Box::new(color::Yellow),
        }
    }

    pub fn alien() -> Enemy {
        Enemy {
            health: 30,
            position: Position::newRandom(),
            weapon: Weapon::createPistol(),
            color: Box::new(color::LightGreen),
        }
    }

    pub fn elf() -> Enemy {
        Enemy {
            health: 30,
            position: Position::newRandom(),
            weapon: Weapon::createBow(),
            color: Box::new(color::Blue),
        }
    }
    
    pub fn machine() -> Enemy {
        Enemy {
            health: 20,
            position: Position::newRandom(),
            weapon: Weapon::createShotgun(),
            color: Box::new(color::AnsiValue::grayscale(5)),
        }
    }
}