#![allow(non_snake_case)] // stop warning for not using snake case
use std::io;
use bigint::U512;
fn main() {
    println!("Insert a number to calculate the fractorial");
    let mut number =String::new();

    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("The fractorial of {0} is {1}", number, fractorial(number));
}

fn fractorial(n: u32) -> U512 {
    fractorialRecursive(1.into(), n)
}

// I would normally use function overloading here but that is not a good idea for rust.
/*
For fractorials numbers can become very large fast. Around 12 times it overflows
Using u128 over u32 fixes now overflows above 34
Using a crate called bigint (4.2.0) allows me to use U512 and go to 98 until overflow
*/ 
fn fractorialRecursive(x: U512, n: u32) -> U512 {
    if x >= n.into() || x < 1.into() {return x};
    x * fractorialRecursive(x + 1.into(), n)
}