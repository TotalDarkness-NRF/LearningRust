pub fn structs() {
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