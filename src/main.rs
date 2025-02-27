use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process;

#[derive(Clone)]
struct Contribution {
    username: String,
    repo: String,
    commits: u32,
}

fn main() {
    let mut contributions = load_contributions();

    loop {
        println!("\n📌 Open Source Contribution Tracker");
        println!("1️⃣ Add Contribution");
        println!("2️⃣ List Contributions");
        println!("3️⃣ Delete All Contributions");
        println!("4️⃣ Exit");
        print!("👉 Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                let contrib = add_contribution();
                contributions.push(contrib.clone());
                save_contribution(&contrib);
                println!("✅ Contribution saved!");
            }
            "2" => list_contributions(&contributions),
            "3" => delete_all_contributions(),
            "4" => {
                println!("👋 Exiting... Have a great day!");
                process::exit(0);
            }
            _ => println!("❌ Invalid choice. Please enter 1, 2, 3, or 4."),
        }
    }
}

fn add_contribution() -> Contribution {
    let username = get_input("📝 Enter GitHub username: ");
    let repo = get_input("📂 Enter repository name: ");
    let commits: u32 = loop {
        let input = get_input("🔢 Enter number of commits: ");
        match input.parse() {
            Ok(num) => break num,
            Err(_) => println!("❌ Invalid number! Please enter a valid integer."),
        }
    };

    Contribution { username, repo, commits }
}

fn list_contributions(contributions: &Vec<Contribution>) {
    if contributions.is_empty() {
        println!("📭 No contributions recorded yet.");
    } else {
        println!("\n📋 Recorded Contributions:");
        println!("----------------------------------");
        println!("{:<3} {:<15} {:<20} {:<8}", "#", "Username", "Repository", "Commits");
        println!("----------------------------------");

        for (i, c) in contributions.iter().enumerate() {
            println!(
                "{:<3} {:<15} {:<20} {:<8}",
                i + 1,
                c.username,
                c.repo,
                c.commits
            );
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn save_contribution(contrib: &Contribution) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("contributions.txt")
        .expect("❌ Failed to open file");

    writeln!(file, "{},{},{}", contrib.username, contrib.repo, contrib.commits)
        .expect("❌ Failed to write to file");
}

fn load_contributions() -> Vec<Contribution> {
    let mut contributions = Vec::new();
    if let Ok(contents) = fs::read_to_string("contributions.txt") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                if let Ok(commits) = parts[2].parse() {
                    contributions.push(Contribution {
                        username: parts[0].to_string(),
                        repo: parts[1].to_string(),
                        commits,
                    });
                }
            }
        }
    }
    contributions
}

fn delete_all_contributions() {
    match fs::remove_file("contributions.txt") {
        Ok(_) => println!("🗑️ All contributions deleted successfully."),
        Err(_) => println!("⚠️ No contributions found to delete."),
    }
}
