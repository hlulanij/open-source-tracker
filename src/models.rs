#[derive(Debug)]
pub struct Contribution {
    pub username: String,
    pub repo: String,
    pub commits: u32,
}

impl Contribution {
    pub fn new(username: &str, repo: &str, commits: u32) -> Self {
        Self {
            username: username.to_string(),
            repo: repo.to_string(),
            commits,
        }
    }

    pub fn display(&self) {
        println!(
            "{} has made {} commits to {}",
            self.username, self.commits, self.repo
        );
    }
}
