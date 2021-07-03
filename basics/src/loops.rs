pub fn loops() {
    // The infinite loop
    loop {
        break; // use this to break out of the loop
    }
    // The while loop.
    let mut condition: bool = true;
    while condition {
        // Keep looping until conditon is not true
        condition = false;
    }
    // Loops can be used as expressions
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];
    // For loops are more applicable and can easily go through a collection
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}