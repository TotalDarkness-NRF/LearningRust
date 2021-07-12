use termion::event::Key;

pub struct Controls {
    pub quit: Key,
    pub reset: Key,
    pub up: Key,
    pub left: Key,
    pub down: Key,
    pub right: Key,
    pub attackUp: Key,
    pub attackLeft: Key,
    pub attackDown: Key,
    pub attackRight: Key,
}

impl Controls {
    pub fn new() -> Self {
        Controls {
            quit: Key::Char('q'),
            reset: Key::Char('R'),
            up: Key::Char('w'),
            left: Key::Char('a'),
            down: Key::Char('s'),
            right: Key::Char('d'),
            attackUp: Key::Ctrl('w'),
            attackLeft: Key::Ctrl('a'),
            attackDown: Key::Ctrl('s'),
            attackRight: Key::Ctrl('d'), 
        }
    }
}