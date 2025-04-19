pub struct Commit {
    pub commit_id: String,
    pub message: String,
    pub date: String,
    pub changes: String,
}

impl Commit {
    pub fn get_change_summary(&self) -> String {
        format!("{} - {}", self.date, self.message)
    }

    pub fn display_commit_details(&self) {
        println!(
            "Commit ID: {}\nMessage: {}\nDate: {}\nChanges: {}",
            self.commit_id, self.message, self.date, self.changes
        );
    }
}

