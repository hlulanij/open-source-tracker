use crate::models::Contribution;

pub struct Tracker {
    contributions: Vec<Contribution>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            contributions: Vec::new(),
        }
    }

    pub fn add_contribution(&mut self, contribution: Contribution) {
        self.contributions.push(contribution);
    }

    pub fn list_contributions(&self) {
        for contrib in &self.contributions {
            contrib.display();
        }
    }
}
