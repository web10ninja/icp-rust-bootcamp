use std::io;

fn main() {
    // 1. Prompt the user for their name
    println!("What is your name?");

    // 2. Read the user's input
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // 3. Print a personalized greeting
    let name = name.trim(); // Remove newline
    println!("Hello, {}!", name);

    // BONUS: Print the current date
    // Uncomment the following lines after adding `chrono = "0.4"` to Cargo.toml:
    use chrono::Local;
    println!("Today's date is: {}", Local::now().format("%Y-%m-%d"));
}