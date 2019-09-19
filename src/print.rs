pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Print int
    println!("Number: {}", 1);

    // Print basic formatting
    println!("{} is from {}", "Ian", "BRA");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Ian", "BRA", "code");

    // Named Args
    println!(
        "{name} likes to play {activity}",
        name = "Ian",
        activity = "Basketball"
    );

    // Placeholder traits
    println!("binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 +10);
}
