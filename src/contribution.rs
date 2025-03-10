use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contribution {
    pub username: String,
    pub repo_name: String,
    pub commits: u32,
}

impl Contribution {
    pub fn new(username: String, repo_name: String, commits: u32) -> Self {
        Self {
            username,
            repo_name,
            commits,
        }
    }
}
