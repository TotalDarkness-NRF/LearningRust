#![allow(non_snake_case)] // stop warning for not using snake case

mod module;
use crate::module::anotherModule;

fn main() {
    variables();
    ifelse();
    compoundTypes();
    loops();
    ownership();
    structs();
    module::publicFunction();
    module::anotherModule::moduleFunction();
    anotherModule::moduleFunction();
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

fn compoundTypes() {
    // tuple and arrays are immutable
    let tupple: (i32, f64, u32) = (500, 6.4, 99); //create a tuple, we can specify individual element types
    let (x, y, z) = tupple; // Destructure a tuple and place the respective elemnts.
    println!(
        "Tupple elements: 1:{0} 2:{1} 3:{2}",
        tupple.0, tupple.1, tupple.2
    ); // access tuple elements
    println!("x:{0} y:{1}, z:{2}", x, y, z);

    //let array = [1, 2, 3, 4, 5]; // create without specifying type or size
    //let array: [i32; 5] = [1, 1, 3, 4, 5]; // specify type and size (first type then semi colon and size)
    let array = [5; 3]; // create an array of 3 elements with only the element 5
    println!(
        "Array elements: 1:{0} 2:{1} 3:{2}",
        array[0], array[1], array[2]
    );
}

/*
functions are declared with fn and wrapped in {}
They can either be a statment or expression. A statment returns nothing while expression returns a value
Paramenters are declared in the brackets using variable: type.
Return values are specified using -> type after the brackets and before the {}
A final return does not require a semi colon. Use this if its the final return
 */

fn loops() {
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

/*
From https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ownership-rules
Ownership Rules
   Each value in Rust has a variable that’s called its owner.
   There can only be one owner at a time.
   When the owner goes out of scope, the value will be dropped.
 */
fn ownership() {
    let _s = "Hello"; // Strin literal (Can not be mutated)
    let mut s = String::from("Hello"); // String (Can be mutated)
    s.push_str(" world");
    println!("{}", s);
    /*
    String literal and String deal with memory differnetly and allow for String to be mutable
    This is because string literal get hardcoded into the binary.
    */
    /*
    When a variable is out of scope the memory is already called to be "dropped"
    and will become available again. This allows rust to quickly deal with memory.
    Best of both worlds!
    No real need to deal with every bit of memory
    Ability to control memory if required
    */
    let x = String::from("String");
    //let y = x; // If we attempt this y becomes an invalid refrence. We would move x into y
    let y = x.clone(); // Instead use clone to make a new version of y (deep copy)
    println!("s1 = {}, s2 = {}", x, y);
    // This is not true for any primitive values. We can duplicate values just fine
    let n = 32;
    let z = n;
    println!("n1 = {}, n2 = {}", n, z);

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        //println!("{}", s);                   // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still to use x afterward
                   /*
                   Copy is a term that allows a value to be Copied and wont be Dropped on reassigning
                   Using this we can move around the ownership of pointer in memory and move it to different variables
                   If we do not to use this we can make refrences and borrow values
                   */
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Pass the pointer allow it to be borrowed and not take ownership

    println!("The length of '{}' is {}.", s1, len);

    /*
    By defualt we can not modify something we have refrenced to.
    We must add &mut before to make it mutable
    We can not have more than 1 refrence at a time of a variable
    We would need to change its scope using {}
    Refrences must always be valid! No dangling
     */
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn structs() {
    // # types of structs: classic, tuple and unit
    // You may initiate it using a random order
    let student1 = Student {
        name: String::from("Student1"),
        remote: true,
        level: 2,
    };
    // You may initiate it using normal order
    let student2 = Student {
        name: String::from("Student2"),
        level: 1,
        remote: false,
    };
    let marks1 = Grades('A', 'B', 'F', 'D', 0.0);
    let marks2 = Grades('A', 'A', 'A', 'B', 1.0);
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        student1.name, student1.level, student1.remote, marks1.0, marks1.1, marks1.2, marks1.3, marks1.4
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        student2.name, student2.level, student2.remote, marks2.0, marks2.1, marks2.2, marks2.3, marks2.4
    );

    /*
    These println are long and dumb. There is a way to make it easier. 
    Before the struct add #[derive(Debug)]
    In print use {:?} or {:#?} for pretty-print
    */
    println!("Normal debug print");
    println!("student1 is {0:?}, marks1 is {1:?}", student1, marks1);
    println!("student2 is {0:?}, marks2 is {1:?}", student2, marks2);
    println!("Pretty debug print");
    println!("student1 is {0:#?}, marks1 is {1:#?}", student1, marks1);
    println!("student2 is {0:#?}, marks2 is {1:#?}", student2, marks2);
}
// A clasic struct
#[derive(Debug)]
struct Student {
    name: String,
    level: u8,
    remote: bool,
}
// A tuple struct
#[derive(Debug)]
struct Grades(char, char, char, char, f32);
// Unit struct
//struct Unit;