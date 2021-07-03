pub fn compoundTypes() {
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