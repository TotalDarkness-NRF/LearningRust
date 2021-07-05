use std::io;

pub fn fibbonacciCalculate() {
    println!("Insert a number to calculate the fibbonnaci number");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");
    println!("Show full sequence? Type y or n");
    let mut showSequence: bool = false;
    
    loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        break match response.trim() {
            "y" => showSequence = true,
            "n" => (),
            _ => {
                println!("Please enter y or n");
                continue;
            }
        }
    }

    let sequence = fibbonnaci(number);
    let fibbonnaciNumber = sequence.last();
    
    
    if fibbonnaciNumber.is_some() { 
        println!("Fibbonnaci number {} is {}", number, fibbonnaciNumber.unwrap());
    }
    if showSequence { println!("{:?}", sequence); }
}

fn fibbonnaci(n: u32) -> Vec<u128> {
    fibbonnaciRecursive(0, 1, 1, n,  &mut Vec::new())
}

fn fibbonnaciRecursive(x: u128, y: u128, count: u32, n: u32, sequence: &mut Vec<u128>) -> Vec<u128> {
    sequence.push(x);
    if count == n {
        return sequence.to_vec();
    }
    fibbonnaciRecursive(y, x + y, count + 1, n, sequence)
}