#![allow(non_snake_case)] // stop warning for not using snake case
fn main() {
    println!("{}", fractorial(10));
}

fn fractorial(n: u32) -> u32 {
    fractorialRecursive(1, n)
}

fn fractorialRecursive(x: u32, n: u32) -> u32 {
    if x >= n || x < 1 {return x};
    x * fractorialRecursive(x + 1, n)
}
