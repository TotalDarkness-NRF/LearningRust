/*
From https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ownership-rules
Ownership Rules
   Each value in Rust has a variable that’s called its owner.
   There can only be one owner at a time.
   When the owner goes out of scope, the value will be dropped.
 */
 pub fn ownership() {
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