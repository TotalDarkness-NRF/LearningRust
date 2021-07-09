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
        Position {x:1, y:2}
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