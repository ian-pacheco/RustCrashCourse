/*
Primitive types--
signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
floating point: f32, f64
char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool either true or false
and the unit type (), whose only possible value is an empty tuple: ()
arrays like [1, 2, 3]
tuples like (1, true)
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

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("{:?}", (sum, difference, product, quotient, remainder));

}
