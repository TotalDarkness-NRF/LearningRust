#![allow(non_snake_case)] // stop warning for not using snake case

use termion::{clear, color, cursor, get_tty, raw::{IntoRawMode, RawTerminal}, screen, terminal_size};
use std::{fs::File, io::Write};

// using https://docs.rs/termion/1.5.6/termion/

fn main() {
    let mut terminal = Terminal::getRaw();
    write!(terminal.terminal, "{}{}{}", screen::ToAlternateScreen, clear::All, cursor::Hide).unwrap();
    write!(terminal.terminal, "{}Red{}\n\r", color::Fg(color::Red), color::Fg(color::Reset)).unwrap();
    let boundary = terminal.getBoundaries();
    println!("{} {}", boundary.getX(), boundary.getY());
    let mut coords = TerminalCoordiante::new(20, 10);
    terminal.drawBox(&coords, &color::Green);
    coords.set(20, 3);
    terminal.drawBox(&coords, &color::Magenta);
    terminal.eraseBox(&TerminalCoordiante::new(20, 10));
}

struct TerminalCoordiante {
    x: u16,
    y: u16,
}

impl TerminalCoordiante {
    pub fn new(x: u16, y: u16) -> Self {
        TerminalCoordiante {x, y}
    }
    pub fn newOrigin() -> Self {
        TerminalCoordiante {x:0, y:0}
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

struct Terminal {
    terminal: RawTerminal<File>
}

impl Terminal {
    pub fn new(terminal: RawTerminal<File>) -> Self {
        Terminal{terminal}
    }

    pub fn getRaw() -> Self {
        Terminal{terminal: get_tty().unwrap().into_raw_mode().unwrap()}
    }

    fn write(&mut self, message: String) {
        write!(self.terminal, "{}", message).unwrap();
    }

    pub fn drawBox(&mut self, pos: &TerminalCoordiante, color: &dyn color::Color) {
        self.write(format!(
            "{}{}{} {}",
            cursor::Save,
            cursor::Goto(pos.getX(), pos.getY()),
            color::Bg(color),
            cursor::Restore)
        );
    }

    pub fn eraseBox(&mut self, pos: &TerminalCoordiante) {
        self.write(format!("{} ", cursor::Goto(pos.getX(), pos.getY())));
    }

    pub fn getBoundaries(&self) -> TerminalCoordiante {
        let (x, y) = terminal_size().unwrap();
        TerminalCoordiante::new(x, y)
    }
}