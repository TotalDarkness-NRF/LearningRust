use std::io::{stdin, Write};

use rand::{prelude::ThreadRng, thread_rng};
use termion::{clear, color, cursor, event::Key, input::TermRead, screen};

use crate::{character::Character, position::Position, terminal::{Terminal, getBoundaries}};

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
                Key::Char('w') => self.moveCharacter(Direction::Up),
                Key::Char('a') => self.moveCharacter(Direction::Right),
                Key::Char('s') => self.moveCharacter(Direction::Down),
                Key::Char('d') => self.moveCharacter(Direction::Left),
                _ => {}
            }

            let bound = getBoundaries();
            self.terminal.write(format!("{} {} {} {}", 
            self.character.position.getX(), self.character.position.getY(),
            bound.getX(), bound.getY()));
            self.terminal.drawBox(&self.character.position, &color::Green);
            self.terminal.terminal.flush().unwrap();
        }
    }

    fn moveCharacter(&mut self, direction: Direction) {
        self.terminal.eraseBox(&self.character.position);
        match direction {
            Direction::Up => self.character.moveUp(),
            Direction::Down => self.character.moveDown(),
            Direction::Left => self.character.moveLeft(),
            Direction::Right => self.character.moveRight(),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}