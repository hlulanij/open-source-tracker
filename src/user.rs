use crate::contribution::Contribution;

pub struct User {
    pub username: String,
    pub email: String,
    pub contributions: Vec<Contribution>,
}

impl User {
    pub fn add_contribution(&mut self, contribution: Contribution) {
        self.contributions.push(contribution);
    }

    pub fn view_contributions(&self) {
        for c in &self.contributions {
            c.display_summary();
        }
    }

    pub fn update_profile(&mut self, new_email: String) {
        self.email = new_email;
    }
}

