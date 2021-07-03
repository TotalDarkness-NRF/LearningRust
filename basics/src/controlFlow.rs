pub fn ifelse() {
    let x = 10;
    let n = 20;
    // Does not require parenthesis
    if n > x {
        // requires blocks
        println!("The number is greater than {}", x);
    } else if n < x {
        println!("The number is less than {}", x);
    } else {
        println!("The number is {}", x);
    }

    let x = if x > 20 { "Yes" } else { "No" };
    // Use shadowing to change x type and use if else as expression
    println!("Is x greated than 20? A: {}", x);
}