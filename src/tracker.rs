use std::fs::File;
use std::io::{self, Read, Write};
use serde_json;
use crate::contribution::Contribution;

pub struct Tracker {
    contributions: Vec<Contribution>,
}

impl Tracker {
    pub fn new() -> Self {
        let mut tracker = Tracker {
            contributions: Vec::new(),
        };
        tracker.load_contributions().ok(); // Try to load contributions
        tracker
    }

    // Add a contribution
    pub fn add_contribution(&mut self, user: &str, project: &str, contribution_type: &str) {
        let contribution = Contribution::new(user, project, contribution_type);
        self.contributions.push(contribution);
        self.save_contributions().unwrap();
    }

    // List all contributions
    pub fn list_contributions(&self) {
        if self.contributions.is_empty() {
            println!("No contributions yet.");
        } else {
            for contribution in &self.contributions {
                println!(
                    "User: {} | Project: {} | Contribution: {}",
                    contribution.user, contribution.project, contribution.contribution_type
                );
            }
        }
    }

    // Save contributions to a JSON file
    pub fn save_contributions(&self) -> io::Result<()> {
        let file = File::create("contributions.json")?;
        serde_json::to_writer(file, &self.contributions)?;
        Ok(())
    }

    // Load contributions from a JSON file
    pub fn load_contributions(&mut self) -> io::Result<()> {
        let mut file = File::open("contributions.json")?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        self.contributions = serde_json::from_str(&data)?;
        Ok(())
    }
}
