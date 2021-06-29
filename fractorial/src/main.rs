#![allow(non_snake_case)] // stop warning for not using snake case
use std::io;
fn main() {
    println!("Insert a number to calculate the fractorial");
    let mut number =String::new();

    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("The fractorial of {0} is {1}", number, fractorial(number));
}

fn fractorial(n: u32) -> u32 {
    fractorialRecursive(1, n)
}

// I would normally use function overloading here but that is not a good idea for rust.

fn fractorialRecursive(x: u32, n: u32) -> u32 {
    if x >= n || x < 1 {return x};
    x * fractorialRecursive(x + 1, n)
}