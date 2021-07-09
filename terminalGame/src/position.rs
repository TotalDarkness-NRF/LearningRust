use crate::terminal::isInBoundary;

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
        if self.respectBoundary(x)
        && self.respectBoundary(y - 1)
        && isInBoundary(&Position::new(x, y)) {
            self.x = x;
            self.y = y;
            return true;
        }
        false
    }

    fn respectBoundary(&self, n: u16) -> bool {
        isInBoundary(self) && n > 0
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}