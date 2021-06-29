#![allow(non_snake_case)] // stop warning for not using snake case
fn main() {
    variables();
    ifelse();
    compoundTypes();
}

fn variables() {
    let mut x = 45; 
    /*
    let creates a variable
    mut allows it to be mutable (changable later)
    x is the variable name
    : i32 sets the variable type (what is after the colon)
    Use https://doc.rust-lang.org/book/ch03-02-data-types.html 
    to learn more types
     */
     println!("Variable x is {}", x);
     x = 20;
     println!("Variable x is {}", x);
}

fn ifelse() {
    let x = 10;
    let n = 20;
    // Does not require parenthesis
    if n > x { // requires blocks
        println!("The number is greater than {}", x);
    } else if n < x {
        println!("The number is less than {}", x);
    } else {
        println!("The number is {}", x);
    }

    let x = if x > 20 {"Yes"} else {"No"}; 
    // Use shadowing to change x type and use if else as expression
    println!("Is x greated than 20? A: {}", x);
}

fn compoundTypes() {
    // tuple and arrays are immutable
    let tupple: (i32, f64, u32) = (500, 6.4, 99); //create a tuple, we can specify individual element types
    let (x, y, z) = tupple; // Destructure a tuple and place the respective elemnts.
    println!("Tupple elements: 1:{0} 2:{1} 3:{2}", tupple.0, tupple.1, tupple.2); // access tuple elements
    println!("x:{0} y:{1}, z:{2}", x, y, z);

    //let array = [1, 2, 3, 4, 5]; // create without specifying type or size
    //let array: [i32; 5] = [1, 1, 3, 4, 5]; // specify type and size (first type then semi colon and size)
    let array = [5;3]; // create an array of 3 elements with only the element 5
    println!("Array elements: 1:{0} 2:{1} 3:{2}", array[0], array[1], array[2]);
}

/*
functions are declared with fn and wrapped in {}
They can either be a statment or expression. A statment returns nothing while expression returns a value
Paramenters are declared in the brackets using variable: type.
Return values are specified using -> type after the brackets and before the {}
A final return does not require a semi colon. Use this if its the final return
 */