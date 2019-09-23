// Conditionals - Used to check the condition of something and act like it

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What do you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If(ternario)
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
