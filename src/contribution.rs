use crate::repository::Repository;
use crate::commit::Commit;

pub struct Contribution {
    pub contribution_id: String,
    pub date: String, // For simplicity
    pub commit_count: usize,
    pub repository_name: String,
    pub commits: Vec<Commit>,
}

impl Contribution {
    pub fn log_commit(&mut self, commit: Commit) {
        self.commits.push(commit);
        self.commit_count += 1;
    }

    pub fn calculate_contributions(&self) -> usize {
        self.commit_count
    }

    pub fn display_summary(&self) {
        println!(
            "Contribution ID: {}, Repo: {}, Commits: {}",
            self.contribution_id, self.repository_name, self.commit_count
        );
    }
}
