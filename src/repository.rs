pub struct Repository {
    pub repo_name: String,
    pub description: String,
    pub owner: String,
    pub language: String,
    pub contributors: Vec<String>,
}

impl Repository {
    pub fn add_contributors(&mut self, contributor: String) {
        self.contributors.push(contributor);
    }

    pub fn get_contributors(&self) -> Vec<String> {
        self.contributors.clone()
    }
}

