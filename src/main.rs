use std::io::{self, Write};
mod tracker;
mod contribution;

use tracker::Tracker;

fn main() {
    let mut tracker = Tracker::new();

    loop {
        println!("\nOpen Source Contribution Tracker");
        println!("1. Add Contribution");
        println!("2. List Contributions");
        println!("3. Exit");
        print!("Please select an option (1-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        match input {
            1 => {
                println!("\nEnter GitHub Username:");
                let mut username = String::new();
                io::stdin().read_line(&mut username).expect("Failed to read username");

                println!("Enter Project Name:");
                let mut project_name = String::new();
                io::stdin().read_line(&mut project_name).expect("Failed to read project name");

                println!("Enter Contribution Type (e.g., bug fix, documentation):");
                let mut contribution_type = String::new();
                io::stdin().read_line(&mut contribution_type).expect("Failed to read contribution type");

                tracker.add_contribution(
                    username.trim(),
                    project_name.trim(),
                    contribution_type.trim(),
                );
                println!("Contribution added successfully.");
            }
            2 => tracker.list_contributions(),
            3 => {
                println!("Saving contributions and exiting...");
                tracker.save_contributions().unwrap();
                break;
            }
            _ => println!("Invalid option. Please enter a number between 1 and 3."),
        }
    }
}
