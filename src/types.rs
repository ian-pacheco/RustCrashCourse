/*
Primitive types--
(number od bits they take in memory)
Intergers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
characters: char
Tuples
Arrays
*/

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4545445454545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean

    let is_active = true;

    // Get bool from expression
    let is_greater = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
