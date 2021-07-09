use std::{fs::File, io::Write};

use termion::{clear, color::{self, Color}, cursor, get_tty, raw::{IntoRawMode, RawTerminal}, screen, terminal_size};

use crate::position::Position;

pub struct Terminal {
    terminal: RawTerminal<File>,
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
        self.restoreCursorWrite(pos, format!("{} ", color::Bg(color)));
    }

    pub fn drawChar(&mut self, pos: &Position, character: char) {
        self.restoreCursorWrite(pos, String::from(character));
    }

    pub fn eraseBox(&mut self, position: &Position) {
        self.restoreCursorWrite(position, " ".to_string());
    }
    
    pub fn restoreCursorWrite(&mut self, pos: &Position, message: String) {
        if isInBoundary(pos) {
            self.write(
        format!(
                "{}{}{}",
                cursor::Save,
                cursor::Goto(pos.getX(), pos.getY()),
                message
                )
            );
            self.write(cursor::Restore.to_string());
        }
    }
    
    pub fn flush(&mut self) {
        self.terminal.flush().unwrap();
    }

    pub fn begin(&mut self) {
        self.write(format!(
            "{}{}{}",
            screen::ToAlternateScreen,
            clear::All,
            cursor::Hide
        ));
    }

    pub fn exit(&mut self) {
        self.write(format!("{}{}", cursor::Show, screen::ToMainScreen));
        self.terminal.suspend_raw_mode().unwrap();
    }

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
