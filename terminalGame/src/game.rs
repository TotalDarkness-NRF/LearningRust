use std::io::{stdin, Write};

use rand::{prelude::ThreadRng, thread_rng};
use termion::{clear, color, cursor, event::Key, input::TermRead, screen};

use crate::{character::Character, position::{Position, Direction}, terminal::{Terminal, getBoundaries}};

pub struct Game {
    character: Character,
    points: u16,
    terminal: Terminal,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Game {
            character: Character::create(Position::newOrigin(), Box::new(color::LightRed)),
            points: 0,
            terminal: Terminal::getRaw(),
            rng: thread_rng(),
        }
    }

    pub fn start(&mut self) {
        let stdin = stdin();
        self.terminal.write( format!("{}{}{}", screen::ToAlternateScreen, clear::All, cursor::Hide));
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
                Key::Char('q') => {self.quit(); break},
                Key::Char('w') => self.character.moves(&mut self.terminal, Direction::Up),
                Key::Char('a') => self.character.moves(&mut self.terminal, Direction::Right),
                Key::Char('s') => self.character.moves(&mut self.terminal, Direction::Down),
                Key::Char('d') => self.character.moves(&mut self.terminal, Direction::Left),
                Key::Ctrl('w') => {
                    let position: Position = Position::copy(self.character.getPosition());
                    self.character.getWeapon().shoot(&mut self.terminal, Direction::Up, position);
                },
                Key::Ctrl('a') => {
                    let position: Position = Position::copy(self.character.getPosition());
                    self.character.getWeapon().shoot(&mut self.terminal, Direction::Right, position);
                },
                Key::Ctrl('s') => {
                    let position: Position = Position::copy(self.character.getPosition());
                    self.character.getWeapon().shoot(&mut self.terminal, Direction::Down, position);
                },
                Key::Ctrl('d') => {
                    let position: Position = Position::copy(self.character.getPosition());
                    self.character.getWeapon().shoot(&mut self.terminal, Direction::Left, position);
                },
                _ => {}
            }

            let bound = getBoundaries();
            self.terminal.write(format!("{} {} {} {}", 
            self.character.getPosition().getX(), self.character.getPosition().getY(),
            bound.getX(), bound.getY()));
            self.terminal.drawBox(&self.character.getPosition(), self.character.getColor());
            self.terminal.terminal.flush().unwrap();
        }
    }

    fn quit(&mut self) {
        self.terminal.write(format!("{}{}", cursor::Show, screen::ToMainScreen));
        self.terminal.terminal.suspend_raw_mode().unwrap();
    }
}