use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Contribution {
    username: String,
    repo: String,
    commits: u32,
}

struct TrackerApp {
    contributions: Vec<Contribution>,
}

impl TrackerApp {
    fn new() -> Self {
        TrackerApp {
            contributions: Vec::new(),
        }
    }

    fn load_from_file(&mut self) {
        if let Ok(contents) = fs::read_to_string("contributions.json") {
            let contributions: Vec<Contribution> = serde_json::from_str(&contents).unwrap_or_default();
            self.contributions = contributions;
        }
    }

    fn save_to_file(&self) {
        let contents = serde_json::to_string(&self.contributions).expect("Error serializing data");
        fs::write("contributions.json", contents).expect("Error writing to file");
    }

    fn add_contribution(&mut self, username: String, repo: String, commits: u32) {
        let contribution = Contribution {
            username,
            repo,
            commits,
        };
        self.contributions.push(contribution);
    }

    fn list_contributions(&self) {
        if self.contributions.is_empty() {
            println!("No contributions recorded yet.");
        } else {
            for (i, contribution) in self.contributions.iter().enumerate() {
                println!(
                    "{}. {} - {} ({} commits)",
                    i + 1,
                    contribution.username,
                    contribution.repo,
                    contribution.commits
                );
            }
        }
    }

    fn get_u32_input(&self, prompt: &str) -> u32 {
        loop {
            println!("{}", prompt);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<u32>() {
                Ok(value) => return value,
                Err(_) => println!("Invalid input. Please enter a valid number."),
            }
        }
    }

    fn get_string_input(&self, prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
}

fn main() {
    let mut tracker = TrackerApp::new();
    tracker.load_from_file();

    loop {
        println!("\nOpen Source Contribution Tracker");
        println!("1. Add Contribution");
        println!("2. List Contributions");
        println!("3. Exit");

        let choice = tracker.get_u32_input("Please select an option (1-3):");

        match choice {
            1 => {
                // Add Contribution
                let username = tracker.get_string_input("Enter GitHub Username:");
                let repo = tracker.get_string_input("Enter Repository Name:");
                let commits = tracker.get_u32_input("Enter Number of Commits:");

                tracker.add_contribution(username, repo, commits);
                tracker.save_to_file();
                println!("Contribution added!");
            }
            2 => {
                // List Contributions
                tracker.list_contributions();
            }
            3 => {
                // Exit
                println!("Exiting... Bye!");
                tracker.save_to_file();
                break;
            }
            _ => {
                println!("Please enter a valid option.");
            }
        }
    }
}
