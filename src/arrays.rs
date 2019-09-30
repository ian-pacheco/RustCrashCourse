// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut number: [i32; 5] = [1, 2, 3, 4, 5];

    // Re assing value
    number[2] = 20;

    println!("{:?}", number);

    // Get single value
    println!("Single value {}", number[0]);

    // Get array length
    println!("Array length: {}", number.len());

    // Arrays are stack allocated
    println!("Arrays occupaies {} bytes", mem::size_of_val(&number));

    // Get Slice
    let slice: &[i32] = &number[0..2];

    println!("Slice {:?}", slice);
}
