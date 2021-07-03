pub fn variables() {
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