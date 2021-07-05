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

    let (fibbonnaciNumber, sequence) = fibbonnaci(number);

    println!("{}", fibbonnaciNumber);
    if showSequence { println!("{:?}", sequence) };
}

fn fibbonnaci(n: u32) -> (u128, Vec<u128>) {
    fibbonnaciRecursive(0, 1, 1, n,  &mut Vec::new())
}

fn fibbonnaciRecursive(x: u128, y: u128, count: u32, n: u32, sequence: &mut Vec<u128>) -> (u128,  Vec<u128>) {
    if count == n {
        sequence.push(x);
        return (x, sequence.to_vec());
    }
    sequence.push(x);
    fibbonnaciRecursive(y, x + y, count + 1, n, sequence)
}