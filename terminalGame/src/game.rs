use std::{process::exit, sync::mpsc::{Receiver, channel}, thread, time::Duration};

use rand::{prelude::ThreadRng, thread_rng};
use termion::{color, event::Key};

use crate::{
    character::Character,
    controls::Controls,
    player::Player,
    position::{Direction, Position},
    terminal::Terminal,
};

pub struct Game {
    character: Player,
    points: u16,
    terminal: Terminal,
    controls: Controls,
    rng: ThreadRng,
    events: Receiver<Key>,
}

impl Game {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        // Key inputs based off https://github.com/andrewhalle/termsnake
        // Have to make a channel and send key events over it so that we don't block the main loop (This next causes infinite loop)
        thread::spawn(move || {
            for key in Terminal::getKeys() {
                tx.send(key.unwrap()).unwrap();
            }
        });
        
        Game {
            character: Player::create(Position::newOrigin(), Box::new(color::LightRed)),
            points: 0,
            terminal: Terminal::getRaw(),
            controls: Controls::new(),
            rng: thread_rng(),
            events: rx,
        }
    }

    pub fn start(&mut self) {
        self.terminal.begin();
        self.gameloop();
    }

    fn gameloop(&mut self) {
        loop {
            let bound = Terminal::getBoundaries();
            self.terminal.write(format!(
                "{}{}{} {} {}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine,
                bound.getX(),
                bound.getY(),
                self.character.to_string()
            ));
            self.handleEvents();
            self.character.update(&mut self.terminal);
            self.terminal.flush();
            thread::sleep(Duration::from_millis(50)); // TODO see what we can do
        }
    }

    fn handleEvents(&mut self) {
        match self.events.try_recv() {
            Ok(key) => self.handleKey(key),
            Err(_) => {}
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
            self.character.attack(Direction::Up);
        } else if key == controls.attackLeft {
            self.character.attack(Direction::Left);
        } else if key == controls.attackDown {
            self.character.attack(Direction::Down);
        } else if key == controls.attackRight {
            self.character.attack(Direction::Right);
        }
    }

    fn quit(&mut self) {
        self.terminal.exit();
        exit(0);
    }
}
