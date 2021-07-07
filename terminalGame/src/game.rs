use std::io::{stdin, Write};

use rand::{prelude::ThreadRng, thread_rng};
use termion::{clear, color, cursor, event::Key, input::TermRead, screen};

use crate::{character::Character, position::Position, terminal::Terminal};

pub struct Game {
    character: Character,
    points: u16,
    terminal: Terminal,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Game {
            character: Character::create(Position::newOrigin()),
            points: 0,
            terminal: Terminal::getRaw(),
            rng: thread_rng(),
        }
    }

    pub fn start(&mut self) {
        let stdin = stdin();
        write!(self.terminal.terminal, "{}{}{}", screen::ToAlternateScreen, clear::All, cursor::Hide).unwrap();
        for c in stdin.keys() {
            self.terminal.write(format!(
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            ));

            match c.unwrap() {
                /* 
                Key::Char('q') => break,
                Key::Char(c) => println!("{}", c),
                Key::Alt(c) => println!("^{}", c),
                Key::Ctrl(c) => println!("*{}", c),
                Key::Esc => println!("ESC"),
                Key::Left => println!("←"),
                Key::Right => println!("→"),
                Key::Up => println!("↑"),
                Key::Down => println!("↓"),
                Key::Backspace => println!("×"),
                */
                Key::Char('q') => break,
                Key::Char('w') => self.character.moveUp(),
                Key::Char('a') => self.character.moveRight(),
                Key::Char('s') => self.character.moveDown(),
                Key::Char('d') => self.character.moveLeft(),
                _ => {}
            }

            self.terminal.drawBox(&self.character.position, &color::Green);
            self.terminal.terminal.flush().unwrap();
        }
    }
}
