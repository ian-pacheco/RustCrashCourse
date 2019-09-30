// tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Ian", "SJC", 29);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // Destructuring pattern, breaks a tuple in separated values
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The values are {}, {}, {}", x.0, x.1, x.2);
}
