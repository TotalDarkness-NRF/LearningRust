#![allow(non_snake_case)] // stop warning for not using snake case

use crate::game::Game;

mod game;
mod character;
mod terminal;
mod position;

// using https://docs.rs/termion/1.5.6/termion/

fn main() {
    let mut game = Game::new();
    game.start();
    //TODO clear character old once moved
    /*
    let mut terminal = Terminal::getRaw();
    let mut rng = rand::thread_rng();
    write!(terminal.terminal, "{}{}{}", screen::ToAlternateScreen, clear::All, cursor::Hide).unwrap();
    write!(terminal.terminal, "{}Red{}\n\r", color::Fg(color::Red), color::Fg(color::Reset)).unwrap();
    let boundary = terminal.getBoundaries();
    let mut x = rng.gen_range(1..boundary.getX());
    let mut y = rng.gen_range(1..boundary.getY());
    println!("{} {}", boundary.getX(), boundary.getY());
    let mut coords = TerminalCoordiante::new(x, y);
    terminal.drawBox(&coords, &color::Green);
    x = rng.gen_range(1..boundary.getX());
    y = rng.gen_range(1..boundary.getY());
    coords.set(x, y);
    terminal.drawBox(&coords, &color::Magenta);
    //terminal.eraseBox(&TerminalCoordiante::new(20, 10));
     */
}