use rand::{Rng, thread_rng};

use crate::terminal::Terminal;

pub struct Position {
    x: u16,
    y: u16,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position::new(self.x, self.y)
    }
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position {x, y}
    }

    pub fn newOrigin() -> Self {
        Position::new(1, 2)
    }

    pub fn newRandom() -> Self {
        let mut rng = thread_rng();
        let bounds = Terminal::getBoundaries();
        let (x, y) = (rng.gen_range(1..bounds.getX()), rng.gen_range(2..bounds.getY()));
        Position::new(x, y)
    }

    pub fn getX(&self) -> u16 {
        self.x
    }

    pub fn getY(&self) -> u16 {
        self.y
    }

    pub fn moveUp(&mut self) -> bool {
        self.setY(self.y - 1)
    }

    pub fn moveDown(&mut self) -> bool {
        self.setY(self.y + 1)
    }

    pub fn moveRight(&mut self) -> bool {
        self.setX(self.x + 1)
    }

    pub fn moveLeft(&mut self) -> bool {
        self.setX(self.x - 1)
    }

    pub fn setX(&mut self, x: u16) -> bool {
        self.set(x, self.y)
    }

    pub fn setY(&mut self, y: u16) -> bool {
        self.set(self.x, y)
    }

    pub fn set(&mut self, x: u16, y: u16) -> bool {
        if self.respectBoundary(x, y) {
            self.x = x;
            self.y = y;
            return true;
        }
        false
    }

    fn respectBoundary(&self, x: u16, y: u16) -> bool {
        // Check current and future positions
        self.isInBoundary() && x > 0 && y > 1 && Position::new(x, y).isInBoundary()
    }

    pub fn isInBoundary(&self) -> bool {
        // When both are zero the program will panic
        self.x > 0 && self.y > 0 && {
            let boundary = Terminal::getBoundaries();
            self.x <= boundary.x && self.y <= boundary.y
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}