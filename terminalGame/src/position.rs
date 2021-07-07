use crate::terminal::isInBoundary;

pub struct Position {
    x: u16,
    y: u16,
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

    pub fn setX(&mut self, x: u16) {
        if self.respectBoundary(x) && isInBoundary(&Position::new(x, self.y)) {
            self.x = x;
        }
    }

    pub fn setY(&mut self, y: u16) {
        if self.respectBoundary(y - 1) && isInBoundary(&Position::new(self.x, y)) {
            self.y = y;
        }
    }

    pub fn set(&mut self, x: u16, y: u16) {
        self.setX(x);
        self.setY(y);
    }

    fn respectBoundary(&self, n: u16) -> bool {
        isInBoundary(self) && n > 0
    }
}