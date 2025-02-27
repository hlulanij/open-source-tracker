use std::io;
use crate::models::Contribution;
use crate::tracker::Tracker;

pub fn run_cli() {
    let mut tracker = Tracker::new();
    loop {
        println!("\nOpen Source Contribution Tracker");
        println!("1. Add Contribution");
        println!("2. List Contributions");
        println!("3. Exit");
        print!("Choose an option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter username: ");
                let mut username = String::new();
                io::stdin().read_line(&mut username).expect("Failed to read input");

                println!("Enter repository name: ");
                let mut repo = String::new();
                io::stdin().read_line(&mut repo).expect("Failed to read input");

                println!("Enter number of commits: ");
                let mut commits = String::new();
                io::stdin().read_line(&mut commits).expect("Failed to read input");

                let commits: u32 = match commits.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number. Try again.");
                        continue;
                    }
                };

                let contribution = Contribution::new(username.trim(), repo.trim(), commits);
                tracker.add_contribution(contribution);
                println!("Contribution added!");
            }
            "2" => {
                println!("\nListing Contributions:");
                tracker.list_contributions();
            }
            "3" => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
