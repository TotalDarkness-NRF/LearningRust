use std::{process::exit, sync::mpsc::{Receiver, channel}, thread, time::Duration};
use termion::{color, event::Key};

use crate::{character::Character, controls::Controls, enemy::Enemy, player::Player, position::{Direction, Position}, terminal::Terminal};

pub struct Game {
    player: Player,
    points: u16,
    terminal: Terminal,
    controls: Controls,
    events: Receiver<Key>,
}

impl Game {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        // Key inputs based off https://github.com/andrewhalle/termsnake
        // Have to make a channel and send key events over it so that we don't block the main loop (The next() causes infinite loop)
        thread::spawn(move || {
            for key in Terminal::getKeys() {
                tx.send(key.unwrap()).unwrap();
            }
        });
        
        Game {
            player: Player::create(Position::newOrigin(), Box::new(color::LightRed)),
            points: 0,
            terminal: Terminal::getRaw(),
            controls: Controls::new(),
            events: rx,
        }
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
                self.player.to_string()
            ));
            self.handleEvents();
            self.player.update(&mut self.terminal);
            self.terminal.flush();
            thread::sleep(Duration::from_millis(10)); // TODO see what we can do
        }
    }

    fn handleEvents(&mut self) {
        if let Ok(key) = self.events.try_recv() {
            self.handleKey(key);
        }
    }

    fn handleKey(&mut self, key: Key) {
        let controls: &Controls = &self.controls;
        if key == controls.quit {
            self.quit();
        } else if key == controls.reset {
            self.reset();
        } else {
            let player = &mut self.player;
            if key == controls.up {
                player.moves(&mut self.terminal, Direction::Up);
            } else if key == controls.left {
                player.moves(&mut self.terminal, Direction::Left);
            } else if key == controls.down {
                player.moves(&mut self.terminal, Direction::Down);
            } else if key == controls.right {
                player.moves(&mut self.terminal, Direction::Right);
            } else if key == controls.attackUp {
                player.attack(Direction::Up);
            } else if key == controls.attackLeft {
                player.attack(Direction::Left);
            } else if key == controls.attackDown {
                player.attack(Direction::Down);
            } else if key == controls.attackRight {
                player.attack(Direction::Right);
            } else if key == Key::Char('r') {
                Enemy::new().draw(&mut self.terminal); // TODO remove later just for testing
            }
        }
    }

    pub fn start(&mut self) {
        self.terminal.begin();
        self.gameloop();
    }

    fn reset(&mut self) {
        self.points = 0;
        self.player = Player::create(Position::newOrigin(), Box::new(color::LightRed));
        self.terminal.clearAll();
    }



    fn quit(&mut self) {
        self.terminal.exit();
        exit(0);
    }
}