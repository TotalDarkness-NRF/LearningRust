use std::{fs::File, io::Write};

use termion::{color::{self, Color}, cursor, get_tty, raw::{IntoRawMode, RawTerminal}, terminal_size};

use crate::position::Position;

pub struct Terminal {
    pub terminal: RawTerminal<File>,
}

impl Terminal {
    pub fn getRaw() -> Self {
        Terminal {
            terminal: get_tty().unwrap().into_raw_mode().unwrap(),
        }
    }

    pub fn write(&mut self, message: String) {
        write!(self.terminal, "{}", message).unwrap();
    }

    pub fn drawBox(&mut self, pos: &Position, color: &dyn Color) {
        self.restoreCursorWrite(pos, bgColor(color));
    }

    pub fn drawChar(&mut self, pos: &Position, character: char) {
        self.restoreCursorWrite(pos, String::from(character));
    }

    pub fn eraseBox(&mut self, pos: &Position) {
        self.restoreCursorWrite(pos, "".to_string());
    }
    
    fn restoreCursorWrite(&mut self, pos: &Position, message: String) {
        if isInBoundary(pos) {
            self.write(
        format!(
                "{}{}{} {}",
                cursor::Save,
                cursor::Goto(pos.getX(), pos.getY()),
                message,
                cursor::Restore
                )
            );
        }
    }

}

fn bgColor(color: &dyn color::Color) -> String {
    format!("{}", color::Bg(color))
}

pub fn getBoundaries() -> Position {
    let (x, y) = terminal_size().unwrap();
    Position::new(x, y)
}

pub fn isInBoundary(pos: &Position) -> bool {
    // When both are zero the program will panic
    pos.getX() > 0 && pos.getY() > 0 && {
        let boundary = getBoundaries();
        pos.getX() < boundary.getX() && pos.getY() < boundary.getY()
    }
}
