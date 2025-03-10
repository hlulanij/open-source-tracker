use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contribution {
    pub user: String,
    pub project: String,
    pub contribution_type: String,
}

impl Contribution {
    pub fn new(user: &str, project: &str, contribution_type: &str) -> Self {
        Contribution {
            user: user.to_string(),
            project: project.to_string(),
            contribution_type: contribution_type.to_string(),
        }
    }
}
