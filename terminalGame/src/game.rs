use std::{io::stdin, process::exit};

use rand::{prelude::ThreadRng, thread_rng};
use termion::{color, event::Key, input::TermRead};

use crate::{
    character::Character,
    controls::Controls,
    player::Player,
    position::{Direction, Position},
    terminal::{getBoundaries, Terminal},
};

pub struct Game {
    character: Player,
    points: u16,
    terminal: Terminal,
    controls: Controls,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Game {
            character: Player::create(Position::newOrigin(), Box::new(color::LightRed)),
            points: 0,
            terminal: Terminal::getRaw(),
            controls: Controls::new(),
            rng: thread_rng(),
        }
    }

    pub fn start(&mut self) {
        let stdin = stdin();
        self.terminal.begin();
        // TODO the game only updates during key inputs
        for key in stdin.keys() {
            self.terminal.write(format!(
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            ));
            self.handleKey(key.unwrap());

            let bound = getBoundaries();
            self.terminal.write(format!(
                "{} {} {}",
                bound.getX(),
                bound.getY(),
                self.character.to_string()
            ));
            self.character.update(&mut self.terminal);
            self.terminal.flush();
        }
    }

    fn handleKey(&mut self, key: Key) {
        let controls: &Controls = &self.controls;
        if key == controls.quit {
            self.quit();
        } else if key == controls.up {
            self.character.moves(&mut self.terminal, Direction::Up);
        } else if key == controls.left {
            self.character.moves(&mut self.terminal, Direction::Left);
        } else if key == controls.down {
            self.character.moves(&mut self.terminal, Direction::Down);
        } else if key == controls.right {
            self.character.moves(&mut self.terminal, Direction::Right);
        } else if key == controls.attackUp {
            self.character.attack(&mut self.terminal, Direction::Up);
        } else if key == controls.attackLeft {
            self.character.attack(&mut self.terminal, Direction::Left);
        } else if key == controls.attackDown {
            self.character.attack(&mut self.terminal, Direction::Down);
        } else if key == controls.attackRight {
            self.character.attack(&mut self.terminal, Direction::Right);
        }
    }

    fn quit(&mut self) {
        self.terminal.exit();
        exit(0);
    }
}