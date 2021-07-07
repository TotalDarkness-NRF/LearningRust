use std::{fs::File, io::Write};

use termion::{color, cursor, get_tty, raw::{IntoRawMode, RawTerminal}, terminal_size};

use crate::position::Position;

pub struct Terminal {
    pub terminal: RawTerminal<File>
}

impl Terminal {
    pub fn new(terminal: RawTerminal<File>) -> Self {
        Terminal{terminal}
    }

    pub fn getRaw() -> Self {
        Terminal{terminal: get_tty().unwrap().into_raw_mode().unwrap()}
    }

    pub fn write(&mut self, message: String) {
        write!(self.terminal, "{}", message).unwrap();
    }

    pub fn drawBox(&mut self, pos: &Position, color: &dyn color::Color) {
        if self.isInBoundary(pos) {
            self.write(format!(
                "{}{}{} {}",
                cursor::Save,
                cursor::Goto(pos.getX(), pos.getY()),
                color::Bg(color),
                cursor::Restore)
            );
        }
    }

    pub fn eraseBox(&mut self, pos: &Position) {
        if self.isInBoundary(pos) {
            self.write(format!("{} ", cursor::Goto(pos.getX(), pos.getY())));
        }   
    }

    pub fn getBoundaries(&self) -> Position {
        let (x, y) = terminal_size().unwrap();
        Position::new(x, y)
    }

    pub fn isInBoundary(&self, pos: &Position) -> bool {
        // When both are zero the program will panic
        !(pos.getX() == 0 && pos.getY() == 0) && {
            let boundary = self.getBoundaries();
            boundary.getX() >= pos.getX() && boundary.getY() >= pos.getY()
        }
    }
}