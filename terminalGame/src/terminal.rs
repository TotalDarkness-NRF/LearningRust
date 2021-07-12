use std::{fs::File, io::Write};

use termion::{clear, color::{self, Color}, cursor, get_tty, input::{Keys, TermRead}, raw::{IntoRawMode, RawTerminal}, screen, terminal_size};

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

    pub fn getKeys() -> Keys<File>{
        get_tty().unwrap().keys()
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
        if pos.isInBoundary() {
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

    pub fn clearAll(&mut self) {
        self.write(format!("{}{}", clear::All, cursor::Goto::default()));
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

    pub fn getBoundaries() -> Position {
        let (x, y) = terminal_size().unwrap();
        Position::new(x, y)
    }
}