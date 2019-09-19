// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ian";
    let mut age = 29;
    println!("My name is {} and I am {}yo", name, age);

    age = 30;
    println!("My name is {} and I am {}yo", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assing multiple vars
    let (my_name, my_age) = ("Ian", 29);
    println!("{} is {}", my_name, my_age);
}
