#![allow(non_snake_case)] // stop warning for not using snake case

use crate::game::Game;

mod game;
mod character;
mod terminal;
mod position;
mod weapon;
mod player;
mod controls;
mod enemy;

// using https://docs.rs/termion/1.5.6/termion/

fn main() {
    Game::new().start();
}