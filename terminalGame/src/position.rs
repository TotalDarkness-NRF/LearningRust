pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position {x, y}
    }
    pub fn newOrigin() -> Self {
        Position {x:1, y:1}
    }

    pub fn getX(&self) -> u16 {
        self.x
    }

    pub fn getY(&self) -> u16 {
        self.y
    }

    pub fn setX(&mut self, x: u16) {
        self.x = x;
    }

    pub fn setY(&mut self, y: u16) {
        self.y = y;
    }

    pub fn set(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}