use std::io;

struct Contribution {
    username: String,
    repo: String,
    commits: u32,
}

fn main() {
    let mut contributions: Vec<Contribution> = Vec::new();

    loop {
        println!("\nOpen Source Contribution Tracker");
        println!("1. Add Contribution");
        println!("2. List Contributions");
        println!("3. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                let contrib = add_contribution();
                contributions.push(contrib);
                println!("✅ Contribution added!");
            }
            "2" => list_contributions(&contributions),
            "3" => {
                println!("Exiting... 👋");
                break;
            }
            _ => println!("❌ Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}

fn add_contribution() -> Contribution {
    let username = get_input("Enter GitHub username: ");
    let repo = get_input("Enter repository name: ");
    let commits: u32 = get_input("Enter number of commits: ")
        .parse()
        .expect("Please enter a valid number");

    Contribution { username, repo, commits }
}

fn list_contributions(contributions: &Vec<Contribution>) {
    if contributions.is_empty() {
        println!("No contributions recorded yet.");
    } else {
        println!("\nRecorded Contributions:");
        for (i, c) in contributions.iter().enumerate() {
            println!(
                "{}. {} has made {} commits to {}",
                i + 1, c.username, c.commits, c.repo
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
