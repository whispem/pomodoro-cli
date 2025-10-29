use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("=== Pomodoro Timer ===");
    println!("Enter your Pomodoro session duration (in minutes, e.g.: 25):");

    // Get user input
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input!");
    let minutes: u64 = input.trim().parse().expect("Please enter a valid number!");

    let seconds = minutes * 60;

    println!("\nStarting Pomodoro for {} minutes!\n", minutes);
    for remaining in (1..=seconds).rev() {
        print!("\rTime left: {:02}:{:02}", remaining / 60, remaining % 60);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\n\n🍅 Pomodoro done! Take a break. 🎉");
}
